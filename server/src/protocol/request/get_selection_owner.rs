//! TODO doc

use super::Request;
use crate::ctx::client::Client;
use crate::ctx::Context;
use crate::protocol;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;
use crate::util;
use std::mem::size_of;

/// The header of the request's reply.
#[repr(C, packed)]
pub struct GetSelectionOwnerReply {
	/// TODO doc
	reply_type: u8,
	/// Padding.
	_padding0: u8,
	/// The sequence number of the request associated with the reply.
	seq_nbr: u16,
	/// The length of the reply in units of 4 bytes.
	reply_length: u32,
	/// The ID of the owner window.
	owner: u32,
	/// Padding.
	_padding1: [u8; 20],
}

/// Header of the `GetSelectionOwner` request.
#[repr(C, packed)]
pub struct GetSelectionOwnerHdr {
	/// The atom representing the selection.
	atom: u32,
}

/// Structure representing the request.
pub struct GetSelectionOwner {
	/// The atom representing the selection.
	atom: u32,
}

impl Request for GetSelectionOwner {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), HandleError> {
		let selection_name = ctx
			.get_atom(self.atom)
			.ok_or(HandleError::Client(Error::Atom(self.atom)))?;
		let owner = ctx
			.get_selection(&selection_name)
			.map(|selection| selection.get_owner())
			.flatten()
			.map(|owner| owner.get())
			.unwrap_or(0);

		let hdr = GetSelectionOwnerReply {
			reply_type: protocol::REPLY_TYPE_REPLY,
			_padding0: 0,
			seq_nbr,
			reply_length: 0,
			owner,
			_padding1: [0; 20],
		};
		client.write_obj(&hdr).map_err(|e| HandleError::IO(e))?;

		Ok(())
	}
}

/// Parses `GetSelectionOwner`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<GetSelectionOwnerHdr>() {
		return Ok(None);
	}

	let hdr: &GetSelectionOwnerHdr = unsafe { util::reinterpret(&buff[0]) };

	Ok(Some(Box::new(GetSelectionOwner {
		atom: hdr.atom,
	})))
}
