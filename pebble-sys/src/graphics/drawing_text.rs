//! Functions to draw text into a graphics context
//!
//! See [Graphics Context] for more information about the graphics context.
//!
//! Other drawing functions and related documentation:
//!
//! - [Drawing Primitives]
//! - [Drawing Paths]
//! - [Graphics Types]

unsafe extern "C" {
	//TODO
}

use core::{ffi::c_void, ptr::NonNull};

/// Text overflow mode controls the way text overflows when the string that is drawn does not fit inside the area constraint.
///
/// # See also
///
/// [`graphics_draw_text`], [`text_layer_set_overflow_mode`]
#[repr(C)]
pub union GTextOverflowMode {
	pub u8: u8,
}

/// //TODO: Check!
impl GTextOverflowMode {
	/// On overflow, wrap words to a new line below the current one.
	/// Once vertical space is consumed, the last line may be clipped.
	pub const WORD_WRAP: Self = Self { u8: 0 };

	/// On overflow, wrap words to a new line below the current one.
	/// Once vertical space is consumed, truncate as needed to fit a trailing ellipsis (...).
	/// Clipping may occur if the vertical space cannot accomodate the first line of text.
	pub const TRAILING_ELLIPSIS: Self = Self { u8: 1 };

	/// Acts like [`TRAILING_ELLIPSIS`], plus trims leading and trailing newlines, while treating all other newlines as spaces.
	pub const FILL: Self = Self { u8: 2 };
}

/// Text alignment controls the way the text is aligned inside the box the text is drawn into.
///
/// # See also
///
/// [`graphics_draw_text`], [`text_layer_set_text_alignment`]

#[repr(C)]
pub union GTextAlignment {
	pub u8: u8,
}

/// //TODO: Check!
impl GTextAlignment {
	/// Aligns the text to the left of the drawing box.
	pub const LEFT: Self = Self { u8: 0 };
	/// Aligns the text centered inside the drawing box.
	pub const CENTER: Self = Self { u8: 1 };
	/// Aligns the text to the right of the drawing box.
	pub const RIGHT: Self = Self { u8: 2 };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HGTextAttributes(NonNull<c_void>);
