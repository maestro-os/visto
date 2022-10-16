//! This module implements polling, allowing to make the process wait until something must be done.

use std::os::unix::prelude::AsRawFd;

/// Structure handling polling.
pub struct PollHandler {
	/// The list of file descriptors to poll onto.
	fds: Vec<libc::pollfd>,
}

impl PollHandler {
	/// Creates a new instance.
	pub fn new() -> Self {
		Self {
			fds: Vec::new(),
		}
	}

	/// Adds the file descriptor of the given object.
	pub fn add_fd<T: AsRawFd>(&mut self, obj: &T) {
		self.fds.push(libc::pollfd {
			fd: obj.as_raw_fd(),
			events: libc::POLLIN,
			revents: 0,
		});
	}

	/// Removes the file descriptor of the given object.
	///
	/// If the object isn't in being polled, the function does nothing.
	pub fn remove_fd<T: AsRawFd>(&mut self, obj: &T) {
		self.fds.retain(| e | {
			e.fd != obj.as_raw_fd()
		});
	}

	/// Polls on every registered file descriptors.
	///
	/// This function blocks until at least one file descriptor is ready.
	pub fn poll(&mut self) {
		unsafe {
			libc::poll(self.fds.as_mut_ptr(), self.fds.len() as _, -1);
		}
	}
}
