//! Extension `BIG-REQUEST` allows to increase the size of requests by modifying the header.

use visto::extension::Extension;
use visto::protocol::request::RequestReader;

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
	fn read(&self, buff: &[u8]) -> Result<Option<(Box<dyn Request>, usize)>, Box<dyn Error>> {
		// If not enough bytes are available, return
		let mut hdr_len = size_of::<XRequest>();
		if buff.len() < req {
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

		// If the request is too long, ignore it
		if req > MAX_REQUEST_LEN {
			// TODO
			todo!();
		}
		// If not enough bytes are available, return
		if buff.len() < req {
			return Ok(None);
		}

		let opcode = hdr.major_opcode;
		let buff = &buff[hdr_len..];

		let request = self.handle(opcode, buff)?;
		Ok(request.map(|r| (r, req)))

	}
}

#[no_mangle]
pub extern fn init(ext: &Extension) -> bool {
	// TODO
	true
}

#[no_mangle]
pub extern fn fini() {
	// TODO
	todo!();
}
