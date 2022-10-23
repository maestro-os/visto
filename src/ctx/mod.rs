//! TODO doc

pub mod client;
pub mod screen;
pub mod window;

use client::Client;
use crate::drm;
use crate::net::poll::PollHandler;
use screen::Screen;
use std::collections::LinkedList;
use window::Window;

/// Structure representing a context.
pub struct Context {
	/// The list of screens.
	screens: Vec<Screen>,
	/// The list of windows.
	windows: Vec<Window>,

	/// The list of clients.
	clients: LinkedList<Client>,
}

impl Context {
	/// Creates a new instance.
	pub fn new() -> Self {
		Self {
			screens: Vec::new(),
			windows: Vec::new(),

			clients: LinkedList::new(),
		}
	}

	/// Scans for screens on DRM.
	pub fn scan_screens(&mut self) {
		self.screens.clear();

		for dev in drm::DRICard::scan() {
			// TODO Remove `take`
			for conn in drm::DRIConnector::scan(&dev).into_iter().take(1) {
				let root = Window::new_root();
				self.windows.push(root);
				let root_id = 0; // TODO

				let screen = Screen::new(conn, root_id);
				self.screens.push(screen);
			}
		}
	}

	/// Returns an immutable reference to the list of screens.
	pub fn get_screens(&self) -> &[Screen] {
		&self.screens
	}

	/// Returns a mutable reference to the list of screens.
	pub fn get_screens_mut(&mut self) -> &mut [Screen] {
		&mut self.screens
	}

	/// Adds a new client.
	///
	/// `poll_handler` is the poll handler on which the stream is to be registered.
	pub fn add_client(&mut self, client: Client, poll_handler: &mut PollHandler) {
		poll_handler.add_fd(client.get_stream());
		self.clients.push_back(client);
	}

	/// Ticks every connected client.
	///
	/// `poll_handler` is the poll handler on which the stream is to be registered.
	pub fn tick_clients(&mut self, poll_handler: &mut PollHandler) {
		let mut cursor = self.clients.cursor_front_mut();

		while let Some(client) = cursor.current() {
			match client.tick(&self.screens) {
				// On error, remove client
				Err(e) => {
					println!("Client disconnect: {}", e);

					if let Some(removed) = cursor.remove_current() {
						poll_handler.remove_fd(removed.get_stream());
					}
				},

				_ => {},
			}

			cursor.move_next();
		}
	}
}
