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

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HWnd(pub NonZeroUsize);

/// RECT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rect {
	pub left: i32,
	pub top: i32,
	pub right: i32,
	pub bottom: i32,
}

/// LUID
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Luid {
	pub low_part: u32,
	pub high_part: i32,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Handle(pub NonZeroUsize);

/// POINT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

/// FILETIME
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FileTime {
	pub low_date_time: u32,
	pub high_date_time: u32,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HInstance(pub NonZeroUsize);

/// SIZE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Size {
	pub cx: i32,
	pub cy: i32,
}

/// CHAR
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Char {
	pub value: u8,
}

/// POINTL
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct POIntl {
	pub x: i32,
	pub y: i32,
}

/// RECTL
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RECtl {
	pub left: i32,
	pub top: i32,
	pub right: i32,
	pub bottom: i32,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct BStr<'a>(pub &'a u16);

