//! TODO doc

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::error::Error;
use crate::util;
use std::mem::size_of;
use super::Request;

/// Header of the `GetProperty` request.
#[repr(C, packed)]
pub struct GetPropertyHdr {
	/// The window to get the property from.
	window: u32,
	/// The property.
	property: u32,
	/// TODO doc
	atom_type: u32,
	/// TODO doc
	long_offset: u32,
	/// TODO doc
	long_length: u32,
}

/// Structure representing the request.
pub struct GetProperty {
	/// The window to get the property from.
	window: u32,
	/// The property.
	property: u32,

	// TODO

	/// Tells whether the property must be deleted.
	delete: bool,
}

impl Request for GetProperty {
	fn handle(
		&self,
		_ctx: &mut Context,
		client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), Box<dyn std::error::Error>> {
		// TODO
		Ok(())
	}
}

/// Parses `GetProperty`.
///
/// If `delete` is nonzero, the function deletes the property from the window.
pub fn read(buff: &[u8], delete: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<GetPropertyHdr>() {
		return Ok(None);
	}

	let hdr: &GetPropertyHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	Ok(Some(Box::new(GetProperty {
		window: hdr.window,
		property: hdr.property,

		delete: delete != 0,
	})))
}
