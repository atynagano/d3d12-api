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
pub struct DxcResult(pub(crate) DxcOperationResult);

impl Deref for DxcResult {
	type Target = DxcOperationResult;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcResult {
	const IID: &'static GUID = &GUID::from_u128(0x58346cdadde7449794616f87af5e0659u128);
}

impl Com for DxcResult {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcResult {
	pub fn HasOutput(&self, dxc_out_kind: DxcOutKind) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DxcOutKind) -> Bool
				= transmute(vt[6]);
			let _ret_ = f(vt, dxc_out_kind);
			_ret_.to_bool()
		}
	}

	pub fn GetOutput<T: IUnknown + From<UnknownWrapper>>(&self, dxc_out_kind: DxcOutKind, object: Option<&mut Option<T>>) -> Result<DxcBlobUtf16, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut object_out_: Option<Unknown> = None;
			let mut output_name_out_: Option<DxcBlobUtf16> = None;
			let f: extern "system" fn(Param<Self>, DxcOutKind, &GUID, *mut c_void, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, dxc_out_kind, T::IID, transmute(if object_out_.is_some() { Some(&mut object_out_) } else { None }), transmute(&mut output_name_out_));
			if let Some(object_out_) = object_out_ { *object.unwrap_unchecked() = Some(T::from(UnknownWrapper(object_out_))); }
			if _ret_.is_ok() { if let Some(output_name_out_) = output_name_out_ { return Ok(output_name_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn get_output(&self) { todo!() }

	pub fn GetNumOutputs(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[8]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetOutputByIndex(&self, index: u32) -> DxcOutKind {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> DxcOutKind
				= transmute(vt[9]);
			let _ret_ = f(vt, index);
			_ret_
		}
	}

	pub fn PrimaryOutput(&self) -> DxcOutKind {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DxcOutKind
				= transmute(vt[10]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait IDxcResult: IDxcOperationResult {
	fn as_result(&self) -> &DxcResult;
	fn into_result(self) -> DxcResult;
}

impl IDxcResult for DxcResult {
	fn as_result(&self) -> &DxcResult { self }
	fn into_result(self) -> DxcResult { self }
}
impl IDxcOperationResult for DxcResult {
	fn as_operation_result(&self) -> &DxcOperationResult { &self.0.as_operation_result() }
	fn into_operation_result(self) -> DxcOperationResult { self.0.into_operation_result() }
}

impl IUnknown for DxcResult {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcResult {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxcOperationResult::from(v))
    }
}

