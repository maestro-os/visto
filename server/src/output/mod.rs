//! TODO doc
//!
//! Access to the system's display hardware is done through the Direct Rendering Manager (DRM).

pub mod card;
pub mod connector;
pub mod framebuffer;

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
/// DRM ioctl command: Get DRM encoder.
const DRM_IOCTL_MODE_GETCRTC: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xa1,
	connector::DRMModeCRTC
);
/// DRM ioctl command: Get DRM encoder.
const DRM_IOCTL_MODE_GETENCODER: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xa6,
	connector::DRMModeEncoder
);
/// DRM ioctl command: Get DRM connector.
const DRM_IOCTL_MODE_GETCONNECTOR: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xa7,
	connector::DRMModeGetConnector
);
/// DRM ioctl command: Creates a framebuffer.
const DRM_IOCTL_MODE_ADDFB: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xae,
	framebuffer::DRMModeFBCmd
);
/// DRM ioctl command: Removes a framebuffer.
const DRM_IOCTL_MODE_RMFB: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xaf,
	u32
);

/// DRM ioctl command: Create a dumb buffer.
const DRM_IOCTL_MODE_CREATE_DUMB: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xb2,
	connector::DRMModeCreateDumb
);
/// DRM ioctl command: Map a dumb buffer.
const DRM_IOCTL_MODE_MAP_DUMB: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xb3,
	connector::DRMModeMapDumb
);
/// DRM ioctl command: Destroy a dumb buffer.
const DRM_IOCTL_MODE_DESTROY_DUMB: u64 = iowr!(
	DRM_IOCTL_BASE,
	0xb4,
	connector::DRMModeDestroyDumb
);
