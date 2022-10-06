//! TODO doc

pub mod client;
pub mod screen;
pub mod window;

use client::Client;
use screen::Screen;
use window::Window;

/// Structure representing a context.
pub struct Context {
	/// The list of screens.
	screens: Vec<Screen>,
	/// The list of windows.
	windows: Vec<Window>,

	/// The list of clients.
	clients: Vec<Client>,

	// TODO
}
