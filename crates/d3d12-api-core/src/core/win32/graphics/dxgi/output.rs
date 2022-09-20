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
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiOutput(pub(crate) DxgiObject);

impl Deref for DxgiOutput {
	type Target = DxgiObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiOutput {
	const IID: &'static GUID = &GUID::from_u128(0xae02eedbc73546908d525a8dc20213aau128);
}

impl Com for DxgiOutput {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiOutput {
	pub fn GetDesc(&self) -> Result<DxgiOutputDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiOutputDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiOutputDesc) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn FindClosestMatchingMode(&self, mode_to_match: &DxgiModeDesc, concerned_device: Option<&Unknown>) -> Result<DxgiModeDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut closest_match_out_: MaybeUninit<DxgiModeDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, &DxgiModeDesc, *mut DxgiModeDesc, *const c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, mode_to_match, closest_match_out_.as_mut_ptr(), option_to_vtable(concerned_device));
			if _ret_.is_ok() { Ok(closest_match_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn WaitForVBlank(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn TakeOwnership(&self, device: &Unknown, exclusive: bool) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, Bool) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, device.vtable(), exclusive.to_bool());
			_ret_.ok()
		}
	}

	pub fn ReleaseOwnership(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[12]);
			let _ret_ = f(vt);
		}
	}

	pub fn GetGammaControlCapabilities(&self) -> Result<DxgiGammaControlCapabilities, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut gamma_caps_out_: MaybeUninit<DxgiGammaControlCapabilities> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiGammaControlCapabilities) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, gamma_caps_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(gamma_caps_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn SetGammaControl(&self, array: &DxgiGammaControl) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &DxgiGammaControl) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, array);
			_ret_.ok()
		}
	}

	pub fn GetGammaControl(&self) -> Result<DxgiGammaControl, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut array_out_: MaybeUninit<DxgiGammaControl> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiGammaControl) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, array_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(array_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn SetDisplaySurface(&self, scanout_surface: &DxgiSurface) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, scanout_surface.vtable());
			_ret_.ok()
		}
	}

	pub fn GetDisplaySurfaceData(&self, destination: &DxgiSurface) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, destination.vtable());
			_ret_.ok()
		}
	}

	pub fn GetFrameStatistics(&self) -> Result<DxgiFrameStatistics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut stats_out_: MaybeUninit<DxgiFrameStatistics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiFrameStatistics) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, stats_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(stats_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiOutput: IDxgiObject {
	fn as_output(&self) -> &DxgiOutput;
	fn into_output(self) -> DxgiOutput;
}

impl IDxgiOutput for DxgiOutput {
	fn as_output(&self) -> &DxgiOutput { self }
	fn into_output(self) -> DxgiOutput { self }
}
impl IDxgiObject for DxgiOutput {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiOutput {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiOutput {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiObject::from(v))
    }
}

