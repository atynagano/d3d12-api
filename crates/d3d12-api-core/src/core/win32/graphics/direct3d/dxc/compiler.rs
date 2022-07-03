#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
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

	fn Compile(&self, source: &(impl IDxcBlob + ?Sized), source_name: Option<&str>, entry_point: Option<&str>, target_profile: &str, arguments: Option<&[&str]>, defines: &[DxcDefine], include_handler: Option<&DxcIncludeHandler>, ) -> Result<(DxcOperationResult), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcOperationResult> = None;
		let f: extern "system" fn(Param<Self>, source: VTable, source_name: *const u16, entry_point: *const u16, target_profile: *const u16, arguments: *const PWStr, arg_count: u32, defines: *const DxcDefine, define_count: u32, include_handler: Option<VTable>, _result: &mut Option<DxcOperationResult>, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, source.vtable(), source_name.map(str::to_utf16).as_ptr_or_null(), entry_point.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), arguments.to_utf16_vec().ptr(), arguments.len() as u32, defines.as_ptr_or_null(), defines.len() as u32, option_to_vtable(include_handler), &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn Preprocess(&self, source: &(impl IDxcBlob + ?Sized), source_name: Option<&str>, arguments: Option<&[&str]>, defines: &[DxcDefine], include_handler: Option<&DxcIncludeHandler>, ) -> Result<(DxcOperationResult), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcOperationResult> = None;
		let f: extern "system" fn(Param<Self>, source: VTable, source_name: *const u16, arguments: *const PWStr, arg_count: u32, defines: *const DxcDefine, define_count: u32, include_handler: Option<VTable>, _result: &mut Option<DxcOperationResult>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, source.vtable(), source_name.map(str::to_utf16).as_ptr_or_null(), arguments.to_utf16_vec().ptr(), arguments.len() as u32, defines.as_ptr_or_null(), defines.len() as u32, option_to_vtable(include_handler), &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn Disassemble(&self, source: &(impl IDxcBlob + ?Sized), ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _disassembly: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, source: VTable, _disassembly: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, source.vtable(), &mut _disassembly, );
		if ret.is_ok() {
			if let (Some(_disassembly)) = (_disassembly) {
				return Ok((_disassembly));
			}
		}
		Err(ret)
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
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

