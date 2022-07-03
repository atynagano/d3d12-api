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

	fn GetOptionName(&self, ) -> Result<(PWStr), HResult> {
		let vt = self.as_param();
		let mut _result: PWStr = PWStr::zeroed();
		let f: extern "system" fn(Param<Self>, _result: &mut PWStr, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetDescription(&self, ) -> Result<(PWStr), HResult> {
		let vt = self.as_param();
		let mut _result: PWStr = PWStr::zeroed();
		let f: extern "system" fn(Param<Self>, _result: &mut PWStr, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetOptionArgCount(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _count: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _count: &mut u32, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, &mut _count, );
		if ret.is_ok() {
			return Ok((_count));
		}
		Err(ret)
	}

	fn GetOptionArgName(&self, arg_index: u32, ) -> Result<(PWStr), HResult> {
		let vt = self.as_param();
		let mut _result: PWStr = PWStr::zeroed();
		let f: extern "system" fn(Param<Self>, arg_index: u32, _result: &mut PWStr, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, arg_index, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetOptionArgDescription(&self, arg_index: u32, ) -> Result<(PWStr), HResult> {
		let vt = self.as_param();
		let mut _result: PWStr = PWStr::zeroed();
		let f: extern "system" fn(Param<Self>, arg_index: u32, _result: &mut PWStr, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, arg_index, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
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
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

