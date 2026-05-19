//! Standard C types, functions, constants, etc.
//!
//! The standard C functions available here, such as [`snprintf()`](TODO) and [`time()`](TODO), are provided by the firmware.
//! Using these functions will not significantly increase the size of your app beyond what is needed to call the function.
//! You may use other standard C functions not listed here, but be aware that not all will successfully be added to your app,
//! and if they are added, your app's binary size will increase accordingly.

pub mod memory;
