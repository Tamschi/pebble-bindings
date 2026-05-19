use crate::user_interface::window::HWindow;

unsafe extern "C" {
	pub fn window_stack_push(window: HWindow, animated: bool);
	pub fn window_stack_pop(animated: bool) -> Option<HWindow>;
	pub fn window_stack_pop_all(animated: bool);
	pub fn window_stack_remove(window: HWindow, animated: bool) -> bool;
	pub fn window_stack_get_top_window() -> Option<HWindow>;
	pub fn window_stack_contains_window(window: HWindow) -> bool;
}
