//! An ID allocator allows to allocate IDs in a fixed range.

use std::iter::Step;
use std::ops::Range;
use std::ops::RangeInclusive;

// TODO Avoid storing every IDs

/// Struccture representing an ID allocator.
pub struct IDAllocator<T: Clone + Ord + Step> {
	/// The list of free IDs.
	free: Vec<T>,
	/// The list of used IDs.
	used: Vec<T>,
}

impl<T: Clone + Ord + Step> IDAllocator<T> {
	/// Creates a new instance from the given range.
	pub fn from_range(range: Range<T>) -> Self {
		Self {
			free: range.collect(),
			used: Vec::new(),
		}
	}

	/// Creates a new instance from the given inclusive range.
	pub fn from_range_inclusive(range: RangeInclusive<T>) -> Self {
		Self {
			free: range.collect(),
			used: Vec::new(),
		}
	}

	/// Allocates a new ID. If no ID is available, the function returns None.
	pub fn alloc(&mut self) -> Option<T> {
		let id = self.free.pop()?;

		let i = match self.used.binary_search(&id) {
			Ok(i) => i,
			Err(i) => i,
		};
		self.used.insert(i, id.clone());

		Some(id)
	}

	/// Frees the given ID. If the ID is not allocated, the function does nothing.
	pub fn free(&mut self, id: T) {
		match self.used.binary_search(&id) {
			Ok(i) => {
				self.used.remove(i);
				self.free.push(id);
			}

			Err(_) => {}
		};
	}
}
