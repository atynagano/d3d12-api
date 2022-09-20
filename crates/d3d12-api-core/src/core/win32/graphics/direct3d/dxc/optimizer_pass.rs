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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcOptimizerPass(pub(crate) Unknown);

impl Deref for DxcOptimizerPass {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcOptimizerPass {
	const IID: &'static GUID = &GUID::from_u128(0xae2cd79fcc22453f9b6bb124e7a5204cu128);
}

impl Com for DxcOptimizerPass {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcOptimizerPass {
	pub fn GetOptionName(&self) -> Result<PWStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<PWStr> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetDescription(&self) -> Result<PWStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<PWStr> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetOptionArgCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetOptionArgName(&self, arg_index: u32) -> Result<PWStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<PWStr> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, arg_index, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetOptionArgDescription(&self, arg_index: u32) -> Result<PWStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<PWStr> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, arg_index, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcOptimizerPass: IUnknown {
	fn as_optimizer_pass(&self) -> &DxcOptimizerPass;
	fn into_optimizer_pass(self) -> DxcOptimizerPass;
}

impl IDxcOptimizerPass for DxcOptimizerPass {
	fn as_optimizer_pass(&self) -> &DxcOptimizerPass { self }
	fn into_optimizer_pass(self) -> DxcOptimizerPass { self }
}
impl IUnknown for DxcOptimizerPass {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcOptimizerPass {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

