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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiOutput1(pub(crate) DxgiOutput);

impl Deref for DxgiOutput1 {
	type Target = DxgiOutput;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiOutput1 {
	const IID: &'static GUID = &GUID::from_u128(0x00cddea8939b4b83a340a685226666ccu128);
}

impl Com for DxgiOutput1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiOutput1 {
	pub fn FindClosestMatchingMode1(&self, mode_to_match: &DxgiModeDesc1, concerned_device: Option<&Unknown>) -> Result<DxgiModeDesc1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut closest_match_out_: MaybeUninit<DxgiModeDesc1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, &DxgiModeDesc1, *mut DxgiModeDesc1, *const c_void) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, mode_to_match, closest_match_out_.as_mut_ptr(), option_to_vtable(concerned_device));
			if _ret_.is_ok() { Ok(closest_match_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetDisplaySurfaceData1(&self, destination: &DxgiResource) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, destination.vtable());
			_ret_.ok()
		}
	}

	pub fn DuplicateOutput(&self, device: &Unknown) -> Result<DxgiOutputDuplication, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut output_duplication_out_: Option<DxgiOutputDuplication> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, device.vtable(), transmute(&mut output_duplication_out_));
			if _ret_.is_ok() { if let Some(output_duplication_out_) = output_duplication_out_ { return Ok(output_duplication_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiOutput1: IDxgiOutput {
	fn as_output1(&self) -> &DxgiOutput1;
	fn into_output1(self) -> DxgiOutput1;
}

impl IDxgiOutput1 for DxgiOutput1 {
	fn as_output1(&self) -> &DxgiOutput1 { self }
	fn into_output1(self) -> DxgiOutput1 { self }
}
impl IDxgiOutput for DxgiOutput1 {
	fn as_output(&self) -> &DxgiOutput { &self.0.as_output() }
	fn into_output(self) -> DxgiOutput { self.0.into_output() }
}

impl IDxgiObject for DxgiOutput1 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiOutput1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiOutput1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiOutput::from(v))
    }
}

