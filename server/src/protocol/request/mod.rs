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

/// Trait representing an object used to read a request.
pub trait RequestReader {
	/// Reads a request from the given buffer.
	/// If not enough data is present in the buffer, the function returns None.
	fn read(&self, buff: &[u8]) -> Result<Option<(Box<dyn Request>, usize)>, Box<dyn Error>>;

	/// TODO doc
	fn handle(&self, opcode: u8, buff: &[u8]) -> Result<Option<Box<dyn Request>>, Box<dyn Error>> {
		// TODO rm
		println!("=> {}", opcode);

		let request = match Opcode::from_id(opcode) {
			// TODO
			Some(Opcode::QueryExtension) => QueryExtension::read(buff)?
				.map(|r| Box::new(r) as Box<dyn Request>),

			_ => {
				// TODO Add support for extensions
				// TODO Handle invalid opcodes

				todo!();
			}
		};

		Ok(request)
	}
}

/// The default request reader.
pub struct DefaultRequestReader {}

impl RequestReader for DefaultRequestReader {
	fn read(&self, buff: &[u8]) -> Result<Option<(Box<dyn Request>, usize)>, Box<dyn Error>> {
		// If not enough bytes are available, return
		let hdr_len = size_of::<XRequest>();
		if buff.len() < hdr_len {
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

		let opcode = hdr.major_opcode;
		let buff = &buff[hdr_len..];

		let request = self.handle(opcode, buff)?;
		Ok(request.map(|r| (r, req)))
	}
}
