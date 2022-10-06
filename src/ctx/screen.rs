//! A "screen" is a monitor attached to the server on which render is done.

/// Structure representing a screen.
pub struct Screen {
	/// The width of the screen.
	width: usize,
	/// The height of the screen.
	height: usize,
}

impl Screen {
	/// Creates a new instance.
	///
	/// Arguments:
	/// - `width` is the width of the screen in pixels.
	/// - `height` is the height of the screen in pixels.
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
		}
	}

	/// Returns the width of the screen in pixels.
	pub fn get_width(&self) -> usize {
		self.width
	}

	/// Returns the height of the screen in pixels.
	pub fn get_height(&self) -> usize {
		self.height
	}
}
