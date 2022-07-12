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
pub struct DxcCompilerArgs(pub(crate) Unknown);

impl Guid for DxcCompilerArgs {
	const IID: &'static GUID = &GUID::from_u128(0x73effe2a70dc45f89690eff64c02429du128);
}

impl Clone for DxcCompilerArgs {
	fn clone(&self) -> Self { DxcCompilerArgs(self.0.clone()) }
}

pub trait IDxcCompilerArgs: IUnknown {
	fn as_compiler_args(&self) -> &DxcCompilerArgs;
	fn into_compiler_args(self) -> DxcCompilerArgs;

	fn GetArguments(&self, ) -> *const () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> *const c_void
				= transmute(vt[3]);
			let _ret_ = f(vt, );
			_ret_ as _
		}
	}

	fn GetCount(&self, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u32
				= transmute(vt[4]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn AddArguments(&self, arguments: Option<&[&str]>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_arguments, _len_arguments) = arguments.deconstruct();
			let f: extern "system" fn(Param<Self>, arguments: *const PWStr, arg_count: u32, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, arguments.to_utf16_vec().ptr(), _len_arguments as u32, );
			_ret_.ok()
		}
	}

	fn AddArgumentsUTF8(&self, arguments: Option<&[&str]>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_arguments, _len_arguments) = arguments.deconstruct();
			let f: extern "system" fn(Param<Self>, arguments: *const PStr, arg_count: u32, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, arguments.to_null_terminated_vec().ptr(), _len_arguments as u32, );
			_ret_.ok()
		}
	}

	fn AddDefines(&self, defines: &[DxcDefine], ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_defines, _len_defines) = defines.deconstruct();
			let f: extern "system" fn(Param<Self>, defines: *const DxcDefine, define_count: u32, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, _ptr_defines, _len_defines as u32, );
			_ret_.ok()
		}
	}
}

impl IDxcCompilerArgs for DxcCompilerArgs {
	fn as_compiler_args(&self) -> &DxcCompilerArgs { self }
	fn into_compiler_args(self) -> DxcCompilerArgs { self }
}

impl From<Unknown> for DxcCompilerArgs {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcCompilerArgs {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

