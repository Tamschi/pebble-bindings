//! Functions related to logging from apps.
//!
//! This module contains the functions necessary to log messages through Bluetooth.

//TODO: Enum AppLogLevel

use core::ffi::{c_char, c_int};

unsafe extern "C" {
	pub fn app_log(
		log_level: u8,
		src_filename: *const c_char,
		src_line_number: c_int,
		fmt: *const c_char,
		...
	);
}
