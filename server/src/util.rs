//! This module implements utility functions.

/// Reinterprets the given pointer in the given type.
pub unsafe fn reinterpret<A, B>(ptr: *const A) -> &'static B {
	&*(ptr as *const B)
}
