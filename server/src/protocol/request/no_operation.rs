//! The `NoOperation` request does nothing.

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::error::Error;
use super::Request;

/// Header of the `NoOperation` request.
#[repr(C, packed)]
pub struct NoOperationHdr {}

/// Structure representing the request
pub struct NoOperation {}

impl Request for NoOperation {
	fn handle(
		&self,
		_ctx: &mut Context,
		_client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), Box<dyn std::error::Error>> {
		Ok(())
	}
}

/// Parses `NoOperation`.
pub fn read(_buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	Ok(Some(Box::new(NoOperation {})))
}
