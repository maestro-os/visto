//! The `OpenFont` request allows to load a font.

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::ctx;
use crate::protocol::error::Error;
use crate::protocol::pad;
use crate::util;
use std::mem::size_of;
use std::str::FromStr;
use std::str;
use super::HandleError;
use super::Request;

/// The header of the request.
#[repr(C, packed)]
struct OpenFontHdr {
	/// The ID of the loaded font.
	fid: u32,
	/// The length of the font's name.
	name_length: u16,

	/// Padding.
	_padding: u16,
}

/// Structure representing the request.
pub struct OpenFont {
	/// The ID of the loaded font.
	fid: u32,

	/// The name of the font.
	name: String,
}

impl Request for OpenFont {
	fn handle(
		&self,
		ctx: &mut Context,
		_client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), HandleError> {
		// TODO If the font is already loaded, re-use it with the given ID

		let font_path = ctx.search_font(&self.name)
			.map_err(|e| HandleError::IO(e))?
			.unwrap(); // TODO handle None
		let font = ctx::open_font(&font_path).unwrap(); // TODO handle None

		ctx.add_font(self.fid, font);

		Ok(())
	}
}

/// Parses `OpenFont`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<OpenFontHdr>() {
		return Ok(None);
	}
	let hdr: &OpenFontHdr = unsafe { util::reinterpret(&buff[0]) };

	let len =
		size_of::<OpenFontHdr>() + hdr.name_length as usize + pad(hdr.name_length as usize);
	if buff.len() < len {
		return Ok(None);
	}

	let name_begin = size_of::<OpenFontHdr>();
	let name_end = name_begin + hdr.name_length as usize;
	let name = str::from_utf8(&buff[name_begin..name_end]).unwrap(); // TODO Handle error

	Ok(Some(Box::new(OpenFont {
		fid: hdr.fid,

		name: String::from_str(name).unwrap(),
	})))
}
