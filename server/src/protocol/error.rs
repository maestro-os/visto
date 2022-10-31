//! This module implements error replies.

use crate::protocol;

/// Structure representing an error in the protocol's format.
#[repr(C, packed)]
pub struct XError {
	/// The reply type.
	reply_type: u8,
	/// The error code.
	code: u8,
	/// The sequence number.
	sequence_number: u16,

	/// Optional data.
	data0: u32,

	/// The minor opcode of the request associated with the error.
	minor_opcode: u16,
	/// The major opcode of the request associated with the error.
	major_opcode: u8,

	/// Optional data.
	data1: [u8; 21],
}

/// Structure representing a custom error.
pub struct CustomError {
	/// The error code.
	pub code: u8,

	/// Optional data.
	pub data0: u32,
	/// Optional data.
	pub data1: [u8; 21],
}

/// Enumeration of X protocol errors.
pub enum Error {
	/// TODO doc
	Request,
	/// Bad value.
	Value(u32),
	/// TODO doc
	Window(u32),
	/// TODO doc
	Pixmap(u32),
	/// TODO doc
	Atom(u32),
	/// TODO doc
	Cursor(u32),
	/// TODO doc
	Font(u32),
	/// TODO doc
	Match,
	/// TODO doc
	Drawable(u32),
	/// TODO doc
	Access,
	/// TODO doc
	Alloc,
	/// TODO doc
	Colormap(u32),
	/// TODO doc
	GContext(u32),
	/// TODO doc
	IDChoice(u32),
	/// TODO doc
	Name,
	/// TODO doc
	Length,
	/// TODO doc
	Implementation,

	/// Custom error.
	Custom(CustomError),
}

impl Error {
	/// Returns the code associated with the error.
	pub fn get_code(&self) -> u8 {
		match self {
			Self::Request        => 1,
			Self::Value(_)       => 2,
			Self::Window(_)      => 3,
			Self::Pixmap(_)      => 4,
			Self::Atom(_)        => 5,
			Self::Cursor(_)      => 6,
			Self::Font(_)        => 7,
			Self::Match          => 8,
			Self::Drawable(_)    => 9,
			Self::Access         => 10,
			Self::Alloc          => 11,
			Self::Colormap(_)    => 12,
			Self::GContext(_)    => 13,
			Self::IDChoice(_)    => 14,
			Self::Name           => 15,
			Self::Length         => 16,
			Self::Implementation => 17,

			Self::Custom(e) => e.code,
		}
	}

	/// Converts the error to the protocol's format.
	///
	/// Arguments:
	/// - `seq_nbr` is the sequence number of the request.
	/// - `minor` is the minor opcode of the request.
	/// - `major` is the major opcode of the request.
	pub fn to_protocol(self, seq_nbr: u16, minor: u16, major: u8) -> XError {
		let code = self.get_code();
		let (data0, data1) = match self {
			Self::Request        => (0, [0; 21]),
			Self::Value(val)     => (val, [0; 21]),
			Self::Window(val)    => (val, [0; 21]),
			Self::Pixmap(val)    => (val, [0; 21]),
			Self::Atom(val)      => (val, [0; 21]),
			Self::Cursor(val)    => (val, [0; 21]),
			Self::Font(val)      => (val, [0; 21]),
			Self::Match          => (0, [0; 21]),
			Self::Drawable(val)  => (val, [0; 21]),
			Self::Access         => (0, [0; 21]),
			Self::Alloc          => (0, [0; 21]),
			Self::Colormap(val)  => (val, [0; 21]),
			Self::GContext(val)  => (val, [0; 21]),
			Self::IDChoice(val)  => (val, [0; 21]),
			Self::Name           => (0, [0; 21]),
			Self::Length         => (0, [0; 21]),
			Self::Implementation => (0, [0; 21]),

			Self::Custom(e) => (e.data0, e.data1),
		};

		XError {
			reply_type: protocol::REPLY_TYPE_ERROR,
			code,
			sequence_number: seq_nbr,

			data0,

			minor_opcode: minor,
			major_opcode: major,

			data1,
		}
	}
}
