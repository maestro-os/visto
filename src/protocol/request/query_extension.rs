//! The QueryExtension request allows to ask for an extention to be loaded.

use crate::ctx::client::Client;
use crate::protocol::pad;
use crate::util;
use std::error::Error;
use std::mem::size_of;
use std::str::FromStr;
use std::str;
use super::Request;

/// The header of the request.
struct QueryExtensionHdr {
	/// The length of the name of the extention.
	name_length: u16,

	/// Padding.
	_padding: u16,
}

/// Structurer representing the request.
pub struct QueryExtension {
	/// The name of the extension.
	name: String,
}

impl Request for QueryExtension {
	fn read(buff: &[u8]) -> Result<Option<Self>, Box<dyn Error>> {
		if buff.len() < size_of::<QueryExtensionHdr>() {
			return Ok(None);
		}
		let hdr: &QueryExtensionHdr = unsafe {
			util::reinterpret(&buff[0])
		};

		let len = size_of::<QueryExtensionHdr>()
			+ hdr.name_length as usize
			+ pad(hdr.name_length as usize);
		if buff.len() < len {
			return Ok(None);
		}

		let name_begin = size_of::<QueryExtensionHdr>();
		let name_end = name_begin + hdr.name_length as usize;
		let name = str::from_utf8(&buff[name_begin..name_end]).unwrap(); // TODO Handle error

		Ok(Some(Self {
			name: String::from_str(name).unwrap(),
		}))
	}

	fn handle(&self, client: &mut Client) -> Result<(), Box<dyn Error>> {
		println!("name: {}", self.name);
		// TODO
		todo!();
	}
}
