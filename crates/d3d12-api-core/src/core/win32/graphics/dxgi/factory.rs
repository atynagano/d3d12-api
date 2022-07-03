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
use crate::core::win32::system::com::*;
#[repr(C)]
pub struct DxgiFactory(pub(crate) DxgiObject);

impl Guid for DxgiFactory {
	const IID: &'static GUID = &GUID::from_u128(0x7b7166ec21c744aeb21ac9ae321ae369u128);
}

impl Clone for DxgiFactory {
	fn clone(&self) -> Self { DxgiFactory(self.0.clone()) }
}

pub trait IDxgiFactory: IDxgiObject {
	fn as_factory(&self) -> &DxgiFactory;
	fn into_factory(self) -> DxgiFactory;

	fn EnumAdapters(&self, adapter: u32, ) -> Result<(DxgiAdapter), HResult> {
		let vt = self.as_param();
		let mut _adapter: Option<DxgiAdapter> = None;
		let f: extern "system" fn(Param<Self>, adapter: u32, _adapter: &mut Option<DxgiAdapter>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, adapter, &mut _adapter, );
		if ret.is_ok() {
			if let (Some(_adapter)) = (_adapter) {
				return Ok((_adapter));
			}
		}
		Err(ret)
	}

	fn MakeWindowAssociation(&self, window_handle: HWnd, flags: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, window_handle: HWnd, flags: u32, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, window_handle, flags, );
		ret.ok()
	}

	fn GetWindowAssociation(&self, ) -> Result<(HWnd), HResult> {
		let vt = self.as_param();
		let mut _window_handle: HWnd = HWnd::zeroed();
		let f: extern "system" fn(Param<Self>, _window_handle: &mut HWnd, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, &mut _window_handle, );
		if ret.is_ok() {
			return Ok((_window_handle));
		}
		Err(ret)
	}

	fn CreateSwapChain(&self, device: &(impl IUnknown + ?Sized), desc: &DxgiSwapChainDesc, ) -> Result<(DxgiSwapChain), HResult> {
		let vt = self.as_param();
		let mut _swap_chain: Option<DxgiSwapChain> = None;
		let f: extern "system" fn(Param<Self>, device: VTable, desc: &DxgiSwapChainDesc, _swap_chain: &mut Option<DxgiSwapChain>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, device.vtable(), desc, &mut _swap_chain, );
		if ret.is_ok() {
			if let (Some(_swap_chain)) = (_swap_chain) {
				return Ok((_swap_chain));
			}
		}
		Err(ret)
	}

	fn CreateSoftwareAdapter(&self, module: HInstance, ) -> Result<(DxgiAdapter), HResult> {
		let vt = self.as_param();
		let mut _adapter: Option<DxgiAdapter> = None;
		let f: extern "system" fn(Param<Self>, module: HInstance, _adapter: &mut Option<DxgiAdapter>, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, module, &mut _adapter, );
		if ret.is_ok() {
			if let (Some(_adapter)) = (_adapter) {
				return Ok((_adapter));
			}
		}
		Err(ret)
	}
}

impl IDxgiFactory for DxgiFactory {
	fn as_factory(&self) -> &DxgiFactory { self }
	fn into_factory(self) -> DxgiFactory { self }
}

impl IDxgiObject for DxgiFactory {
	fn as_object(&self) -> &DxgiObject { &self.0 }
	fn into_object(self) -> DxgiObject { self.0 }
}

impl From<Unknown> for DxgiFactory {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiFactory {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

