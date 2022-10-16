//! TODO doc

#![feature(linked_list_cursors)]
#![feature(unix_socket_peek)]

mod ctx;
mod drm;
mod net;
mod protocol;
mod util;

use ctx::Context;
use ctx::client::Client;
use net::Listener;
use std::env;
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

/// Parses command line arguments.
fn parse_args() -> Args {
	let mut args = Args::default();

	// TODO Read environment variables

	let iter = env::args().skip(1);
	for arg in iter {
		match arg.as_str() {
			"-network" => args.network = true,
			"-nocursor" => args.cursor = false,

			_ if matches!(arg.chars().next(), Some(':')) => {
				args.display = arg[1..].parse::<usize>().unwrap_or_else(| _ | {
					eprintln!("Invalid display `{}`", arg);
					exit(1);
				});
			},

			_ => {
				eprintln!("Invalid argument `{}`", arg);
				exit(1);
			},
		}
	}

	args
}

fn main() {
	// Parsing arguments
	let args = parse_args();

	// Creating context
	let mut ctx = Context::new();
	ctx.scan_screens();

	// Creating listener
	let unix_path = format!("/tmp/.X11-unix/X{}", args.display);
	let tcp_port = {
		if args.network {
			Some(6000 + args.display as u16)
		} else {
			None
		}
	};
	let mut listener = Listener::new(&unix_path, tcp_port)
		.unwrap_or_else(| e | {
			eprintln!("Cannot listen for incoming connections: {}", e);
			exit(1);
		});

	loop {
		// Waiting until something has to be done
		listener.get_poll_handler().poll();

		// TODO Add a maximum number of clients

		// Accept a client
		match listener.accept() {
			Ok(Some(stream)) => {
				let client = Client::new(stream);
				ctx.add_client(client, listener.get_poll_handler());
			},

			Ok(None) => {},

			Err(e) => {
				eprintln!("Failed to accept client connection: {}", e);
			},
		}

		// TODO Listen for keyboard/mouse input

		// Ticking clients
		ctx.tick_clients(listener.get_poll_handler());

		// TODO Render if necessary
	}
}
