//! This module implements connection on the X server protocol.

/// Most Significant Byte first.
pub const MSB_FIRST: u8 = 0x42;
/// Least Significant Byte first.
pub const LSB_FIRST: u8 = 0x6c;

/// Connect response: Failed
pub const FAILED: u8 = 0;
/// Connect response: Success
pub const SUCCESS: u8 = 1;
/// Connect response: Authentication required
pub const AUTHENTICATE: u8 = 2;

/// Sent by the client at the beginning of the connection.
#[repr(C, packed)]
pub struct ClientConnect {
	/// Value specifying the byte order for the client.
	pub byte_order: u8,

	/// Padding.
	pub _padding0: u8,

	/// Major version of the protocol.
	pub protocol_major_version: u16,
	/// Minor version of the protocol.
	pub protocol_minor_version: u16,

	/// The length of the authorization protocol name.
	pub authorization_protocol_name_length: u16,
	/// The length of the authorization protocol data.
	pub authorization_protocol_data_length: u16,

	/// Padding.
	pub _padding1: u16,
}

/// Response to client connect: failed
#[repr(C, packed)]
pub struct ConnectFailed {
	/// The length of the reason message.
	pub reason_len: u8,

	/// Major version of the protocol.
	pub protocol_major_version: u16,
	/// Minor version of the protocol.
	pub protocol_minor_version: u16,

	/// The length of additional data, in units of 4 bytes.
	pub additional_data_len: u16,
}

/// Response to client connect: success
#[repr(C, packed)]
pub struct ConnectSuccess {
	/// Padding.
	pub _padding0: u8,

	/// Major version of the protocol.
	pub protocol_major_version: u16,
	/// Minor version of the protocol.
	pub protocol_minor_version: u16,

	/// The length of additional data, in units of 4 bytes.
	pub additional_data_len: u16,

	/// TODO doc
	pub release_number: u32,
	/// TODO doc
	pub resource_id_base: u32,
	/// TODO doc
	pub resource_id_mask: u32,
	/// TODO doc
	pub motion_buffer_size: u32,
	/// TODO doc
	pub vendor_length: u16,
	/// TODO doc
	pub max_request_length: u16,
	/// TODO doc
	pub roots_screens_number: u8,
	/// TODO doc
	pub pixmap_formats_count: u8,
	/// TODO doc
	pub image_byte_order: u8,

	/// TODO doc
	pub bitmap_format_bit_order: u8,
	/// TODO doc
	pub bitmap_format_scanline_unit: u8,
	/// TODO doc
	pub bitmap_format_scanline_pad: u8,

	/// TODO doc
	pub min_keycode: u8,
	/// TODO doc
	pub max_keycode: u8,

	/// Padding.
	pub _padding1: u32,
}

/// Response to client connect: authentication
#[repr(C, packed)]
pub struct ConnectAuth {
	/// Padding.
	pub _padding: [u8; 5],

	/// The length of additional data, in units of 4 bytes.
	pub additional_data_len: u16,
}
