//! Utility functions for managing an application's memory.

use core::ffi::c_void;

unsafe extern "C" {
	/// Calculates the number of bytes of heap memory *not* currently being used by the application.
	pub safe fn heap_bytes_free() -> usize;

	/// Calculates the number of bytes of heap memory currently being used by the application.
	pub safe fn heap_bytes_used() -> usize;

	/// Flushes the data cache and invalidates the instruction cache for the given region of memory, if necessary.
	///
	/// This is only required when your app is loading or modifying code in memory and intends to execute it.
	/// On some platforms, code executed may be cached internally to improve performance.
	/// After writing to memory, but before executing, this function must be called in order to avoid undefined behavior.
	///
	/// On platforms without caching, this performs no operation.
	pub fn memory_cache_flush(start: *const c_void, size: usize);
}
