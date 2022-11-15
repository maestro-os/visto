//! The `UngrabServer` request undos the `GrabServer` request.

use super::Request;
use crate::ctx::client::Client;
use crate::ctx::Context;
use crate::protocol::error::Error;
use crate::protocol::request::HandleError;

/// Header of the `UngrabServer` request.
#[repr(C, packed)]
pub struct UngrabServerHdr {}

/// Structure representing the request
pub struct UngrabServer {}

impl Request for UngrabServer {
	fn handle(
		&self,
		ctx: &mut Context,
		_client: &mut Client,
		_seq_nbr: u16,
	) -> Result<(), HandleError> {
		ctx.ungrab();
		Ok(())
	}
}

/// Parses `UngrabServer`.
pub fn read(_buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	Ok(Some(Box::new(UngrabServer {})))
}
