//! TODO doc

use crate::net::Stream;
use crate::protocol::XRequest;
use std::io;
use std::mem::size_of;

/// The state of a client.
pub enum ClientState {
	/// The server is waiting for a connect request.
	Waiting,

	/// Connection succeeded.
	ConnectSucess,
	/// Connection failed.
	ConnectFailed,
}

/// A client of the display server.
pub struct Client {
	/// The client's socket.
	stream: Stream,

	/// The client's state.
	state: ClientState,
}

impl Client {
	/// Creates a new instance with the given socket.
	pub fn new(stream: Stream) -> Self {
		Self {
			stream,

			state: ClientState::Waiting,
		}
	}

	/// Handles an incoming request, if any.
	fn handle_request(&self) -> io::Result<()> {
		// Reading request header
		let mut buff: [u8; size_of::<XRequest>()] = [0; size_of::<XRequest>()];
		let len = self.stream.peek(&mut buff)?;
		if len < buff.len() {
			return Ok(());
		}
		let _request_hdr = unsafe {
			&*(buff.as_ptr() as *const XRequest)
		};

		// TODO Read the whole request

		Ok(())
	}

	/// Ticks the client.
	pub fn tick(&mut self) -> io::Result<()> {
		// TODO Notify client of event if necessary

		// Reading input data
		match self.state {
			ClientState::Waiting | ClientState::ConnectFailed => {
				// TODO Wait for a connection request
				Ok(())
			},

			ClientState::ConnectSucess => {
				self.handle_request()
			},
		}
	}
}
