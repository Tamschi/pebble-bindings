//! Layer that displays and formats a text string.
//!
//! //TODO: Image
//!
//! The geometric information (bounds, frame) of the Layer is used as the "box" in which the text is drawn.
//! The [TextLayer](`HTextLayer`) also has a number of other properties that influence how the text is drawn.
//! Most important of these properties are: a pointer to the string to draw itself, the font, the text color,
//! the background color of the layer, the overflow mode and alignment of the text inside the layer.

use core::{
	ffi::{c_char, c_void},
	ops::Deref,
	ptr::NonNull,
};

use crate::{
	graphics::{
		drawing_text::{GTextAlignment, GTextOverflowMode},
		fonts::GFont,
		graphics_types::{GColor, GRect, GSize},
	},
	user_interface::layers::HLayer,
};

unsafe extern "C" {
	/// Creates a new TextLayer on the heap and initializes it with the default values.
	///
	/// - Font: Raster Gothic 14-point Boldface (system font)
	/// - Text Alignment: [`GTextAlignmentLeft`](`TODO`)
	/// - Text color: [`BLACK`](`crate::graphics::graphics_types::color_definitions::BLACK`)
	/// - Background color: [`WHITE`](`crate::graphics::graphics_types::color_definitions::WHITE`);
	/// - Clips: `true`
	/// - Hidden: `false`
	/// - Caching: `false`
	///
	/// The text layer is automatically marked dirty after this operation.
	pub safe fn text_layer_create(frame: GRect) -> Result<HTextLayer, ()>;

	/// Destroys a [TextLayer](`HTextLayer`) previously created by [`text_layer_create`].
	pub fn text_layer_destroy(text_layer: HTextLayer);

	/// Gets the "root" Layer of the text layer, which is the parent for the sub- layers used for its implementation.
	///
	/// //TODO: What happens if passed NULL?
	pub fn text_layer_get_layer(text_layer: HTextLayer) -> HLayer;

	/// Sets the pointer to the string where the TextLayer is supposed to find the text at a later point in time, when it needs to draw itself.
	///
	/// # Safety
	///
	/// `text` must point to a null-terminated and valid UTF-8 string.
	///
	/// The string is not copied, so its buffer most likely cannot be stack allocated, but is recommended to be a buffer that is long-lived, at least as long as the TextLayer is part of a visible Layer hierarchy.
	///
	/// # See also
	///
	/// [`text_layer_get_text`]
	pub fn text_layer_set_text(text_layer: HTextLayer, text: *const c_char);

	/// Gets the pointer to the string that the TextLayer is using.
	///
	/// # See also
	///
	/// [`text_layer_set_text`]
	pub fn text_layer_get_text(text_layer: HTextLayer) -> *const c_char;

	/// Sets the background color of the bounding box that will be drawn behind the text.
	///
	/// # See also
	///
	/// [`text_layer_set_text_color`]
	pub fn text_layer_set_background_color(text_layer: HTextLayer, color: GColor);

	/// Sets the color of text that will be drawn.
	///
	/// # See also
	///
	/// [`text_layer_set_background_color`]
	pub fn text_layer_set_text_color(text_layer: HTextLayer, color: GColor);

	/// Sets the line break mode of the TextLayer.
	pub fn text_layer_set_overflow_mode(text_layer: HTextLayer, line_mode: GTextOverflowMode);

	/// Sets the font of the TextLayer.
	///
	/// # See also
	///
	/// [`fonts_get_system_font`], [`fonts_load_custom_font`]
	pub fn text_layer_set_font(text_layer: HTextLayer, font: GFont);

	/// Sets the alignment of the TextLayer.
	pub fn text_layer_set_text_alignment(text_layer: HTextLayer, text_alignment: GTextAlignment);

	/// Enables text flow following the boundaries of the screen and pagination that introduces extra line spacing at page breaks to avoid partially clipped lines for the TextLayer.
	/// If the TextLayer is part of a [ScrollLayer] the ScrollLayer's frame will be used to configure paging.
	///
	/// # Note
	///
	/// Make sure the TextLayer is part of the view hierarchy before calling this function. Otherwise it has no effect.
	///
	/// # See also
	///
	/// [`text_layer_restore_default_text_flow_and_paging`], [`graphics_text_attributes_enable_screen_text_flow`], [`graphics_text_attributes_enable_paging`]
	pub fn text_layer_enable_screen_text_flow_and_paging(text_layer: HTextLayer, inset: u8);

	/// Restores text flow and paging for the TextLayer to the rectangular defaults.
	///
	/// # See also
	///
	/// [`text_layer_enable_screen_text_flow_and_paging`], [`graphics_text_attributes_restore_default_text_flow`], [`graphics_text_attributes_restore_default_paging`]
	pub fn text_layer_restore_default_text_flow_and_paging(text_layer: HTextLayer);

	/// Calculates the size occupied by the current text of the TextLayer.
	pub fn text_layer_get_content_size(text_layer: HTextLayer) -> GSize;

	/// Update the size of the text layer This is a convenience function to update the frame of the TextLayer.
	pub fn text_layer_set_size(text_layer: HTextLayer, max_size: GSize);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HTextLayer(NonNull<c_void>);

impl Deref for HTextLayer {
	type Target = HLayer;

	fn deref(&self) -> &Self::Target {
		unsafe {
			//SAFETY: "Supertype" cast according to documentation.
			&*(self as *const Self).cast()
		}
	}
}
