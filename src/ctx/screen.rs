//! A "screen" is a monitor attached to the server on which rendering is done (also called "sink").

use crate::drm;

/// Structure representing a screen.
pub struct Screen {
	/// The connector, the interface to the screen.
	dri_connector: drm::DRIConnector,

	/// The screen's current mode.
	curr_mode: Option<drm::DRMModeModeinfo>,
}

impl Screen {
	/// Creates a new instance.
	///
	/// `conn` is the connector associated with the screen.
	pub fn new(conn: drm::DRIConnector) -> Self {
		Self {
			dri_connector: conn,

			curr_mode: None,
		}
	}

	/// Returns the size of the screen in millimeters.
	pub fn get_screen_size(&self) -> (u32, u32) {
		(self.dri_connector.mm_width, self.dri_connector.mm_height)
	}

	/// Returns the list of available modes for the screen.
	pub fn get_available_modes(&self) -> &[drm::DRMModeModeinfo] {
		&self.dri_connector.modes
	}

	/// Returns the current mode of the screen.
	/// If the current mode is unknown, the function returns None.
	pub fn get_mode(&self) -> Option<&drm::DRMModeModeinfo> {
		self.curr_mode.as_ref()
	}

	/// Performs modesetting for the screen with the given mode.
	pub fn set_mode(&mut self) {
		// TODO
		todo!();
	}

	// TODO create/get framebuffer
}
