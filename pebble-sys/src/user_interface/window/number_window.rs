//! A ready-made window prompting the user to pick a number
//!
//! TODO: Images

use crate::user_interface::window::HWindow;
use core::{
	ffi::{c_char, c_void},
	ptr::NonNull,
};

unsafe extern "C" {
	/// Creates a new NumberWindow on the heap and initializes it with the default values.
	///
	/// # Note
	///
	/// The number window is not pushed to the window stack. Use [`super::window_stack_push()`] to do this.
	///
	/// # Safety
	///
	///  `label` Must be long-lived and cannot be stack-allocated.
	pub fn number_window_create(
		label: *const c_char,
		callbacks: NumberWindowCallbacks,
		callback_context: *mut c_void,
	) -> Result<HNumberWindow, ()>;

	/// Destroys a NumberWindow previously created by number_window_create.
	pub fn number_window_destroy(number_window: HNumberWindow);

	/// Sets the text of the title or prompt label.
	///
	/// # Safety
	///
	///  `label` Must be long-lived and cannot be stack-allocated.
	pub fn number_window_set_label<'a>(number_window: HNumberWindow, label: *const c_char);

	/// Sets the maximum value this field can hold.
	pub fn number_window_set_max(number_window: HNumberWindow, max: i32);

	/// Sets the minimum value this field can hold.
	pub fn number_window_set_min(number_window: HNumberWindow, min: i32);

	/// Sets the current value of the field.
	pub fn number_window_set_value(number_window: HNumberWindow, value: i32);

	/// Sets the amount by which to increment/decrement by on a button click.
	pub fn number_window_set_step_size(number_window: HNumberWindow, step: i32);

	/// Gets the current value.
	pub fn number_window_get_value(number_window: HNumberWindow) -> i32;

	/// Gets the "root" Window of the number window.
	pub fn number_window_get_window(number_window: HNumberWindow) -> HWindow;
}

/// Data structure containing all the callbacks for a NumberWindow.
#[repr(C)]
pub struct NumberWindowCallbacks {
	/// Called as the value is incremented.
	pub incremented: Option<NumberWindowCallback>,
	/// Called as the value is decremented.
	pub decremented: Option<NumberWindowCallback>,
	/// Called as the value is confirmed, i.e. as the SELECT button is clicked.
	pub selected: Option<NumberWindowCallback>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HNumberWindow(NonNull<c_void>);

/// A [`NumberWindow`] callback.
pub type NumberWindowCallback = extern "C" fn(number_window: HNumberWindow, context: *mut c_void);
