//! A connector represents a screen.

use crate::output::card::DRICard;
use std::os::unix::io::AsRawFd;
use super::DRM_IOCTL_MODE_GETCONNECTOR;

/// Structure to get a connector's mode informations from DRM.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct DRMModeModeinfo {
	/// Pixel clock in kHz.
	pub clock: u32,
	/// Horizontal display size.
	pub hdisplay: u16,
	/// Horizontal sync start.
	pub hsync_start: u16,
	/// Horizontal sync end.
	pub hsync_end: u16,
	/// Horizontal total size.
	pub htotal: u16,
	/// Horizontal skew.
	pub hskew: u16,
	/// Vertical display size.
	pub vdisplay: u16,
	/// Vertical sync start.
	pub vsync_start: u16,
	/// Vertical sync end.
	pub vsync_end: u16,
	/// Vertical total size.
	pub vtotal: u16,
	/// Vertical scan.
	pub vscan: u16,

	/// Approximate vertical refresh rate in Hz.
	pub vrefresh: u32,

	/// Bitmask of misc. flags.
	pub flags: u32,
	/// Bitmask of type flags.
	pub type_: u32,
	/// String describing the mode resolution.
	pub name: [u8; 32],
}

/// Structure to get a connector's informations from DRM.
#[derive(Debug, Default)]
#[repr(C)]
pub struct DRMModeGetConnector {
	/// Pointer to array of object IDs.
	encoders_ptr: u64,
	/// Pointer to struct DRMModeModeinfo array.
	modes_ptr: u64,
	/// Pointer to array of property IDs.
	props_ptr: u64,
	/// Pointer to array of property values.
	prop_values_ptr: u64,

	/// Number of modes.
	count_modes: u32,
	/// Number of properties.
	count_props: u32,
	/// Number of encoders.
	count_encoders: u32,

	/// Object ID of the current encoder.
	encoder_id: u32,
	/// Object ID of the connector.
	connector_id: u32,
	/// Type of the connector.
	connector_type: u32,
	/// Type-specific connector number.
	/// 
	/// This is not an object ID. This is a per-type connector number. Each (type, type_id)
	/// combination is unique across all connectors of a DRM device.
	connector_type_id: u32,

	/// Status of the connector.
	connection: u32,
	/// Width of the connected sink in millimeters.
	mm_width: u32,
	/// Height of the connected sink in millimeters.
	mm_height: u32,
	/// Subpixel order of the connected sink.
	subpixel: u32,

	/// Padding, must be zero.
	pad: u32,
}

/// Structure representing a connector.
#[derive(Debug)]
pub struct DRIConnector {
	/// Width of the connected sink in millimeters.
	pub mm_width: u32,
	/// Height of the connected sink in millimeters.
	pub mm_height: u32,

	/// List of encoders.
	encoders: Vec<u32>,
	/// List of modes.
	pub modes: Vec<DRMModeModeinfo>,
	/// List of props.
	props: Vec<u32>,
	/// List of prop values.
	prop_values: Vec<u64>,
}

impl DRIConnector {
	/// Loads the connector with ID `id`. If the connector doesn't exist, the function returns
	/// None.
	///
	/// `card` is the card associated with the connector to be loaded.
	pub fn load(card: &DRICard, id: u32) -> Option<Self> {
		let fd = card.get_device().as_raw_fd();

		let mut conn = DRMModeGetConnector::default();
		conn.connector_id = id;

		let res = unsafe {
			libc::ioctl(
				fd,
				DRM_IOCTL_MODE_GETCONNECTOR,
				&mut conn as *const _
			)
		};
		if res < 0 {
			return None;
		}
		if conn.count_encoders <= 0 || conn.count_modes <= 0 || conn.count_props <= 0 {
			return None;
		}

		let mut connector = DRIConnector {
			mm_width: conn.mm_width,
			mm_height: conn.mm_height,

			encoders: vec![0; conn.count_encoders as usize],
			modes: vec![DRMModeModeinfo::default(); conn.count_modes as usize],
			props: vec![0; conn.count_props as usize],
			prop_values: vec![0; conn.count_props as usize],
		};

		conn.encoders_ptr = connector.encoders.as_mut_ptr() as _;
		conn.modes_ptr = connector.modes.as_mut_ptr() as _;
		conn.props_ptr = connector.props.as_mut_ptr() as _;
		conn.prop_values_ptr = connector.prop_values.as_mut_ptr() as _;

		let res = unsafe {
			libc::ioctl(
				fd,
				DRM_IOCTL_MODE_GETCONNECTOR,
				&mut conn as *const _
			)
		};
		if res < 0 {
			return None;
		}

		// TODO If count changes (hotplug), retry

		Some(connector)
	}

	/// Scans for connectors from the given card.
	pub fn scan(card: &DRICard) -> Vec<Self> {
		let mut connectors = vec![];

		for id in card.get_connector_ids() {
			if let Some(conn) = Self::load(card, *id) {
				connectors.push(conn);
			}
		}

		connectors
	}

	// TODO Set CRTC and encoder

	/// Sets the given mode for the connector.
	pub fn set_mode(&self, _mode: &DRMModeModeinfo) {
		// TODO
	}

	// TODO Framebuffer functions
}
