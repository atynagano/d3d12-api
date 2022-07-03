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

	fn GetDesc(&self, ) -> Result<(DxgiOutputDesc), HResult> {
		let vt = self.as_param();
		let mut _desc: DxgiOutputDesc = DxgiOutputDesc::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiOutputDesc, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, &mut _desc, );
		if ret.is_ok() {
			return Ok((_desc));
		}
		Err(ret)
	}

	fn FindClosestMatchingMode(&self, mode_to_match: &DxgiModeDesc, concerned_device: Option<&Unknown>, ) -> Result<(DxgiModeDesc), HResult> {
		let vt = self.as_param();
		let mut _closest_match: DxgiModeDesc = DxgiModeDesc::zeroed();
		let f: extern "system" fn(Param<Self>, mode_to_match: &DxgiModeDesc, _closest_match: &mut DxgiModeDesc, concerned_device: Option<VTable>, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, mode_to_match, &mut _closest_match, option_to_vtable(concerned_device), );
		if ret.is_ok() {
			return Ok((_closest_match));
		}
		Err(ret)
	}

	fn WaitForVBlank(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn TakeOwnership(&self, device: &(impl IUnknown + ?Sized), exclusive: bool, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, device: VTable, exclusive: Bool, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, device.vtable(), exclusive.to_bool(), );
		ret.ok()
	}

	fn ReleaseOwnership(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, );
	}

	fn GetGammaControlCapabilities(&self, ) -> Result<(DxgiGammaControlCapabilities), HResult> {
		let vt = self.as_param();
		let mut _gamma_caps: DxgiGammaControlCapabilities = DxgiGammaControlCapabilities::zeroed();
		let f: extern "system" fn(Param<Self>, _gamma_caps: &mut DxgiGammaControlCapabilities, ) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, &mut _gamma_caps, );
		if ret.is_ok() {
			return Ok((_gamma_caps));
		}
		Err(ret)
	}

	fn SetGammaControl(&self, array: &DxgiGammaControl, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, array: &DxgiGammaControl, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, array, );
		ret.ok()
	}

	fn GetGammaControl(&self, ) -> Result<(DxgiGammaControl), HResult> {
		let vt = self.as_param();
		let mut _array: DxgiGammaControl = DxgiGammaControl::zeroed();
		let f: extern "system" fn(Param<Self>, _array: &mut DxgiGammaControl, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, &mut _array, );
		if ret.is_ok() {
			return Ok((_array));
		}
		Err(ret)
	}

	fn SetDisplaySurface(&self, scanout_surface: &(impl IDxgiSurface + ?Sized), ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, scanout_surface: VTable, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, scanout_surface.vtable(), );
		ret.ok()
	}

	fn GetDisplaySurfaceData(&self, destination: &(impl IDxgiSurface + ?Sized), ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, destination: VTable, ) -> HResult
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, destination.vtable(), );
		ret.ok()
	}

	fn GetFrameStatistics(&self, ) -> Result<(DxgiFrameStatistics), HResult> {
		let vt = self.as_param();
		let mut _stats: DxgiFrameStatistics = DxgiFrameStatistics::zeroed();
		let f: extern "system" fn(Param<Self>, _stats: &mut DxgiFrameStatistics, ) -> HResult
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, &mut _stats, );
		if ret.is_ok() {
			return Ok((_stats));
		}
		Err(ret)
	}
}

impl IDxgiOutput for DxgiOutput {
	fn as_output(&self) -> &DxgiOutput { self }
	fn into_output(self) -> DxgiOutput { self }
}

impl IDxgiObject for DxgiOutput {
	fn as_object(&self) -> &DxgiObject { &self.0 }
	fn into_object(self) -> DxgiObject { self.0 }
}

impl From<Unknown> for DxgiOutput {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiOutput {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

