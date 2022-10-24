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
use std::sync::Arc;
use std::sync::Mutex;

/// The path to the list of extensions.
pub const LIST_PATH: &str = "extensions.json"; // TODO

lazy_static! {
	/// The list of extensions.
	static ref EXTENSIONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

	/// The list of loaded extensions, by name.
	static ref LOADED_EXTENSIONS: Mutex<HashMap<String, Arc<Mutex<Extension>>>>
		= Mutex::new(HashMap::new());
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

	// TODO Use a format different than JSON to allow appending with shell redirections (>>)
	*EXTENSIONS.lock()
		.unwrap() = serde_json::from_str(&content)?;
	Ok(())
}


/// A loaded extension.
pub struct Extension {
	/// The name of the extension.
	name: String,

	/// The loaded shared library.
	lib: libloading::Library,

	/// Major opcode allocated to the extension.
	major_opcode: u8,
	/// First event allocated to the extension.
	first_event: u8,
	/// First error allocated to the extension.
	first_error: u8,
}

impl Extension {
	/// Loads the extentions with the given name and path.
	pub fn load(name: String, path: &Path) -> Result<Arc<Mutex<Self>>, Box<dyn Error>> {
		let lib = unsafe {
			libloading::Library::new(path)
		}?;
		let success = unsafe {
			let init_func: libloading::Symbol<unsafe extern fn() -> bool> = lib.get(b"init")?;
			init_func()
		};
		if !success {
			// TODO Error
			todo!();
		}

		let ext = Self {
			name: name.clone(),

			lib,

			major_opcode: 0, // TODO
			first_event: 0, // TODO
			first_error: 0, // TODO
		};

		LOADED_EXTENSIONS.lock()
			.unwrap()
			.insert(name.clone(), Arc::new(Mutex::new(ext)));
		Ok(Self::get(&name).unwrap())
	}

	/// Returns the extension with the given name.
	///
	/// If the extension is not loaded, the function returns None.
	pub fn get(name: &str) -> Option<Arc<Mutex<Self>>> {
		LOADED_EXTENSIONS.lock()
			.unwrap()
			.get(name)
			.cloned()
	}

	/// Returns the major opcode allocated to the extension.
	pub fn get_major_opcode(&self) -> u8 {
		self.major_opcode
	}

	/// Returns the first event allocated to the extension.
	pub fn get_first_event(&self) -> u8 {
		self.first_event
	}

	/// Returns the first error allocated to the extension.
	pub fn get_first_error(&self) -> u8 {
		self.first_error
	}
}

impl Drop for Extension {
	fn drop(&mut self) {
		let fini_func: Result<libloading::Symbol<unsafe extern fn()>, _> = unsafe {
			self.lib.get(b"fini")
		};
		if let Ok(fini_func) = fini_func {
			unsafe {
				fini_func();
			}
		}

		// TODO Free major opcode
		// TODO Free first event
		// TODO Free first error
	}
}

/// Queries for the extension with the given name.
///
/// If not loaded, the function tries to load the module with the given name.
/// If the extension doesn't exist, the function returns None.
pub fn query(name: &str) -> Result<Option<Arc<Mutex<Extension>>>, Box<dyn Error>> {
	if let Some(ext) = Extension::get(name) {
		return Ok(Some(ext));
	}

	match EXTENSIONS.lock().unwrap().get(name) {
		Some(ext_path) => {
			// Loading the extension
			let ext = Extension::load(name.to_owned(), Path::new(ext_path))?;
			Ok(Some(ext))
		},

		None => Ok(None),
	}
}
