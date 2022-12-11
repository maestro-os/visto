//! Polling allows to avoid wasting CPU cycles reading in a loop on file descriptors.
//! Instead, the process is put to sleep by the kernel until something is available for reading.

use std::os::unix::prelude::AsRawFd;

/// The poll handler takes a list of objects working on a file descriptor and allows to wait for
/// events on them.
///
/// If a file descriptor is closed, it is automaticaly removed from the handler's list.
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
		self.fds.retain(|e| e.fd != obj.as_raw_fd());
	}

	// TODO Try to avoid allocation
	/// Polls on every registered file descriptors, blocking until at least one file descriptor is
	/// ready.
	///
	/// The function returns the list of file descriptors ready for reading.
	pub fn poll(&mut self) -> Vec<i32> {
		unsafe {
			libc::poll(self.fds.as_mut_ptr(), self.fds.len() as _, -1);
		}

		// Remove invalid (closed?) file descriptors from the list
		// FIXME self.fds.retain(|fd| fd.revents | libc::POLLNVAL == 0);

		// Collecting results
		self.fds
			.iter()
			.filter(|fd| fd.revents & libc::POLLIN != 0)
			.map(|fd| fd.fd)
			.collect()
	}
}
