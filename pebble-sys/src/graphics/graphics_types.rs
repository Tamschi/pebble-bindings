use core::{ffi::c_void, mem, ptr::NonNull};

use crate::sys_helpers::u2;

pub mod color_definitions;

#[repr(C)]
pub struct GPoint {
	pub x: i16,
	pub y: i16,
}

#[repr(C)]
pub struct GRect {
	pub origin: GPoint,
	pub size: GSize,
}

#[repr(C)]
pub struct GSize {
	pub w: i16,
	pub h: i16,
}

#[repr(C)]
pub union GColor8 {
	pub argb: u8,
}

impl GColor8 {
	pub const fn a(&self) -> u2 {
		unsafe { mem::transmute((self.argb >> 6) & 0b11) }
	}
	pub const fn r(&self) -> u2 {
		unsafe { mem::transmute((self.argb >> 4) & 0b11) }
	}
	pub const fn g(&self) -> u2 {
		unsafe { mem::transmute((self.argb >> 2) & 0b11) }
	}
	pub const fn b(&self) -> u2 {
		unsafe { mem::transmute((self.argb >> 0) & 0b11) }
	}

	pub const fn set_a(&mut self, a: u2) {
		unsafe {
			self.argb &= 0b00111111;
			self.argb |= (a as u8) << 6;
		}
	}
	pub const fn set_r(&mut self, r: u2) {
		unsafe {
			self.argb &= 0b11001111;
			self.argb |= (r as u8) << 4;
		}
	}
	pub const fn set_g(&mut self, g: u2) {
		unsafe {
			self.argb &= 0b11110011;
			self.argb |= (g as u8) << 2;
		}
	}
	pub const fn set_b(&mut self, b: u2) {
		unsafe {
			self.argb &= 0b11111100;
			self.argb |= (b as u8) << 0;
		}
	}
}

pub type GColor = GColor8;

#[repr(transparent)]
pub struct HGBitmap(NonNull<c_void>);
#[repr(transparent)]
pub struct HGBitmapSequence(NonNull<c_void>);
#[repr(transparent)]
pub struct HGContext(NonNull<c_void>);

pub const fn g_color_from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> GColor8 {
	GColor8 {
		argb: (alpha & 0b11_00_00_00)
			| ((red & 0b11_00_00_00) >> 2)
			| ((green & 0b11_00_00_00) >> 4)
			| ((blue & 0b11_00_00_00) >> 6),
	}
}

pub const fn g_color_from_rgb(red: u8, green: u8, blue: u8) -> GColor8 {
	g_color_from_rgba(red, green, blue, 255)
}

pub const fn g_color_from_hex(v: u32) -> GColor8 {
	g_color_from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8)
}
