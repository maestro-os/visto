//! The `GrabServer` request locks the server in order for it to be usable only by the current
//! client.

use super::Request;
use crate::ctx::client::Client;
use crate::ctx::Context;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;

/// Header of the `GrabServer` request.
#[repr(C, packed)]
pub struct GrabServerHdr {}

/// Structure representing the request
pub struct GrabServer {}

impl Request for GrabServer {
	fn handle(
		&self,
		ctx: &mut Context,
		client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), HandleError> {
		ctx.grab_by(client);
		Ok(())
	}
}

/// Parses `GrabServer`.
pub fn read(_buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	Ok(Some(Box::new(GrabServer {})))
}
