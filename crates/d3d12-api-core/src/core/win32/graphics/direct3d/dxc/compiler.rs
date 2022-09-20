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
pub struct DxcCompiler(pub(crate) Unknown);

impl Deref for DxcCompiler {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcCompiler {
	const IID: &'static GUID = &GUID::from_u128(0x8c210bf3011f44228d706f9acb8db617u128);
}

impl Com for DxcCompiler {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcCompiler {
	pub fn Compile(&self, source: &DxcBlob, source_name: Option<&str>, entry_point: Option<&str>, target_profile: &str, arguments: Option<&[&str]>, defines: &[DxcDefine], include_handler: Option<&DxcIncludeHandler>) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let (arguments_ptr_, arguments_len_) = arguments.deconstruct();
			let (defines_ptr_, defines_len_) = defines.deconstruct();
			let mut result_out_: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, VTable, *const u16, *const u16, *const u16, *const PWStr, u32, *const DxcDefine, u32, *const c_void, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, source.vtable(), source_name.map(str::to_utf16).as_ptr_or_null(), entry_point.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), arguments.to_utf16_vec().ptr(), arguments_len_ as u32, defines_ptr_, defines_len_ as u32, option_to_vtable(include_handler), transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn Preprocess(&self, source: &DxcBlob, source_name: Option<&str>, arguments: Option<&[&str]>, defines: &[DxcDefine], include_handler: Option<&DxcIncludeHandler>) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let (arguments_ptr_, arguments_len_) = arguments.deconstruct();
			let (defines_ptr_, defines_len_) = defines.deconstruct();
			let mut result_out_: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, VTable, *const u16, *const PWStr, u32, *const DxcDefine, u32, *const c_void, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, source.vtable(), source_name.map(str::to_utf16).as_ptr_or_null(), arguments.to_utf16_vec().ptr(), arguments_len_ as u32, defines_ptr_, defines_len_ as u32, option_to_vtable(include_handler), transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn Disassemble(&self, source: &DxcBlob) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut disassembly_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, source.vtable(), transmute(&mut disassembly_out_));
			if _ret_.is_ok() { if let Some(disassembly_out_) = disassembly_out_ { return Ok(disassembly_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcCompiler: IUnknown {
	fn as_compiler(&self) -> &DxcCompiler;
	fn into_compiler(self) -> DxcCompiler;
}

impl IDxcCompiler for DxcCompiler {
	fn as_compiler(&self) -> &DxcCompiler { self }
	fn into_compiler(self) -> DxcCompiler { self }
}
impl IUnknown for DxcCompiler {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcCompiler {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

