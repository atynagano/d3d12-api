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
pub struct DxgiSwapChain(pub(crate) DxgiDeviceSubObject);

impl Guid for DxgiSwapChain {
	const IID: &'static GUID = &GUID::from_u128(0x310d36a0d2e74c0aaa046a9d23b8886au128);
}

impl Clone for DxgiSwapChain {
	fn clone(&self) -> Self { DxgiSwapChain(self.0.clone()) }
}

pub trait IDxgiSwapChain: IDxgiDeviceSubObject {
	fn as_swap_chain(&self) -> &DxgiSwapChain;
	fn into_swap_chain(self) -> DxgiSwapChain;

	fn Present(&self, sync_interval: u32, flags: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, sync_interval: u32, flags: u32, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, sync_interval, flags, );
		ret.ok()
	}

	fn GetBuffer<T: IUnknown>(&self, buffer: u32, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _surface: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, buffer: u32, riid: &GUID, _surface: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, buffer, T::IID, &mut _surface, );
		if ret.is_ok() {
			if let (Some(_surface)) = (_surface) {
				return Ok((T::from(_surface)));
			}
		}
		Err(ret)
	}

	fn SetFullscreenState(&self, fullscreen: bool, target: Option<&DxgiOutput>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, fullscreen: Bool, target: Option<VTable>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, fullscreen.to_bool(), option_to_vtable(target), );
		ret.ok()
	}

	fn GetFullscreenState(&self, fullscreen: Option<&mut Bool>, mut target: Option<&mut Option<DxgiOutput>>,) -> Result<(), HResult> {
		let vt = self.as_param();
		if let Some(ref mut target) = target { **target = None; }
		let f: extern "system" fn(Param<Self>, fullscreen: Option<&mut Bool>, target: Option<&mut Option<DxgiOutput>>, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, fullscreen, target, );
		ret.ok()
	}

	fn GetDesc(&self, ) -> Result<(DxgiSwapChainDesc), HResult> {
		let vt = self.as_param();
		let mut _desc: DxgiSwapChainDesc = DxgiSwapChainDesc::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiSwapChainDesc, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, &mut _desc, );
		if ret.is_ok() {
			return Ok((_desc));
		}
		Err(ret)
	}

	fn ResizeBuffers(&self, buffer_count: u32, width: u32, height: u32, new_format: DxgiFormat, swap_chain_flags: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, buffer_count: u32, width: u32, height: u32, new_format: DxgiFormat, swap_chain_flags: u32, ) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, buffer_count, width, height, new_format, swap_chain_flags, );
		ret.ok()
	}

	fn ResizeTarget(&self, new_target_parameters: &DxgiModeDesc, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, new_target_parameters: &DxgiModeDesc, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, new_target_parameters, );
		ret.ok()
	}

	fn GetContainingOutput(&self, ) -> Result<(DxgiOutput), HResult> {
		let vt = self.as_param();
		let mut _output: Option<DxgiOutput> = None;
		let f: extern "system" fn(Param<Self>, _output: &mut Option<DxgiOutput>, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, &mut _output, );
		if ret.is_ok() {
			if let (Some(_output)) = (_output) {
				return Ok((_output));
			}
		}
		Err(ret)
	}

	fn GetFrameStatistics(&self, ) -> Result<(DxgiFrameStatistics), HResult> {
		let vt = self.as_param();
		let mut _stats: DxgiFrameStatistics = DxgiFrameStatistics::zeroed();
		let f: extern "system" fn(Param<Self>, _stats: &mut DxgiFrameStatistics, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, &mut _stats, );
		if ret.is_ok() {
			return Ok((_stats));
		}
		Err(ret)
	}

	fn GetLastPresentCount(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _last_present_count: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _last_present_count: &mut u32, ) -> HResult
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, &mut _last_present_count, );
		if ret.is_ok() {
			return Ok((_last_present_count));
		}
		Err(ret)
	}
}

impl IDxgiSwapChain for DxgiSwapChain {
	fn as_swap_chain(&self) -> &DxgiSwapChain { self }
	fn into_swap_chain(self) -> DxgiSwapChain { self }
}

impl IDxgiDeviceSubObject for DxgiSwapChain {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0 }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0 }
}

impl IDxgiObject for DxgiSwapChain {
	fn as_object(&self) -> &DxgiObject { &self.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0 }
}

impl From<Unknown> for DxgiSwapChain {
    fn from(v: Unknown) -> Self {
        Self(DxgiDeviceSubObject::from(v))
    }
}

impl IUnknown for DxgiSwapChain {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

