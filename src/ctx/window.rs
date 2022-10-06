//! TODO doc

/// A window to be rendered on screen.
pub struct Window {
	/// If true, the window has class InputOutput. If false, the function is InputOnly.
	output: bool,

	/// The depth of the pixmap.
	depth: u8,

	/// The X position of the window.
	x: i16,
	/// The Y position of the window.
	y: i16,

	/// The width of the window.
	width: u16,
	/// The height of the window.
	height: u16,
	/// The width of the window's border.
	border_width: u16,
}
