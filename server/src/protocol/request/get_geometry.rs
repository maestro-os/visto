//! TODO doc

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;
use crate::protocol;
use crate::util;
use std::mem::size_of;
use std::num::NonZeroU32;
use super::Request;

/// The header of the request's reply.
#[repr(C, packed)]
pub struct GetGeometryReply {
	/// TODO doc
	reply_type: u8,
	/// TODO doc
	depth: u8,
	/// The sequence number associated with the request.
	seq_nbr: u16,
	/// The length of the reply in units of 4 bytes.
	reply_length: u32,
	/// The root window.
	root: u32,
	/// The X position.
	x: i16,
	/// The Y position.
	y: i16,
	/// The width.
	width: u16,
	/// The height.
	height: u16,
	/// The width of the border.
	border_width: u16,
	/// Padding.
	_padding: [u8; 10],
}

/// Header of the `GetGeometry` request.
#[repr(C, packed)]
pub struct GetGeometryHdr {
	/// The ID of the drawable.
	drawable: u32,
}

/// Structure representing the request.
pub struct GetGeometry {
	/// The ID of the drawable.
	drawable: u32,
}

impl Request for GetGeometry {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), HandleError> {
		let drawable = NonZeroU32::new(self.drawable)
			.ok_or(HandleError::Client(Error::Drawable(self.drawable)))?;
		let drawable = ctx.get_drawable(drawable)
			.ok_or(HandleError::Client(Error::Drawable(self.drawable)))?;
		let rect = drawable.get_rectangle();

		let reply = GetGeometryReply {
			reply_type: protocol::REPLY_TYPE_REPLY,
			depth: drawable.get_depth(),
			seq_nbr,
			reply_length: 0,
			root: drawable.get_root(),
			x: rect.x,
			y: rect.y,
			width: rect.width,
			height: rect.height,
			border_width: drawable.get_border_width(),
			_padding: [0; 10],
		};
		client.write_obj(&reply)
			.map_err(|e| HandleError::IO(e))?;

		Ok(())
	}
}

/// Parses `GetGeometry`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<GetGeometryHdr>() {
		return Ok(None);
	}

	let hdr: &GetGeometryHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	Ok(Some(Box::new(GetGeometry {
		drawable: hdr.drawable,
	})))
}
