//! The `QueryExtension` request allows to ask for an extention to be loaded.

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::extension;
use crate::protocol::pad;
use crate::protocol;
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
	/// The type of the reply (normal).
	reply_type: u8,
	/// Padding.
	_padding0: u8,
	/// Sequence number.
	seq_nbr: u16,
	/// The length of the reply.
	reply_length: u32,

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

/// Structure representing the request.
pub struct QueryExtension {
	/// The name of the extension.
	name: String,
}

impl Request for QueryExtension {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), Box<dyn Error>> {
		let ext = extension::query(ctx, &self.name).unwrap_or_else(|e| {
			eprintln!("Couldn't load extension `{}`: {}", self.name, e);
			None
		});
		let present = ext.is_some();

		let (
			major_opcode,
			first_event,
			first_error,
		) = match ext {
			Some(ext) => {
				let ext = ext.lock().unwrap();

				(
					ext.get_major_opcode(),
					ext.get_first_event(),
					ext.get_first_error(),
				)
			},

			None => (0, 0, 0),
		};

		let reply = QueryExtensionReply {
			reply_type: protocol::REPLY_TYPE_REPLY,
			_padding0: 0,
			seq_nbr,
			reply_length: 0,

			present: if present { 1 } else { 0 },
			major_opcode,
			first_event,
			first_error,

			_padding1: [0; 20],
		};
		client.write_reply(&reply)?;

		Ok(())
	}
}

/// Parses `QueryExtension`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Box<dyn Error>> {
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

	Ok(Some(Box::new(QueryExtension {
		name: String::from_str(name).unwrap(),
	})))
}
