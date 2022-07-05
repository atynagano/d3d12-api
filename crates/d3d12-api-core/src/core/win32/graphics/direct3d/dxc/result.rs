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

	fn HasOutput(&self, dxc_out_kind: DxcOutKind, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, dxc_out_kind: DxcOutKind, ) -> Bool
				= transmute(vt[6]);
			let _ret_ = f(vt, dxc_out_kind, );
			_ret_.to_bool()
		}
	}

	fn GetOutput<T: IUnknown>(&self, dxc_out_kind: DxcOutKind, object: Option<&mut Option<T>>, ) -> Result<DxcBlobUtf16, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_object: Option<Unknown> = None;
			let mut _out_output_name: Option<DxcBlobUtf16> = None;
			let f: extern "system" fn(Param<Self>, dxc_out_kind: DxcOutKind, iid: &GUID, object: *mut c_void, output_name: *mut c_void, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, dxc_out_kind, T::IID, transmute(if object.is_some() { Some(&mut _out_object) } else { None }), transmute(&mut _out_output_name), );
			if let Some(_out_object) = _out_object { *object.unwrap_unchecked() = Some(T::from(_out_object)); }
			if _ret_.is_ok() {
				if let Some(_out_output_name) = _out_output_name {
					return Ok(_out_output_name);
				}
			}
			Err(_ret_)
		}
	}

	fn GetNumOutputs(&self, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u32
				= transmute(vt[8]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn GetOutputByIndex(&self, index: u32, ) -> DxcOutKind {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, index: u32, ) -> DxcOutKind
				= transmute(vt[9]);
			let _ret_ = f(vt, index, );
			_ret_
		}
	}

	fn PrimaryOutput(&self, ) -> DxcOutKind {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> DxcOutKind
				= transmute(vt[10]);
			let _ret_ = f(vt, );
			_ret_
		}
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

