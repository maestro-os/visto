//! TODO doc

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;
use crate::protocol;
use crate::util;
use std::cmp::min;
use std::mem::size_of;
use std::num::NonZeroU32;
use super::Request;

/// The header of the request's reply.
#[repr(C, packed)]
pub struct GetPropertyReply {
	/// The type of the reply.
	reply_type: u8,
	/// The format of the property.
	format: u8,
	/// The sequence number of the request.
	seq_nbr: u16,
	/// Length of the reply in 4 bytes units.
	reply_length: u32,
	/// Atom with the name of the property type.
	property_type: u32,
	/// Number of bytes remaining after the returned value.
	bytes_after: u32,
	/// The length of the value in format units.
	length: u32,
	/// Padding.
	_padding: [u8; 12],
}

/// Header of the `GetProperty` request.
#[repr(C, packed)]
pub struct GetPropertyHdr {
	/// The window to get the property from.
	window: u32,
	/// The property.
	property: u32,
	/// The atom representing the type of the property.
	property_type: u32,
	/// The offset of the beginning of the data to be returned.
	long_offset: u32,
	/// The length of the data to be returned.
	long_length: u32,
}

/// Structure representing the request.
pub struct GetProperty {
	/// The window to get the property from.
	window: u32,
	/// The property.
	property: u32,

	/// The atom representing the type of the property.
	property_type: u32,

	/// The offset of the beginning of the data to be returned.
	long_offset: u32,
	/// The length of the data to be returned.
	long_length: u32,

	/// Tells whether the property must be deleted.
	delete: bool,
}

impl Request for GetProperty {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), HandleError> {
		let prop_name = ctx.get_atom(self.property)
			.ok_or(HandleError::Client(Error::Atom(self.property)))?
			.to_owned();
		let wid = NonZeroU32::new(self.window)
			.ok_or(HandleError::Client(Error::Window(self.window)))?;
		let win = ctx.get_window_mut(wid)
			.ok_or(HandleError::Client(Error::Window(self.window)))?;

		if let Some(prop) = win.get_property(&prop_name) {
			let data = prop.get_data();

			let start_off = 4 * self.long_offset as usize;
			if start_off > data.len() {
				return Err(HandleError::Client(Error::Value(self.long_offset)));
			}
			let len = min(data.len() - start_off, 4 * self.long_length as usize);
			let bytes_after = data.len() - (start_off + len);

			let data = data[start_off..(start_off + len)].to_vec();

			let format = prop.get_format();
			let property_type = prop.get_type();

			// Delete propery if necessary
			if self.delete && self.property_type == prop.get_type() && bytes_after == 0 {
				win.delete_property(&prop_name);
				// TODO Send PropertyNotify on window
			}

			let reply_length = if format != 0 {
				(len + protocol::pad(len)) / 4
			} else {
				0
			};

			// Write header
			let hdr = GetPropertyReply {
				reply_type: protocol::REPLY_TYPE_REPLY,
				format,
				seq_nbr,
				reply_length: reply_length as u32,
				property_type,
				bytes_after: bytes_after as u32,
				length: (len / (format as usize / 8)) as u32,
				_padding: [0; 12],
			};
			client.write_obj(&hdr)
				.map_err(|e| HandleError::IO(e))?;

			if format != 0 {
				// Write data
				client.write(&data)
					.map_err(|e| HandleError::IO(e))?;

				// Write padding
				let pad: [u8; 4] = [0; 4];
				client.write(&pad[..protocol::pad(len)])
					.map_err(|e| HandleError::IO(e))?;
			}
		} else {
			let hdr = GetPropertyReply {
				reply_type: protocol::REPLY_TYPE_REPLY,
				format: 0,
				seq_nbr,
				reply_length: 0,
				property_type: 0,
				bytes_after: 0,
				length: 0,
				_padding: [0; 12],
			};
			client.write_obj(&hdr)
				.map_err(|e| HandleError::IO(e))?;
		}

		Ok(())
	}
}

/// Parses `GetProperty`.
///
/// If `delete` is nonzero, the function deletes the property from the window.
pub fn read(buff: &[u8], delete: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.len() < size_of::<GetPropertyHdr>() {
		return Ok(None);
	}

	let hdr: &GetPropertyHdr = unsafe {
		util::reinterpret(&buff[0])
	};

	Ok(Some(Box::new(GetProperty {
		window: hdr.window,
		property: hdr.property,

		property_type: hdr.property_type,

		long_offset: hdr.long_offset,
		long_length: hdr.long_length,

		delete: delete != 0,
	})))
}
