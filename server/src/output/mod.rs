//! TODO doc
//!
//! Access to the system's display hardware is done through the Direct Rendering Manager (DRM).

pub mod card;
pub mod connector;

// TODO
// - Iterate on encoders available for each connectors
// - Associate a CRTC for each encoder
// - List valid modes for each connectors
// - Select a mode for each connector
// - Create framebuffers

/// ioctl macro: TODO doc
macro_rules! ioc {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		(($a) << 30) | (($b) << 8) | ($c) | (($d) << 16)
	}
}

/// ioctl macro: TODO doc
macro_rules! io {
	($a:expr, $b:expr) => {
		ioc!(0, $a, $b, 0)
	}
}

/// ioctl macro: TODO doc
macro_rules! iow {
	($a:expr, $b:expr, $c:ty) => {
		ioc!(1, $a, $b, std::mem::size_of::<$c>() as u64)
	}
}

/// ioctl macro: TODO doc
macro_rules! ior {
	($a:expr, $b:expr, $c:ty) => {
		ioc!(2, $a, $b, std::mem::size_of::<$c>() as u64)
	}
}

/// ioctl macro: TODO doc
macro_rules! iowr {
	($a:expr, $b:expr, $c:ty) => {
		ioc!(3, $a, $b, std::mem::size_of::<$c>() as u64)
	}
}

/// DRM ioctl command base.
const DRM_IOCTL_BASE: u64 = b'd' as u64;
/// DRM ioctl command: Get DRM card resources.
const DRM_IOCTL_MODE_GETRESOURCES: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xa0,
	card::DRMModeCardRes
);
/// DRM ioctl command: Get DRM connector.
const DRM_IOCTL_MODE_GETCONNECTOR: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xa7,
	connector::DRMModeGetConnector
);
