//! TODO doc

mod buffer;
mod true_type;

use buffer::RenderBuffer;

/// The set of parameters tweaking font rendering.
pub struct RenderParams {
	// TODO
}

/// Trait representing a loaded font.
pub trait Font {
	/// Renders the given text with the font.
	///
	/// Arguments:
	/// - `text` is the text to render.
	/// - `buff` is the buffer on which the text is to be rendered.
	/// - `params` is the set of parameters tweaking font rendering.
	fn render(
		&self,
		text: &str,
		buff: &mut RenderBuffer,
		params: &RenderParams
	);
}

/// Trait representing a font engine.
pub trait FontEngine {
	// TODO Font parsing error struct
	/// Loads a font.
	///
	/// `buff` is the content of the font's file.
	fn load(&self, buff: &[u8]) -> Result<Box<dyn Font>, ()>;
}
