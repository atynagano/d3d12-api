#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::num::NonZeroUsize;
use std::mem::{MaybeUninit, size_of_val, transmute};
use std::ops::Deref;
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcVersionInfo3(pub(crate) Unknown);

impl Deref for DxcVersionInfo3 {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcVersionInfo3 {
	const IID: &'static GUID = &GUID::from_u128(0x5e13e8439d25473c9ad203b2d0b44b1eu128);
}

impl Com for DxcVersionInfo3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcVersionInfo3 {
	pub unsafe fn GetCustomVersionString(&self) { todo!() }
}

pub trait IDxcVersionInfo3: IUnknown {
	fn as_version_info3(&self) -> &DxcVersionInfo3;
	fn into_version_info3(self) -> DxcVersionInfo3;
}

impl IDxcVersionInfo3 for DxcVersionInfo3 {
	fn as_version_info3(&self) -> &DxcVersionInfo3 { self }
	fn into_version_info3(self) -> DxcVersionInfo3 { self }
}
impl IUnknown for DxcVersionInfo3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcVersionInfo3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

