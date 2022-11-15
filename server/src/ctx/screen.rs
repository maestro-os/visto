//! A "screen" is a monitor attached to the server on which rendering is done (also called "sink").
//!
//! Since a desktop can be split on several screens, each screens has its own virtual position to
//! determine on which screen the pointer must appears when hitting a corner.

use crate::output::card::DRICard;
use crate::output::connector::DRIConnector;
use crate::output::connector::DRMModeModeinfo;
use crate::output::framebuffer::Framebuffer;
use crate::protocol;
use std::mem::size_of;
use std::num::NonZeroU32;
use std::ptr;

/// Structure representing a screen.
pub struct Screen<'a> {
	/// A reference to the card device.
	dev: &'a DRICard,

	/// The connector, the interface to the screen.
	conn: DRIConnector,
	// TODO Do not store since it can be changed by an external program?
	/// The screen's current mode.
	mode: DRMModeModeinfo,
	/// The ID of the screen's CRTC.
	crtc: u32,

	/// The framebuffers.
	fbs: [Framebuffer<'a>; 2],
	/// The index of the current framebuffer.
	curr_fb: usize,

	/// The absolute virtual X position of the screen.
	x: u32,
	/// The absolute virtual Y position of the screen.
	y: u32,

	/// The ID of the root window of the screen.
	root_win_id: NonZeroU32,
}

impl<'a> Screen<'a> {
	/// Creates a new instance.
	///
	/// Arguments:
	/// - `dev` is a reference to the connector's card device.
	/// - `conn` is the connector associated with the screen.
	/// - `x` is the absolute virtual X position of the screen.
	/// - `y` is the absolute virtual Y position of the screen.
	/// - `mode` is the current mode of the screen.
	/// - `root_win_id` is the ID of the root window of the screen.
	pub fn new(
		dev: &'a DRICard,
		conn: DRIConnector,
		mode: DRMModeModeinfo,
		x: u32,
		y: u32,
		root_win_id: NonZeroU32,
	) -> Self {
		// TODO Handle error
		let crtc = conn.get_crtc(&dev).unwrap().crtc_id;

		let mut fbs = [
			Framebuffer::new(dev, mode.hdisplay as _, mode.vdisplay as _).unwrap(),
			Framebuffer::new(dev, mode.hdisplay as _, mode.vdisplay as _).unwrap(),
		];
		// TODO Handle errors
		fbs[0].map().unwrap();
		fbs[1].map().unwrap();

		Self {
			dev,

			conn,
			mode,
			crtc,

			fbs,
			curr_fb: 0,

			x,
			y,

			root_win_id,
		}
	}

	/// Returns the size of the screen in millimeters.
	pub fn get_screen_size_mm(&self) -> (u32, u32) {
		(self.conn.mm_width, self.conn.mm_height)
	}

	/// Returns the list of available modes for the screen.
	pub fn get_available_modes(&self) -> &[DRMModeModeinfo] {
		&self.conn.modes
	}

	/// Returns the current mode of the screen.
	pub fn get_current_mode(&self) -> &DRMModeModeinfo {
		&self.mode
	}

	/// Returns the size of the screen in pixels.
	///
	/// If no mode is selected, the function returns None.
	pub fn get_screen_size(&self) -> (u16, u16) {
		(self.mode.hdisplay, self.mode.vdisplay)
	}

	/// Tells whether two screens are adjacents.
	///
	/// This function is commutative.
	pub fn adj(&self, other: &Self) -> bool {
		// TODO Check if can be simplified
		let x_adj = (self.x <= other.x) && (self.x + self.mode.hdisplay as u32 + 1 >= other.x)
			|| (other.x <= self.x) && (other.x + other.mode.hdisplay as u32 + 1 >= self.x);
		let y_adj = (self.y <= other.y) && (self.y + self.mode.vdisplay as u32 + 1 >= other.y)
			|| (other.y <= self.y) && (other.y + other.mode.vdisplay as u32 + 1 >= self.y);

		x_adj && y_adj
	}

	/// Returns the ID of the screen's root window.
	pub fn get_root_window_id(&self) -> NonZeroU32 {
		self.root_win_id
	}

	/// Returns the protocol representation of the screen.
	pub fn to_protocol_screen(&self) -> Vec<u8> {
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
			root: self.root_win_id.get(),
			default_colormap: 0, // TODO
			white_pixel: 0xffffff,
			black_pixel: 0x000000,
			current_input_masks: 0, // TODO

			pixels_width: self.mode.hdisplay,
			pixels_height: self.mode.vdisplay,
			millimeters_width: self.conn.mm_width as _,
			millimeters_height: self.conn.mm_height as _,

			min_installed_maps: 1, // TODO
			max_installed_maps: 1, // TODO

			root_visual: 0,    // TODO
			backing_stores: 0, // TODO
			save_unders: 0,    // TODO
			root_depth: 24,    // TODO

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
				size_of::<protocol::Screen>(),
			);
			off += size_of::<protocol::Screen>();

			ptr::copy_nonoverlapping::<u8>(
				&depth as *const _ as *const u8,
				&mut data[off],
				size_of::<protocol::Depth>(),
			);
			off += size_of::<protocol::Depth>();

			ptr::copy_nonoverlapping::<u8>(
				&visual as *const _ as *const u8,
				&mut data[off],
				size_of::<protocol::Visual>(),
			);
		}

		data
	}

	/// Returns an immutable reference to the current framebuffer to use for rendering.
	pub fn get_curr_fb(&self) -> &Framebuffer {
		&self.fbs[self.curr_fb]
	}

	/// Swap frame buffers, thus displaying the next frame to the screen.
	pub fn swap_buffers(&mut self) {
		let fb = &self.fbs[self.curr_fb];

		self.conn.page_flip(&self.dev, self.crtc, fb);
		self.curr_fb = (self.curr_fb + 1) % self.fbs.len();
	}
}
