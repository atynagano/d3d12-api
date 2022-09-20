#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;
use crate::core::win32::foundation::*;

/// WICRect
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WicRect {
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

/// WICBitmapPattern
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WicBitmapPattern<'a> {
	pub position: u64,
	pub length: u32,
	pub pattern: &'a u8,
	pub mask: &'a u8,
	pub end_of_stream: Bool,
}

