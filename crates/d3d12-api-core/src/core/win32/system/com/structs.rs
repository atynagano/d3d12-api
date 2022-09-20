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
use crate::core::win32::system::com::*;

/// STATSTG
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union Cy {
	pub anonymous: CyAnonymousStruct,
	pub int64: i64,
}

impl std::fmt::Debug for Cy {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// _Anonymous_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CyAnonymousStruct {
	pub lo: u32,
	pub hi: i32,
}

/// VARIANT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Variant<'a> {
	pub anonymous: VariantAnonymousUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VariantAnonymousUnion<'a> {
	// todo
	i: &'a i32,
}

impl std::fmt::Debug for VariantAnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// _Anonymous_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VariantAnonymousUnionAnonymousStruct<'a> {
	pub vt: u16,
	pub reserved1: u16,
	pub reserved2: u16,
	pub reserved3: u16,
	pub anonymous: VariantAnonymousUnionAnonymousStructAnonymousUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VariantAnonymousUnionAnonymousStructAnonymousUnion<'a> {
	// todo
	i: &'a i32,
}

impl std::fmt::Debug for VariantAnonymousUnionAnonymousStructAnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// EXCEPINFO
#[repr(C)]
pub struct ExCepInfo<'a> {
	pub code: u16,
	pub reserved: u16,
	pub bstr_source: BStr<'a>,
	pub bstr_description: BStr<'a>,
	pub bstr_help_file: BStr<'a>,
	pub help_context: u32,
	pub pv_reserved: &'a c_void,
	pub pfn_deferred_fill_in: LPExCepFinoDeferredFilLin,
	pub scode: i32,
}

