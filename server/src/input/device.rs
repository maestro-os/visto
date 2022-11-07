//! TODO doc

use crate::util;
use std::ffi::c_int;
use std::ffi::c_short;
use std::fs::File;
use std::io::Read;
use std::io;
use std::mem::size_of;
use std::os::unix::prelude::AsRawFd;
use std::path::PathBuf;

// TODO Allow buffering of several events at once

/// EvDev notifies events in the format represented by this structure.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct EvDevInputEvent {
	time: libc::timeval,
	r#type: c_short,
	code: c_short,
	value: c_int,
}

/// Structure representing an input device.
pub struct InputDevice {
	/// The device's file.
	file: File,

	/// A buffer storing a partial event structure.
	buff: [u8; size_of::<EvDevInputEvent>()],
	/// The cursor on the buffer.
	cursor: usize,
}

impl InputDevice {
	/// Returns a device from the given device file's path.
	pub fn from_path(path: &PathBuf) -> io::Result<Self> {
		Ok(Self {
			file: File::open(path)?,

			buff: [0; size_of::<EvDevInputEvent>()],
			cursor: 0,
		})
	}

	/// Returns the next event. The function blocks until at least one event is available.
	///
	/// If EOF has been reached, the function returns None.
	pub fn next(&mut self) -> io::Result<Option<EvDevInputEvent>> {
		loop {
			let len = self.file.read(&mut self.buff[self.cursor..])?;
			if len == 0 {
				break;
			}

			self.cursor += len;
		}

		if self.cursor >= size_of::<EvDevInputEvent>() {
			let ev = *unsafe {
				util::reinterpret(&self.buff)
			};
			self.cursor = 0;

			Ok(Some(ev))
		} else {
			Ok(None)
		}
	}
}

impl AsRawFd for InputDevice {
	fn as_raw_fd(&self) -> i32 {
		self.file.as_raw_fd()
	}
}
