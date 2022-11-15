//! The `CreateGC` request allows to create a graphics context.

use super::Request;
use crate::ctx::client::Client;
use crate::ctx::gc;
use crate::ctx::gc::GC;
use crate::ctx::Context;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;
use crate::util;
use std::mem::size_of;

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
	) -> Result<(), HandleError> {
		client.set_gc(self.cid, self.gc.clone());
		Ok(())
	}
}

/// Parses `CreateGC`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<CreateGCHdr>() {
		return Ok(None);
	}

	let hdr: &CreateGCHdr = unsafe { util::reinterpret(&buff[0]) };

	let mut values = vec![];

	let mut off = size_of::<CreateGCHdr>();
	let set_bits_iter = (0..=22).filter(|i| hdr.bitmask & (1 << i) != 0);
	for id in set_bits_iter {
		let size = match id {
			0 => 1,
			1 => 4,
			2 => 4,
			3 => 4,
			4 => 2,
			5 => 1,
			6 => 1,
			7 => 1,
			8 => 1,
			9 => 1,
			10 => 4,
			11 => 4,
			12 => 2,
			13 => 2,
			14 => 4,
			15 => 1,
			16 => 1,
			17 => 2,
			18 => 2,
			19 => 4,
			20 => 2,
			21 => 1,
			22 => 1,

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
			0 => gc::Value::Function((val as u8).try_into()?),
			1 => gc::Value::PlaneMask(val),
			2 => gc::Value::Foreground(val),
			3 => gc::Value::Background(val),
			4 => gc::Value::LineWidth(val as _),
			5 => gc::Value::LineStyle((val as u8).try_into()?),
			6 => gc::Value::CapStyle((val as u8).try_into()?),
			7 => gc::Value::JoinStyle((val as u8).try_into()?),
			8 => gc::Value::FillStyle((val as u8).try_into()?),
			9 => gc::Value::FillRule((val as u8).try_into()?),
			10 => gc::Value::Tile(val),
			11 => gc::Value::Stipple(val),
			12 => gc::Value::TileStippleXOrigin(val as _),
			13 => gc::Value::TileStippleYOrigin(val as _),
			14 => gc::Value::Font(val),
			15 => gc::Value::SubwindowMode((val as u8).try_into()?),
			16 => gc::Value::GraphicsExposures(val as _),
			17 => gc::Value::ClipXOrigin(val as _),
			18 => gc::Value::ClipYOrigin(val as _),
			19 => gc::Value::ClipMask(val),
			20 => gc::Value::DashOffset(val as _),
			21 => gc::Value::Dashes(val as _),
			22 => gc::Value::ArcMode((val as u8).try_into()?),

			_ => unreachable!(),
		};

		values.push(val);
	}

	let gc = GC {
		drawable: hdr.drawable,
		values,
	};

	Ok(Some(Box::new(CreateGC {
		cid: hdr.cid,
		gc,
	})))
}
