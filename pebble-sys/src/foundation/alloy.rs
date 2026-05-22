//! > The APIs on this page will only work with SDK 4.9+.

unsafe extern "C" {
	/// Create and start a Moddable XS virtual machine for an Alloy app.
	///
	/// # Parameters
	///
	/// - `creation`: Configuration record, or NULL for default settings.
	///
	/// #TODO
	///
	/// Can the parameter be a transient reference?
	pub fn moddable_createMachine(creation: *const ModdableCreationRecord);
}

/// Configuration record for creating a Moddable XS virtual machine.
///
/// Used with [`moddable_createMachine()`] to customize the JS runtime.
/// Set recordSize to `mem::size_of::<ModdableCreationRecord()>` for version compatibility.
#[repr(C)]
pub struct ModdableCreationRecord {
	/// Size of this struct in bytes (for versioning)
	record_size: u32,

	/// Stack size in bytes (0 for default)
	stack: u32,

	/// Slot heap size in bytes (0 for default)
	slot: u32,

	/// Chunk heap size in bytes (0 for default)
	chunk: u32,

	/// Combination of kModdableCreationFlag* values.
	flags: u32,
}

/// Flag to enable XS instrumentation logging over Bluetooth.
///
/// When set, the Moddable XS engine will log instrumentation data (e.g. memory usage, slot/chunk/stack statistics) via app_log.
/// Logging is only active when a Bluetooth log listener is connected; otherwise this flag has no effect.
///
/// # See also
///
/// [`ModdableCreationRecord`]
pub const K_MODDABLE_CREATION_FLAG_LOG_INSTRUMENTATION: u32 = 1;
