//! This module implements the X protocol.

pub mod msg;

/// The header of a request.
#[repr(C, packed)]
pub struct XRequest {
	/// The major opcode of the request.
	major_opcode: u8,
	/// One-byte padding.
	_padding: u8,
	/// The total length of the request, including the header, in units of 4 bytes.
	length: u16,
}

/// The header of a reply.
#[repr(C, packed)]
pub struct XReply {
	/// The total length of the reply, including the header, in units of 4 bytes.
	length: u16,
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
	x: i16,
	/// Y position.
	y: i16,
}

/// A rectangle.
pub struct Rectangle {
	/// X position.
	x: i16,
	/// Y position.
	y: i16,

	/// The width of the rectangle.
	width: u16,
	/// The height of the rectangle.
	height: u16,
}

/// An arc.
pub struct Arc {
	/// X position.
	x: i16,
	/// Y position.
	y: i16,

	/// The width of the arc.
	width: u16,
	/// The height of the arc.
	height: u16,

	/// TODO doc
	angle1: i16,
	/// TODO doc
	angle2: i16,
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
	family: HostFamily,

	/// The address of the host.
	address: Vec<u8>,
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

/// TODO doc
#[repr(C, packed)]
pub struct Format {
	/// TODO doc
	depth: u8,
	/// TODO doc
	bits_per_pixel: u8,
	/// TODO doc
	scanline_pad: u8,
}

/// TODO doc
#[repr(C, packed)]
pub struct Screen {
	/// TODO doc
	root: u32,
	/// TODO doc
	default_colormap: u32,
	/// TODO doc
	white_pixel: u32,
	/// TODO doc
	black_pixel: u32,
	/// TODO doc
	current_input_masks: Event,

	/// The screen's width in pixels.
	pixels_width: u16,
	/// The screen's height in pixels.
	pixels_height: u16,
	/// The screen's width in millimeters.
	millimeters_width: u16,
	/// The screen's height in millimeters.
	millimeters_height: u16,

	/// TODO doc
	min_installed_maps: u16,
	/// TODO doc
	max_installed_maps: u16,

	/// TODO doc
	root_visual: u32,
	/// TODO doc
	backing_stores: u8,
	/// TODO doc
	save_unders: u8,
	/// TODO doc
	root_depth: u8,

	/// The number of allowed depths.
	allowed_depths_len: u8,
	/// Allowed depths.
	/// This field doesn't have type `[Depth]` because Depth is Unsized.
	allowed_depths: [u8],
}

/// TODO doc
#[repr(C, packed)]
pub struct Depth {
	/// TODO doc
	depth: u8,

	/// Padding.
	_padding0: u8,

	/// TODO doc
	visuals_len: u16,

	/// Padding.
	_padding1: u32,

	/// TODO doc
	visuals: [Visual],
}

/// TODO doc
#[repr(C, packed)]
pub struct Visual {
	/// TODO doc
	visual_id: u32,
	/// TODO doc
	class: u8,
	/// TODO doc
	bits_per_rgb_value: u8,
	/// TODO doc
	colormap_entries: u16,

	/// TODO doc
	red_mask: u32,
	/// TODO doc
	green_mask: u32,
	/// TODO doc
	blue_mask: u32,

	/// TODO doc
	_padding: u32,
}
