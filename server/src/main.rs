//! TODO doc

#![feature(linked_list_cursors)]
#![feature(step_trait)]

pub mod ctx;
pub mod extension;
pub mod id_allocator;
pub mod input;
pub mod net;
pub mod output;
pub mod poll;
pub mod protocol;
pub mod screens_layout;
pub mod util;

use ctx::Context;
use ctx::client::Client;
use id_allocator::IDAllocator;
use input::InputManager;
use net::Listener;
use output::card::DRICard;
use poll::PollHandler;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

/// The release number.
pub const RELEASE_NUMBER: u32 = 0;

/// Structure containing command line arguments.
struct Args {
	/// The display number.
	display: usize,

	/// Tells whether the server listens through network.
	network: bool,

	/// Tells whether the cursor is enabled.
	cursor: bool,
}

impl Args {
	fn default() -> Self {
		Self {
			display: 0,

			network: false,

			cursor: true,
		}
	}
}

/// Parses a display descriptor from the given string.
fn parse_display(s: &str) -> Result<usize, String> {
	if let Some(first) = s.chars().next() {
		if first != ':' {
			return Err(format!("Invalid display `{}`", s));
		}
	}

	s[1..]
		.parse::<usize>()
		.map_err(|_| format!("Invalid display `{}`", s))
}

/// Parses command line arguments.
fn parse_args() -> Result<Args, String> {
	let mut args = Args::default();

	// Reading environment variables
	if let Ok(disp) = env::var("DISPLAY") {
		args.display = parse_display(&disp)?;
	}

	let iter = env::args().skip(1);
	for arg in iter {
		match arg.as_str() {
			"-network" => args.network = true,
			"-nocursor" => args.cursor = false,

			_ if matches!(arg.chars().next(), Some(':')) => {
				args.display = parse_display(&arg)?;
			}

			_ => return Err(format!("Invalid argument `{}`", arg)),
		}
	}

	Ok(args)
}

#[allow(dead_code)]
fn main() {
	// Parsing arguments
	let args = parse_args()
		.unwrap_or_else(|e| {
			eprintln!("error parsing arguments: {}", e);
			exit(1);
		});

	// Reading extensions list
	extension::load_extensions_list(Path::new(extension::LIST_PATH))
		.unwrap_or_else(|e| {
			eprintln!("error reading extensions list: {}", e);
			exit(1);
		});

	// Scanning for DRI cards
	let dri_cards = DRICard::scan();

	let mut poll = PollHandler::new();

	// Scanning for input devices
	let mut input_manager = InputManager::new(&mut poll)
		.unwrap_or_else(|e| {
			eprintln!("error initializing input manager: {}", e);
			exit(1);
		});

	// Creating context
	let mut ctx = Context::new();
	ctx.init_screens(&dri_cards, None); // TODO read layout from config if present

	// Creating listener
	let unix_path = PathBuf::from(format!("/tmp/.X11-unix/X{}", args.display));
	let tcp_port = {
		if args.network {
			Some(6000 + args.display as u16)
		} else {
			None
		}
	};
	let mut listener = Listener::new(&unix_path, tcp_port, &mut poll)
		.unwrap_or_else(|e| {
			eprintln!("Cannot listen for incoming connections: {}", e);
			exit(1);
		});

	let mut client_id_allocator = IDAllocator::from_range(0..8192);
	loop {
		// Waiting until something has to be done
		let fds = poll.poll();

		// TODO Add a maximum number of clients

		// Accept a client
		match listener.accept() {
			Ok(Some(stream)) => {
				let id = client_id_allocator.alloc().unwrap(); // TODO Handle error
				let client = Client::new(id, stream);

				ctx.add_client(client, &mut poll);
			}

			Ok(None) => {}

			Err(e) => {
				eprintln!("Failed to accept client connection: {}", e);
			}
		}

		// Ticking clients
		ctx.tick_clients(&mut poll);

		// Handle inputs
		while let Ok(Some(input)) = input_manager.next(&fds) {
			// TODO
			//println!("input: {:?}", input);
		}

		ctx.render();
	}
}
