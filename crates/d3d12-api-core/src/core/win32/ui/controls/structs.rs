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

/// MARGINS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Margins {
	pub cx_left_width: i32,
	pub cx_right_width: i32,
	pub cy_top_height: i32,
	pub cy_bottom_height: i32,
}

