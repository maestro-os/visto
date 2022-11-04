//! TODO doc

pub mod client;
pub mod gc;
pub mod pointer;
pub mod screen;
pub mod window;

use client::Client;
use crate::output::card::DRICard;
use crate::output::connector::DRIConnector;
use crate::poll::PollHandler;
use crate::protocol::Rectangle;
use crate::protocol::request::RequestReadFn;
use screen::Screen;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::num::NonZeroU32;
use window::Window;

// TODO Move in its own module?
/// Trait representing an object that can be drawn.
pub trait Drawable {
	/// Returns the number of bits used to represent a pixel.
	fn get_depth(&self) -> u8;

	/// Returns the root of the drawable.
	fn get_root(&self) -> u32;

	/// Returns the rectangle representing the position and dimensions of the drawable.
	fn get_rectangle(&self) -> Rectangle;

	/// Returns the width of the border.
	fn get_border_width(&self) -> u16;
}

// TODO Move in its own module?
/// TODO doc
pub struct Selection {
	/// The window ID of the owner of the selection.
	owner: Option<NonZeroU32>,

	// TODO
}

impl Selection {
	/// Returns the owner of the selection. If the selection has no owner, the function
	/// returns None.
	pub fn get_owner(&self) -> Option<NonZeroU32> {
		self.owner
	}
}

/// Structure representing a context.
pub struct Context {
	/// The list of screens.
	screens: Vec<Screen>,
	/// The list of windows.
	windows: HashMap<u32, Window>,

	/// The list of atoms on the server. The key is the ID of the atom.
	atoms: HashMap<u32, String>,
	/// The list of selections on the server. The key is the name of the selection.
	selections: HashMap<String, Selection>,

	/// The list of clients.
	/// An unsafe cell is used to allow double borrow of the context.
	clients: UnsafeCell<LinkedList<Client>>,
	/// The client currently grabbing the server.
	grabbing_client: Option<u32>,

	/// Requests handlers registered by extensions.
	/// The key is the major opcode and the value is the handler.
	custom_requests: HashMap<u8, Box<RequestReadFn>>,
}

impl Context {
	/// Creates a new instance.
	pub fn new() -> Self {
		Self {
			screens: Vec::new(),
			windows: HashMap::new(),

			atoms: HashMap::from([
				(1, "PRIMARY".to_owned()),
				(2, "SECONDARY".to_owned()),
				(3, "ARC".to_owned()),
				(4, "ATOM".to_owned()),
				(5, "BITMAP".to_owned()),
				(6, "CARDINAL".to_owned()),
				(7, "COLORMAP".to_owned()),
				(8, "CURSOR".to_owned()),
				(9, "CUT_BUFFER0".to_owned()),
				(10, "CUT_BUFFER1".to_owned()),
				(11, "CUT_BUFFER2".to_owned()),
				(12, "CUT_BUFFER3".to_owned()),
				(13, "CUT_BUFFER4".to_owned()),
				(14, "CUT_BUFFER5".to_owned()),
				(15, "CUT_BUFFER6".to_owned()),
				(16, "CUT_BUFFER7".to_owned()),
				(17, "DRAWABLE".to_owned()),
				(18, "FONT".to_owned()),
				(19, "INTEGER".to_owned()),
				(20, "PIXMAP".to_owned()),
				(21, "POINT".to_owned()),
				(22, "RECTANGLE".to_owned()),
				(23, "RESOURCE_MANAGER".to_owned()),
				(24, "RGB_COLOR_MAP".to_owned()),
				(25, "RGB_BEST_MAP".to_owned()),
				(26, "RGB_BLUE_MAP".to_owned()),
				(27, "RGB_DEFAULT_MAP".to_owned()),
				(28, "RGB_GRAY_MAP".to_owned()),
				(29, "RGB_GREEN_MAP".to_owned()),
				(30, "RGB_RED_MAP".to_owned()),
				(31, "STRING".to_owned()),
				(32, "VISUALID".to_owned()),
				(33, "WINDOW".to_owned()),
				(34, "WM_COMMAND".to_owned()),
				(35, "WM_HINTS".to_owned()),
				(36, "WM_CLIENT_MACHINE".to_owned()),
				(37, "WM_ICON_NAME".to_owned()),
				(38, "WM_ICON_SIZE".to_owned()),
				(39, "WM_NAME".to_owned()),
				(40, "WM_NORMAL_HINTS".to_owned()),
				(41, "WM_SIZE_HINTS".to_owned()),
				(42, "WM_ZOOM_HINTS".to_owned()),
				(43, "MIN_SPACE".to_owned()),
				(44, "NORM_SPACE".to_owned()),
				(45, "MAX_SPACE".to_owned()),
				(46, "END_SPACE".to_owned()),
				(47, "SUPERSCRIPT_X".to_owned()),
				(48, "SUPERSCRIPT_Y".to_owned()),
				(49, "SUBSCRIPT_X".to_owned()),
				(50, "SUBSCRIPT_Y".to_owned()),
				(51, "UNDERLINE_POSITION".to_owned()),
				(52, "UNDERLINE_THICKNESS".to_owned()),
				(53, "STRIKEOUT_ASCENT".to_owned()),
				(54, "STRIKEOUT_DESCENT".to_owned()),
				(55, "ITALIC_ANGLE".to_owned()),
				(56, "X_HEIGHT".to_owned()),
				(57, "QUAD_WIDTH".to_owned()),
				(58, "WEIGHT".to_owned()),
				(59, "POINT_SIZE".to_owned()),
				(60, "RESOLUTION".to_owned()),
				(61, "COPYRIGHT".to_owned()),
				(62, "NOTICE".to_owned()),
				(63, "FONT_NAME".to_owned()),
				(64, "FAMILY_NAME".to_owned()),
				(65, "FULL_NAME".to_owned()),
				(66, "CAP_HEIGHT".to_owned()),
				(67, "WM_CLASS".to_owned()),
				(68, "WM_TRANSIENT_FOR".to_owned()),
			]),
			selections: HashMap::new(),

			clients: UnsafeCell::new(LinkedList::new()),
			grabbing_client: None,

			custom_requests: HashMap::new(),
		}
	}

	/// Scans for screens on DRM.
	pub fn scan_screens(&mut self) {
		self.screens.clear();

		for dev in DRICard::scan() {
			// TODO Remove `take`
			for conn in DRIConnector::scan(&dev).into_iter().take(1) {
				let root = Window::new_root(1920, 1080); // TODO Pass dimensions of the screen
				let root_id = 1; // TODO
				self.windows.insert(root_id, root);

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

	/// Returns the drawable with the given ID.
	pub fn get_drawable(&self, id: u32) -> Option<&dyn Drawable> {
		// TODO Handle pixmaps

		self.get_window(id).map(|d| d as &dyn Drawable)
	}

	/// Returns an immutable reference to the window with the given ID.
	pub fn get_window(&self, wid: u32) -> Option<&Window> {
		self.windows.get(&wid)
	}

	/// Returns a mutable reference to the window with the given ID.
	pub fn get_window_mut(&mut self, wid: u32) -> Option<&mut Window> {
		self.windows.get_mut(&wid)
	}

	/// Returns the value of the atom with the given ID. If the atom doesn't exist, the function
	/// returns None.
	pub fn get_atom(&self, id: u32) -> Option<&String> {
		self.atoms.get(&id)
	}

	/// Returns the ID of the atom with the given name.
	pub fn get_atom_from_name(&self, name: &str) -> Option<u32> {
		// TODO Optimize
		self.atoms.iter()
			.filter_map(|(i, n)| {
				if n == name {
					Some(i)
				} else {
					None
				}
			})
			.cloned()
			.next()
	}

	/// Creates an atom with the given name and returns its ID.
	pub fn create_atom(&mut self, name: String) -> u32 {
		// TODO use clean atom ID allocator
		let mut id = 0;
		for i in 1..=((1 << 29) - 1) {
			if !self.atoms.contains_key(&i) {
				id = i;
				break;
			}
		}
		assert!(id != 0);

		self.atoms.insert(id, name);
		id
	}

	/// Returns the selection with the given name. If the selection doesn't exist, the function
	/// returns None.
	pub fn get_selection(&self, name: &str) -> Option<&Selection> {
		self.selections.get(name)
	}

	/// Adds a new client.
	///
	/// `poll_handler` is the poll handler on which the stream is to be registered.
	pub fn add_client(&mut self, client: Client, poll_handler: &mut PollHandler) {
		poll_handler.add_fd(client.get_stream());

		unsafe {
			(*self.clients.get()).push_back(client);
		}
	}

	/// Ticks every connected client.
	///
	/// `poll_handler` is the poll handler on which the stream is to be registered.
	pub fn tick_clients(&mut self, poll_handler: &mut PollHandler) {
		let mut cursor = unsafe {
			(*self.clients.get()).cursor_front_mut()
		};

		while let Some(client) = cursor.current() {
			match client.tick(self) {
				// On error, remove client
				Err(e) => {
					println!("Client disconnect: {}", e);

					// If the client is grabbing the server, ungrab
					if let Some(grabbing) = self.grabbing_client.clone() {
						if grabbing == client.get_id() {
							self.grabbing_client = None;
						}
					}

					if let Some(removed) = cursor.remove_current() {
						poll_handler.remove_fd(removed.get_stream());
					}
				},

				_ => {},
			}

			cursor.move_next();
		}
	}

	/// Returns an immutable reference to the list of custom requests.
	pub fn get_custom_requests(&self) -> &HashMap<u8, Box<RequestReadFn>> {
		&self.custom_requests
	}

	/// Returns a mutable reference to the list of custom requests.
	pub fn get_custom_requests_mut(&mut self) -> &mut HashMap<u8, Box<RequestReadFn>> {
		&mut self.custom_requests
	}

	/// Makes the server be grabbed by the given client.
	pub fn grab_by(&mut self, client: &Client) {
		self.grabbing_client = Some(client.get_id());
	}

	/// Ungrabs the server.
	pub fn ungrab(&mut self) {
		self.grabbing_client = None;
	}
}
