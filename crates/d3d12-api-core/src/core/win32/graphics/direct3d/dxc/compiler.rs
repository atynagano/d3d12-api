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
pub struct DxcCompiler(pub(crate) Unknown);

impl Guid for DxcCompiler {
	const IID: &'static GUID = &GUID::from_u128(0x8c210bf3011f44228d706f9acb8db617u128);
}

impl Clone for DxcCompiler {
	fn clone(&self) -> Self { DxcCompiler(self.0.clone()) }
}

pub trait IDxcCompiler: IUnknown {
	fn as_compiler(&self) -> &DxcCompiler;
	fn into_compiler(self) -> DxcCompiler;

	fn Compile(&self, source: &(impl IDxcBlob + ?Sized), source_name: Option<&str>, entry_point: Option<&str>, target_profile: &str, arguments: Option<&[&str]>, defines: &[DxcDefine], include_handler: Option<&DxcIncludeHandler>, ) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_arguments, _len_arguments) = arguments.deconstruct();
			let (_ptr_defines, _len_defines) = defines.deconstruct();
			let mut _out_result: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, source: VTable, source_name: *const u16, entry_point: *const u16, target_profile: *const u16, arguments: *const PWStr, arg_count: u32, defines: *const DxcDefine, define_count: u32, include_handler: *const c_void, result: *mut c_void, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, source.vtable(), source_name.map(str::to_utf16).as_ptr_or_null(), entry_point.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), arguments.to_utf16_vec().ptr(), _len_arguments as u32, _ptr_defines, _len_defines as u32, option_to_vtable(include_handler), transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn Preprocess(&self, source: &(impl IDxcBlob + ?Sized), source_name: Option<&str>, arguments: Option<&[&str]>, defines: &[DxcDefine], include_handler: Option<&DxcIncludeHandler>, ) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_arguments, _len_arguments) = arguments.deconstruct();
			let (_ptr_defines, _len_defines) = defines.deconstruct();
			let mut _out_result: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, source: VTable, source_name: *const u16, arguments: *const PWStr, arg_count: u32, defines: *const DxcDefine, define_count: u32, include_handler: *const c_void, result: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, source.vtable(), source_name.map(str::to_utf16).as_ptr_or_null(), arguments.to_utf16_vec().ptr(), _len_arguments as u32, _ptr_defines, _len_defines as u32, option_to_vtable(include_handler), transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn Disassemble(&self, source: &(impl IDxcBlob + ?Sized), ) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_disassembly: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, source: VTable, disassembly: *mut c_void, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, source.vtable(), transmute(&mut _out_disassembly), );
			if _ret_.is_ok() {
				if let Some(_out_disassembly) = _out_disassembly {
					return Ok(_out_disassembly);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcCompiler for DxcCompiler {
	fn as_compiler(&self) -> &DxcCompiler { self }
	fn into_compiler(self) -> DxcCompiler { self }
}

impl From<Unknown> for DxcCompiler {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcCompiler {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

