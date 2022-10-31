//! Extension `BIG-REQUEST` allows to increase the size of requests by modifying the header.

mod big_req_enable;

use std::mem::size_of;
use visto::ctx::Context;
use visto::extension::Extension;
use visto::protocol::XRequest;
use visto::protocol::error::Error;
use visto::protocol::error::XError;
use visto::protocol::request::MAX_REQUEST_LEN;
use visto::protocol::request::Request;
use visto::protocol::request::RequestReader;
use visto::protocol::request;
use visto::util;

/// The big request header.
#[repr(C, packed)]
struct BigRequestHdr {
	/// The normal header.
	hdr: XRequest,

	/// The extended size.
	extended_length: u32,
}

/// Reader for big requests.
pub struct BigRequestReader {}

impl RequestReader for BigRequestReader {
	fn read(
		&self,
		ctx: &Context,
		buff: &[u8],
	) -> Result<Option<(Box<dyn Request>, usize)>, XError> {
		// If not enough bytes are available, return
		let mut hdr_len = size_of::<XRequest>();
		if buff.len() < hdr_len {
			return Ok(None);
		}

		let hdr: &XRequest = unsafe {
			util::reinterpret(&buff[0])
		};
		// Required number of bytes
		let mut req = hdr.length as usize * 4;
		if req == 0 {
			if buff.len() < size_of::<BigRequestHdr>() {
				return Ok(None);
			}
			let hdr: &BigRequestHdr = unsafe {
				util::reinterpret(&buff[0])
			};

			req = hdr.extended_length as usize * 4;
			hdr_len += 4;
		}

		let opcode = hdr.major_opcode;

		// If the request is too long, ignore it
		if req > MAX_REQUEST_LEN {
			let err = Error::Length;
			// TODO seq nbr
			return Err(err.to_protocol(0, 0, opcode));
		}
		// If not enough bytes are available, return
		if buff.len() < req {
			return Ok(None);
		}

		let opcode = hdr.major_opcode;
		let buff = &buff[hdr_len..req];

		match request::build_request(ctx, opcode, buff, hdr.optional) {
			Ok(request) => Ok(request.map(|r| (r, req))),

			// TODO seq nbr
			Err(e) => Err(e.to_protocol(0, 0, opcode)),
		}

	}
}

#[no_mangle]
pub extern fn init(ctx: &mut Context, ext: &Extension) -> bool {
	ctx.get_custom_requests_mut().insert(ext.get_major_opcode(), Box::new(&big_req_enable::read));
	true
}

#[no_mangle]
pub extern fn fini() {
	// TODO Unregister request
}
