use core::ffi::{c_int, c_void};

#[repr(transparent)]
pub struct ButtonId(pub c_int);
impl ButtonId {
	/// Back button
	pub const BACK: Self = Self(0);
	/// Up button
	pub const UP: Self = Self(1);
	/// Select (middle) button
	pub const SELECT: Self = Self(2);
	/// Down button
	pub const DOWN: Self = Self(3);
}

#[repr(transparent)]
pub struct ClickRecognizerRef(*mut c_void);
pub type ClickHandler = extern "C" fn(recognizer: ClickRecognizerRef, context: *mut c_void);
pub type ClickConfigProvider = extern "C" fn(context: *mut c_void);
