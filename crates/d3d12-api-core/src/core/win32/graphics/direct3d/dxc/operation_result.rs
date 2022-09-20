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
pub struct DxcOperationResult(pub(crate) Unknown);

impl Deref for DxcOperationResult {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcOperationResult {
	const IID: &'static GUID = &GUID::from_u128(0xcedb484ad4e9445ab991ca21ca157dc2u128);
}

impl Com for DxcOperationResult {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcOperationResult {
	pub fn GetStatus(&self) -> Result<HResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut status_out_: MaybeUninit<HResult> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut HResult) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, status_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(status_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetResult(&self, mut result: Option<&mut Option<DxcBlob>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut result) = result { **result = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(result));
			_ret_.ok()
		}
	}

	pub fn get_result(&self) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetErrorBuffer(&self, mut errors: Option<&mut Option<DxcBlobEncoding>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut errors) = errors { **errors = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, transmute(errors));
			_ret_.ok()
		}
	}

	pub fn get_error_buffer(&self) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut errors_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, transmute(&mut errors_out_));
			if _ret_.is_ok() { if let Some(errors_out_) = errors_out_ { return Ok(errors_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcOperationResult: IUnknown {
	fn as_operation_result(&self) -> &DxcOperationResult;
	fn into_operation_result(self) -> DxcOperationResult;
}

impl IDxcOperationResult for DxcOperationResult {
	fn as_operation_result(&self) -> &DxcOperationResult { self }
	fn into_operation_result(self) -> DxcOperationResult { self }
}
impl IUnknown for DxcOperationResult {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcOperationResult {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

