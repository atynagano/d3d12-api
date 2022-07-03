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
pub struct DxcResult(pub(crate) DxcOperationResult);

impl Guid for DxcResult {
	const IID: &'static GUID = &GUID::from_u128(0x58346cdadde7449794616f87af5e0659u128);
}

impl Clone for DxcResult {
	fn clone(&self) -> Self { DxcResult(self.0.clone()) }
}

pub trait IDxcResult: IDxcOperationResult {
	fn as_result(&self) -> &DxcResult;
	fn into_result(self) -> DxcResult;

	fn HasOutput(&self, dxc_out_kind: DxcOutKind, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dxc_out_kind: DxcOutKind, ) -> Bool
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, dxc_out_kind, );
		return (ret.to_bool());
	}

	fn GetOutput<T: IUnknown>(&self, dxc_out_kind: DxcOutKind, object: Option<&mut Option<T>>, ) -> Result<(DxcBlobUtf16), HResult> {
		let vt = self.as_param();
		let mut out_object: Option<Unknown> = None;
		let mut _output_name: Option<DxcBlobUtf16> = None;
		let f: extern "system" fn(Param<Self>, dxc_out_kind: DxcOutKind, iid: &GUID, object: Option<&mut Option<Unknown>>, _output_name: &mut Option<DxcBlobUtf16>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, dxc_out_kind, T::IID, if object.is_some() { Some(&mut out_object) } else { None }, &mut _output_name, );
		if let (Some(object), Some(out_object)) = (object, out_object) { *object = Some(T::from(out_object)); }
		if ret.is_ok() {
			if let (Some(_output_name)) = (_output_name) {
				return Ok((_output_name));
			}
		}
		Err(ret)
	}

	fn GetNumOutputs(&self, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u32
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetOutputByIndex(&self, index: u32, ) -> (DxcOutKind) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, index: u32, ) -> DxcOutKind
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, index, );
		return (ret);
	}

	fn PrimaryOutput(&self, ) -> (DxcOutKind) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> DxcOutKind
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl IDxcResult for DxcResult {
	fn as_result(&self) -> &DxcResult { self }
	fn into_result(self) -> DxcResult { self }
}

impl IDxcOperationResult for DxcResult {
	fn as_operation_result(&self) -> &DxcOperationResult { &self.0 }
	fn into_operation_result(self) -> DxcOperationResult { self.0 }
}

impl From<Unknown> for DxcResult {
    fn from(v: Unknown) -> Self {
        Self(DxcOperationResult::from(v))
    }
}

impl IUnknown for DxcResult {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

