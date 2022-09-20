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
use crate::core::win32::system::ole::*;
use crate::core::win32::system::com::*;

/// PARAMDESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ParamDesc<'a> {
	pub pparamdescex: &'a ParamDescEx<'a>,
	pub param_flags: u16,
}

/// PARAMDESCEX
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ParamDescEx<'a> {
	pub bytes: u32,
	pub var_default_value: Variant<'a>,
}

/// ARRAYDESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ArrayDesc<'a> {
	pub tdesc_elem: TypeDesc<'a>,
	pub dims: u16,
	pub rgbounds: [SafeArrayBound; 1],
}

