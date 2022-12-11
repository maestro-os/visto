//! A framebuffer is a region of memory containing the data for each pixels to be displayed on
//! screen.

use super::card::DRICard;
use super::DRM_IOCTL_MODE_ADDFB;
use super::DRM_IOCTL_MODE_CREATE_DUMB;
use super::DRM_IOCTL_MODE_MAP_DUMB;
use super::DRM_IOCTL_MODE_RMFB;
use std::ffi::c_void;
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::ptr::null_mut;
use std::ptr::NonNull;

/// Structure used by the command to create a dumb buffer.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct DRMModeCreateDumb {
	/// The height of the buffer in pixels.
	height: u32,
	/// The width of the buffer in pixels.
	width: u32,
	/// Bits-Per-Pixel
	bpp: u32,
	/// Flags.
	flags: u32,
	/// The handle to the create buffer.
	handle: u32,
	/// TODO doc
	pitch: u32,
	/// The size of the buffer in bytes.
	size: u32,
}

/// Structure used by the command to create a framebuffer.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct DRMModeFBCmd {
	/// The framebuffer's ID.
	fb_id: u32,
	/// The width of the framebuffer in pixels.
	width: u32,
	/// The height of the framebuffer in pixels.
	height: u32,
	/// TODO doc
	pitch: u32,
	/// Bits-Per-Pixel
	bpp: u32,
	/// The depth.
	depth: u32,
	/// The handle to the dumb buffer.
	handle: u32,
}

/// TODO doc
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct DRMModeMapDumb {
	/// The dumb buffer's handdle.
	handle: u32,
	/// Padding.
	pad: u32,
	/// The offset at which the buffer is located on the card's file.
	offset: u64,
}

/// Structure representing a framebuffer.
pub struct Framebuffer<'a> {
	card: &'a DRICard,

	/// The handle to the dumb buffer.
	dumb_handle: u32,
	/// The framebuffer's ID.
	fb_id: u32,

	/// The pointer to the memory chunk the buffer is mapped to.
	buff: Option<NonNull<u32>>,
	/// The length of the buffer in bytes.
	buff_len: usize,
}

impl<'a> Framebuffer<'a> {
	/// Creates a framebuffer using the given width and height.
	pub fn new(card: &'a DRICard, width: u32, height: u32) -> Result<Self, ()> {
		let fd = card.get_device().as_raw_fd();

		// Create dumb buffer
		let mut dumb_buff = DRMModeCreateDumb::default();
		dumb_buff.height = height;
		dumb_buff.width = width;
		dumb_buff.bpp = 32;
		dumb_buff.flags = 0;
		let res = unsafe { libc::ioctl(fd, DRM_IOCTL_MODE_CREATE_DUMB, &mut dumb_buff as *mut _) };
		if res < 0 {
			return Err(());
		}

		// Create framebuffer
		let mut cmd = DRMModeFBCmd::default();
		cmd.width = width;
		cmd.height = height;
		cmd.pitch = dumb_buff.pitch;
		cmd.bpp = 32;
		cmd.depth = 24;
		cmd.handle = dumb_buff.handle;
		let res = unsafe { libc::ioctl(fd, DRM_IOCTL_MODE_ADDFB, &mut cmd as *mut _) };
		if res < 0 {
			return Err(());
		}

		Ok(Self {
			card,

			dumb_handle: dumb_buff.handle,
			fb_id: cmd.fb_id,

			buff: None,
			buff_len: dumb_buff.size as _,
		})
	}

	/// Returns the ID of the framebuffer.
	pub fn get_id(&self) -> u32 {
		self.fb_id
	}

	/// Maps the framebuffer to memory.
	pub fn map(&mut self) -> Result<(), ()> {
		let fd = self.card.get_device().as_raw_fd();

		let mut cmd = DRMModeMapDumb::default();
		cmd.handle = self.dumb_handle;
		let res = unsafe { libc::ioctl(fd, DRM_IOCTL_MODE_MAP_DUMB, &mut cmd as *mut _) };
		if res < 0 {
			return Err(());
		}

		let buff_ptr = unsafe {
			libc::mmap(
				null_mut::<c_void>(),
				self.buff_len,
				libc::PROT_READ | libc::PROT_WRITE,
				libc::MAP_SHARED,
				fd,
				cmd.offset as _,
			)
		};
		if buff_ptr.is_null() || buff_ptr == libc::MAP_FAILED {
			return Err(());
		}
		self.buff = NonNull::new(buff_ptr as *mut u32);

		Ok(())
	}

	/// Returns the pointer to the buffer.
	pub fn get_buffer_ptr(&self) -> Option<NonNull<u32>> {
		self.buff
	}

	/// Returns the length of the buffer in pixels.
	pub fn get_buffer_len(&self) -> usize {
		self.buff_len / size_of::<u32>()
	}
}

impl<'a> Drop for Framebuffer<'a> {
	fn drop(&mut self) {
		// If the buffer is mapped, free it
		if let Some(mut buff) = self.buff {
			unsafe {
				libc::munmap(buff.as_mut() as *mut _ as *mut _, self.buff_len);
			}

			// TODO destroy dumb buffer
		}

		// Remove the framebuffer
		let fd = self.card.get_device().as_raw_fd();
		unsafe {
			libc::ioctl(fd, DRM_IOCTL_MODE_RMFB, &self.fb_id);
		}
	}
}
