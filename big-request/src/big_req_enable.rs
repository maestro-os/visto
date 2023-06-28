//! This `BigReqEnable` request allows to enable big requests.

use crate::BigRequestReader;
use visto::ctx::client::Client;
use visto::ctx::Context;
use visto::protocol;
use visto::protocol::error::Error;
use visto::protocol::request::HandleError;
use visto::protocol::request::Request;
use visto::protocol::request::MAX_REQUEST_LEN;

/// Reply to `BigReqEnable`.
#[repr(C, packed)]
pub struct BigReqEnableReply {
	/// The type of the reply (normal).
	reply_type: u8,
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

/// Structure representing the `BigReqEnable` request.
pub struct BigReqEnable {}

impl Request for BigReqEnable {
	fn handle(
		&self,
		_ctx: &mut Context,
		client: &mut Client,
		seq_nbr: u16,
	) -> Result<(), HandleError> {
		client.set_request_reader(Box::new(BigRequestReader {}));

		let reply = BigReqEnableReply {
			reply_type: protocol::REPLY_TYPE_REPLY,
			_padding0: 0,
			seq_nbr,
			reply_length: 0,

			max_request_length: (MAX_REQUEST_LEN / 4) as _,

			_padding1: 0,
		};
		client.write_obj(&reply).map_err(HandleError::IO)?;

		Ok(())
	}
}

/// Parses `BigReqEnable`.
pub fn read(buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	if buff.is_empty() {
		Ok(Some(Box::new(BigReqEnable {})))
	} else {
		Err(Error::Length)
	}
}
