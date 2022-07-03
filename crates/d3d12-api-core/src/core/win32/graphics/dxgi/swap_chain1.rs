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
#[repr(C)]
pub struct DxgiSwapChain1(pub(crate) DxgiSwapChain);

impl Guid for DxgiSwapChain1 {
	const IID: &'static GUID = &GUID::from_u128(0x790a45f70d424876983a0a55cfe6f4aau128);
}

impl Clone for DxgiSwapChain1 {
	fn clone(&self) -> Self { DxgiSwapChain1(self.0.clone()) }
}

pub trait IDxgiSwapChain1: IDxgiSwapChain {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1;
	fn into_swap_chain1(self) -> DxgiSwapChain1;

	fn GetDesc1(&self, ) -> Result<(DxgiSwapChainDesc1), HResult> {
		let vt = self.as_param();
		let mut _desc: DxgiSwapChainDesc1 = DxgiSwapChainDesc1::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiSwapChainDesc1, ) -> HResult
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, &mut _desc, );
		if ret.is_ok() {
			return Ok((_desc));
		}
		Err(ret)
	}

	fn GetFullscreenDesc(&self, ) -> Result<(DxgiSwapChainFullScreenDesc), HResult> {
		let vt = self.as_param();
		let mut _desc: DxgiSwapChainFullScreenDesc = DxgiSwapChainFullScreenDesc::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiSwapChainFullScreenDesc, ) -> HResult
			= unsafe { transmute(vt[19]) };
		let ret = f(vt, &mut _desc, );
		if ret.is_ok() {
			return Ok((_desc));
		}
		Err(ret)
	}

	fn GetHwnd(&self, ) -> Result<(HWnd), HResult> {
		let vt = self.as_param();
		let mut _hwnd: HWnd = HWnd::zeroed();
		let f: extern "system" fn(Param<Self>, _hwnd: &mut HWnd, ) -> HResult
			= unsafe { transmute(vt[20]) };
		let ret = f(vt, &mut _hwnd, );
		if ret.is_ok() {
			return Ok((_hwnd));
		}
		Err(ret)
	}

	fn GetCoreWindow<T: IUnknown>(&self, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _unk: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, refiid: &GUID, _unk: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[21]) };
		let ret = f(vt, T::IID, &mut _unk, );
		if ret.is_ok() {
			if let (Some(_unk)) = (_unk) {
				return Ok((T::from(_unk)));
			}
		}
		Err(ret)
	}

	fn Present1(&self, sync_interval: u32, present_flags: u32, present_parameters: &DxgiPresentParameters, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, sync_interval: u32, present_flags: u32, present_parameters: &DxgiPresentParameters, ) -> HResult
			= unsafe { transmute(vt[22]) };
		let ret = f(vt, sync_interval, present_flags, present_parameters, );
		ret.ok()
	}

	fn IsTemporaryMonoSupported(&self, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Bool
			= unsafe { transmute(vt[23]) };
		let ret = f(vt, );
		return (ret.to_bool());
	}

	fn GetRestrictToOutput(&self, ) -> Result<(DxgiOutput), HResult> {
		let vt = self.as_param();
		let mut _restrict_to_output: Option<DxgiOutput> = None;
		let f: extern "system" fn(Param<Self>, _restrict_to_output: &mut Option<DxgiOutput>, ) -> HResult
			= unsafe { transmute(vt[24]) };
		let ret = f(vt, &mut _restrict_to_output, );
		if ret.is_ok() {
			if let (Some(_restrict_to_output)) = (_restrict_to_output) {
				return Ok((_restrict_to_output));
			}
		}
		Err(ret)
	}

	fn SetBackgroundColor(&self, color: &DxgiRgba, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, color: &DxgiRgba, ) -> HResult
			= unsafe { transmute(vt[25]) };
		let ret = f(vt, color, );
		ret.ok()
	}

	fn GetBackgroundColor(&self, ) -> Result<(DxgiRgba), HResult> {
		let vt = self.as_param();
		let mut _color: DxgiRgba = DxgiRgba::zeroed();
		let f: extern "system" fn(Param<Self>, _color: &mut DxgiRgba, ) -> HResult
			= unsafe { transmute(vt[26]) };
		let ret = f(vt, &mut _color, );
		if ret.is_ok() {
			return Ok((_color));
		}
		Err(ret)
	}

	fn SetRotation(&self, rotation: DxgiModeRotation, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, rotation: DxgiModeRotation, ) -> HResult
			= unsafe { transmute(vt[27]) };
		let ret = f(vt, rotation, );
		ret.ok()
	}

	fn GetRotation(&self, ) -> Result<(DxgiModeRotation), HResult> {
		let vt = self.as_param();
		let mut _rotation: DxgiModeRotation = DxgiModeRotation::zeroed();
		let f: extern "system" fn(Param<Self>, _rotation: &mut DxgiModeRotation, ) -> HResult
			= unsafe { transmute(vt[28]) };
		let ret = f(vt, &mut _rotation, );
		if ret.is_ok() {
			return Ok((_rotation));
		}
		Err(ret)
	}
}

impl IDxgiSwapChain1 for DxgiSwapChain1 {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1 { self }
	fn into_swap_chain1(self) -> DxgiSwapChain1 { self }
}

impl IDxgiSwapChain for DxgiSwapChain1 {
	fn as_swap_chain(&self) -> &DxgiSwapChain { &self.0 }
	fn into_swap_chain(self) -> DxgiSwapChain { self.0 }
}

impl IDxgiDeviceSubObject for DxgiSwapChain1 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.0 }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.0 }
}

impl IDxgiObject for DxgiSwapChain1 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0 }
}

impl From<Unknown> for DxgiSwapChain1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSwapChain::from(v))
    }
}

impl IUnknown for DxgiSwapChain1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

