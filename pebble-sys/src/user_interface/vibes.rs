use core::marker::PhantomData;

#[repr(C)]
pub struct VibePattern<'a> {
	/// Pointer to an array of segment durations in on(off on)*off? order, up to 10_000ms each.
	/// There must be at least one duration!
	pub durations: *const u32,
	/// Length of the array.
	pub num_segments: u32,
	pub phantom: PhantomData<&'a u32>,
}

unsafe extern "C" {
	pub fn vibes_cancel();
	pub fn vibes_short_pulse();
	pub fn vibes_long_pulse();
	pub fn vibes_double_pulse();
	pub fn vibes_enqueue_custom_pattern(pattern: VibePattern);
}
