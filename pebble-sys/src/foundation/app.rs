//! App entry point and event loop.
//!
//! App is a module that provides you with an event loop for your Pebble app.
//! All interaction between Pebble apps and the underlying Pebble OS takes place through an event loop.
//! Before calling the [`app_event_loop()`] function, you subscribe to event services and implement event handlers.
//! Each handler receives specific types of Events dispatched throughout the life of the Pebble watchapp.
//!
//! The [`app_event_loop()`] function takes care of both waiting for new events to become available on the watchapp event bus and routing new events to the appropriate handler. Event Service allows an app to directly register for different types of events.
//! This function will block until the watchapp is ready to exit, and should be placed in the app's main() function.
//!
//! A watchapp typically configures and uses the [`app_event_loop()`] as follows:
//!
//! ```
//! #[no_mangle]
//! pub extern "C" fn main() -> i32 {
//!   // do set up here
//!
//!   // Enter the main event loop. This will block until the app is ready to exit.
//!   app_event_loop();
//!
//!   // do clean up here
//! }
//! ```

unsafe extern "C" {
	/// The event loop for C apps, to be used in app's main().
	///
	/// Will block until the app is ready to exit.
	pub safe fn app_event_loop();
}
