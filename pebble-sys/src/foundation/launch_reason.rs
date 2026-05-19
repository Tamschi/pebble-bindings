//! API for checking what caused the application to launch.
//!
//! This includes the system, launch by user interaction (User selects the application from the launcher menu),
//! launch by the mobile or a mobile companion application, or launch by a scheduled wakeup event for the specified application.

use core::ffi::c_int;

unsafe extern "C" {
	/// Provides the method used to launch the current application.
	pub fn launch_reason() -> AppLaunchReason;

	/// Get the argument passed to the app when it was launched.
	///
	/// # Note
	///
	/// Currently the only way to pass arguments to apps is by using an openWatchApp action on a pin.
	///
	/// # Returns
	///
	/// The argument passed to the app, or 0 if the app wasn't launched from a Launch App action.
	pub fn launch_get_args() -> u32;
}

/// AppLaunchReason is used to inform the application about how it was launched.
///
/// New launch reasons may be added in the future. As a best practice,
/// it is recommended to only handle the cases that the app needs to know about,
/// rather than trying to handle all possible launch reasons.
#[repr(transparent)]
pub struct AppLaunchReason(pub c_int);
impl AppLaunchReason {
	pub const SYSTEM: Self = Self(0);
	pub const USER: Self = Self(1);
	pub const PHONE: Self = Self(2);
	pub const WAKEUP: Self = Self(3);
	pub const WORKER: Self = Self(4);
	pub const QUICK_LAUNCH: Self = Self(5);
	pub const TIMELINE_ACTION: Self = Self(6);
	pub const SMARTSTRAP: Self = Self(7);
}
