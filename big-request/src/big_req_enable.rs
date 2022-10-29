//! This module implements the `BigReqEnable` request.

use crate::BigRequestReader;
use std::error::Error;
use visto::ctx::Context;
use visto::ctx::client::Client;
use visto::protocol::request::MAX_REQUEST_LEN;
use visto::protocol::request::Request;

/// TODO doc
#[repr(C, packed)]
pub struct BigReqEnableReply {
	/// TODO doc
	type_: u8,
	/// Padding.
	_padding0: u8,
	/// Sequence number.
	seq_nbr: u16,
	/// The length of the reply.
	reply_length: u32,

	/// Maximum length of a request.
	max_request_length: u32,

	/// Padding.
	_padding1: u16,
}

/// TODO doc
pub struct BigReqEnable {}

impl Request for BigReqEnable {
	fn handle(
		&self,
		_ctx: &mut Context,
		client: &mut Client,
	) -> Result<(), Box<dyn Error>> {
		let seq_nbr = client.next_sequence_number();

		client.set_request_reader(Box::new(BigRequestReader {}));

		let reply = BigReqEnableReply {
			type_: 1, // TODO Use constant
			_padding0: 0,
			seq_nbr,
			reply_length: 0,

			max_request_length: (MAX_REQUEST_LEN / 4) as _,

			_padding1: 0,
		};
		client.write_reply(&reply)?;

		Ok(())
	}
}

/// TODO doc
pub fn read(buff: &[u8]) -> Result<Option<Box<dyn Request>>, Box<dyn Error>> {
	if !buff.is_empty() {
		return Err("TODO".into()); // TODO
	}

	Ok(Some(Box::new(BigReqEnable {})))
}
