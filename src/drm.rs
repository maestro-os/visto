//! This module implements the interface to the kernel's DRM.

use std::fs::File;
use std::os::unix::io::AsRawFd;

/// DRM ioctl command base.
const DRM_IOCTL_BASE: u64 = b'd' as u64;
/// DRM ioctl command: TODO doc
const DRM_IOCTL_MODE_GETRESOURCES: u64 = DRM_IOCTL_BASE + 0xa0;

/// TODO doc
#[derive(Debug, Default)]
#[repr(C)]
struct DRMModeCardRes {
	/// TODO doc
	fb_id_ptr: u64,
	/// TODO doc
	crtc_id_ptr: u64,
	/// TODO doc
	connector_id_ptr: u64,
	/// TODO doc
	encoder_id_ptr: u64,
	/// TODO doc
	count_fbs: u32,
	/// TODO doc
	count_crtcs: u32,
	/// TODO doc
	count_connectors: u32,
	/// TODO doc
	count_encoders: u32,
	/// TODO doc
	min_width: u32,
	/// TODO doc
	max_width: u32,
	/// TODO doc
	min_height: u32,
	/// TODO doc
	max_height: u32,
}

/// Structure representing a DRI device.
pub struct DRICard {
	// TODO
}

/// Scan DRI's devices and returns a list of available devices.
pub fn scan_devices() -> Vec<DRICard> {
	let mut devs = vec![];

	for i in 0..16 {
		let path = format!("/dev/dri/card{}", i);
		println!("{}", path);
		if let Ok(file) = File::open(path) {
			let fd = file.as_raw_fd();

			let mut card_res = DRMModeCardRes::default();
			let res = unsafe {
				libc::ioctl(
					fd,
					DRM_IOCTL_MODE_GETRESOURCES,
					&mut card_res as *const _
				)
			};
			println!("-> {} {} {:?}", res, std::io::Error::last_os_error(), card_res);

			// TODO Insert on success
			devs.push(DRICard {
				// TODO
			});
		}
	}

	devs
}
