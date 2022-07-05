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
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StatStg<'a> {
	pub pwcs_name: PWStr<'a>,
	pub ty: u32,
	pub size: u64,
	pub mtime: FileTime,
	pub ctime: FileTime,
	pub atime: FileTime,
	pub grf_mode: u32,
	pub grf_locks_supported: u32,
	pub clsid: GUID,
	pub grf_state_bits: u32,
	pub reserved: u32,
}

