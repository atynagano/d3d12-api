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

#[repr(C)]
pub struct DxcOptimizerPass(pub(crate) Unknown);

impl Guid for DxcOptimizerPass {
	const IID: &'static GUID = &GUID::from_u128(0xae2cd79fcc22453f9b6bb124e7a5204cu128);
}

impl Clone for DxcOptimizerPass {
	fn clone(&self) -> Self { DxcOptimizerPass(self.0.clone()) }
}

pub trait IDxcOptimizerPass: IUnknown {
	fn as_optimizer_pass(&self) -> &DxcOptimizerPass;
	fn into_optimizer_pass(self) -> DxcOptimizerPass;

	fn GetOptionName(&self, ) -> Result<PWStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<PWStr> = None;
			let f: extern "system" fn(Param<Self>, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetDescription(&self, ) -> Result<PWStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<PWStr> = None;
			let f: extern "system" fn(Param<Self>, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetOptionArgCount(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_count: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_count: *mut u32, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, _out_count.as_mut_ptr(), );
			Ok(_out_count.assume_init())
		}
	}

	fn GetOptionArgName(&self, arg_index: u32, ) -> Result<PWStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<PWStr> = None;
			let f: extern "system" fn(Param<Self>, arg_index: u32, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, arg_index, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetOptionArgDescription(&self, arg_index: u32, ) -> Result<PWStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<PWStr> = None;
			let f: extern "system" fn(Param<Self>, arg_index: u32, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, arg_index, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcOptimizerPass for DxcOptimizerPass {
	fn as_optimizer_pass(&self) -> &DxcOptimizerPass { self }
	fn into_optimizer_pass(self) -> DxcOptimizerPass { self }
}

impl From<Unknown> for DxcOptimizerPass {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcOptimizerPass {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

