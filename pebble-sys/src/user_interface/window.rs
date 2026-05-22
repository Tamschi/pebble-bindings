//! The basic building block of the user interface
//!
//! Windows are the top-level elements in the UI hierarchy and the basic building blocks for a Pebble UI.
//! A single window is always displayed at a time on Pebble, with the exception of when animating from one window to the other,
//! which, in that case, is managed by the window stack. You can stack windows on top of each other, but only the topmost window will be visible.
//!
//! Users wearing a Pebble typically interact with the content and media displayed in a window,
//! clicking and pressing buttons on the watch, depending on what they see and wish to respond to in a window.
//!
//! Windows serve to display a hierarchy of layers on the screen and handle user input.
//! When a window is visible, its root Layer (and all its child layers) are drawn onto the screen automatically.
//!
//! You need a window, which always fills the entire screen, to display images, text, and graphics in your Pebble app.
//! A layer by itself doesn’t display on Pebble; it must be in the current window’s layer hierarchy to be visible.
//!
//! The Window Stack serves as the global manager of what window is presented and makes sure that input events are forwarded to the topmost window.
//!
//! Refer to the [User Interface Layers chapter in the Pebble Developer Guides](https://developer.getpebble.com/guides/pebble-apps/display-and-animations/layers/)
//! (chapter "Window") for a conceptual overview of Window, the Window Stack and relevant code examples.

use super::clicks::{ButtonId, ClickConfigProvider, ClickHandler};
use crate::{graphics::graphics_types::GColor, user_interface::layers::HLayer};
use core::{ffi::c_void, ptr::NonNull};

pub mod number_window;

pub type WindowHandler = extern "C" fn(window: HWindow);

unsafe extern "C" {
	/// Creates a new Window on the heap and initializes it with the default values.
	///
	/// - Background color: [`GColorWhite`]
	/// - Root layer's `update_proc`: function that fills the window's background using `background_color`.
	/// - `click_config_provider`: [`None`]
	/// - `window_handlers`: all [`None`]
	///
	/// # Returns
	///
	/// [`Ok`] with pointer to the window. [`Err`] if the window could not be created
	pub safe fn window_create() -> Result<HWindow, ()>;

	/// Destroys a Window previously created by window_create.
	pub fn window_destroy(window: HWindow);

	/// Sets the click configuration provider callback function on the window.
	///
	/// This will automatically setup the input handlers of the window as well to use the click recognizer subsystem.
	///
	/// # See also
	///
	/// [`super::clicks`], [`ClickConfigProvider`]
	pub fn window_set_click_config_provider(
		window: HWindow,
		click_config_provider: Option<ClickConfigProvider>,
	);

	/// Same as [`window_set_click_config_provider()`], but will assign a custom context pointer (instead of the window pointer) that will be passed into the [`ClickHandler`] click event handlers.
	pub fn window_set_click_config_provider_with_context(
		window: HWindow,
		click_config_provider: Option<ClickConfigProvider>,
		context: *mut c_void,
	);

	/// Gets the current click configuration provider of the window.
	pub fn window_get_click_config_provider(window: HWindow) -> Option<ClickConfigProvider>;

	/// Gets the current click configuration provider context of the window.
	pub fn window_get_click_config_context(window: HWindow) -> *mut c_void;

	/// Sets the window handlers of the window. These handlers get called e.g. when the user enters or leaves the window.
	pub fn window_set_window_handlers(window: HWindow, handlers: WindowHandlers);

	/// Gets the root Layer of the window.
	///
	/// The root layer is the layer at the bottom of the layer hierarchy for this window.
	/// It is the window's "canvas" if you will.
	///
	/// By default, the root layer only draws a solid fill with the window's background color.
	pub fn window_get_root_layer(window: HWindow) -> HLayer;

	/// Sets the background color of the window, which is drawn automatically by the root layer of the window.
	///
	/// # See also
	///
	/// [`window_get_root_layer()`]
	pub fn window_set_background_color(window: HWindow, background_color: GColor);

	/// Gets whether the window has been loaded.
	///
	/// If a window is loaded, its [`.load`](`WindowHandlers::load`) handler has been called (and the [`.unload`](`WindowHandlers::unload`) handler has not been called since).
	///
	/// # See also
	///
	/// [`WindowHandlers`]
	pub fn window_is_loaded(window: HWindow) -> bool;

	/// Sets a pointer to developer-supplied data that the window uses, to provide a means to access the data at later times in one of the window event handlers.
	///
	/// # See also
	///
	/// [`window_get_user_data()`]
	pub fn window_set_user_data(window: HWindow, data: *mut c_void);

	/// Gets the pointer to developer-supplied data that was previously set using [`window_set_user_data()`].
	pub fn window_get_user_data(window: HWindow) -> *mut c_void;

	/// Subscribe to single click events.
	///
	/// # Notes
	///
	/// Must be called from the [`ClickConfigProvider`].
	///
	/// [`window_single_click_subscribe()`] and [`window_single_repeating_click_subscribe()`] conflict, and cannot both be used on the same button.
	///
	/// When there is a multi_click and/or long_click setup, there will be a delay before the single click
	///
	/// # Parameters
	///
	/// - `handler`: The [`ClickHandler`] to fire on this event. handler will get fired. On the other hand, when there is no multi_click nor long_click setup, the single click handler will fire directly on button down.
	///
	/// # See also
	///
	/// [`super::clicks`]
	pub fn window_single_click_subscribe(button_id: ButtonId, handler: ClickHandler);

	/// Subscribe to single click event, with a repeat interval. A single click is detected every time "repeat_interval_ms" has been reached.
	///
	/// # Notes
	///
	/// Must be called from the [`ClickConfigProvider`].
	///
	/// [`window_single_click_subscribe()`] and [`window_single_repeating_click_subscribe()`] conflict, and cannot both be used on the same button.
	///
	/// The back button cannot be overridden with a repeating click.
	///
	/// # Parameters
	///
	/// - `repeat_interval_ms`: When holding down, how many milliseconds before the handler is fired again. A value of 0ms means "no repeat timer". The minimum is 30ms, and values below will be disregarded. If there is a long-click handler subscribed on this button, `repeat_interval_ms` will not be used.
	pub fn window_single_repeating_click_subscribe(
		button_id: ButtonId,
		repeat_interval_ms: u16,
		handler: ClickHandler,
	);

	pub fn window_multi_click_subscribe(
		button_id: ButtonId,
		min_clicks: u8,
		max_clicks: u8,
		timeout: u16,
		last_click_only: bool,
		handler: ClickHandler,
	);
	pub fn window_long_click_subscribe(
		button_id: ButtonId,
		delay_ms: u16,
		down_handler: ClickHandler,
		up_handler: ClickHandler,
	);
	pub fn window_raw_click_subscribe(
		button_id: ButtonId,
		down_handler: ClickHandler,
		up_handler: ClickHandler,
		context: Option<NonNull<c_void>>,
	);
	pub fn window_set_click_context(button_id: ButtonId, context: *mut c_void);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HWindow(NonNull<c_void>);

#[repr(C)]
pub struct WindowHandlers {
	pub load: Option<WindowHandler>,
	pub appear: Option<WindowHandler>,
	pub disappear: Option<WindowHandler>,
	pub unload: Option<WindowHandler>,
}
