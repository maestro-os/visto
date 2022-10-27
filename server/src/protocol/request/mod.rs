//! This module implements each requests of the X protocol.

pub mod query_extension;

use query_extension::QueryExtension;

use crate::ctx::client::Client;
use crate::protocol::XRequest;
use crate::util;
use std::error::Error;
use std::mem::size_of;
use super::opcode::Opcode;

/// The maximum length of a request in bytes.
pub const MAX_REQUEST_LEN: usize = 1 << 16; // TODO Increase?

/// Trait representing a request.
pub trait Request {
	/// Parses the request from the given buffer.
	fn read(buff: &[u8]) -> Result<Option<Self>, Box<dyn Error>> where Self: Sized;

	/// Handles the request for the given client.
	fn handle(&self, client: &mut Client) -> Result<(), Box<dyn Error>>;
}

/// Reads a request from the given buffer.
/// If not enough data is present in the buffer, the function returns None.
pub fn read(buff: &[u8]) -> Result<Option<(Box<dyn Request>, usize)>, Box<dyn Error>> {
	// If not enough bytes are available, return
	let req = size_of::<XRequest>();
	if buff.len() < req {
		return Ok(None);
	}

	let hdr: &XRequest = unsafe {
		util::reinterpret(&buff[0])
	};
	// Required number of bytes
	let req = hdr.length as usize * 4;

	// If the request is too long, ignore it
	if req > MAX_REQUEST_LEN {
		// TODO
		todo!();
	}
	// If not enough bytes are available, return
	if buff.len() < req {
		return Ok(None);
	}

	// TODO rm
	println!("=> {}", hdr.major_opcode);

	let opcode = Opcode::from_id(hdr.major_opcode);
	let buff = &buff[size_of::<XRequest>()..];

	let request = match opcode {
		// TODO
		Some(Opcode::QueryExtension) => QueryExtension::read(buff)?
			.map(|r| Box::new(r) as Box<dyn Request>),

		_ => {
			// TODO Add support for extensions
			// TODO Handle invalid opcodes

			todo!();
		},
	};

	Ok(request.map(|r| (r, req)))
}
