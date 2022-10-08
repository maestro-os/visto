//! This module implements connection on the X server protocol.

/// Most Significant Byte first.
const MSB_FIRST: u8 = 0x42;
/// Least Significant Byte first.
const LSB_FIRST: u8 = 0x6c;

/// Connect response: Failed
const FAILED: u8 = 0;
/// Connect response: Success
const SUCCESS: u8 = 1;
/// Connect response: Authentication required
const AUTHENTICATE: u8 = 2;

/// Sent by the client at the beginning of the connection.
#[repr(C, packed)]
pub struct ClientConnect {
	/// Value specifying the byte order for the client.
	byte_order: u8,

	/// Padding.
	_padding0: u8,

	/// Major version of the protocol.
	protocol_major_version: u16,
	/// Minor version of the protocol.
	protocol_minor_version: u16,

	/// The length of the authorization protocol name.
	authorization_protocol_name_length: u16,
	/// The length of the authorization protocol data.
	authorization_protocol_data: u16,

	/// Padding.
	_padding1: u16,

	/// Remaining fields.
	data: [u8],
}

/// Response to client connect: failed
#[repr(C, packed)]
pub struct ConnectFailed {
	/// The length of the reason message.
	reason: u8,

	/// Major version of the protocol.
	protocol_major_version: u16,
	/// Minor version of the protocol.
	protocol_minor_version: u16,

	/// The length of additional data, in units of 4 bytes.
	additional_data_len: u16,
	/// Additional data.
	additional_data: [u8],
}

/// Response to client connect: success
#[repr(C, packed)]
pub struct ConnectSuccess {
	/// Padding.
	_padding: u8,

	/// Major version of the protocol.
	protocol_major_version: u16,
	/// Minor version of the protocol.
	protocol_minor_version: u16,

	/// The length of additional data, in units of 4 bytes.
	additional_data_len: u16,

	/// TODO doc
	release_number: u32,
	/// TODO doc
	resource_id_base: u32,
	/// TODO doc
	resource_id_mask: u32,
	/// TODO doc
	motion_buffer_size: u32,
	/// TODO doc
	vendor_length: u16,
	/// TODO doc
	max_request_length: u16,
	/// TODO doc
	roots_screens_number: u8,
	/// TODO doc
	pixmap_formats_count: u8,
	/// TODO doc
	image_byte_order: u8,

	/// TODO doc
	bitmap_format_bit_order: u8,
	/// TODO doc
	bitmap_format_scanline_unit: u8,
	/// TODO doc
	bitmap_format_scanline_pad: u8,

	/// TODO doc
	min_keycode: u8,
	/// TODO doc
	max_keycode: u8,

	/// Padding.
	_padding1: u32,

	/// Additional data.
	additional_data: [u8],
}

/// Response to client connect: authentication
#[repr(C, packed)]
pub struct ConnectAuth {
	/// Padding.
	_padding: [u8; 5],

	/// The length of additional data, in units of 4 bytes.
	additional_data_len: u16,
	/// Additional data.
	additional_data: [u8],
}
