//! TODO doc

use crate::protocol::BackingStore;
use crate::protocol::BitGravity;
use crate::protocol::Class;
use crate::protocol::MapState;
use crate::protocol::Rectangle;
use crate::protocol::WinGravity;
use std::collections::HashMap;
use super::Drawable;

/// A property associated to a window.
#[derive(Debug)]
pub struct Property {
	/// The atom of the name of the data's type.
	property_type: u32,
	/// The data's format.
	format: u8,

	/// The property's data.
	data: Vec<u8>,
}

impl Property {
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
	/// Tells whether the window is a root window.
	root: bool,
	/// The ID of the parent window.
	parent: u32,

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
	/// - `width` is the width of the window.
	/// - `height` is the height of the window.
	pub fn new_root(width: u16, height: u16) -> Self {
		Self {
			root: true,
			parent: 0,

			depth: 24, // TODO
			rect: Rectangle {
				x: 0,
				y: 0,

				width,
				height,
			},
			border_width: 0,

			properties: HashMap::new(),

			attributes: WindowAttributes::default(),
		}
	}

	/// Creates a window.
	///
	/// Arguments:
	/// - `parent` the ID of the parent window.
	/// - `rect` represents the position and dimensions of the window relative to its parent.
	pub fn new(parent: u32, rect: Rectangle) -> Self {
		Self {
			root: false,
			parent: parent,

			depth: 24, // TODO
			rect,
			border_width: 0,

			properties: HashMap::new(),

			attributes: WindowAttributes::default(),
		}
	}

	/// Returns the depth of the window.
	pub fn set_depth(&mut self, depth: u8) {
		self.depth = depth;
	}

	/// Sets the position and size of the window.
	pub fn set_rectangle(&mut self, rect: Rectangle) {
		if self.root && (rect.x != 0 || rect.y != 0) {
			return;
		}

		self.rect = rect;
	}

	/// Sets the width of the border.
	pub fn set_border_width(&mut self, border_width: u16) {
		self.border_width = border_width;
	}

	/// Returns the property with the given name. If the property doesn't exist, the function
	/// returns None.
	pub fn get_property(&self, name: &str) -> Option<&Property> {
		self.properties.get(name)
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
