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
use crate::core::win32::system::com::structured_storage::*;
use crate::core::win32::foundation::*;

/// PROPVARIANT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PropVariant<'a> {
	pub anonymous: PropVariantAnonymousUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union PropVariantAnonymousUnion<'a> {
	// todo
	i: &'a i32,
}

impl std::fmt::Debug for PropVariantAnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// _Anonymous_e__Struct
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PropVariantAnonymousUnionAnonymousStruct<'a> {
	pub vt: u16,
	pub reserved1: u16,
	pub reserved2: u16,
	pub reserved3: u16,
	pub anonymous: PropVariantAnonymousUnionAnonymousStructAnonymousUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union PropVariantAnonymousUnionAnonymousStructAnonymousUnion<'a> {
	// todo
	i: &'a i32,
}

impl std::fmt::Debug for PropVariantAnonymousUnionAnonymousStructAnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

/// PROPBAG2
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PropBag2<'a> {
	pub ty: u32,
	pub vt: u16,
	pub cf_type: u16,
	pub hint: u32,
	pub pstr_name: PWStr<'a>,
	pub clsid: GUID,
}

