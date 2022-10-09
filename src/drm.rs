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
	/// The path to the device file.
	path: String,
	/// The open device file.
	dev: File,
}

/// Scan DRI's devices and returns a list of available devices.
pub fn scan_devices() -> Vec<DRICard> {
	let mut devs = vec![];

	for i in 0..16 {
		let path = format!("/dev/dri/card{}", i);
		if let Ok(dev) = File::open(&path) {
			let fd = dev.as_raw_fd();

			let mut card_res = DRMModeCardRes::default();
			let res = unsafe {
				libc::ioctl(
					fd,
					DRM_IOCTL_MODE_GETRESOURCES,
					&mut card_res as *const _
				)
			};

			// On error, ignore the device and continue iteration
			if res < 0 {
				continue;
			}

			// TODO rm
			println!("-> {} {} {:?}", res, std::io::Error::last_os_error(), card_res);

			// TODO Allocate buffers for:
			// - fb_id_ptr
			// - crtc_id_ptr
			// - connector_id_ptr
			// - encoder_id_ptr
			// TODO Get connectors list
			// TODO Iterate on encoders available for each connectors
			// TODO Associate a CRTC for each encoder

			// TODO List valid modes for each connectors
			// TODO Select a mode for each connector
			// TODO Create framebuffers

			devs.push(DRICard {
				path,
				dev,
			});
		}
	}

	devs
}
