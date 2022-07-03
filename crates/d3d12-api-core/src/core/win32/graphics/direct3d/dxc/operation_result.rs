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

	fn GetStatus(&self, ) -> Result<(HResult), HResult> {
		let vt = self.as_param();
		let mut _status: HResult = HResult::zeroed();
		let f: extern "system" fn(Param<Self>, _status: &mut HResult, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, &mut _status, );
		if ret.is_ok() {
			return Ok((_status));
		}
		Err(ret)
	}

	fn GetResult(&self, mut result: Option<&mut Option<DxcBlob>>,) -> Result<(), HResult> {
		let vt = self.as_param();
		if let Some(ref mut result) = result { **result = None; }
		let f: extern "system" fn(Param<Self>, result: Option<&mut Option<DxcBlob>>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, result, );
		ret.ok()
	}

	fn GetErrorBuffer(&self, mut errors: Option<&mut Option<DxcBlobEncoding>>,) -> Result<(), HResult> {
		let vt = self.as_param();
		if let Some(ref mut errors) = errors { **errors = None; }
		let f: extern "system" fn(Param<Self>, errors: Option<&mut Option<DxcBlobEncoding>>, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, errors, );
		ret.ok()
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
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

