//! Vertical, bar-shaped control widget on the right edge of the window
//!
//! TODO: Image
//!
//! ActionBarLayer is a Layer that displays a bar on the right edge of the window.
//! The bar can contain up to 3 icons, each corresponding with one of the buttons on the right side of the watch.
//! The purpose of each icon is to provide a hint (feed-forward) to what action a click on the respective button will cause.
//!
//! The action bar is useful when there are a few (up to 3) main actions that are desirable to be able to take quickly, literally with one press of a button.
//!
//! # More actions
//!
//! If there are more than 3 actions the user might want to take:
//!
//! Try assigning the top and bottom icons of the action bar to the two most immediate actions and use the middle icon to push a Window with a MenuLayer with less immediate actions.
//!
//! Secondary actions that are not vital, can be "hidden" under a long click.
//! Try to group similar actions to one button. For example, in a Music app,
//! a single click on the top button is tied to the action to jump to the previous track. Holding that same button means seek backwards.
//!
//! # Directionality mapping
//!
//! When the top and bottom buttons are used to control navigating through a (potentially virtual, non-visible) list of items, follow this guideline:
//!
//! Tie the top button to the action that goes to the previous item in the list, for example "jump to previous track" in a Music app.
//!
//! Tie the bottom button to the action that goes to the next item in the list, for example "jump to next track" in a Music app.
//!
//! # Geometry
//!
//! The action bar's width varies per platform. 30px on most displays, 34px on Emery and Gabbro, and 40px on Chalk. Use the ACTION_BAR_WIDTH define.
//!
//! Icons should not be wider than 28 pixels, or taller than 18 pixels. It is recommended to use a size of around 15 x 15 pixels for the "visual core" of the icon, and extending or contracting where needed.
//!
//! # Example Code
//!
//! The code example below shows how to do the initial setup of the action bar in a window's .load handler.
//! Configuring the button actions is similar to the process when using [`window_set_click_config_provider()`](`crate::window::window_set_click_config_provider()`).
//!
//! See [Clicks](`crate::clicks`) for more information.
//!
//! ```c
//! // C
//! ActionBarLayer *action_bar;
//!
//! // The implementation of my_next_click_handler and my_previous_click_handler
//! // is omitted for the sake of brevity. See the Clicks reference docs.
//!
//! void click_config_provider(void *context) {
//!   window_single_click_subscribe(BUTTON_ID_DOWN, (ClickHandler) my_next_click_handler);
//!   window_single_click_subscribe(BUTTON_ID_UP, (ClickHandler) my_previous_click_handler);
//! }
//!
//! void window_load(Window *window) {
//!   ...
//!   // Initialize the action bar:
//!   action_bar = action_bar_layer_create();
//!   // Associate the action bar with the window:
//!   action_bar_layer_add_to_window(action_bar, window);
//!   // Set the click config provider:
//!   action_bar_layer_set_click_config_provider(action_bar,
//!                                              click_config_provider);
//!
//!   // Set the icons:
//!   // The loading of the icons is omitted for brevity... See gbitmap_create_with_resource()
//!   action_bar_layer_set_icon_animated(action_bar, BUTTON_ID_UP, my_icon_previous, true);
//!   action_bar_layer_set_icon_animated(action_bar, BUTTON_ID_DOWN, my_icon_next, true);
//! }
//! ```
//!

use core::{ffi::c_void, ops::Deref, ptr::NonNull};

use crate::user_interface::layers::{HLayer, action_bar_layer};

unsafe extern "C" {
	/// Creates a new ActionBarLayer on the heap and initalizes it with the default values.
	///
	/// - Background color: [`BLACK`](`crate::graphics::graphics_types::color_definitions::g_color::BLACK`);
	/// - No click configuration provider ([`None`])
	/// - No icons
	/// - Not added to / associated with any window, thus not catching any button input yet.
	pub fn action_bar_layer_create() -> Option<HActionBarLayer>;

	/// Destroys a ActionBarLayer previously created by [`action_bar_layer_create`].
	pub fn action_bar_layer_destroy(action_bar_layer: HActionBarLayer) -> Option<HActionBarLayer>;

	/// Gets the "root" Layer of the action bar layer,
	/// which is the parent for the sub- layers used for its implementation.
	pub fn action_bar_layer_get_layer(action_bar_layer: HActionBarLayer) -> HLayer;

	/// Sets the context parameter, which will be passed in to [`ClickHandler`](`crate::user_interface::clicks::ClickHandler`)
	/// callbacks and the [`ClickConfigProvider`](`crate::user_interface::clicks::ClickConfigProvider`)
	/// callback of the action bar.
	///
	/// # Note
	///
	/// **By default, a pointer to the action bar itself** is passed in,
	/// if the context has not been set or if it has been set to [`None`].
	///
	/// # See also
	///
	/// [`action_bar_layer_set_click_config_provider()`], [`crate::user_interface::clicks`]
	pub fn action_bar_layer_set_context(
		action_bar: HActionBarLayer,
		context: Option<NonNull<c_void>>,
	);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HActionBarLayer(NonNull<c_void>);

impl Deref for HActionBarLayer {
	type Target = HLayer;

	fn deref(&self) -> &Self::Target {
		unsafe {
			//SAFETY: "Supertype" cast according to documentation.
			&*(self as *const Self).cast()
		}
	}
}
