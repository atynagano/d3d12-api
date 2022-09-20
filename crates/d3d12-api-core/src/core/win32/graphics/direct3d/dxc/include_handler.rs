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
use crate::core::win32::graphics::direct3d::dxc::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcIncludeHandler(pub(crate) Unknown);

impl Deref for DxcIncludeHandler {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcIncludeHandler {
	const IID: &'static GUID = &GUID::from_u128(0x7f61fc7d950d467fb3e33c02fb49187cu128);
}

impl Com for DxcIncludeHandler {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcIncludeHandler {
	pub fn LoadSource(&self, filename: &str, mut include_source: Option<&mut Option<DxcBlob>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut include_source) = include_source { **include_source = None; }
			let f: extern "system" fn(Param<Self>, *const u16, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, filename.to_utf16().as_ptr_or_null(), transmute(include_source));
			_ret_.ok()
		}
	}

	pub fn load_source(&self, filename: &str) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut include_source_out_: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, filename.to_utf16().as_ptr_or_null(), transmute(&mut include_source_out_));
			if _ret_.is_ok() { if let Some(include_source_out_) = include_source_out_ { return Ok(include_source_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcIncludeHandler: IUnknown {
	fn as_include_handler(&self) -> &DxcIncludeHandler;
	fn into_include_handler(self) -> DxcIncludeHandler;
}

impl IDxcIncludeHandler for DxcIncludeHandler {
	fn as_include_handler(&self) -> &DxcIncludeHandler { self }
	fn into_include_handler(self) -> DxcIncludeHandler { self }
}
impl IUnknown for DxcIncludeHandler {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcIncludeHandler {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

