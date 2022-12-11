//! TODO doc

use std::error::Error;
use std::path::Path;

/// The default path to use for fonts search.
const DEFAULT_FONT_SEARCH_PATH: &str = "/usr/share/fonts";

/// Structure representing a loaded font.
pub struct Font {
	// TODO
}

impl Font {
	// TODO Use custom error
	/// Opens a font from the given path.
	pub fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
		// TODO
		todo!();
	}
}
