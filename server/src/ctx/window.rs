//! TODO doc

use crate::protocol::Rectangle;
use std::collections::HashMap;

/// A property associated to a window.
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

/// A window to be rendered on screen.
pub struct Window {
	/// Tells whether the window is a root window.
	root: bool,
	/// If true, the window has class InputOutput. If false, the function is InputOnly.
	output: bool,

	/// The depth of the pixmap.
	depth: u8,

	/// The position and size of the window.
	rect: Rectangle,

	/// The width of the window's border.
	border_width: u16,

	/// The list of properties of the window. The key is the name of the property.
	properties: HashMap<String, Property>,
}

impl Window {
	/// Creates a new root window.
	/// By default, the window has size 0*0.
	pub fn new_root() -> Self {
		Self {
			root: true,
			output: true,

			depth: 24, // TODO

			rect: Rectangle {
				x: 0,
				y: 0,

				width: 0,
				height: 0,
			},

			border_width: 0,

			properties: HashMap::new(),
		}
	}

	/// Returns the position and size of the window.
	pub fn get_rectangle(&self) -> &Rectangle {
		&self.rect
	}

	/// Sets the position and size of the window.
	pub fn set_rectangle(&mut self, rect: Rectangle) {
		if self.root && (rect.x != 0 || rect.y != 0) {
			return;
		}

		self.rect = rect;
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
}
