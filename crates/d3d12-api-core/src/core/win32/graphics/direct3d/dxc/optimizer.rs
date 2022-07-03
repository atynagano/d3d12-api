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
pub struct DxcOptimizer(pub(crate) Unknown);

impl Guid for DxcOptimizer {
	const IID: &'static GUID = &GUID::from_u128(0x25740e2e9cba401b91194fb42f39f270u128);
}

impl Clone for DxcOptimizer {
	fn clone(&self) -> Self { DxcOptimizer(self.0.clone()) }
}

pub trait IDxcOptimizer: IUnknown {
	fn as_optimizer(&self) -> &DxcOptimizer;
	fn into_optimizer(self) -> DxcOptimizer;

	fn GetAvailablePassCount(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _count: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _count: &mut u32, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, &mut _count, );
		if ret.is_ok() {
			return Ok((_count));
		}
		Err(ret)
	}

	fn GetAvailablePass(&self, index: u32, ) -> Result<(DxcOptimizerPass), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcOptimizerPass> = None;
		let f: extern "system" fn(Param<Self>, index: u32, _result: &mut Option<DxcOptimizerPass>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, index, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn RunOptimizer(&self, blob: &(impl IDxcBlob + ?Sized), options: &[&str], mut output_text: Option<&mut Option<DxcBlobEncoding>>,) -> Result<(DxcBlob), HResult> {
		let vt = self.as_param();
		let mut _output_module: Option<DxcBlob> = None;
		if let Some(ref mut output_text) = output_text { **output_text = None; }
		let f: extern "system" fn(Param<Self>, blob: VTable, options: *const PWStr, option_count: u32, _output_module: &mut Option<DxcBlob>, output_text: Option<&mut Option<DxcBlobEncoding>>, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, blob.vtable(), options.to_utf16_vec().ptr(), options.len() as u32, &mut _output_module, output_text, );
		if ret.is_ok() {
			if let (Some(_output_module)) = (_output_module) {
				return Ok((_output_module));
			}
		}
		Err(ret)
	}
}

impl IDxcOptimizer for DxcOptimizer {
	fn as_optimizer(&self) -> &DxcOptimizer { self }
	fn into_optimizer(self) -> DxcOptimizer { self }
}

impl From<Unknown> for DxcOptimizer {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcOptimizer {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

