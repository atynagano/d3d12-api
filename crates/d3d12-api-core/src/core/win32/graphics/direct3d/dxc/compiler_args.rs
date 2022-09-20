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
pub struct DxcCompilerArgs(pub(crate) Unknown);

impl Deref for DxcCompilerArgs {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcCompilerArgs {
	const IID: &'static GUID = &GUID::from_u128(0x73effe2a70dc45f89690eff64c02429du128);
}

impl Com for DxcCompilerArgs {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcCompilerArgs {
	pub fn GetArguments(&self) -> *const *const u8 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> *const *const u8
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn AddArguments(&self, arguments: Option<&[&str]>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (arguments_ptr_, arguments_len_) = arguments.deconstruct();
			let f: extern "system" fn(Param<Self>, *const PWStr, u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, arguments.to_utf16_vec().ptr(), arguments_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn AddArgumentsUTF8(&self, arguments: Option<&[&str]>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (arguments_ptr_, arguments_len_) = arguments.deconstruct();
			let f: extern "system" fn(Param<Self>, *const PStr, u32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, arguments.to_null_terminated_vec().ptr(), arguments_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn AddDefines(&self, defines: &[DxcDefine]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (defines_ptr_, defines_len_) = defines.deconstruct();
			let f: extern "system" fn(Param<Self>, *const DxcDefine, u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, defines_ptr_, defines_len_ as u32);
			_ret_.ok()
		}
	}
}

pub trait IDxcCompilerArgs: IUnknown {
	fn as_compiler_args(&self) -> &DxcCompilerArgs;
	fn into_compiler_args(self) -> DxcCompilerArgs;
}

impl IDxcCompilerArgs for DxcCompilerArgs {
	fn as_compiler_args(&self) -> &DxcCompilerArgs { self }
	fn into_compiler_args(self) -> DxcCompilerArgs { self }
}
impl IUnknown for DxcCompilerArgs {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcCompilerArgs {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

