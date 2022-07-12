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

	fn GetAvailablePassCount(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_count: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_count: *mut u32, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, _out_count.as_mut_ptr(), );
			Ok(_out_count.assume_init())
		}
	}

	fn GetAvailablePass(&self, index: u32, ) -> Result<DxcOptimizerPass, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcOptimizerPass> = None;
			let f: extern "system" fn(Param<Self>, index: u32, result: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, index, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn RunOptimizer(&self, blob: &(impl IDxcBlob + ?Sized), options: &[&str], mut output_text: Option<&mut Option<DxcBlobEncoding>>, ) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_options, _len_options) = options.deconstruct();
			let mut _out_output_module: Option<DxcBlob> = None;
			if let Some(ref mut output_text) = output_text { **output_text = None; }
			let f: extern "system" fn(Param<Self>, blob: VTable, options: *const PWStr, option_count: u32, output_module: *mut c_void, output_text: *mut c_void, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, blob.vtable(), options.to_utf16_vec().ptr(), _len_options as u32, transmute(&mut _out_output_module), transmute(output_text), );
			if _ret_.is_ok() {
				if let Some(_out_output_module) = _out_output_module {
					return Ok(_out_output_module);
				}
			}
			Err(_ret_)
		}
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
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

