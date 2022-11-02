//! TODO doc

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::BackingStore;
use crate::protocol::BitGravity;
use crate::protocol::Class;
use crate::protocol::MapState;
use crate::protocol::WinGravity;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;
use crate::protocol;
use crate::util;
use std::mem::size_of;
use super::Request;

/// The header of the request's reply.
#[repr(C, packed)]
pub struct GetWindowAttributesReply {
	/// TODO doc
	reply_type: u8,
	/// TODO doc
	backing_store: BackingStore,
	/// The sequence number of the request associated with the reply.
	seq_nbr: u16,
	/// The length of the reply in units of 4 bytes.
	reply_length: u32,
	/// TODO doc
	visual: u32,
	/// TODO doc
	class: Class,
	/// TODO doc
	bit_gravity: BitGravity,
	/// TODO doc
	win_gravity: WinGravity,
	/// TODO doc
	backing_places: u32,
	/// TODO doc
	backing_pixel: u32,
	/// TODO doc
	save_under: u8,
	/// TODO doc
	map_is_installed: u8,
	/// TODO doc
	map_state: MapState,
	/// TODO doc
	override_redirect: u8,
	/// TODO doc
	colormap: u32,
	/// TODO doc
	all_event_masks: u32,
	/// TODO doc
	your_event_mask: u32,
	/// TODO doc
	do_not_propagate_mask: u16,
	/// Padding.
	_padding: [u8; 2],
}

/// Header of the `GetWindowAttributes` request.
#[repr(C, packed)]
pub struct GetWindowAttributesHdr {
	/// The window.
	window: u32,
}

/// Structure representing the request.
pub struct GetWindowAttributes {
	/// The window.
	window: u32,
}

impl Request for GetWindowAttributes {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), HandleError> {
		let win = ctx.get_window_mut(self.window)
			.ok_or(HandleError::Client(Error::Window(self.window)))?;

		let hdr = GetWindowAttributesReply {
			reply_type: protocol::REPLY_TYPE_REPLY,
			backing_store: win.attributes.backing_store,
			seq_nbr,
			reply_length: 3,
			visual: win.attributes.visual,
			class: win.attributes.class,
			bit_gravity: win.attributes.bit_gravity,
			win_gravity: win.attributes.win_gravity,
			backing_places: win.attributes.backing_places,
			backing_pixel: win.attributes.backing_pixel,
			save_under: if win.attributes.save_under { 1 } else { 0 },
			map_is_installed: win.attributes.map_is_installed,
			map_state: win.attributes.map_state,
			override_redirect: if win.attributes.override_redirect { 1 } else { 0 },
			colormap: win.attributes.colormap,
			all_event_masks: win.attributes.event_mask,
			your_event_mask: 0, // TODO
			do_not_propagate_mask: win.attributes.do_not_propagate_mask as _,
			_padding: [0; 2],
		};
		client.write_obj(&hdr)
			.map_err(|e| HandleError::IO(e))?;

		Ok(())
	}
}

/// Parses `GetWindowAttributes`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<GetWindowAttributesHdr>() {
		return Ok(None);
	}

	let hdr: &GetWindowAttributesHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	Ok(Some(Box::new(GetWindowAttributes {
		window: hdr.window,
	})))
}
