//! The QueryExtension request allows to ask for an extention to be loaded.

use crate::ctx::client::Client;
use crate::extension;
use crate::protocol::pad;
use crate::util;
use std::error::Error;
use std::mem::size_of;
use std::str::FromStr;
use std::str;
use super::Request;

/// The header of the request.
#[repr(C, packed)]
struct QueryExtensionHdr {
	/// The length of the name of the extention.
	name_length: u16,

	/// Padding.
	_padding: u16,
}

/// The reply.
#[repr(C, packed)]
struct QueryExtensionReply {
	/// TODO doc
	type_: u8,
	/// Padding.
	_padding0: u8,
	/// Sequence number.
	seq_nbr: u16,
	/// The length of the reply.
	reply_length: u32, // TODO Check if in units of 4 bytes

	/// Tells whether the extension is present.
	present: u8,
	/// The major opcode of the extension.
	major_opcode: u8,
	/// The first event of the extension.
	first_event: u8,
	/// The first error of the extension.
	first_error: u8,

	/// Padding.
	_padding1: [u8; 20],
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
		let seq_nbr = client.next_sequence_number();
		let present = extension::query(&self.name)?;

		let reply = QueryExtensionReply {
			type_: 1, // TODO Use constant
			_padding0: 0,
			seq_nbr,
			// TODO Check if in units of 4 bytes
			reply_length: size_of::<QueryExtensionReply>() as _,

			present: if present { 1 } else { 0 },
			major_opcode: 0, // TODO Allocated for extension
			first_event: 0, // TODO Allocated for extension
			first_error: 0, // TODO Allocated for extension

			_padding1: [0; 20],
		};
		client.write(&reply)?;

		Ok(())
	}
}
