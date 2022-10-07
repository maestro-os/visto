//! TODO doc

use std::env;
use std::process::exit;

mod ctx;
mod protocol;

/// Structure containing command line arguments.
struct Args {
	/// The display number.
	display: usize,

	/// Tells whether the server listens through network.
	listen: bool,

	/// Tells whether the cursor is enabled.
	cursor: bool,
}

impl Args {
	fn default() -> Self {
		Self {
			display: 0,

			listen: false,

			cursor: true,
		}
	}
}

/// Parses command line arguments.
fn parse_args() -> Args {
	let mut args = Args::default();

	let iter = env::args().skip(1);
	for arg in iter {
		match arg.as_str() {
			"-listen" => args.listen = true,
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
	let _args = parse_args();

	// TODO Modesetting
	// TODO Get screen(s) resolution
	// TODO Create context
	// TODO Create socket(s)
}
