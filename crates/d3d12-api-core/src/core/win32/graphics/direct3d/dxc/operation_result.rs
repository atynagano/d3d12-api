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
pub struct DxcOperationResult(pub(crate) Unknown);

impl Guid for DxcOperationResult {
	const IID: &'static GUID = &GUID::from_u128(0xcedb484ad4e9445ab991ca21ca157dc2u128);
}

impl Clone for DxcOperationResult {
	fn clone(&self) -> Self { DxcOperationResult(self.0.clone()) }
}

pub trait IDxcOperationResult: IUnknown {
	fn as_operation_result(&self) -> &DxcOperationResult;
	fn into_operation_result(self) -> DxcOperationResult;

	fn GetStatus(&self, ) -> Result<HResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_status: MaybeUninit<HResult> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_status: *mut HResult, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, _out_status.as_mut_ptr(), );
			Ok(_out_status.assume_init())
		}
	}

	fn GetResult(&self, mut result: Option<&mut Option<DxcBlob>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut result) = result { **result = None; }
			let f: extern "system" fn(Param<Self>, result: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(result), );
			_ret_.ok()
		}
	}

	fn get_result(&self, ) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, result: *mut c_void, ) -> HResult
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

	fn GetErrorBuffer(&self, mut errors: Option<&mut Option<DxcBlobEncoding>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut errors) = errors { **errors = None; }
			let f: extern "system" fn(Param<Self>, errors: *mut c_void, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, transmute(errors), );
			_ret_.ok()
		}
	}

	fn get_error_buffer(&self, ) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_errors: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, errors: *mut c_void, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, transmute(&mut _out_errors), );
			if _ret_.is_ok() {
				if let Some(_out_errors) = _out_errors {
					return Ok(_out_errors);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcOperationResult for DxcOperationResult {
	fn as_operation_result(&self) -> &DxcOperationResult { self }
	fn into_operation_result(self) -> DxcOperationResult { self }
}

impl From<Unknown> for DxcOperationResult {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcOperationResult {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

