//! API for the application to notify the system of the reason it will exit.
//!
//! If the application has not specified an exit reason before it exits,
//! then the exit reason will default to APP_EXIT_NOT_SPECIFIED.
//!
//! Only an application can set its exit reason. The system will not modify it.

use core::ffi::c_int;

unsafe extern "C" {
	// Set the app exit reason to a new reason.
	pub fn exit_reason_set(exit_reason: AppExitReason);
}

#[repr(transparent)]
pub struct AppExitReason(pub c_int);
impl AppExitReason {
	pub const NOT_SPECIFIED: Self = Self(0);
	pub const ACTION_PERFORMED_SUCCESSFULLY: Self = Self(1);
}
