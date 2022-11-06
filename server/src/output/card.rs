//! A card is a device handling screens.

use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use super::DRM_IOCTL_MODE_GETRESOURCES;

/// Structure to get card resources from DRM.
#[derive(Debug, Default)]
#[repr(C)]
pub struct DRMModeCardRes {
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
			let dev = OpenOptions::new()
				.read(true)
				.write(true)
				.open(&path);
			let dev = match dev {
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

	/// Returns the device file associated with the card.
	pub fn get_device(&self) -> &File {
		&self.dev
	}

	/// Returns the list of IDs of the card's framebuffers.
	pub fn get_fb_ids(&self) -> &[u32] {
		&self.fb_ids
	}

	/// Returns the list of IDs of the card's CRTCs.
	pub fn get_crtc_ids(&self) -> &[u32] {
		&self.crtc_ids
	}

	/// Returns the list of IDs of the card's connectors.
	pub fn get_connector_ids(&self) -> &[u32] {
		&self.connector_ids
	}

	/// Returns the list of IDs of the card's encoders.
	pub fn get_encoder_ids(&self) -> &[u32] {
		&self.encoder_ids
	}
}
