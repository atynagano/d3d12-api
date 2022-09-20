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
pub struct DxcOptimizer(pub(crate) Unknown);

impl Deref for DxcOptimizer {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcOptimizer {
	const IID: &'static GUID = &GUID::from_u128(0x25740e2e9cba401b91194fb42f39f270u128);
}

impl Com for DxcOptimizer {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcOptimizer {
	pub fn GetAvailablePassCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetAvailablePass(&self, index: u32) -> Result<DxcOptimizerPass, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcOptimizerPass> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, index, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn RunOptimizer(&self, blob: &DxcBlob, options: &[&str], mut output_text: Option<&mut Option<DxcBlobEncoding>>) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let (options_ptr_, options_len_) = options.deconstruct();
			let mut output_module_out_: Option<DxcBlob> = None;
			if let Some(ref mut output_text) = output_text { **output_text = None; }
			let f: extern "system" fn(Param<Self>, VTable, *const PWStr, u32, *mut c_void, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, blob.vtable(), options.to_utf16_vec().ptr(), options_len_ as u32, transmute(&mut output_module_out_), transmute(output_text));
			if _ret_.is_ok() { if let Some(output_module_out_) = output_module_out_ { return Ok(output_module_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn run_optimizer(&self) { todo!() }
}

pub trait IDxcOptimizer: IUnknown {
	fn as_optimizer(&self) -> &DxcOptimizer;
	fn into_optimizer(self) -> DxcOptimizer;
}

impl IDxcOptimizer for DxcOptimizer {
	fn as_optimizer(&self) -> &DxcOptimizer { self }
	fn into_optimizer(self) -> DxcOptimizer { self }
}
impl IUnknown for DxcOptimizer {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcOptimizer {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

