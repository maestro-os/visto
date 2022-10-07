//! TODO doc

use crate::protocol::Rectangle;

/// A window to be rendered on screen.
pub struct Window {
	/// If true, the window has class InputOutput. If false, the function is InputOnly.
	output: bool,

	/// The depth of the pixmap.
	depth: u8,

	/// The position and size of the window.
	rect: Rectangle,

	/// The width of the window's border.
	border_width: u16,
}
