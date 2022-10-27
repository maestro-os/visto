//! TODO doc

use crate::protocol::Rectangle;

/// A window to be rendered on screen.
pub struct Window {
	/// Tells whether the window is a root window.
	root: bool,
	/// If true, the window has class InputOutput. If false, the function is InputOnly.
	output: bool,

	/// The depth of the pixmap.
	depth: u8,

	/// The position and size of the window.
	rect: Rectangle,

	/// The width of the window's border.
	border_width: u16,
}

impl Window {
	/// Creates a new root window.
	/// By default, the window has size 0*0.
	pub fn new_root() -> Self {
		Self {
			root: true,
			output: true,

			depth: 24, // TODO

			rect: Rectangle {
				x: 0,
				y: 0,

				width: 0,
				height: 0,
			},

			border_width: 0,
		}
	}

	/// Returns the position and size of the window.
	pub fn get_rectangle(&self) -> &Rectangle {
		&self.rect
	}

	/// Sets the position and size of the window.
	pub fn set_rectangle(&mut self, rect: Rectangle) {
		if self.root && (rect.x != 0 || rect.y != 0) {
			return;
		}

		self.rect = rect;
	}
}
