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
pub struct DxcLinker(pub(crate) Unknown);

impl Deref for DxcLinker {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcLinker {
	const IID: &'static GUID = &GUID::from_u128(0xf1b5be2a62dd4327a1c242ac1e1e78e6u128);
}

impl Com for DxcLinker {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcLinker {
	pub fn RegisterLibrary(&self, lib_name: Option<&str>, lib: &DxcBlob) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, VTable) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, lib_name.map(str::to_utf16).as_ptr_or_null(), lib.vtable());
			_ret_.ok()
		}
	}

	pub fn Link(&self, entry_name: Option<&str>, target_profile: &str, lib_names: &[&str], arguments: Option<&[&str]>) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let (lib_names_ptr_, lib_names_len_) = lib_names.deconstruct();
			let (arguments_ptr_, arguments_len_) = arguments.deconstruct();
			let mut result_out_: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *const u16, *const PWStr, u32, *const PWStr, u32, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, entry_name.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), lib_names.to_utf16_vec().ptr(), lib_names_len_ as u32, arguments.to_utf16_vec().ptr(), arguments_len_ as u32, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcLinker: IUnknown {
	fn as_linker(&self) -> &DxcLinker;
	fn into_linker(self) -> DxcLinker;
}

impl IDxcLinker for DxcLinker {
	fn as_linker(&self) -> &DxcLinker { self }
	fn into_linker(self) -> DxcLinker { self }
}
impl IUnknown for DxcLinker {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcLinker {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

