//! TODO doc

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::error::Error;
use crate::util;
use std::mem::size_of;
use super::Request;
use super::create_window;

/// Header of the `ChangeWindowAttributes` request.
#[repr(C, packed)]
pub struct ChangeWindowAttributesHdr {
	/// The window.
	window: u32,

	/// The mask of attributes being changed.
	value_mask: u32,
}

/// Structure representing the request.
pub struct ChangeWindowAttributes {
	/// The window.
	window: u32,

	/// The list of attributes to change.
	changed_attrs: Vec<create_window::AttrValue>,
}

impl Request for ChangeWindowAttributes {
	fn handle(
		&self,
		ctx: &mut Context,
		_client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), Box<dyn std::error::Error>> {
		let win = ctx.get_window_mut(self.window)
			.ok_or(Box::new(Error::Window(self.window)))?;
		create_window::set_attrs(&mut win.attributes, &self.changed_attrs);

		Ok(())
	}
}

/// Parses `ChangeWindowAttributes`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<ChangeWindowAttributesHdr>() {
		return Ok(None);
	}

	let hdr: &ChangeWindowAttributesHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	let attrs_buff = &buff[size_of::<ChangeWindowAttributesHdr>()..];
	let changed_attrs = create_window::read_attrs(hdr.value_mask, attrs_buff)?;

	Ok(Some(Box::new(ChangeWindowAttributes {
		window: hdr.window,

		changed_attrs,
	})))
}
