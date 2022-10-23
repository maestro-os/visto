//! This module implements each requests of the X protocol.

pub mod query_extention;

use crate::ctx::client::Client;
use crate::protocol::XRequest;
use crate::util;
use std::error::Error;
use std::io;
use std::mem::size_of;
use super::opcode::Opcode;

/// The maximum length of a request in bytes.
pub const MAX_REQUEST_LEN: usize = 1 << 16; // TODO Increase?

/// Trait representing a request.
pub trait Request {
	/// Handles the request for the given client.
	fn handle(&self, client: &mut Client) -> Result<(), Box<dyn Error>>;
}

/// Reads a request from the given buffer.
/// If not enough data is present in the buffer, the function returns None.
pub fn read(buff: &[u8]) -> io::Result<Option<(Box<dyn Request>, usize)>> {
	// If not enough bytes are available, return
	let req = size_of::<XRequest>();
	if buff.len() < req {
		return Ok(None);
	}

	let hdr: &XRequest = unsafe {
		util::reinterpret(&buff[0])
	};
	// If not enough bytes are available, return
	let req = hdr.length as usize * 4;
	if buff.len() < req {
		return Ok(None);
	}
	// If the request is too long, ignore
	if buff.len() > MAX_REQUEST_LEN {
		// TODO
		todo!();
	}

	// TODO rm
	println!("=> {}", hdr.major_opcode);

	let opcode = Opcode::from_id(hdr.major_opcode);

	match opcode {
		// TODO

		_ => {
			// TODO Add support for extentions
			// TODO Handle invalid opcodes

			todo!();
		},
	}
}
