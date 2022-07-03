#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;
use crate::core::win32::foundation::*;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SecurityAttributes<'a> {
	pub length: u32,
	pub security_descriptor: &'a c_void,
	pub inherit_handle: Bool,
}

