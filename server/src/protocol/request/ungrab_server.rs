//! The `UngrabServer` request undos the `GrabServer` request.

use crate::ctx::Context;
use crate::ctx::client::Client;
use crate::protocol::error::Error;
use super::Request;

/// Header of the `UngrabServer` request.
#[repr(C, packed)]
pub struct UngrabServerHdr {}

/// Structure representing the request
pub struct UngrabServer {}

impl Request for UngrabServer {
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

/// Parses `UngrabServer`.
pub fn read(_buff: &[u8], _: u8) -> Result<Option<Box<dyn Request>>, Error> {
	Ok(Some(Box::new(UngrabServer {})))
}
