//! TODO doc

/// A pointer displayed on a screen.
#[derive(Default)]
pub struct Pointer {
	/// The ID of the screen the pointer is located on.
	screen: u32,

	/// The X position of the pointer relative to the screen's top-left corner.
	x: i16,
	/// The Y position of the pointer relative to the screen's top-left corner.
	y: i16,

	/// The ID of the cursor associated with the pointer.
	cursor: u32,
}
