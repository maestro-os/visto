//! Extensions can be loaded by the server to handle more features than only the basics.
//!
//! Each extension is represented by a shared library, and identified with a name.
//!
//! A file allows to specify associations names to shared libraries.

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Mutex;

/// The path to the list of extensions.
pub const LIST_PATH: &str = "extensions.json"; // TODO

lazy_static! {
	/// The list of extensions.
	static ref EXTENSIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

	/// The list of loaded extensions, by name.
	static ref LOADED_EXTENSIONS: Mutex<HashMap<String, Extension>> = Mutex::new(HashMap::new());
}

/// Loads the list of extensions from the file at the given path.
///
/// If the file doesn't exist, the function does nothing.
pub fn load_extensions_list(path: &Path) -> Result<(), Box<dyn Error>> {
	let content = match fs::read_to_string(path) {
		Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
		Err(e) => return Err(e.into()),
		Ok(c) => c,
	};

	*EXTENSIONS.lock().unwrap() = serde_json::from_str(&content)?;
	Ok(())
}


/// A loaded extension.
pub struct Extension {
	/// The name of the extension.
	name: String,

	// TODO
}

impl Extension {
	/// Loads the extentions with the given name and path.
	pub fn load(_name: String, _path: &Path) -> Result<Self, Box<dyn Error>> {
		// TODO
		todo!();
	}

	/// Tells whether the extension with the given name is loaded.
	pub fn is_loaded(name: &str) -> bool {
		LOADED_EXTENSIONS.lock().unwrap().contains_key(name)
	}
}

/// Queries for the extension with the given name.
///
/// If not loaded, the function tries to load the module with the given name.
/// If the module does not exist, the function returns `false`.
pub fn query(name: &str) -> Result<bool, Box<dyn Error>> {
	if Extension::is_loaded(name) {
		return Ok(true);
	}

	match EXTENSIONS.lock().unwrap().get(name) {
		Some(ext_path) => {
			// Loading the extension
			Extension::load(name.to_owned(), Path::new(ext_path))?;

			Ok(true)
		},

		None => Ok(false),
	}
}
