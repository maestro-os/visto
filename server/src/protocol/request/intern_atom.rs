//! TODO doc

use super::Request;
use crate::ctx::client::Client;
use crate::ctx::Context;
use crate::protocol;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;
use crate::util;
use std::mem::size_of;
use std::str;
use std::str::FromStr;

/// The header of the request's reply.
#[repr(C, packed)]
pub struct InternAtomReply {
	/// TODO doc
	reply_type: u8,
	/// Padding.
	_padding0: u8,
	/// The sequence number of the associated request.
	seq_nbr: u16,
	/// The length of the reply in units of 4 bytes.
	reply_length: u32,
	/// The atom.
	atom: u32,
	/// Padding.
	_padding1: [u8; 20],
}

/// Header of the `InternAtom` request.
#[repr(C, packed)]
pub struct InternAtomHdr {
	/// The length of the atom's name.
	name_length: u16,
	/// Padding.
	_padding: [u8; 2],
}

/// Structure representing the request.
pub struct InternAtom {
	/// The name of the atom.
	name: String,

	/// If false, the atom is created if it doesn't exist.
	only_if_exists: bool,
}

impl Request for InternAtom {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), HandleError> {
		let atom = match ctx.get_atom_from_name(&self.name) {
			Some(atom) => atom,
			None if !self.only_if_exists => ctx.create_atom(self.name.clone()),
			None => 0,
		};

		let hdr = InternAtomReply {
			reply_type: protocol::REPLY_TYPE_REPLY,
			_padding0: 0,
			seq_nbr,
			reply_length: 0,
			atom,
			_padding1: [0; 20],
		};
		client.write_obj(&hdr).map_err(|e| HandleError::IO(e))?;

		Ok(())
	}
}

/// Parses `InternAtom`.
///
/// If `only_if_exists` is zero, the atom is created if it doesn't exist.
pub fn read(buff: &[u8], only_if_exists: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<InternAtomHdr>() {
		return Ok(None);
	}

	let hdr: &InternAtomHdr = unsafe { util::reinterpret(&buff[0]) };

	if buff.len() < size_of::<InternAtomHdr>() + hdr.name_length as usize {
		return Ok(None);
	}

	let len = size_of::<InternAtomHdr>()
		+ hdr.name_length as usize
		+ protocol::pad(hdr.name_length as usize);
	if buff.len() < len {
		return Ok(None);
	}

	let name_begin = size_of::<InternAtomHdr>();
	let name_end = name_begin + hdr.name_length as usize;
	let name = str::from_utf8(&buff[name_begin..name_end]).unwrap(); // TODO Handle error

	Ok(Some(Box::new(InternAtom {
		name: String::from_str(name).unwrap(),

		only_if_exists: only_if_exists != 0,
	})))
}
