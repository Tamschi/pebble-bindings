//! Slightly Rust-y Pebble watch platform C API bindings.
//!
//! The documentation for this crate is based (relatively verbatim) on <https://developer.repebble.com/docs/c/>.
//!
//! >  To learn more about how to use this SDK, take a look at the [Writing Apps for Pebble](https://developer.repebble.com/guides/pebble-apps/) section of the Developer Guides.
//! >
//! > You can also view the [C SDK Tutorial](https://developer.repebble.com/getting-started/watchface-tutorial/part1/).
//! > This will guide you through the first stages to creating a Pebble watchface, adding customized images and fonts and extra web-based content.
//!
//! TODO: Document Rust project setup through deployment.

#![no_std]
#![warn(clippy::pedantic)]
// Matching the SDK documentation.
#![allow(clippy::module_name_repetitions)]

use core::{ffi::c_char, panic::PanicInfo};

pub mod foundation;
pub mod graphics;
pub mod standard_c;
pub mod user_interface;

#[cfg_attr(feature = "panic_handler", panic_handler)]
#[cfg_attr(not(feature = "panic_handler"), expect(dead_code))]
fn panic(_info: &PanicInfo) -> ! {
	use foundation::logging::app_log;
	unsafe {
		let panic = &*("### PANIC ###\0" as *const str as *const _ as *const c_char);
		let todo = &*("TODO: Output trace somehow.\0" as *const str as *const _ as *const c_char);
		app_log(1, panic, -1, todo);
	}
	loop {}
}

pub mod sys_helpers {
	//! Helpers original to this crate.

	#[allow(non_camel_case_types)]
	#[repr(u8)]
	pub enum u2 {
		_0 = 0b00,
		_1 = 0b01,
		_2 = 0b10,
		_3 = 0b11,
	}
}
