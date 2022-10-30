//! A "screen" is a monitor attached to the server on which rendering is done (also called "sink").

use crate::drm;
use crate::protocol;
use std::mem::size_of;
use std::ptr;

/// Structure representing a screen.
pub struct Screen {
	/// The connector, the interface to the screen.
	dri_connector: drm::DRIConnector,

	/// The screen's current mode.
	curr_mode: Option<drm::DRMModeModeinfo>,

	/// The ID of the root window of the screen.
	root_win_id: u32,
}

impl Screen {
	/// Creates a new instance.
	///
	/// Arguments:
	/// - `conn` is the connector associated with the screen.
	/// - `root_win_id` is the ID of the root window of the screen.
	pub fn new(conn: drm::DRIConnector, root_win_id: u32) -> Self {
		Self {
			dri_connector: conn,

			curr_mode: None,

			root_win_id,
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

	/// Returns the protocol representation of the screen.
	pub fn to_protocol_screen(&self) -> Vec<u8> {
		// Getting pixels width/height
		let (pixels_width, pixels_height) = match &self.curr_mode {
			Some(mode) => (mode.hdisplay, mode.vdisplay),
			None => (0, 0),
		};

		// TODO Fill according to screen informations
		let visual = protocol::Visual {
			visual_id: 0, // TODO
			class: protocol::VisualClass::DirectColor,
			bits_per_rgb_value: 24,
			colormap_entries: 1 << 8, // TODO

			red_mask: 0xff0000,
			green_mask: 0x00ff00,
			blue_mask: 0x0000ff,

			_padding: 0,
		};
		let depth = protocol::Depth {
			depth: 24,

			_padding0: 0,

			visuals_len: 1,

			_padding1: 0,
		};
		let screen = protocol::Screen {
			root: self.root_win_id,
			default_colormap: 0, // TODO
			white_pixel: 0xffffff,
			black_pixel: 0x000000,
			current_input_masks: 0, // TODO

			pixels_width,
			pixels_height,
			millimeters_width: self.dri_connector.mm_width as _,
			millimeters_height: self.dri_connector.mm_height as _,

			min_installed_maps: 1, // TODO
			max_installed_maps: 1, // TODO

			root_visual: 0, // TODO
			backing_stores: 0, // TODO
			save_unders: 0, // TODO
			root_depth: 24, // TODO

			allowed_depths_len: 1, // TODO
		};

		let len = size_of::<protocol::Screen>()
			+ size_of::<protocol::Depth>()
			+ size_of::<protocol::Visual>();
		let mut data = vec![0; len];

		let mut off = 0;
		unsafe {
			ptr::copy_nonoverlapping::<u8>(
				&screen as *const _ as *const u8,
				&mut data[off],
				size_of::<protocol::Screen>()
			);
			off += size_of::<protocol::Screen>();

			ptr::copy_nonoverlapping::<u8>(
				&depth as *const _ as *const u8,
				&mut data[off],
				size_of::<protocol::Depth>()
			);
			off += size_of::<protocol::Depth>();

			ptr::copy_nonoverlapping::<u8>(
				&visual as *const _ as *const u8,
				&mut data[off],
				size_of::<protocol::Visual>()
			);
		}

		data
	}
}
