//! Math routines.
//!
//! Below is a code example that uses the trigonometry functions to calculate the coordinate at which the second hand of a watch ends, using seconds from the system time.
//!
//! ```c
//! // C
//! GPoint secondHand;
//! GPoint center;
//! struct tm *tick_time = ...;
//! int32_t secondHandLength = ...;
//! ...
//! int32_t second_angle = TRIG_MAX_ANGLE * tick_time->tm_sec / 60;
//! secondHand.y = (-cos_lookup(second_angle) * secondHandLength / TRIG_MAX_RATIO) + center.y;
//! secondHand.x = (sin_lookup(second_angle) * secondHandLength / TRIG_MAX_RATIO) + center.x;
//! ```

unsafe extern "C" {
	/// Look-up the sine of the given angle from a pre-computed table.
	///
	/// # Parameters
	///
	/// - `angle`: The angle for which to compute the sine.
	///   The angle value is scaled linearly, such that a value of 0x10000 corresponds to 360 degrees or 2 PI radians.
	///
	pub fn sin_lookup(angle: i32) -> i32;

	/// Look-up the cosine of the given angle from a pre-computed table.
	///
	/// This is equivalent to calling `sin_lookup(angle + TRIG_MAX_ANGLE / 4)`.
	///
	/// # Parameters
	///
	/// - `angle`: The angle for which to compute the cosine.
	///   The angle value is scaled linearly, such that a value of 0x10000 corresponds to 360 degrees or 2 PI radians.
	pub fn cos_lookup(angle: i32) -> i32;

	/// Look-up the arctangent of a given x, y pair The angle value is scaled linearly, such that a value of 0x10000 corresponds to 360 degrees or 2 PI radians.
	pub fn atan2_lookup(y: i16, x: i16) -> i32;
}

/// The largest value that can result from a call to [`sin_lookup`] or [`cos_lookup`].
///
/// For a code example, see the detailed description at the top of this chapter: [`Math`](`crate::foundation::math`).
pub const TRIG_MAX_RATIO: i32 = 0xffff;

/// Angle value that corresponds to 360 degrees or 2 PI radians.
///
/// # See also
///
/// [`sin_lookup`], [`cos_lookup`]
pub const TRIG_MAX_ANGLE: i32 = 0x10000;

//TODO: TRIGANGLE_TO_DEG(trig_angle)
//TODO: DEG_TO_TRIGANGLE(angle)
