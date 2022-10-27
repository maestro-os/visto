//! This module implements the interface to the kernel's DRM.

use std::fs::File;
use std::os::unix::io::AsRawFd;

// TODO Rest of init
// - Iterate on encoders available for each connectors
// - Associate a CRTC for each encoder
// - List valid modes for each connectors
// - Select a mode for each connector
// - Create framebuffers

/// TODO doc
macro_rules! ioc {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		(($a) << 30) | (($b) << 8) | ($c) | (($d) << 16)
	}
}

/// TODO doc
macro_rules! io {
	($a:expr, $b:expr) => {
		ioc!(0, $a, $b, 0)
	}
}

/// TODO doc
macro_rules! iow {
	($a:expr, $b:expr, $c:ty) => {
		ioc!(1, $a, $b, std::mem::size_of::<$c>() as u64)
	}
}

/// TODO doc
macro_rules! ior {
	($a:expr, $b:expr, $c:ty) => {
		ioc!(2, $a, $b, std::mem::size_of::<$c>() as u64)
	}
}

/// TODO doc
macro_rules! iowr {
	($a:expr, $b:expr, $c:ty) => {
		ioc!(3, $a, $b, std::mem::size_of::<$c>() as u64)
	}
}

/// DRM ioctl command base.
const DRM_IOCTL_BASE: u64 = b'd' as u64;
/// DRM ioctl command: TODO doc
const DRM_IOCTL_MODE_GETRESOURCES: u64 = iowr!(DRM_IOCTL_BASE, 0xa0, DRMModeCardRes);
/// DRM ioctl command: TODO doc
const DRM_IOCTL_MODE_GETCONNECTOR: u64 = iowr!(DRM_IOCTL_BASE, 0xa7, DRMModeGetConnector);

/// TODO doc
#[derive(Debug, Default)]
#[repr(C)]
struct DRMModeCardRes {
	/// The pointer to the framebuffers IDs list.
	fb_id_ptr: u64,
	/// The pointer to the CRTCs IDs list.
	crtc_id_ptr: u64,
	/// The pointer to the connectors IDs list.
	connector_id_ptr: u64,
	/// The pointer to the encoders IDs list.
	encoder_id_ptr: u64,
	/// The number of framebuffers available.
	count_fbs: u32,
	/// The number of CRTCs available.
	count_crtcs: u32,
	/// The number of connectors available.
	count_connectors: u32,
	/// The number of encoders available.
	count_encoders: u32,
	/// The minimum width of a framebuffer.
	min_width: u32,
	/// The minimum height of a framebuffer.
	max_width: u32,
	/// The maximum width of a framebuffer.
	min_height: u32,
	/// The maximum height of a framebuffer.
	max_height: u32,
}

/// Structure representing a DRM connector.
#[derive(Debug, Default)]
#[repr(C)]
struct DRMModeGetConnector {
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

/// Structure storing a mode's informations.
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

/// Structure representing a DRI device.
#[derive(Debug)]
pub struct DRICard {
	/// The path to the device file.
	path: String,
	/// The open device file.
	dev: File,

	/// The minimum width of a framebuffer.
	fb_min_width: u32,
	/// The maximum width of a framebuffer.
	fb_max_width: u32,
	/// The minimum height of a framebuffer.
	fb_min_height: u32,
	/// The maximum height of a framebuffer.
	fb_max_height: u32,

	/// List of framebuffer IDs.
	fb_ids: Vec<u32>,
	/// List of CRTC IDs.
	crtc_ids: Vec<u32>,
	/// List of connectors IDs.
	connector_ids: Vec<u32>,
	/// List of encoders IDs.
	encoder_ids: Vec<u32>,
}

impl DRICard {
	/// Loads the device with ID `id`. If the device doesn't exist, the function returns None.
	pub fn load(id: usize) -> Option<Self> {
		let path = format!("/dev/dri/card{}", id);

		loop {
			let dev = match File::open(&path) {
				Ok(dev) => dev,
				Err(_) => return None,
			};
			let fd = dev.as_raw_fd();

			let mut card_res = DRMModeCardRes::default();
			let res = unsafe {
				libc::ioctl(
					fd,
					DRM_IOCTL_MODE_GETRESOURCES,
					&mut card_res as *const _
				)
			};
			if res < 0 {
				return None;
			}

			let card = Self {
				path: path.clone(),
				dev,

				fb_min_width: card_res.min_width,
				fb_max_width: card_res.max_width,
				fb_min_height: card_res.min_height,
				fb_max_height: card_res.max_height,

				fb_ids: vec![0; card_res.count_fbs as usize],
				crtc_ids: vec![0; card_res.count_crtcs as usize],
				connector_ids: vec![0; card_res.count_connectors as usize],
				encoder_ids: vec![0; card_res.count_encoders as usize],
			};

			if card_res.count_fbs > 0 {
				card_res.fb_id_ptr = card.fb_ids.as_ptr() as _;
			}
			if card_res.count_crtcs > 0 {
				card_res.crtc_id_ptr = card.crtc_ids.as_ptr() as _;
			}
			if card_res.count_connectors > 0 {
				card_res.connector_id_ptr = card.connector_ids.as_ptr() as _;
			}
			if card_res.count_encoders > 0 {
				card_res.encoder_id_ptr = card.encoder_ids.as_ptr() as _;
			}

			let res = unsafe {
				libc::ioctl(
					fd,
					DRM_IOCTL_MODE_GETRESOURCES,
					&mut card_res as *const _
				)
			};

			// TODO If count changes (hotplug), retry

			if res >= 0 {
				return Some(card);
			}
		}
	}

	/// Scan DRI's devices and returns a list of available devices.
	pub fn scan() -> Vec<Self> {
		let mut devs = vec![];

		for i in 0..16 {
			if let Some(dev) = Self::load(i) {
				devs.push(dev);
			}
		}

		devs
	}
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
		let fd = card.dev.as_raw_fd();

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

		let mut connector = DRIConnector {
			mm_width: conn.mm_width,
			mm_height: conn.mm_height,

			encoders: vec![0; conn.count_encoders as usize],
			modes: vec![DRMModeModeinfo::default(); conn.count_modes as usize],
			props: vec![0; conn.count_props as usize],
			prop_values: vec![0; conn.count_props as usize],
		};

		if conn.count_encoders > 0 {
			conn.encoders_ptr = connector.encoders.as_mut_ptr() as _;
		}
		if conn.count_modes > 0 {
			conn.modes_ptr = connector.modes.as_mut_ptr() as _;
		}
		if conn.count_props > 0 {
			conn.props_ptr = connector.props.as_mut_ptr() as _;
			conn.prop_values_ptr = connector.prop_values.as_mut_ptr() as _;
		}

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

		for id in &card.connector_ids {
			if let Some(conn) = Self::load(card, *id) {
				connectors.push(conn);
			}
		}

		connectors
	}
}
