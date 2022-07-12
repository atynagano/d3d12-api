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
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::system::com::*;

#[repr(C)]
pub struct DxgiOutput(pub(crate) DxgiObject);

impl Guid for DxgiOutput {
	const IID: &'static GUID = &GUID::from_u128(0xae02eedbc73546908d525a8dc20213aau128);
}

impl Clone for DxgiOutput {
	fn clone(&self) -> Self { DxgiOutput(self.0.clone()) }
}

pub trait IDxgiOutput: IDxgiObject {
	fn as_output(&self) -> &DxgiOutput;
	fn into_output(self) -> DxgiOutput;

	fn GetDesc(&self, ) -> Result<DxgiOutputDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiOutputDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiOutputDesc, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			Ok(_out_desc.assume_init())
		}
	}

	fn FindClosestMatchingMode(&self, mode_to_match: &DxgiModeDesc, concerned_device: Option<&Unknown>, ) -> Result<DxgiModeDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_closest_match: MaybeUninit<DxgiModeDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, mode_to_match: &DxgiModeDesc, _out_closest_match: *mut DxgiModeDesc, concerned_device: *const c_void, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, mode_to_match, _out_closest_match.as_mut_ptr(), option_to_vtable(concerned_device), );
			Ok(_out_closest_match.assume_init())
		}
	}

	fn WaitForVBlank(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}

	fn TakeOwnership(&self, device: &(impl IUnknown + ?Sized), exclusive: bool, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, device: VTable, exclusive: Bool, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, device.vtable(), exclusive.to_bool(), );
			_ret_.ok()
		}
	}

	fn ReleaseOwnership(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[12]);
			let _ret_ = f(vt, );
		}
	}

	fn GetGammaControlCapabilities(&self, ) -> Result<DxgiGammaControlCapabilities, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_gamma_caps: MaybeUninit<DxgiGammaControlCapabilities> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_gamma_caps: *mut DxgiGammaControlCapabilities, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, _out_gamma_caps.as_mut_ptr(), );
			Ok(_out_gamma_caps.assume_init())
		}
	}

	fn SetGammaControl(&self, array: &DxgiGammaControl, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, array: &DxgiGammaControl, ) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, array, );
			_ret_.ok()
		}
	}

	fn GetGammaControl(&self, ) -> Result<DxgiGammaControl, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_array: MaybeUninit<DxgiGammaControl> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_array: *mut DxgiGammaControl, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, _out_array.as_mut_ptr(), );
			Ok(_out_array.assume_init())
		}
	}

	fn SetDisplaySurface(&self, scanout_surface: &(impl IDxgiSurface + ?Sized), ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, scanout_surface: VTable, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, scanout_surface.vtable(), );
			_ret_.ok()
		}
	}

	fn GetDisplaySurfaceData(&self, destination: &(impl IDxgiSurface + ?Sized), ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, destination: VTable, ) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, destination.vtable(), );
			_ret_.ok()
		}
	}

	fn GetFrameStatistics(&self, ) -> Result<DxgiFrameStatistics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_stats: MaybeUninit<DxgiFrameStatistics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_stats: *mut DxgiFrameStatistics, ) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, _out_stats.as_mut_ptr(), );
			Ok(_out_stats.assume_init())
		}
	}
}

impl IDxgiOutput for DxgiOutput {
	fn as_output(&self) -> &DxgiOutput { self }
	fn into_output(self) -> DxgiOutput { self }
}

impl IDxgiObject for DxgiOutput {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiOutput {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiOutput {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

