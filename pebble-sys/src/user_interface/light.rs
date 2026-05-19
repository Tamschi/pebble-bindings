//! Controlling Pebble's backlight
//!
//! The Light API provides you with functions to turn on Pebble’s backlight or put it back into automatic control.
//! You can trigger the backlight and schedule a timer to automatically disable the backlight after a short delay,
//! which is the preferred method of interacting with the backlight.

use crate::graphics::graphics_types::GColor;

unsafe extern "C" {
	/// Trigger the backlight and schedule a timer to automatically disable the backlight after a short delay.
	/// This is the preferred method of interacting with the backlight.
	pub fn light_enable_interaction();

	/// Turn the watch's backlight on or put it back into automatic control.
	///
	/// Developers should take care when calling this function,
	/// keeping Pebble's backlight on for long periods of time will rapidly deplete the battery.
	///
	/// # Parameters
	///
	/// - `enable`: Turn the backlight on if `true`, otherwise `false` to put it back into automatic control.
	pub fn light_enable(enable: bool);

	/// # Returns
	///
	/// true if the backlight is currently on in any form (on, timed, or fading out).
	/// Returns false only when the backlight is fully off.
	///
	/// Useful for apps that want to behave differently depending on whether the screen is currently lit — e.g. skipping an animation or queuing a visual cue for when the screen wakes.
	pub fn light_is_on() -> bool;

	/// Tint the backlight LED to the given color.
	///
	/// The color persists while the app is foregrounded and is automatically reset to the user's default (white) when the app exits or is preempted by a system notification.
	///
	/// On platforms without a color backlight this is a no-op.
	///
	/// # Note
	///
	/// GColor carries only 2 bits per channel (64 distinct tints).
	/// Use [`light_set_color_rgb888()`] if you need the full 8-bit-per-channel range that the LED hardware supports.
	pub fn light_set_color(color: GColor);

	/// Tint the backlight LED to a packed 24-bit RGB value (0x00RRGGBB).
	///
	/// Same persistence semantics as [`light_set_color()`]:
	/// the override lasts while the app is foregrounded and is reset on app exit or system preempt.
	///
	/// No-op on platforms without a color backlight.
	pub fn light_set_color_rgb888(rgb: u32);

	/// Restore the backlight to the user's default color. Rarely needed — the system resets automatically on app exit.
	///
	/// No-op on platforms without a color backlight.
	pub fn light_set_system_color();
}
