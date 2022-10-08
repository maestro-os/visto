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

impl Context {
	/// Creates a new instance.
	pub fn new() -> Self {
		Self {
			screens: Vec::new(),
			windows: Vec::new(),

			clients: Vec::new(),
		}
	}

	/// Adds a new client.
	pub fn add_client(&mut self, client: Client) {
		self.clients.push(client);
	}

	/// TODO doc
	pub fn tick_clients(&mut self) {
		for c in &mut self.clients {
			c.tick();
		}
	}
}
