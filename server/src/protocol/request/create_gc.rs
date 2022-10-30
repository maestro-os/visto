//! The `CreateGC` request allows to create a graphics context.

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::gc::GC;
use crate::gc;
use crate::util;
use std::error::Error;
use std::mem::size_of;
use super::Request;

/// TODO doc
#[repr(C, packed)]
pub struct CreateGCHdr {
	/// The graphics context's ID.
	cid: u32,
	/// The ID of the drawable.
	drawable: u32,

	/// TODO
	bitmask: u32,
}

/// Value of a graphics context.
#[repr(C, packed)]
pub struct GCValue {
	pub function: u8,
	pub plane_mask: u32,
	pub foreground: u32,
	pub background: u32,
	pub line_width: u16,
	pub line_style: u8,
	pub cap_style: u8,
	pub join_style: u8,
	pub fill_style: u8,
	pub fill_rule: u8,
	pub tile: u32,
	pub stipple: u32,
	pub tile_stipple_x_origin: i16,
	pub tile_stipple_y_origin: i16,
	pub font: u32,
	pub subwindow_mode: u8,
	pub graphics_exposures: u8,
	pub clip_x_origin: i16,
	pub clip_y_origin: i16,
	pub clip_mask: u32,
	pub dash_offset: u16,
	pub dashes: u8,
	pub arc_mode: u8,
}

/// Structure representing the request.
pub struct CreateGC {
	/// The graphics context's ID.
	cid: u32,
	/// The graphics context.
	gc: GC,
}

impl Request for CreateGC {
	fn handle(
		&self,
		_ctx: &mut Context,
		client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), Box<dyn Error>> {
		client.set_gc(self.cid, self.gc.clone());
		Ok(())
	}
}

/// Parses `CreateGC`.
pub fn read(buff: &[u8]) -> Result<Option<Box<dyn Request>>, Box<dyn Error>> {
	if buff.len() < size_of::<CreateGCHdr>() {
		return Ok(None);
	}

	let hdr: &CreateGCHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	let mut values = vec![];

	let values_start = size_of::<CreateGCHdr>();
	for off in (values_start..buff.len()).step_by(size_of::<gc::Value>()) {
		let val: &GCValue = unsafe {
			util::reinterpret(&buff[off])
		};

		values.push(gc::Value {
			function: val.function.try_into()?,
			plane_mask: val.plane_mask,
			foreground: val.foreground,
			background: val.background,
			line_width: val.line_width,
			line_style: val.line_style.try_into()?,
			cap_style: val.cap_style.try_into()?,
			join_style: val.join_style.try_into()?,
			fill_style: val.fill_style.try_into()?,
			fill_rule: val.fill_rule.try_into()?,
			tile: val.tile,
			stipple: val.stipple,
			tile_stipple_x_origin: val.tile_stipple_x_origin,
			tile_stipple_y_origin: val.tile_stipple_y_origin,
			font: val.font,
			subwindow_mode: val.subwindow_mode.try_into()?,
			graphics_exposures: val.graphics_exposures,
			clip_x_origin: val.clip_x_origin,
			clip_y_origin: val.clip_y_origin,
			clip_mask: val.clip_mask,
			dash_offset: val.dash_offset,
			dashes: val.dashes,
			arc_mode: val.arc_mode.try_into()?,
		});
	}

	let gc = GC {
		drawable: hdr.drawable,

		bitmask: hdr.bitmask,

		values,
	};

	Ok(Some(Box::new(CreateGC {
		cid: hdr.cid,
		gc,
	})))
}
