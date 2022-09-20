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

/// DxcShaderHash
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxcShaderHash {
	pub flags: u32,
	pub hash_digest: [u8; 16],
}

/// DxcBuffer
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxcBuffer<'a> {
	pub ptr: &'a c_void,
	pub size: usize,
	pub encoding: u32,
}

/// DxcDefine
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxcDefine<'a> {
	pub name: PWStr<'a>,
	pub value: PWStr<'a>,
}

/// DxcArgPair
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DxcArgPair<'a> {
	pub name: PWStr<'a>,
	pub value: PWStr<'a>,
}

