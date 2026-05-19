use core::ffi::{c_int, c_void};

unsafe extern "C" {
	pub fn malloc(size: usize) -> *mut c_void;
	pub fn calloc(count: usize, size: usize) -> *mut c_void;
	pub fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
	pub fn free(ptr: *mut c_void);
	pub fn memcmp(ptr1: *const c_void, ptr2: *const c_void, n: usize) -> c_int;
	pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	pub fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	pub fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}
