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
pub struct DxcCompiler2(pub(crate) DxcCompiler);

impl Guid for DxcCompiler2 {
	const IID: &'static GUID = &GUID::from_u128(0xa005a9d9b8bb4594b5c90e633bec4d37u128);
}

impl Clone for DxcCompiler2 {
	fn clone(&self) -> Self { DxcCompiler2(self.0.clone()) }
}

pub trait IDxcCompiler2: IDxcCompiler {
	fn as_compiler2(&self) -> &DxcCompiler2;
	fn into_compiler2(self) -> DxcCompiler2;

	fn CompileWithDebug(&self, source: &(impl IDxcBlob + ?Sized), source_name: Option<&str>, entry_point: Option<&str>, target_profile: &str, arguments: Option<&[&str]>, defines: &[DxcDefine], include_handler: Option<&DxcIncludeHandler>, debug_blob_name: Option<&mut PWStr>, mut debug_blob: Option<&mut Option<DxcBlob>>,) -> Result<(DxcOperationResult), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcOperationResult> = None;
		if let Some(ref mut debug_blob) = debug_blob { **debug_blob = None; }
		let f: extern "system" fn(Param<Self>, source: VTable, source_name: *const u16, entry_point: *const u16, target_profile: *const u16, arguments: *const PWStr, arg_count: u32, defines: *const DxcDefine, define_count: u32, include_handler: Option<VTable>, _result: &mut Option<DxcOperationResult>, debug_blob_name: Option<&mut PWStr>, debug_blob: Option<&mut Option<DxcBlob>>, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, source.vtable(), source_name.map(str::to_utf16).as_ptr_or_null(), entry_point.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), arguments.to_utf16_vec().ptr(), arguments.len() as u32, defines.as_ptr_or_null(), defines.len() as u32, option_to_vtable(include_handler), &mut _result, debug_blob_name, debug_blob, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}
}

impl IDxcCompiler2 for DxcCompiler2 {
	fn as_compiler2(&self) -> &DxcCompiler2 { self }
	fn into_compiler2(self) -> DxcCompiler2 { self }
}

impl IDxcCompiler for DxcCompiler2 {
	fn as_compiler(&self) -> &DxcCompiler { &self.0 }
	fn into_compiler(self) -> DxcCompiler { self.0 }
}

impl From<Unknown> for DxcCompiler2 {
    fn from(v: Unknown) -> Self {
        Self(DxcCompiler::from(v))
    }
}

impl IUnknown for DxcCompiler2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

