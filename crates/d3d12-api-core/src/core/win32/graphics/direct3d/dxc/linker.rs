#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{MaybeUninit, size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct3d::dxc::*;

#[repr(C)]
pub struct DxcLinker(pub(crate) Unknown);

impl Guid for DxcLinker {
	const IID: &'static GUID = &GUID::from_u128(0xf1b5be2a62dd4327a1c242ac1e1e78e6u128);
}

impl Clone for DxcLinker {
	fn clone(&self) -> Self { DxcLinker(self.0.clone()) }
}

pub trait IDxcLinker: IUnknown {
	fn as_linker(&self) -> &DxcLinker;
	fn into_linker(self) -> DxcLinker;

	fn RegisterLibrary(&self, lib_name: Option<&str>, lib: &(impl IDxcBlob + ?Sized), ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, lib_name: *const u16, lib: VTable, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, lib_name.map(str::to_utf16).as_ptr_or_null(), lib.vtable(), );
			_ret_.ok()
		}
	}

	fn Link(&self, entry_name: Option<&str>, target_profile: &str, lib_names: &[&str], arguments: Option<&[&str]>, ) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_lib_names, _len_lib_names) = lib_names.deconstruct();
			let (_ptr_arguments, _len_arguments) = arguments.deconstruct();
			let mut _out_result: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, entry_name: *const u16, target_profile: *const u16, lib_names: *const PWStr, lib_count: u32, arguments: *const PWStr, arg_count: u32, result: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, entry_name.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), lib_names.to_utf16_vec().ptr(), _len_lib_names as u32, arguments.to_utf16_vec().ptr(), _len_arguments as u32, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcLinker for DxcLinker {
	fn as_linker(&self) -> &DxcLinker { self }
	fn into_linker(self) -> DxcLinker { self }
}

impl From<Unknown> for DxcLinker {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcLinker {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

