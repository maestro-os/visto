//! TODO doc

use super::Font;
use super::FontEngine;

/// TODO doc
#[repr(C, packed)]
struct OffsetSubtable {
	/// A tag to indicate the OFA scaler to be used to rasterize this font; see the note on the scaler type below for more information.
	scaler_type: u32,
	/// number of tables
	num_tables: u16,
	/// (maximum power of 2 <= numTables)*16
	search_range: u16,
	/// log2(maximum power of 2 <= numTables)
	entry_selector: u16,
	/// numTables*16-searchRange
	range_shift: u16,
}

/// TODO doc
#[repr(C, packed)]
struct TableDirectory {
	/// 4-byte identifier
	tag: u32,
	/// checksum for this table
	checksum: u32,
	/// offset from beginning of sfnt
	offset: u32,
	/// length of this table in byte (actual length not padded length)
	length: u32,
}

/// TODO doc
pub struct TrueTypeEngine {}

impl FontEngine for TrueTypeEngine {
	fn load(&self, _buff: &[u8]) -> Result<Box<dyn Font>, ()> {
		// TODO read offset subtable
		// TODO read table directory

		// TODO compute checksum

		// TODO
		todo!();
	}
}
