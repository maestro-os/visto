//! TODO doc

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::ctx::window::Property;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;
use crate::util;
use std::mem::size_of;
use super::Request;

/// The action to perform on the property.
pub enum ChangePropertyMode {
	/// Replaces the previous value.
	Replace,
	/// Prepends the new data to the previous.
	Prepend,
	/// Appends the new data to the previous.
	Append,
}

impl ChangePropertyMode {
	/// Returns the mode assocciated with the given value.
	///
	/// If the value is invalid, the function returns None.
	pub fn from(val: u8) -> Option<Self> {
		match val {
			0 => Some(Self::Replace),
			1 => Some(Self::Prepend),
			2 => Some(Self::Append),

			_ => None,
		}
	}
}

/// The header of the request.
#[repr(C, packed)]
struct ChangePropertyHdr {
	/// The window's ID.
	window: u32,
	/// The atom of the property's name.
	property: u32,
	/// The atom of the property's type.
	type_atom: u32,
	/// The property's format.
	format: u8,
	/// Padding.
	_padding: [u8; 3],
	/// The length of the value in format units.
	length: u32,
}

/// Structure representing the request.
pub struct ChangeProperty {
	/// The action to perform.
	mode: ChangePropertyMode,

	/// The window's ID.
	window: u32,
	/// The atom of the property's name.
	property: u32,
	/// The atom of the property's type.
	type_atom: u32,
	/// The property's format.
	format: u8,

	/// The property's data.
	data: Vec<u8>,
}

impl Request for ChangeProperty {
	fn handle(
		&self,
		ctx: &mut Context,
		_client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), HandleError> {
		let prop_name = ctx.get_atom(self.property)
			.ok_or(HandleError::Client(Error::Atom(self.property)))?
			.to_owned();
		let win = ctx.get_window_mut(self.window)
			.ok_or(HandleError::Client(Error::Window(self.window)))?;

		if let Some(prop) = win.get_property_mut(&prop_name) {
			let must_match = matches!(self.mode, ChangePropertyMode::Replace);
			let matching = prop.get_type() == self.type_atom && prop.get_format() == self.format;
			if must_match && !matching {
				return Err(HandleError::Client(Error::Match));
			}

			match self.mode {
				ChangePropertyMode::Replace => {
					win.delete_property(&prop_name);

					let prop = Property::new(self.type_atom, self.format, self.data.clone());
					win.create_property(prop_name, prop);
				},

				ChangePropertyMode::Prepend => prop.prepend_data(&self.data),
				ChangePropertyMode::Append => prop.append_data(&self.data),
			}
		} else {
			let prop = Property::new(self.type_atom, self.format, self.data.clone());
			win.create_property(prop_name, prop);
		}

		Ok(())
	}
}

/// Parses `ChangeProperty`.
///
/// `mode` is the action to perform.
pub fn read(buff: &[u8], mode: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<ChangePropertyHdr>() {
		return Ok(None);
	}
	let hdr: &ChangePropertyHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	let mode = ChangePropertyMode::from(mode).ok_or(Error::Value(mode as _))?;

	if !matches!(hdr.format, 8 | 16 | 32) {
		return Err(Error::Value(hdr.format as _));
	}

	let data_len = hdr.length as usize * (hdr.format as usize / 8);

	let data_begin = size_of::<ChangePropertyHdr>();
	let data_end = data_begin + data_len;

	if data_len > buff.len() {
		return Err(Error::Value(hdr.length));
	}

	let data_slice = &buff[data_begin..data_end];
	let data = data_slice.to_vec();

	Ok(Some(Box::new(ChangeProperty {
		mode,

		window: hdr.window,
		property: hdr.property,
		type_atom: hdr.type_atom,
		format: hdr.format,

		data,
	})))
}
