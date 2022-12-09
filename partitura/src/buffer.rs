//! TODO doc

/// A buffer on which a font can be rendered.
pub struct RenderBuffer {
	/// The width of the buffer in pixels.
	width: usize,
	/// The height of the buffer in pixels.
	height: usize,
	/// The number of channels per pixel.
	channels: usize,

	/// The buffer's data.
	data: Vec<u8>,
}

impl RenderBuffer {
	/// Creates a new buffer.
	///
	/// Arguments:
	/// - `width` is the width of the buffer in pixels.
	/// - `height` is the height of the buffer in pixels.
	/// - `channels` is the number of channels per pixel.
	pub fn new(width: usize, height: usize, channels: usize) -> Self {
		Self {
			width,
			height,
			channels,

			data: vec![0; width * height * channels],
		}
	}

	/// Clears the buffer with the given colors.
	///
	/// If the given color doesn't match the number of channels, the function does nothing.
	pub fn clear(&mut self, color: &[u8]) {
		if color.len() != self.channels {
			return;
		}

		for pix in self.data.as_mut_slice().chunks_mut(self.channels) {
			pix.copy_from_slice(color);
		}
	}
}
