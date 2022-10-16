//! TODO doc

use crate::ctx::Screen;
use crate::net::Stream;
use crate::protocol::VENDOR_NAME;
use crate::protocol::XRequest;
use crate::protocol::connect::ClientConnect;
use crate::protocol::connect::ConnectFailed;
use crate::protocol::connect::ConnectSuccess;
use crate::protocol::pad;
use crate::protocol;
use crate::util;
use std::io::Read;
use std::io::Write;
use std::io;
use std::mem::size_of;
use std::ptr;

/// The maximum length of a request.
const MAX_REQUEST_LEN: usize = 1 << 16; // TODO Increase?

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
	/// The buffer to read data from the client.
	buff: [u8; MAX_REQUEST_LEN],

	/// The client's state.
	state: ClientState,

	/// Tells whether the client works in MSB first.
	msb_first: bool,
}

impl Client {
	/// Creates a new instance with the given socket.
	pub fn new(stream: Stream) -> Self {
		Self {
			stream,
			buff: [0; MAX_REQUEST_LEN],

			state: ClientState::Waiting,

			msb_first: false,
		}
	}

	/// Returns an immutable reference to the stream associated with the client.
	pub fn get_stream(&self) -> &Stream {
		&self.stream
	}

	/// Writes a connect failed message with the given reason.
	pub fn write_connect_failed(&mut self, reason: &str) -> io::Result<()> {
		println!("New client connection failed: {}", reason);
		self.state = ClientState::ConnectFailed;

		let reason_len = reason.len();
		let additional_data_len = reason_len + pad(reason_len);

		let msg = ConnectFailed {
			reason_len: reason_len as _,

			protocol_major_version: protocol::MAJOR_VERSION,
			protocol_minor_version: protocol::MINOR_VERSION,

			additional_data_len: (additional_data_len / 4) as u16,
		};

		let len = 1 + size_of::<ConnectFailed>() + additional_data_len as usize;
		let mut buf = vec![0; len];

		// Writing data in buffer
		buf[0] = protocol::connect::FAILED;
		unsafe {
			ptr::copy_nonoverlapping::<u8>(
				&msg as *const _ as *const _,
				&mut buf[1],
				size_of::<ConnectFailed>(),
			);
			ptr::copy_nonoverlapping::<u8>(
				reason.as_bytes().as_ptr(),
				&mut buf[1 + size_of::<ConnectFailed>()],
				reason_len,
			);
		}

		self.stream.write(buf.as_slice())?;
		self.stream.flush()
	}

	/// Writes a connect success message with the given reason.
	///
	/// `screens` is the list of screens.
	pub fn write_connect_success(&mut self, screens: &[Screen]) -> io::Result<()> {
		println!("New client connection succeeded");
		self.state = ClientState::ConnectSucess;

		let screens = screens.iter()
			.map(| s | s.to_protocol_screen())
			.collect::<Vec<Vec<u8>>>();
		let screens_len: usize = screens.iter()
			.map(| s | s.len())
			.sum();

		let additional_data_len = 32 + VENDOR_NAME.len()
			+ pad(VENDOR_NAME.len())
			+ 1 * size_of::<protocol::Format>() // TODO
			+ screens_len;

		let msg = ConnectSuccess {
			_padding0: 0,

			protocol_major_version: protocol::MAJOR_VERSION,
			protocol_minor_version: protocol::MINOR_VERSION,

			additional_data_len: (additional_data_len / 4) as _,

			release_number: crate::RELEASE_NUMBER,
			resource_id_base: !0,
			resource_id_mask: !0,
			motion_buffer_size: 0, // TODO
			vendor_length: VENDOR_NAME.len() as _,
			max_request_length: (MAX_REQUEST_LEN / 4) as u16,
			roots_screens_number: 0, // TODO
			pixmap_formats_count: 1, // TODO
			image_byte_order: 1, // MSB first

			bitmap_format_bit_order: 0, // LSB first
			bitmap_format_scanline_unit: 0, // TODO
			bitmap_format_scanline_pad: 0, // TODO

			min_keycode: 8,
			max_keycode: 255,

			/// Padding.
			_padding1: 0,
		};

		// TODO Get from screens
		let format = protocol::Format {
			depth: 24,
			bits_per_pixel: 24,
			scanline_pad: 0,

			_padding: [0; 5],
		};

		let len = 1 + size_of::<ConnectSuccess>() + additional_data_len;
		let mut buf = vec![0; len];

		let mut off = 0;

		buf[off] = protocol::connect::SUCCESS;
		off += 1;

		unsafe {
			ptr::copy_nonoverlapping::<u8>(
				&msg as *const _ as *const _,
				&mut buf[off],
				size_of::<ConnectSuccess>(),
			);
			off += size_of::<ConnectSuccess>();

			let vendor_name = VENDOR_NAME.as_bytes();
			ptr::copy_nonoverlapping::<u8>(
				vendor_name.as_ptr(),
				&mut buf[off],
				vendor_name.len(),
			);
			off += vendor_name.len() + pad(vendor_name.len());
			println!("=> {off} {:?}", &buf[..off]);

			ptr::copy_nonoverlapping::<u8>(
				&format as *const _ as *const u8,
				&mut buf[off],
				size_of::<protocol::Format>(),
			);
			off += size_of::<protocol::Format>();
		}

		for s in screens {
			unsafe {
				ptr::copy_nonoverlapping::<u8>(
					s.as_ptr() as *const u8,
					&mut buf[off],
					s.len(),
				);
			}

			off += s.len();
		}
		// TODO println!("-> {:?}", buf);

		self.stream.write(buf.as_slice())?;
		self.stream.flush()
	}

	/// Handles an incoming connect request, if any.
	///
	/// `screens` is the list of screens.
	fn handle_connect_request(&mut self, screens: &[Screen]) -> io::Result<()> {
		// Reading request header
		let len = self.stream.peek(&mut self.buff)?;
		if len < size_of::<ClientConnect>() {
			return Ok(());
		}
		let hdr: &ClientConnect = unsafe {
			util::reinterpret(&self.buff[0])
		};

		// If not enough bytes are available, return
		let required_len = size_of::<ClientConnect>()
			+ hdr.authorization_protocol_name_length as usize
			+ pad(hdr.authorization_protocol_name_length as usize)
			+ hdr.authorization_protocol_data_length as usize
			+ pad(hdr.authorization_protocol_data_length as usize);
		if len < required_len {
			return Ok(());
		}
		// Discard remaining bytes
		self.stream.read(&mut self.buff[..required_len])?;

		// Reading request
		match hdr.byte_order {
			protocol::connect::MSB_FIRST => self.msb_first = true,
			protocol::connect::LSB_FIRST => self.msb_first = false,

			// Invalid value
			_ => {
				self.write_connect_failed("Invalid byte_order value")?;
				return Ok(());
			},
		}

		// Checking the protocol version is correct
		let maj_ver = hdr.protocol_major_version;
		let min_ver = hdr.protocol_minor_version;
		if maj_ver != protocol::MAJOR_VERSION {
			self.write_connect_failed(format!(
				"Unsupported protocol version: {}.{}",
				maj_ver,
				min_ver
			).as_str())?;
			return Ok(());
		}

		self.write_connect_success(screens)
	}

	/// Handles an incoming request, if any.
	fn handle_request(&mut self) -> io::Result<()> {
		// Reading request header
		let len = self.stream.peek(&mut self.buff)?;
		if len < size_of::<XRequest>() {
			return Ok(());
		}
		let hdr: &XRequest = unsafe {
			util::reinterpret(&self.buff[0])
		};

		// If not enough bytes are available, return
		let required_len = size_of::<XRequest>(); // TODO
		if len < required_len {
			return Ok(());
		}
		// Discard remaining bytes
		self.stream.read(&mut self.buff[..required_len])?;

		// TODO
		let opcode = hdr.major_opcode;
		println!("=> {}", opcode);

		Ok(())
	}

	/// Ticks the client.
	///
	/// `screens` is the list of screens.
	pub fn tick(&mut self, screens: &[Screen]) -> io::Result<()> {
		// TODO Delete the client if the socket is dead
		// TODO Notify client of event if necessary

		// Reading input data
		match self.state {
			ClientState::Waiting | ClientState::ConnectFailed => {
				self.handle_connect_request(screens)
			},

			ClientState::ConnectSucess => {
				self.handle_request()
			},
		}
	}
}
