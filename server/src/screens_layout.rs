//! TODO doc

// TODO Add support for duplicate screens

/// A screen mode.
pub struct ScreenModeDesc {
	/// The width of the screen in pixels.
	pub width: u16,
	/// The height of the screen in pixels.
	pub height: u16,

	// TODO Other parameters (gamma, etc...)
}

/// Structure representing the position of a screen.
pub struct ScreenLayout {
	/// The name of the screen.
	pub name: String,

	/// The absolute virtual X position of the screen.
	pub x: u32,
	/// The absolute virtual Y position of the screen.
	pub y: u32,

	/// Tells whether the screen is enabled.
	pub enabled: bool,

	/// The mode for the screen.
	pub mode: ScreenModeDesc,
}

/// Structure representing the virtual disposition of screens.
pub struct ScreensLayout {
	/// The list of layouts for each screens.
	pub screens: Vec<ScreenLayout>,
}
