//! The `GrabServer` request locks the server in order for it to be usable only by the current
//! client.

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::error::Error;
use super::Request;

/// Header of the `GrabServer` request.
#[repr(C, packed)]
pub struct GrabServerHdr {}

/// Structure representing the request
pub struct GrabServer {}

impl Request for GrabServer {
	fn handle(
		&self,
		_ctx: &mut Context,
		_client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), Box<dyn std::error::Error>> {
		// TODO
		Ok(())
	}
}

/// Parses `GrabServer`.
pub fn read(_buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	Ok(Some(Box::new(GrabServer {})))
}
