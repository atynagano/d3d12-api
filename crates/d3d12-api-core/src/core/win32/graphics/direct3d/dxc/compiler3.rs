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
pub struct DxcCompiler3(pub(crate) Unknown);

impl Guid for DxcCompiler3 {
	const IID: &'static GUID = &GUID::from_u128(0x228b46875a6a4730900c9702b2203f54u128);
}

impl Clone for DxcCompiler3 {
	fn clone(&self) -> Self { DxcCompiler3(self.0.clone()) }
}

pub trait IDxcCompiler3: IUnknown {
	fn as_compiler3(&self) -> &DxcCompiler3;
	fn into_compiler3(self) -> DxcCompiler3;

	fn Compile<T: IUnknown>(&self, source: &DxcBuffer, arguments: Option<&[&str]>, include_handler: Option<&DxcIncludeHandler>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_arguments, _len_arguments) = arguments.deconstruct();
			let mut _out_result: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, source: &DxcBuffer, arguments: *const PWStr, arg_count: u32, include_handler: *const c_void, riid: &GUID, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, source, arguments.to_utf16_vec().ptr(), _len_arguments as u32, option_to_vtable(include_handler), T::IID, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(T::from(_out_result));
				}
			}
			Err(_ret_)
		}
	}

	fn Disassemble<T: IUnknown>(&self, object: &DxcBuffer, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, object: &DxcBuffer, riid: &GUID, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, object, T::IID, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(T::from(_out_result));
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcCompiler3 for DxcCompiler3 {
	fn as_compiler3(&self) -> &DxcCompiler3 { self }
	fn into_compiler3(self) -> DxcCompiler3 { self }
}

impl From<Unknown> for DxcCompiler3 {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcCompiler3 {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

