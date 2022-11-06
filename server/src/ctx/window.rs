//! TODO doc

use crate::ctx::Context;
use crate::ctx::Screen;
use crate::protocol::BackingStore;
use crate::protocol::BitGravity;
use crate::protocol::Class;
use crate::protocol::MapState;
use crate::protocol::Rectangle;
use crate::protocol::WinGravity;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::num::NonZeroU32;
use super::Drawable;

/// A property associated to a window.
#[derive(Debug)]
pub struct Property {
	// TODO Change to string?
	/// The atom of the name of the data's type.
	property_type: u32,
	/// The data's format.
	format: u8,

	/// The property's data.
	data: Vec<u8>,
}

impl Property {
	/// Creates a property.
	///
	/// Arguments:
	/// - `property_type` is the atom representing the type of the property.
	/// - `format` is the format of the property.
	/// - `data` is the data of the property.
	pub fn new(property_type: u32, format: u8, data: Vec<u8>) -> Self {
		Self {
			property_type,
			format,

			data,
		}
	}

	/// Returns the atom of the type of the property.
	pub fn get_type(&self) -> u32 {
		self.property_type
	}

	/// Returns the format of the property.
	pub fn get_format(&self) -> u8 {
		self.format
	}

	/// Returns a slice to the property's data.
	pub fn get_data(&self) -> &[u8] {
		self.data.as_slice()
	}

	/// Prepends the given data to the current.
	pub fn prepend_data(&mut self, data: &[u8]) {
		let mut new = Vec::with_capacity(self.data.len() + data.len());
		new.extend_from_slice(data);
		new.append(&mut self.data);

		self.data = new;
	}

	/// Appends the given data to the current.
	pub fn append_data(&mut self, data: &[u8]) {
		self.data.extend_from_slice(data);
	}
}

/// Structure storing a window's attributes.
#[derive(Debug)]
pub struct WindowAttributes {
	/// TODO doc
	pub background_pixmap: u32,
	/// TODO doc
	pub background_pixel: u32,
	/// TODO doc
	pub border_pixmap: u32,
	/// TODO doc
	pub border_pixel: u32,
	/// TODO doc
	pub bit_gravity: BitGravity,
	/// TODO doc
	pub win_gravity: WinGravity,
	/// TODO doc
	pub backing_store: BackingStore,
	/// TODO doc
	pub backing_planes: u32,
	/// TODO doc
	pub backing_pixel: u32,
	/// TODO doc
	pub override_redirect: bool,
	/// TODO doc
	pub save_under: bool,
	/// TODO doc
	pub event_mask: u32,
	/// TODO doc
	pub do_not_propagate_mask: u32,
	/// TODO doc
	pub colormap: u32,
	/// TODO doc
	pub cursor: u32,

	/// TODO doc
	pub visual: u32,
	/// TODO doc
	pub class: Class,
	/// TODO doc
	pub backing_places: u32,
	/// TODO doc
	pub map_is_installed: u8,
	/// TODO doc
	pub map_state: MapState,
}

impl Default for WindowAttributes {
	fn default() -> Self {
		// TODO Set correct values
		Self {
			background_pixmap: 0,
			background_pixel: 0,
			border_pixmap: 0,
			border_pixel: 0,
			bit_gravity: BitGravity::Forget,
			win_gravity: WinGravity::Unmap,
			backing_store: BackingStore::NotUseful,
			backing_planes: 0,
			backing_pixel: 0,
			override_redirect: false,
			save_under: false,
			event_mask: 0,
			do_not_propagate_mask: 0,
			colormap: 0,
			cursor: 0,

			visual: 0,
			class: Class::InputOnly,
			backing_places: 0,
			map_is_installed: 0,
			map_state: MapState::Unviewable,
		}
	}
}

/// A window to be rendered on screen.
#[derive(Debug)]
pub struct Window {
	/// The ID of the window.
	id: NonZeroU32,

	/// The ID of the parent window.
	parent: Option<NonZeroU32>,
	/// The list of indexes of the childrens of the current window.
	children: HashSet<NonZeroU32>,

	/// The depth of the pixmap.
	depth: u8,
	/// The position and size of the window.
	rect: Rectangle,
	/// The width of the window's border.
	border_width: u16,

	/// The list of properties of the window. The key is the name of the property.
	properties: HashMap<String, Property>,

	/// The window's attributes.
	pub attributes: WindowAttributes,
}

impl Window {
	/// Creates a new root window.
	///
	/// Arguments:
	/// - `ctx` is the context on which the window will be added.
	/// - `parent` is the ID of the parent window. If None, the window is a root window.
	/// - `rect` represents the position and dimensions of the window relative to its parent.
	///
	/// The function allocates an ID for the window and adds it to the given context.
	///
	/// If the window is root, the X/Y position is zero-ed.
	pub fn new<'c>(
		ctx: &'c mut Context,
		parent: Option<NonZeroU32>,
		mut rect: Rectangle,
	) -> &'c mut Self {
		let id = NonZeroU32::new(1).unwrap(); // TODO Allocate? Take as param?

		if parent.is_none() {
			rect.x = 0;
			rect.y = 0;
		}

		let win = Self {
			id,

			parent: None,
			children: HashSet::new(),

			depth: 24, // TODO
			rect,
			border_width: 0,

			properties: HashMap::new(),

			attributes: WindowAttributes::default(),
		};

		if let Some(parent_id) = win.parent {
			// TODO Return error if None instead of unwrap
			let parent = ctx.get_window_mut(parent_id).unwrap();
			parent.children.insert(id);
		}

		// Insert window in context
		ctx.windows.insert(id, win);

		ctx.windows.get_mut(&id).unwrap()
	}

	/// Returns the ID of the window.
	pub fn get_id(&self) -> NonZeroU32 {
		self.id
	}

	/// Returns the ID of the window's parent.
	pub fn get_parent(&self) -> Option<NonZeroU32> {
		self.parent
	}

	/// Tells whether the window is root.
	pub fn is_root(&self) -> bool {
		self.parent.is_none()
	}

	/// The list indexes of the children of the window.
	pub fn get_children(&self) -> &HashSet<NonZeroU32> {
		&self.children
	}

	/// Returns the depth of the window.
	pub fn set_depth(&mut self, depth: u8) {
		self.depth = depth;
	}

	/// Sets the position and size of the window.
	pub fn set_rectangle(&mut self, rect: Rectangle) {
		if self.is_root() && (rect.x != 0 || rect.y != 0) {
			return;
		}

		self.rect = rect;
	}

	/// Sets the width of the border.
	pub fn set_border_width(&mut self, border_width: u16) {
		self.border_width = border_width;
	}

	/// Returns an immutable reference to the property with the given name.
	/// If the property doesn't exist, the function returns None.
	pub fn get_property(&self, name: &str) -> Option<&Property> {
		self.properties.get(name)
	}

	/// Returns a mutable reference to the property with the given name.
	/// If the property doesn't exist, the function returns None.
	pub fn get_property_mut(&mut self, name: &str) -> Option<&mut Property> {
		self.properties.get_mut(name)
	}

	/// Creates a property with the given name.
	pub fn create_property(&mut self, name: String, prop: Property) {
		self.properties.insert(name, prop);
	}

	/// Deletes the property with the given name. If the property doesn't exist, the function does
	/// nothing.
	pub fn delete_property(&mut self, name: &str) {
		self.properties.remove(name);
	}

	/// Sets the window's attributes.
	pub fn set_attributes(&mut self, attr: WindowAttributes) {
		self.attributes = attr;
	}

	/// Tells whether the window can render anything on screen.
	pub fn is_output(&self) -> bool {
		matches!(self.attributes.class, Class::InputOutput)
	}

	/// Renders the window's background with a single color.
	pub fn render_pixel_background(&self, screen: &Screen) {
		let fb = screen.get_curr_fb();

		let (screen_width, screen_height) = screen.get_screen_size();

		let x = self.rect.x as isize;
		let y = self.rect.y as isize;
		let width = self.rect.width as isize;
		let height = self.rect.height as isize;

		let x_begin = max(x, 0) as usize;
		let y_begin = max(y, 0) as usize;
		let x_end = min((x + width) as usize, screen_width as usize);
		let y_end = min((y + height) as usize, screen_height as usize);

		let ptr = fb.get_buffer_ptr().unwrap().as_ptr();
		for y in y_begin..y_end {
			for x in x_begin..x_end {
				let i = y * width as usize + x;

				// TODO use window's background color
				unsafe {
					*ptr.add(i) = 0xffffff;
				}
			}
		}
	}

	/// Renders the window's background.
	pub fn render_background(&self, screen: &Screen) {
		// TODO If a pixmap is specified, render it to background
		// However, pixel has priority over pixmap

		self.render_pixel_background(screen);
	}

	/// Renders the full window, including children windows.
	pub fn render_full(&self, ctx: &Context, screen: &Screen) {
		if self.is_output() {
			self.render_background(screen);

			// TODO render content
		}

		for c in &self.children {
			if let Some(child_win) = ctx.get_window(*c) {
				child_win.render_full(ctx, screen);
			}
		}
	}
}

impl Drawable for Window {
	fn get_depth(&self) -> u8 {
		self.depth
	}

	fn get_root(&self) -> u32 {
		// TODO
		0
	}

	fn get_rectangle(&self) -> Rectangle {
		self.rect.clone()
	}

	fn get_border_width(&self) -> u16 {
		self.border_width
	}
}
