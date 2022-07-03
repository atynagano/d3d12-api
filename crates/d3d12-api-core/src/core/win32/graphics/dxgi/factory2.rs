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
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::dxgi::*;
#[repr(C)]
pub struct DxgiFactory2(pub(crate) DxgiFactory1);

impl Guid for DxgiFactory2 {
	const IID: &'static GUID = &GUID::from_u128(0x50c83a1ce0724c4887b03630fa36a6d0u128);
}

impl Clone for DxgiFactory2 {
	fn clone(&self) -> Self { DxgiFactory2(self.0.clone()) }
}

pub trait IDxgiFactory2: IDxgiFactory1 {
	fn as_factory2(&self) -> &DxgiFactory2;
	fn into_factory2(self) -> DxgiFactory2;

	fn IsWindowedStereoEnabled(&self, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Bool
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, );
		return (ret.to_bool());
	}

	fn CreateSwapChainForHwnd(&self, device: &(impl IUnknown + ?Sized), wnd: HWnd, desc: &DxgiSwapChainDesc1, fullscreen_desc: Option<&DxgiSwapChainFullScreenDesc>, restrict_to_output: Option<&DxgiOutput>, ) -> Result<(DxgiSwapChain1), HResult> {
		let vt = self.as_param();
		let mut _swap_chain: Option<DxgiSwapChain1> = None;
		let f: extern "system" fn(Param<Self>, device: VTable, wnd: HWnd, desc: &DxgiSwapChainDesc1, fullscreen_desc: Option<&DxgiSwapChainFullScreenDesc>, restrict_to_output: Option<VTable>, _swap_chain: &mut Option<DxgiSwapChain1>, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, device.vtable(), wnd, desc, fullscreen_desc, option_to_vtable(restrict_to_output), &mut _swap_chain, );
		if ret.is_ok() {
			if let (Some(_swap_chain)) = (_swap_chain) {
				return Ok((_swap_chain));
			}
		}
		Err(ret)
	}

	fn CreateSwapChainForCoreWindow(&self, device: &(impl IUnknown + ?Sized), window: &(impl IUnknown + ?Sized), desc: &DxgiSwapChainDesc1, restrict_to_output: Option<&DxgiOutput>, ) -> Result<(DxgiSwapChain1), HResult> {
		let vt = self.as_param();
		let mut _swap_chain: Option<DxgiSwapChain1> = None;
		let f: extern "system" fn(Param<Self>, device: VTable, window: VTable, desc: &DxgiSwapChainDesc1, restrict_to_output: Option<VTable>, _swap_chain: &mut Option<DxgiSwapChain1>, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, device.vtable(), window.vtable(), desc, option_to_vtable(restrict_to_output), &mut _swap_chain, );
		if ret.is_ok() {
			if let (Some(_swap_chain)) = (_swap_chain) {
				return Ok((_swap_chain));
			}
		}
		Err(ret)
	}

	fn GetSharedResourceAdapterLuid(&self, resource: Handle, ) -> Result<(Luid), HResult> {
		let vt = self.as_param();
		let mut _luid: Luid = Luid::zeroed();
		let f: extern "system" fn(Param<Self>, resource: Handle, _luid: &mut Luid, ) -> HResult
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, resource, &mut _luid, );
		if ret.is_ok() {
			return Ok((_luid));
		}
		Err(ret)
	}

	fn RegisterStereoStatusWindow(&self, window_handle: HWnd, msg: u32, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _pdw_cookie: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, window_handle: HWnd, msg: u32, _pdw_cookie: &mut u32, ) -> HResult
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, window_handle, msg, &mut _pdw_cookie, );
		if ret.is_ok() {
			return Ok((_pdw_cookie));
		}
		Err(ret)
	}

	fn RegisterStereoStatusEvent(&self, event: Handle, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _pdw_cookie: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, event: Handle, _pdw_cookie: &mut u32, ) -> HResult
			= unsafe { transmute(vt[19]) };
		let ret = f(vt, event, &mut _pdw_cookie, );
		if ret.is_ok() {
			return Ok((_pdw_cookie));
		}
		Err(ret)
	}

	fn UnregisterStereoStatus(&self, cookie: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, cookie: u32, ) -> ()
			= unsafe { transmute(vt[20]) };
		let ret = f(vt, cookie, );
	}

	fn RegisterOcclusionStatusWindow(&self, window_handle: HWnd, msg: u32, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _pdw_cookie: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, window_handle: HWnd, msg: u32, _pdw_cookie: &mut u32, ) -> HResult
			= unsafe { transmute(vt[21]) };
		let ret = f(vt, window_handle, msg, &mut _pdw_cookie, );
		if ret.is_ok() {
			return Ok((_pdw_cookie));
		}
		Err(ret)
	}

	fn RegisterOcclusionStatusEvent(&self, event: Handle, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _pdw_cookie: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, event: Handle, _pdw_cookie: &mut u32, ) -> HResult
			= unsafe { transmute(vt[22]) };
		let ret = f(vt, event, &mut _pdw_cookie, );
		if ret.is_ok() {
			return Ok((_pdw_cookie));
		}
		Err(ret)
	}

	fn UnregisterOcclusionStatus(&self, cookie: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, cookie: u32, ) -> ()
			= unsafe { transmute(vt[23]) };
		let ret = f(vt, cookie, );
	}

	fn CreateSwapChainForComposition(&self, device: &(impl IUnknown + ?Sized), desc: &DxgiSwapChainDesc1, restrict_to_output: Option<&DxgiOutput>, ) -> Result<(DxgiSwapChain1), HResult> {
		let vt = self.as_param();
		let mut _swap_chain: Option<DxgiSwapChain1> = None;
		let f: extern "system" fn(Param<Self>, device: VTable, desc: &DxgiSwapChainDesc1, restrict_to_output: Option<VTable>, _swap_chain: &mut Option<DxgiSwapChain1>, ) -> HResult
			= unsafe { transmute(vt[24]) };
		let ret = f(vt, device.vtable(), desc, option_to_vtable(restrict_to_output), &mut _swap_chain, );
		if ret.is_ok() {
			if let (Some(_swap_chain)) = (_swap_chain) {
				return Ok((_swap_chain));
			}
		}
		Err(ret)
	}
}

impl IDxgiFactory2 for DxgiFactory2 {
	fn as_factory2(&self) -> &DxgiFactory2 { self }
	fn into_factory2(self) -> DxgiFactory2 { self }
}

impl IDxgiFactory1 for DxgiFactory2 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0 }
	fn into_factory1(self) -> DxgiFactory1 { self.0 }
}

impl IDxgiFactory for DxgiFactory2 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.0 }
	fn into_factory(self) -> DxgiFactory { self.0.0 }
}

impl IDxgiObject for DxgiFactory2 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0 }
}

impl From<Unknown> for DxgiFactory2 {
    fn from(v: Unknown) -> Self {
        Self(DxgiFactory1::from(v))
    }
}

impl IUnknown for DxgiFactory2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

