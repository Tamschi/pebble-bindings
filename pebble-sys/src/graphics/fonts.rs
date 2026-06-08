//! //TODO: Docs

use core::{
	ffi::{c_char, c_void},
	ptr::NonNull,
};

unsafe extern "C" {
	/// Loads a system font corresponding to the specified font key.
	///
	/// See [System Fonts](https://developer.repebble.com/guides/app-resources/system-fonts/)
	/// guide for a list of system fonts.
	///
	/// # Note
	///
	/// This may load a font from the flash peripheral into RAM.
	///
	/// # Returns
	///
	/// An opaque pointer to the loaded font, or, a pointer to the default (fallback) font if the specified font cannot be loaded.
	pub fn fonts_get_system_font(font_key: *const c_char) -> GFont;

	//TODO
}

/// Pointer to opaque font data structure.
///
/// # See also
///
/// [`fonts_load_custom_font()`], [`text_layer_set_font()`], [`graphics_draw_text()`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct GFont(NonNull<c_void>);
