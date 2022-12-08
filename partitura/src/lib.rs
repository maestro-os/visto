//! TODO doc

mod true_type;

/// Trait representing a loaded font.
pub trait Font {
	// TODO
}

/// Trait representing a font engine.
pub trait FontEngine {
	// TODO Font parsing error struct
	/// Loads a font.
	///
	/// `buff` is the content of the font's file.
	fn load(&self, buff: &[u8]) -> Result<Box<dyn Font>, ()>;
}
