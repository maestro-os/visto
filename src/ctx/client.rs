//! TODO doc

use crate::net::Stream;

/// A client of the display server.
pub struct Client {
	/// The client's socket.
	stream: Stream,
}

impl Client {
	/// Creates a new instance with the given socket.
	pub fn new(stream: Stream) -> Self {
		Self {
			stream,
		}
	}

	/// Ticks the client.
	pub fn tick(&mut self) {
		// TODO
		todo!();
	}
}
