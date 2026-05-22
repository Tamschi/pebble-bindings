//! User interface layers for displaying graphic components
//!
//! Layers are objects that can be displayed on a Pebble watchapp window, enabling users to see visual objects, like text or images.
//! Each layer stores the information about its state necessary to draw or redraw the object that it represents and uses graphics routines along with this state to draw itself when asked.
//! Layers can be used to display various graphics.
//!
//! Layers are the basic building blocks for your application UI.
//! Layers can be nested inside each other. Every window has a root layer which is always the topmost layer.
//! You provide a function that is called to draw the content of the layer when needed;
//! or you can use standard layers that are provided by the system, such as text layer, image layer, menu layer, action bar layer, and so on.
//!
//! The Pebble layer hierarchy is the list of things that need to be drawn to the screen. Multiple layers can be arranged into a hierarchy.
//! This enables ordering (front to back), layout and hierarchy. Through relative positioning, visual objects that are grouped together by
//! adding them into the same layer can be moved all at once. This means that the child layers will move accordingly.
//! If a parent layer has clipping enabled, all the children will be clipped to the frame of the parent.
//!
//! Pebble OS provides convenience layers with built-in logic for displaying different graphic components, like text and bitmap layers.
//!
//! Refer to the [User Interface Layers chapter in the Pebble Developer Guides](https://developer.getpebble.com/guides/pebble-apps/display-and-animations/layers/)
//! (chapter "Layers") for a conceptual overview of Layers and relevant code examples.
//!
//! The Modules listed here contain what can be thought of conceptually as subclasses of Layer.
//! The listed types can be safely type-casted to Layer (or Layer * in case of a pointer). The layer_... functions can then be used with the data structures of these subclasses.
//! For example, the following is legal:
//!
//! ```c
//! // C
//! TextLayer *text_layer;
//! ...
//! layer_set_hidden((Layer *)text_layer, true);
//! ```

use crate::{
	graphics::graphics_types::{GPoint, GRect, HGContext},
	user_interface::window::HWindow,
};
use core::{ffi::c_void, ptr::NonNull};

pub mod action_bar_layer;

unsafe extern "C" {
	/// Creates a layer on the heap and sets its frame and bounds. Default values:
	///
	/// - `bounds`: origin (0, 0) and a size equal to the frame that is passed in.
	/// - `clips`: `true`
	/// - `hidden`: `false`
	/// - `update_proc`: `None` (draws nothing)
	pub safe fn layer_create(frame: GRect) -> Result<HLayer, ()>;

	/// Creates a layer on the heap with extra space for callback data, and sets its frame and bounds.
	/// Default values:
	///
	/// - `bounds`: origin (0, 0) and a size equal to the frame that is passed in.
	/// - `clips`: `true`
	/// - `hidden`: `false`
	/// - `update_proc`: `None` (draws nothing)
	///
	/// # Parameters
	///
	/// - `data_size`: The size (in bytes) of memory to allocate for callback data.
	///
	/// # See also
	///
	/// [`layer_create()`], [`layer_set_frame()`], [`layer_set_bounds()`]
	pub safe fn layer_create_with_data(frame: GRect, data_size: usize) -> Result<HLayer, ()>;

	/// Destroys a layer previously created by [`layer_create()`] or [`layer_create_with_data()`].
	pub fn layer_destroy(layer: HLayer);

	/// Marks the complete layer as "dirty", awaiting to be asked by the system to redraw itself.
	///
	/// Typically, this function is called whenever state has changed that affects what the layer is displaying.
	///
	/// - The layer's `.update_proc` will not be called before this function returns, but will be called asynchronously, shortly.
	///
	/// - Internally, a call to this function will schedule a re-render of the window that the layer belongs to.
	///   In effect, all layers in that window's layer hierarchy will be asked to redraw.
	///
	/// - If an earlier re-render request is still pending, this function is a no-op.
	pub fn layer_mark_dirty(layer: HLayer);

	/// Sets the layer's render function.
	///
	/// The system will call the update_proc automatically when the layer needs to redraw itself.
	///
	/// # See also
	///
	/// [`layer_mark_dirty()`]
	pub fn layer_set_update_proc(
		layer: HLayer,
		update_proc: Option<LayerUpdateProc>, //TODO: Check if this is legal!
	);
	pub fn layer_set_frame(layer: HLayer, frame: GRect);
	pub fn layer_get_frame(layer: HLayer) -> GRect;
	pub fn layer_set_bounds(layer: HLayer, bounds: GRect);
	pub fn layer_get_bounds(layer: HLayer) -> GRect;
	pub fn layer_convert_point_to_screen(layer: HLayer, point: GPoint) -> GPoint;
	pub fn layer_convert_rect_to_screen(layer: HLayer, rect: GRect) -> GRect;
	pub fn layer_get_window(layer: HLayer) -> HWindow;
	pub fn layer_remove_from_parent(child: HLayer);
	pub fn layer_remove_child_layers(parent: HLayer);
	pub fn layer_add_child(parent: HLayer, child: HLayer);
	pub fn layer_insert_below_sibling(layer_to_insert: HLayer, below_sibling_layer: HLayer);
	pub fn layer_insert_above_sibling(layer_to_insert: HLayer, above_sibling_layer: HLayer);
	pub fn layer_set_hidden(layer: HLayer, hidden: bool);
	pub fn layer_get_hidden(layer: HLayer) -> bool;
	pub fn layer_set_clips(layer: HLayer, clips: bool);
	pub fn layer_get_clips(layer: HLayer) -> bool;

	pub fn layer_get_data(layer: HLayer) -> NonNull<c_void>; //TODO: Check if this is actually non-null.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HLayer(NonNull<c_void>);

/// Function signature for a Layer's render callback (the name of the type is derived from the words 'update procedure').
///
/// The system will call the `.update_proc` callback whenever the Layer needs to be rendered.
///
/// # See also
///
/// [`crate::graphics`], [`layer_set_update_proc()`]
pub type LayerUpdateProc = extern "C" fn(layer: HLayer, ctx: HGContext);
