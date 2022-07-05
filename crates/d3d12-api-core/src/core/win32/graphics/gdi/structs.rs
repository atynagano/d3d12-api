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
pub struct HMonitor {
	pub value: NonNull<c_void>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HBrush {
	pub value: NonNull<c_void>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HDc {
	pub value: NonNull<c_void>,
}

