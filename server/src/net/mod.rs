//! This module implements socket communications. Both Unix sockets and network sockets are
//! supported.

pub mod poll;

use poll::PollHandler;
use std::io::Read;
use std::io::Write;
use std::io;
use std::net::TcpListener;
use std::net::TcpStream;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::os::unix::prelude::AsRawFd;
use std::os::unix::prelude::RawFd;

/// A client's socket stream.
pub enum Stream {
	/// A Unix stream.
	Unix(UnixStream),

	/// A TCP stream.
	Tcp(TcpStream),
}

impl Read for Stream {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		let len = match self {
			Self::Unix(s) => s.read(buf),
			Self::Tcp(s) => s.read(buf),
		}?;
		println!("read: {:?}", &buf[..len]);
		Ok(len)
	}
}

impl Write for Stream {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		println!("write: {:?}", buf);
		match self {
			Self::Unix(s) => s.write(buf),
			Self::Tcp(s) => s.write(buf),
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		match self {
			Self::Unix(s) => s.flush(),
			Self::Tcp(s) => s.flush(),
		}
	}
}

impl AsRawFd for Stream {
	fn as_raw_fd(&self) -> RawFd {
		match self {
			Self::Unix(s) => s.as_raw_fd(),
			Self::Tcp(s) => s.as_raw_fd(),
		}
	}
}

/// Structure listening for connections.
pub struct Listener {
	/// The Unix listener.
	unix_listener: UnixListener,
	/// The TCP listener.
	tcp_listener: Option<TcpListener>,

	/// The poll handler, used to waiting until something is to be done.
	poll_handler: PollHandler,
}

impl Listener {
	/// Creates a new instance.
	///
	/// Arguments:
	/// - `unix_path` is the path to the Unix socket.
	/// - `tcp_port` is the port on which the . If network listening is not enabled, this argument
	/// must be None.
	pub fn new(unix_path: &str, tcp_port: Option<u16>) -> io::Result<Self> {
		let mut poll_handler = PollHandler::new();

		let unix_listener = UnixListener::bind(unix_path)?;
		unix_listener.set_nonblocking(true)?;
		poll_handler.add_fd(&unix_listener);

		let tcp_listener = match tcp_port {
			Some(tcp_port) => {
				let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", tcp_port))?;
				tcp_listener.set_nonblocking(true)?;
				poll_handler.add_fd(&tcp_listener);

				Some(tcp_listener)
			},
			None => None,
		};

		Ok(Self {
			unix_listener,
			tcp_listener,

			poll_handler,
		})
	}

	/// Returns a mutable reference to the poll handler associated with the listener.
	pub fn get_poll_handler(&mut self) -> &mut PollHandler {
		&mut self.poll_handler
	}

	/// Accepts a new connection. This function is nonblocking and returns None if no new
	/// connection is available.
	pub fn accept(&self) -> io::Result<Option<Stream>> {
		match self.unix_listener.accept() {
			Ok((stream, _)) => return Ok(Some(Stream::Unix(stream))),

			// Try the TCP socket if present
			Err(e) if e.kind() == io::ErrorKind::WouldBlock => {},

			Err(e) => return Err(e),
		}

		if let Some(tcp_listener) = &self.tcp_listener {
			match tcp_listener.accept() {
				Ok((stream, _)) => return Ok(Some(Stream::Tcp(stream))),

				// No new client
				Err(e) if e.kind() == io::ErrorKind::WouldBlock => {},

				Err(e) => return Err(e),
			}
		}

		Ok(None)
	}
}
