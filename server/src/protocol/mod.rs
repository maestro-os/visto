//! This module implements the X protocol.

pub mod connect;
pub mod request;

/// Major version of the protocol.
pub const MAJOR_VERSION: u16 = 11;
/// Minor version of the protocol.
pub const MINOR_VERSION: u16 = 0;

/// The vendor's name.
pub const VENDOR_NAME: &str = "maestro";

/// Reply type: Error
pub const REPLY_TYPE_ERROR: u8 = 0;
/// Reply type: Normal reply
pub const REPLY_TYPE_REPLY: u8 = 1;

/// The header of a request.
#[repr(C, packed)]
pub struct XRequest {
	/// The major opcode of the request.
	pub major_opcode: u8,
	/// A byte to be used as an optional additional data.
	pub optional: u8,
	/// The total length of the request, including the header, in units of 4 bytes.
	pub length: u16,
}

// TODO XError
// TODO XEvent

/// TODO doc
#[repr(u8)]
pub enum BitGravity {
	Forget = 0,
	NorthWest = 1,
	North = 2,
	NorthEast = 3,
	West = 4,
	Center = 5,
	East = 6,
	SouthWest = 7,
	South = 8,
	SouthEast = 9,
	Static = 10,
}

/// TODO doc
#[repr(u8)]
pub enum WinGravity {
	Unmap = 0,
	NorthWest = 1,
	North = 2,
	NorthEast = 3,
	West = 4,
	Center = 5,
	East = 6,
	SouthWest = 7,
	South = 8,
	SouthEast = 9,
	Static = 10,
}

/// Enumeration of events.
#[repr(u32)]
pub enum Event {
	KeyPress = 0x00000001,
	KeyRelease = 0x00000002,
	OwnerGrabButton = 0x01000000,
	ButtonPress = 0x00000004,
	ButtonRelease = 0x00000008,
	EnterWindow = 0x00000010,
	LeaveWindow = 0x00000020,
	PointerMotion = 0x00000040,
	PointerMotionHint = 0x00000080,
	Button1Motion = 0x00000100,
	Button2Motion = 0x00000200,
	Button3Motion = 0x00000400,
	Button4Motion = 0x00000800,
	Button5Motion = 0x00001000,
	ButtonMotion = 0x000002000,
	Exposure = 0x00008000,
	VisibilityChange = 0x00010000,
	StructureNotify = 0x00020000,
	ResizeRedirect = 0x00040000,
	SubstructureNotify = 0x00080000,
	SubstructureRedirect = 0x00100000,
	FocusChange = 0x00200000,
	PropertyChange = 0x00400000,
	ColormapChange = 0x00800000,
	KeymapState = 0x00004000,
}

/// Enumeration of pointer events.
pub enum PointerEvent {
	ButtonPress,
	ButtonRelease,
	EnterWindow,
	LeaveWindow,
	PointerMotion,
	PointerMotionHint,
	Button1Motion,
	Button2Motion,
	Button3Motion,
	Button4Motion,
	Button5Motion,
	ButtonMotion,
	KeymapState,
}

/// Enumeration of device events.
pub enum DeviceEvent {
	KeyPress,
	KeyRelease,
	ButtonPress,
	ButtonRelease,
	PointerMotion,
	Button1Motion,
	Button2Motion,
	Button3Motion,
	Button4Motion,
	Button5Motion,
	ButtonMotion,
}

/// Enumeration of keymasks.
pub enum KeyMask {
	Shift,
	Lock,
	Control,
	Mod1,
	Mod2,
	Mod3,
	Mod4,
	Mod5,
}

/// Enumeration of button masks.
pub enum ButMask {
	Button1,
	Button2,
	Button3,
	Button4,
	Button5,
}

/// A 2D point.
pub struct Point {
	/// X position.
	pub x: i16,
	/// Y position.
	pub y: i16,
}

/// A rectangle.
pub struct Rectangle {
	/// X position.
	pub x: i16,
	/// Y position.
	pub y: i16,

	/// The width of the rectangle.
	pub width: u16,
	/// The height of the rectangle.
	pub height: u16,
}

/// An arc.
pub struct Arc {
	/// X position.
	pub x: i16,
	/// Y position.
	pub y: i16,

	/// The width of the arc.
	pub width: u16,
	/// The height of the arc.
	pub height: u16,

	/// TODO doc
	pub angle1: i16,
	/// TODO doc
	pub angle2: i16,
}

/// Enumeration of host families.
pub enum HostFamily {
	Internet,
	InternetV6,
	ServerInterpreted,
	DECnet,
	Chaos,
}

/// An host.
pub struct Host {
	/// The host family.
	pub family: HostFamily,

	/// The address of the host.
	pub address: Vec<u8>,
}

/// Enumeration of errors.
pub enum Error {
	Access,
	Alloc,
	Atom,
	Colormap,
	Cursor,
	Drawable,
	Font,
	GContext,
	IDChoice,
	Implementation,
	Length,
	Match,
	Name,
	Pixmap,
	Request,
	Value,
	Window,
}

/// Structure representing a X format.
#[repr(C, packed)]
pub struct Format {
	/// TODO doc
	pub depth: u8,
	/// TODO doc
	pub bits_per_pixel: u8,
	/// TODO doc
	pub scanline_pad: u8,

	/// Padding.
	pub _padding: [u8; 5],
}

/// Structure representing a X screen.
#[repr(C, packed)]
pub struct Screen {
	/// TODO doc
	pub root: u32,
	/// TODO doc
	pub default_colormap: u32,
	/// TODO doc
	pub white_pixel: u32,
	/// TODO doc
	pub black_pixel: u32,
	/// TODO doc
	pub current_input_masks: u32,

	/// The screen's width in pixels.
	pub pixels_width: u16,
	/// The screen's height in pixels.
	pub pixels_height: u16,
	/// The screen's width in millimeters.
	pub millimeters_width: u16,
	/// The screen's height in millimeters.
	pub millimeters_height: u16,

	/// TODO doc
	pub min_installed_maps: u16,
	/// TODO doc
	pub max_installed_maps: u16,

	/// TODO doc
	pub root_visual: u32,
	/// TODO doc
	pub backing_stores: u8,
	/// TODO doc
	pub save_unders: u8,
	/// TODO doc
	pub root_depth: u8,

	/// The number of allowed depths.
	pub allowed_depths_len: u8,
}

/// Structure representing a X depth.
#[repr(C, packed)]
pub struct Depth {
	/// The depth.
	pub depth: u8,

	/// Padding.
	pub _padding0: u8,

	/// Number of visuals.
	pub visuals_len: u16,

	/// Padding.
	pub _padding1: u32,
}

/// Enumeration of visual classes.
#[repr(u8)]
pub enum VisualClass {
	/// A degenerate case of GrayScale where values are predefined and read-only.
	StaticGray = 0,
	/// A degenerate case of PseudoColor where red, greeen and blue are equal, producing shades of
	/// gray.
	GrayScale = 1,
	/// A degenerate case PseudoColor where read, green and blue are predefined and read-only.
	StaticColor = 2,
	/// Red, green and blue values that can be changed dynamically.
	PseudoColor = 3,
	/// A degenerate case of DirectColor where red, green and blue values are predefined and
	/// read-only.
	TrueColor = 4,
	/// Dynamic red, green and blue values producing colors.
	DirectColor = 5,
}

/// Structure representing a X visual.
#[repr(C, packed)]
pub struct Visual {
	/// The visual's ID.
	pub visual_id: u32,
	/// The visual class.
	pub class: VisualClass,
	/// The number of bits per RGB values.
	pub bits_per_rgb_value: u8,
	/// TODO doc
	pub colormap_entries: u16,

	/// The mask of bits on which the Red color is encoded.
	pub red_mask: u32,
	/// The mask of bits on which the Green color is encoded.
	pub green_mask: u32,
	/// The mask of bits on which the Blue color is encoded.
	pub blue_mask: u32,

	/// Padding.
	pub _padding: u32,
}

/// Pads the given number `n`.
pub fn pad(n: usize) -> usize {
	(4 - (n % 4)) % 4
}
