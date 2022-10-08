//! This module implements socket communications. Both Unix sockets and network sockets are
//! supported.

use std::io;
use std::net::TcpListener;
use std::net::TcpStream;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;

/// A client's socket stream.
pub enum Stream {
	/// A Unix stream.
	Unix(UnixStream),

	/// A TCP stream.
	Tcp(TcpStream),
}

/// Structure listening for connections.
pub struct Listener {
	/// The Unix listener.
	unix_listener: UnixListener,

	/// The TCP listener.
	tcp_listener: Option<TcpListener>,
}

impl Listener {
	/// Creates a new instance.
	///
	/// Arguments:
	/// - `unix_path` is the path to the Unix socket.
	/// - `tcp_port` is the port on which the . If network listening is not enabled, this argument
	/// must be None.
	pub fn new(unix_path: &str, tcp_port: Option<u16>) -> io::Result<Self> {
		let unix_listener = UnixListener::bind(unix_path)?;
		unix_listener.set_nonblocking(true)?;

		let tcp_listener = match tcp_port {
			Some(tcp_port) => {
				let listener = TcpListener::bind(format!("0.0.0.0:{}", tcp_port))?;
				listener.set_nonblocking(true)?;

				Some(listener)
			},
			None => None,
		};

		Ok(Self {
			unix_listener,

			tcp_listener,
		})
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
