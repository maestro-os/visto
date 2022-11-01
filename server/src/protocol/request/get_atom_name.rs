//! TODO doc

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::error::Error;
use crate::protocol;
use crate::util;
use std::mem::size_of;
use super::Request;

/// The reply.
#[repr(C, packed)]
struct GetAtomNameReply {
	/// The type of the reply (normal).
	reply_type: u8,
	/// Padding.
	_padding0: u8,
	/// Sequence number.
	seq_nbr: u16,
	/// The length of the reply.
	reply_length: u32,

	/// The length of the name.
	name_length: u16,

	/// Padding.
	_padding1: [u8; 22],
}

/// The header of the request.
#[repr(C, packed)]
struct GetAtomNameHdr {
	/// The atom.
	atom: u32,
}

/// Structure representing the request.
pub struct GetAtomName {
	/// The atom.
	atom: u32,
}

impl Request for GetAtomName {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), Box<dyn std::error::Error>> {
		let atom = ctx.get_atom(self.atom)
			.ok_or(Box::new(Error::Atom(self.atom)))?;

		let len = atom.len();
		let pad = protocol::pad(len);
		let reply_length = ((len + pad) / 4) as _;

		// Write header
		let hdr = GetAtomNameReply {
			reply_type: protocol::REPLY_TYPE_REPLY,
			_padding0: 0,
			seq_nbr,
			reply_length,

			name_length: len as _,

			_padding1: [0; 22],
		};
		client.write_obj(&hdr)?;

		// Write name
		client.write(atom.as_bytes());

		// Write padding
		let pad: [u8; 4] = [0; 4];
		client.write(&pad[..protocol::pad(len)])?;

		Ok(())
	}
}

/// Parses `GetAtomName`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<GetAtomNameHdr>() {
		return Ok(None);
	}
	let hdr: &GetAtomNameHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	Ok(Some(Box::new(GetAtomName {
		atom: hdr.atom,
	})))
}
