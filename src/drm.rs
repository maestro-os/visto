//! This module implements the interface to the kernel's DRM.

use std::fs::File;
use std::os::unix::io::AsRawFd;

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

/// TODO doc
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

/// Structure representing a DRI device.
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
	fb_id_ptr: Vec<u32>,
	/// List of CRTC IDs.
	crtc_id_ptr: Vec<u32>,
	/// List of connectors IDs.
	connector_id_ptr: Vec<u32>,
	/// List of encoders IDs.
	encoder_id_ptr: Vec<u32>,
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

				fb_id_ptr: vec![0; card_res.count_fbs as usize],
				crtc_id_ptr: vec![0; card_res.count_crtcs as usize],
				connector_id_ptr: vec![0; card_res.count_connectors as usize],
				encoder_id_ptr: vec![0; card_res.count_encoders as usize],
			};

			card_res.fb_id_ptr = card.fb_id_ptr.as_ptr() as _;
			card_res.crtc_id_ptr = card.crtc_id_ptr.as_ptr() as _;
			card_res.connector_id_ptr = card.connector_id_ptr.as_ptr() as _;
			card_res.encoder_id_ptr = card.encoder_id_ptr.as_ptr() as _;

			let res = unsafe {
				libc::ioctl(
					fd,
					DRM_IOCTL_MODE_GETRESOURCES,
					&mut card_res as *const _
				)
			};
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

	/// Returns the list of connectors associated with the device.
	pub fn get_connectors(&mut self) {
		let mut _conn = DRMModeGetConnector::default();

		// TODO
		todo!();
	}

	// TODO Rest of init
	// - Get connectors list
	// - Iterate on encoders available for each connectors
	// - Associate a CRTC for each encoder
	// - List valid modes for each connectors
	// - Select a mode for each connector
	// - Create framebuffers
}
