//! This module implements support for X protocol clients.

use crate::ctx::gc::GC;
use crate::ctx::Context;
use crate::ctx::Screen;
use crate::net::Stream;
use crate::protocol;
use crate::protocol::connect::ClientConnect;
use crate::protocol::connect::ConnectFailed;
use crate::protocol::connect::ConnectSuccess;
use crate::protocol::pad;
use crate::protocol::request::DefaultRequestReader;
use crate::protocol::request::HandleError;
use crate::protocol::request::RequestReader;
use crate::protocol::request::MAX_REQUEST_LEN;
use crate::protocol::VENDOR_NAME;
use crate::util;
use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem::size_of;
use std::num::Wrapping;
use std::ptr;
use std::slice;

/// The state of a client.
pub enum ClientState {
	/// The server is waiting for a connect request.
	Waiting,

	/// Connection succeeded.
	ConnectSuccess,
	/// Connection failed.
	ConnectFailed,
}

/// A client of the display server.
pub struct Client {
	/// The ID of the client.
	id: u32,
	/// The client's socket.
	stream: Stream,

	/// The buffer to read data from the client.
	buff: Vec<u8>,
	/// The cursor on the buffer.
	buff_cursor: usize,

	/// The client's state.
	state: ClientState,

	/// Tells whether the client works in MSB first.
	msb_first: bool,

	/// The last sequence number.
	sequence_number: Wrapping<u16>,

	/// The current request reader. Changing this value allows to change the behaviour when reading
	/// requests.
	request_reader: Box<dyn RequestReader>,

	/// The list of Graphics Contexts. The key is the ID of the context.
	gcs: HashMap<u32, GC>,
}

impl Client {
	/// Creates a new instance with the given socket.
	///
	/// Arguments:
	/// - `id` is the ID of the client.
	/// - `stream` is the I/O stream associated to the client.
	pub fn new(id: u32, stream: Stream) -> Self {
		Self {
			id,
			stream,

			buff: vec![0; MAX_REQUEST_LEN],
			buff_cursor: 0,

			state: ClientState::Waiting,

			msb_first: false,

			sequence_number: Wrapping(0),

			request_reader: Box::new(DefaultRequestReader {}),

			gcs: HashMap::new(),
		}
	}

	/// Returns the ID of the client.
	pub fn get_id(&self) -> u32 {
		self.id
	}

	/// Returns an immutable reference to the stream associated with the client.
	pub fn get_stream(&self) -> &Stream {
		&self.stream
	}

	/// Returns the next sequence number.
	fn next_sequence_number(&mut self) -> u16 {
		self.sequence_number += 1;
		self.sequence_number.0
	}

	/// Writes the given slice.
	pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
		self.stream.write_all(data)?;
		self.stream.flush()
	}

	/// Writes the given object.
	pub fn write_obj<T>(&mut self, obj: &T) -> io::Result<()> {
		let slice = unsafe { slice::from_raw_parts(obj as *const _ as *const u8, size_of::<T>()) };

		// Adding padding to make requests at least 32 bytes long
		let mut data = vec![0; max(slice.len(), 32)];
		data[..slice.len()].copy_from_slice(slice);

		self.write(&data)
	}

	/// Writes a connect failed message with the given reason.
	pub fn write_connect_failed(&mut self, reason: &str) -> io::Result<()> {
		eprintln!("New client connection failed: {}", reason);
		self.state = ClientState::ConnectFailed;

		let reason_len = reason.len();
		let additional_data_len = reason_len + pad(reason_len);

		let msg = ConnectFailed {
			reason_len: reason_len as _,

			protocol_major_version: protocol::MAJOR_VERSION,
			protocol_minor_version: protocol::MINOR_VERSION,

			additional_data_len: (additional_data_len / 4) as u16,
		};

		let len = 1 + size_of::<ConnectFailed>() + additional_data_len;
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

		self.stream.write_all(&buf)?;
		self.stream.flush()
	}

	/// Writes a connect success message with the given reason.
	///
	/// `screens` is the list of screens.
	pub fn write_connect_success(&mut self, screens: &[Screen]) -> io::Result<()> {
		println!("New client connection succeeded");
		self.state = ClientState::ConnectSuccess;

		let screens = screens
			.iter()
			.map(|s| s.to_protocol_screen())
			.collect::<Vec<Vec<u8>>>();
		let screens_len: usize = screens.iter().map(|s| s.len()).sum();

		let additional_data_len = 32
			+ VENDOR_NAME.len()
			+ pad(VENDOR_NAME.len())
			+ 1 * size_of::<protocol::Format>() // TODO
			+ screens_len;

		let msg = ConnectSuccess {
			_padding0: 0,

			protocol_major_version: protocol::MAJOR_VERSION,
			protocol_minor_version: protocol::MINOR_VERSION,

			additional_data_len: (additional_data_len / 4) as _,

			release_number: crate::RELEASE_NUMBER,
			resource_id_base: 0xfffffff, // TODO use
			resource_id_mask: 0xfffffff, // TODO use
			motion_buffer_size: 0,       // TODO
			vendor_length: VENDOR_NAME.len() as _,
			max_request_length: u16::MAX,
			roots_screens_number: 1, // TODO
			pixmap_formats_count: 1, // TODO
			image_byte_order: 1,     // MSB first

			bitmap_format_bit_order: 0,      // LSB first
			bitmap_format_scanline_unit: 32, // TODO
			bitmap_format_scanline_pad: 8,   // TODO

			min_keycode: 8,
			max_keycode: 255,

			/// Padding.
			_padding1: 0,
		};

		// TODO Get from screens
		let format = protocol::Format {
			depth: 32,
			bits_per_pixel: 24,
			scanline_pad: 8,

			_padding: [0; 5],
		};

		let len = 8 + additional_data_len;
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
			ptr::copy_nonoverlapping::<u8>(vendor_name.as_ptr(), &mut buf[off], vendor_name.len());
			off += vendor_name.len() + pad(vendor_name.len());

			ptr::copy_nonoverlapping::<u8>(
				&format as *const _ as *const u8,
				&mut buf[off],
				size_of::<protocol::Format>(),
			);
			off += size_of::<protocol::Format>();
		}

		for s in screens {
			unsafe {
				ptr::copy_nonoverlapping::<u8>(s.as_ptr() as *const u8, &mut buf[off], s.len());
			}
			off += s.len();
		}

		self.stream.write_all(&buf)?;
		self.stream.flush()
	}

	/// Handles an incoming connect request, if any.
	///
	/// `screens` is the list of screens.
	fn handle_connect_request(&mut self, screens: &[Screen]) -> io::Result<()> {
		if self.buff_cursor < size_of::<ClientConnect>() {
			return Ok(());
		}
		let hdr: &ClientConnect = unsafe { util::reinterpret(&self.buff[0]) };

		// If not enough bytes are available, return
		let required_len = size_of::<ClientConnect>()
			+ hdr.authorization_protocol_name_length as usize
			+ pad(hdr.authorization_protocol_name_length as usize)
			+ hdr.authorization_protocol_data_length as usize
			+ pad(hdr.authorization_protocol_data_length as usize);
		if self.buff_cursor < required_len {
			return Ok(());
		}

		// Reading request
		match hdr.byte_order {
			protocol::connect::MSB_FIRST => self.msb_first = true,
			protocol::connect::LSB_FIRST => self.msb_first = false,

			// Invalid value
			_ => {
				self.write_connect_failed("Invalid byte_order value")?;
				return Ok(());
			}
		}

		// Checking the protocol version is correct
		let maj_ver = hdr.protocol_major_version;
		let min_ver = hdr.protocol_minor_version;
		if maj_ver != protocol::MAJOR_VERSION {
			self.write_connect_failed(
				format!("Unsupported protocol version: {}.{}", maj_ver, min_ver).as_str(),
			)?;
			return Ok(());
		}

		// Discarding used data
		self.buff.rotate_left(required_len);
		self.buff_cursor -= required_len;

		self.write_connect_success(screens)
	}

	/// Handles an incoming request, if any.
	///
	/// `ctx` is the current context.
	fn handle_request(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		loop {
			let buff = &self.buff[..self.buff_cursor];
			if buff.is_empty() {
				return Ok(());
			}

			match self.request_reader.read(ctx, buff) {
				// Handle request
				Ok(Some((request, len))) => {
					// Discarding used data
					self.buff.rotate_left(len);
					self.buff_cursor -= len;

					let seq = self.next_sequence_number();

					// Handle the request
					match request.handle(ctx, self, seq) {
						Ok(_) => {}

						// Client error, send
						Err(HandleError::Client(e)) => {
							// TODO opcode
							let e = e.to_protocol(seq, 0, 0);
							self.write_obj(&e)?;
						}

						// IO error, close connection
						Err(HandleError::IO(e)) => return Err(Box::new(e)),
					}
				}

				// No request to handle, break
				Ok(None) => break,

				// Handle error
				Err(e) => self.write_obj(&e)?,
			}
		}

		Ok(())
	}

	/// Ticks the client.
	///
	/// `ctx` is the current context.
	pub fn tick(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		// Reading incoming data
		if self.buff_cursor < self.buff.len() {
			let len = self.stream.read(&mut self.buff[self.buff_cursor..])?;
			self.buff_cursor += len;
		}

		// TODO Notify client of event if necessary

		// Reading input data
		match self.state {
			ClientState::Waiting | ClientState::ConnectFailed => {
				self.handle_connect_request(&ctx.screens)?;
			}

			ClientState::ConnectSuccess => {
				self.handle_request(ctx)?;
			}
		}

		Ok(())
	}

	/// Sets the request reader for the client.
	pub fn set_request_reader(&mut self, reader: Box<dyn RequestReader>) {
		self.request_reader = reader;
	}

	/// Sets a Graphics Context `gc` with the given ID `cid`.
	pub fn set_gc(&mut self, cid: u32, gc: GC) {
		self.gcs.insert(cid, gc);
	}
}
