#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rect {
	pub left: i32,
	pub top: i32,
	pub right: i32,
	pub bottom: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Luid {
	pub low_part: u32,
	pub high_part: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Handle {
	pub value: NonNull<c_void>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HWnd {
	pub value: NonNull<c_void>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FileTime {
	pub low_date_time: u32,
	pub high_date_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HInstance {
	pub value: NonNull<c_void>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BStr<'a> {
	pub value: &'a u16,
}

