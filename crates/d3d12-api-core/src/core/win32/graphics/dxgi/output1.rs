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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::system::com::*;

#[repr(C)]
pub struct DxgiOutput1(pub(crate) DxgiOutput);

impl Guid for DxgiOutput1 {
	const IID: &'static GUID = &GUID::from_u128(0x00cddea8939b4b83a340a685226666ccu128);
}

impl Clone for DxgiOutput1 {
	fn clone(&self) -> Self { DxgiOutput1(self.0.clone()) }
}

pub trait IDxgiOutput1: IDxgiOutput {
	fn as_output1(&self) -> &DxgiOutput1;
	fn into_output1(self) -> DxgiOutput1;

	fn FindClosestMatchingMode1(&self, mode_to_match: &DxgiModeDesc1, concerned_device: Option<&Unknown>, ) -> Result<DxgiModeDesc1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_closest_match: MaybeUninit<DxgiModeDesc1> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, mode_to_match: &DxgiModeDesc1, _out_closest_match: *mut DxgiModeDesc1, concerned_device: *const c_void, ) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, mode_to_match, _out_closest_match.as_mut_ptr(), option_to_vtable(concerned_device), );
			Ok(_out_closest_match.assume_init())
		}
	}

	fn GetDisplaySurfaceData1(&self, destination: &(impl IDxgiResource + ?Sized), ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, destination: VTable, ) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, destination.vtable(), );
			_ret_.ok()
		}
	}

	fn DuplicateOutput(&self, device: &(impl IUnknown + ?Sized), ) -> Result<DxgiOutputDuplication, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_output_duplication: Option<DxgiOutputDuplication> = None;
			let f: extern "system" fn(Param<Self>, device: VTable, output_duplication: *mut c_void, ) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, device.vtable(), transmute(&mut _out_output_duplication), );
			if _ret_.is_ok() {
				if let Some(_out_output_duplication) = _out_output_duplication {
					return Ok(_out_output_duplication);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxgiOutput1 for DxgiOutput1 {
	fn as_output1(&self) -> &DxgiOutput1 { self }
	fn into_output1(self) -> DxgiOutput1 { self }
}

impl IDxgiOutput for DxgiOutput1 {
	fn as_output(&self) -> &DxgiOutput { &self.0 }
	fn into_output(self) -> DxgiOutput { self.0 }
}

impl IDxgiObject for DxgiOutput1 {
	fn as_object(&self) -> &DxgiObject { &self.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0 }
}

impl From<Unknown> for DxgiOutput1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiOutput::from(v))
    }
}

impl IUnknown for DxgiOutput1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

