//! TODO doc

use super::Request;
use crate::ctx::client::Client;
use crate::ctx::Context;
use crate::protocol;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;
use crate::util;
use std::mem::size_of;
use std::num::NonZeroU32;

/// The reply.
#[repr(C, packed)]
struct QueryPointerReply {
	/// The type of the reply (normal).
	reply_type: u8,
	/// Tells whether the pointer is located on the same screen as the given window.
	same_screen: u8,
	/// Sequence number.
	seq_nbr: u16,
	/// The length of the reply in units of 4 bytes.
	reply_length: u32,

	/// The root window on which the pointer is located.
	root: u32,
	/// The child window on which the pointer is located.
	child: u32,

	/// The X position of the pointer relative to the root window.
	root_x: i16,
	/// The Y position of the pointer relative to the root window.
	root_y: i16,
	/// The X position of the pointer relative to the given window.
	win_x: i16,
	/// The Y position of the pointer relative to the given window.
	win_y: i16,

	/// TODO doc
	mask: u16,

	/// Padding.
	_padding: [u8; 6],
}

/// The header of the request.
#[repr(C, packed)]
struct QueryPointerHdr {
	/// The window.
	window: u32,
}

/// Structure representing the request.
pub struct QueryPointer {
	/// The window.
	window: u32,
}

impl Request for QueryPointer {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), HandleError> {
		let wid =
			NonZeroU32::new(self.window).ok_or(HandleError::Client(Error::Window(self.window)))?;
		let _win = ctx
			.get_window_mut(wid)
			.ok_or(HandleError::Client(Error::Window(self.window)))?;

		let hdr = QueryPointerReply {
			reply_type: protocol::REPLY_TYPE_REPLY,
			same_screen: 1, // TODO
			seq_nbr,
			reply_length: 0,

			root: 1,  // TODO
			child: 0, // TODO

			root_x: 0, // TODO
			root_y: 0, // TODO
			win_x: 0,  // TODO
			win_y: 0,  // TODO

			mask: 0, // TODO

			_padding: [0; 6],
		};
		client.write_obj(&hdr).map_err(|e| HandleError::IO(e))?;

		Ok(())
	}
}

/// Parses `QueryPointer`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<QueryPointerHdr>() {
		return Ok(None);
	}
	let hdr: &QueryPointerHdr = unsafe { util::reinterpret(&buff[0]) };

	Ok(Some(Box::new(QueryPointer {
		window: hdr.window,
	})))
}
