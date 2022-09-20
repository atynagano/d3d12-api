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
pub struct DxcCompiler2(pub(crate) DxcCompiler);

impl Deref for DxcCompiler2 {
	type Target = DxcCompiler;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcCompiler2 {
	const IID: &'static GUID = &GUID::from_u128(0xa005a9d9b8bb4594b5c90e633bec4d37u128);
}

impl Com for DxcCompiler2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcCompiler2 {
	pub fn CompileWithDebug(&self, source: &DxcBlob, source_name: Option<&str>, entry_point: Option<&str>, target_profile: &str, arguments: Option<&[&str]>, defines: &[DxcDefine], include_handler: Option<&DxcIncludeHandler>, debug_blob_name: Option<&mut MaybeUninit<PWStr>>, mut debug_blob: Option<&mut Option<DxcBlob>>) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let (arguments_ptr_, arguments_len_) = arguments.deconstruct();
			let (defines_ptr_, defines_len_) = defines.deconstruct();
			let mut result_out_: Option<DxcOperationResult> = None;
			if let Some(ref mut debug_blob) = debug_blob { **debug_blob = None; }
			let f: extern "system" fn(Param<Self>, VTable, *const u16, *const u16, *const u16, *const PWStr, u32, *const DxcDefine, u32, *const c_void, *mut c_void, Option<&mut MaybeUninit<PWStr>>, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, source.vtable(), source_name.map(str::to_utf16).as_ptr_or_null(), entry_point.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), arguments.to_utf16_vec().ptr(), arguments_len_ as u32, defines_ptr_, defines_len_ as u32, option_to_vtable(include_handler), transmute(&mut result_out_), debug_blob_name, transmute(debug_blob));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcCompiler2: IDxcCompiler {
	fn as_compiler2(&self) -> &DxcCompiler2;
	fn into_compiler2(self) -> DxcCompiler2;
}

impl IDxcCompiler2 for DxcCompiler2 {
	fn as_compiler2(&self) -> &DxcCompiler2 { self }
	fn into_compiler2(self) -> DxcCompiler2 { self }
}
impl IDxcCompiler for DxcCompiler2 {
	fn as_compiler(&self) -> &DxcCompiler { &self.0.as_compiler() }
	fn into_compiler(self) -> DxcCompiler { self.0.into_compiler() }
}

impl IUnknown for DxcCompiler2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcCompiler2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxcCompiler::from(v))
    }
}

