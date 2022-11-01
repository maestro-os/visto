//! TODO doc

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::ctx::window::WindowAttributes;
use crate::protocol::BackingStore;
use crate::protocol::BitGravity;
use crate::protocol::Class;
use crate::protocol::WinGravity;
use crate::protocol::error::Error;
use crate::util;
use std::mem::size_of;
use super::Request;

/// Enumeration of window attribute values read from a request.
pub enum AttrValue {
	BackgroundPixmap(u32),
	BackgroundPixel(u32),
	BorderPixmap(u32),
	BorderPixel(u32),
	BitGravity(BitGravity),
	WinGravity(WinGravity),
	BackingStore(BackingStore),
	BackingPlanes(u32),
	BackingPixel(u32),
	OverrideRedirect(bool),
	SaveUnder(bool),
	EventMask(u32),
	DoNotPropagateMask(u32),
	Colormap(u32),
	Cursor(u32),
}

/// Header of the `CreateWindow` request.
#[repr(C, packed)]
pub struct CreateWindowHdr {
	/// The ID of the window.
	wid: u32,
	/// The ID of window's parent.
	parent: u32,
	/// The X position of the window.
	x: i16,
	/// The Y position of the window.
	y: i16,
	/// The width of the window.
	width: u16,
	/// The height of the window.
	height: u16,
	/// The width of the window's border.
	border_width: u16,
	/// TODO doc
	class: Class,
	/// TODO doc
	visual: u32,
	/// The mask of attributes being set.
	value_mask: u32,
}

/// Structure representing the request.
pub struct CreateWindow {
	/// The ID of the window.
	wid: u32,
	/// The ID of window's parent.
	parent: u32,
	/// The X position of the window.
	x: i16,
	/// The Y position of the window.
	y: i16,
	/// The width of the window.
	width: u16,
	/// The height of the window.
	height: u16,
	/// The width of the window's border.
	border_width: u16,
	/// TODO doc
	class: Class,
	/// TODO doc
	visual: u32,

	/// TODO doc
	depth: u8,

	/// The list of attributes being set.
	attrs: Vec<AttrValue>,
}

impl Request for CreateWindow {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), Box<dyn std::error::Error>> {
		let mut attr = WindowAttributes::default();
		set_attrs(&mut attr, &self.attrs);

		// TODO
		todo!();
	}
}

/// Reads window attributes from a request. The function returns the list of attributes.
///
/// Arguments:
/// - `bitmask` is the bitmask of attributes to read.
/// - `buff` is the buffer containing the attributes.
pub fn read_attrs(bitmask: u32, buff: &[u8]) -> Result<Vec<AttrValue>, Error> {
	let mut values = vec![];

	let mut off = 0;
	let set_bits_iter = (0..=14).filter(|i| bitmask & (1 << i) != 0);
	for id in set_bits_iter {
		let size = match id {
			0 => 4,
			1 => 4,
			2 => 4,
			3 => 4,
			4 => 1,
			5 => 1,
			6 => 1,
			7 => 4,
			8 => 4,
			9 => 1,
			10 => 1,
			11 => 4,
			12 => 4,
			13 => 4,
			14 => 4,

			_ => unreachable!(),
		};

		let val = match size {
			1 => unsafe { *util::reinterpret::<_, u8>(&buff[off]) as u32 },
			2 => unsafe { *util::reinterpret::<_, u16>(&buff[off]) as u32 },
			4 => unsafe { *util::reinterpret::<_, u32>(&buff[off]) as u32 },

			_ => unreachable!(),
		};
		off += size;

		let val = match id {
			0 => AttrValue::BackgroundPixmap(val),
			1 => AttrValue::BackgroundPixel(val),
			2 => AttrValue::BorderPixmap(val),
			3 => AttrValue::BorderPixel(val),
			4 => AttrValue::BitGravity((val as u8).try_into()?),
			5 => AttrValue::WinGravity((val as u8).try_into()?),
			6 => AttrValue::BackingStore((val as u8).try_into()?),
			7 => AttrValue::BackingPlanes(val),
			8 => AttrValue::BackingPixel(val),
			9 => AttrValue::OverrideRedirect(val != 0),
			10 => AttrValue::SaveUnder(val != 0),
			11 => AttrValue::EventMask(val),
			12 => AttrValue::DoNotPropagateMask(val),
			13 => AttrValue::Colormap(val),
			14 => AttrValue::Cursor(val),

			_ => unreachable!(),
		};

		values.push(val);
	}

	Ok(values)
}

/// Sets the given attributes list on the given attributes structure.
pub fn set_attrs(attrs: &mut WindowAttributes, list: &[AttrValue]) {
	for a in list {
		match a {
			AttrValue::BackgroundPixmap(val) => attrs.background_pixmap = *val,
			AttrValue::BackgroundPixel(val) => attrs.background_pixel = *val,
			AttrValue::BorderPixmap(val) => attrs.border_pixmap = *val,
			AttrValue::BorderPixel(val) => attrs.border_pixel = *val,
			AttrValue::BitGravity(val) => attrs.bit_gravity = *val,
			AttrValue::WinGravity(val) => attrs.win_gravity = *val,
			AttrValue::BackingStore(val) => attrs.backing_store = *val,
			AttrValue::BackingPlanes(val) => attrs.backing_planes = *val,
			AttrValue::BackingPixel(val) => attrs.backing_pixel = *val,
			AttrValue::OverrideRedirect(val) => attrs.override_redirect = *val,
			AttrValue::SaveUnder(val) => attrs.save_under = *val,
			AttrValue::EventMask(val) => attrs.event_mask = *val,
			AttrValue::DoNotPropagateMask(val) => attrs.do_not_propagate_mask = *val,
			AttrValue::Colormap(val) => attrs.colormap = *val,
			AttrValue::Cursor(val) => attrs.cursor = *val,
		}
	}
}

/// Parses `CreateWindow`.
///
/// TODO doc: depth
pub fn read(buff: &[u8], depth: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<CreateWindowHdr>() {
		return Ok(None);
	}

	let hdr: &CreateWindowHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	let attrs_buff = &buff[size_of::<CreateWindowHdr>()..];
	let attrs = read_attrs(hdr.value_mask, attrs_buff)?;

	Ok(Some(Box::new(CreateWindow {
		wid: hdr.wid,
		parent: hdr.parent,
		x: hdr.x,
		y: hdr.y,
		width: hdr.width,
		height: hdr.height,
		border_width: hdr.border_width,
		class: hdr.class,
		visual: hdr.visual,

		depth,

		attrs,
	})))
}
