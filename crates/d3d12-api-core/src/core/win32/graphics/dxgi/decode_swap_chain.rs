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
#[repr(C)]
pub struct DxgiDecodeSwapChain(pub(crate) Unknown);

impl Guid for DxgiDecodeSwapChain {
	const IID: &'static GUID = &GUID::from_u128(0x2633066b45144c7a8fd812ea98059d18u128);
}

impl Clone for DxgiDecodeSwapChain {
	fn clone(&self) -> Self { DxgiDecodeSwapChain(self.0.clone()) }
}

pub trait IDxgiDecodeSwapChain: IUnknown {
	fn as_decode_swap_chain(&self) -> &DxgiDecodeSwapChain;
	fn into_decode_swap_chain(self) -> DxgiDecodeSwapChain;

	fn PresentBuffer(&self, buffer_to_present: u32, sync_interval: u32, flags: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, buffer_to_present: u32, sync_interval: u32, flags: u32, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, buffer_to_present, sync_interval, flags, );
		ret.ok()
	}

	fn SetSourceRect(&self, rect: &Rect, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, rect: &Rect, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, rect, );
		ret.ok()
	}

	fn SetTargetRect(&self, rect: &Rect, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, rect: &Rect, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, rect, );
		ret.ok()
	}

	fn SetDestSize(&self, width: u32, height: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, width: u32, height: u32, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, width, height, );
		ret.ok()
	}

	fn GetSourceRect(&self, ) -> Result<(Rect), HResult> {
		let vt = self.as_param();
		let mut _rect: Rect = Rect::zeroed();
		let f: extern "system" fn(Param<Self>, _rect: &mut Rect, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, &mut _rect, );
		if ret.is_ok() {
			return Ok((_rect));
		}
		Err(ret)
	}

	fn GetTargetRect(&self, ) -> Result<(Rect), HResult> {
		let vt = self.as_param();
		let mut _rect: Rect = Rect::zeroed();
		let f: extern "system" fn(Param<Self>, _rect: &mut Rect, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, &mut _rect, );
		if ret.is_ok() {
			return Ok((_rect));
		}
		Err(ret)
	}

	fn GetDestSize(&self, ) -> Result<(u32, u32), HResult> {
		let vt = self.as_param();
		let mut _width: u32 = u32::zeroed();
		let mut _height: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _width: &mut u32, _height: &mut u32, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, &mut _width, &mut _height, );
		if ret.is_ok() {
			return Ok((_width, _height));
		}
		Err(ret)
	}

	fn SetColorSpace(&self, color_space: DxgiMultiplaneOverlayYcbcrFlags, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, color_space: DxgiMultiplaneOverlayYcbcrFlags, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, color_space, );
		ret.ok()
	}

	fn GetColorSpace(&self, ) -> (DxgiMultiplaneOverlayYcbcrFlags) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> DxgiMultiplaneOverlayYcbcrFlags
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl IDxgiDecodeSwapChain for DxgiDecodeSwapChain {
	fn as_decode_swap_chain(&self) -> &DxgiDecodeSwapChain { self }
	fn into_decode_swap_chain(self) -> DxgiDecodeSwapChain { self }
}

impl From<Unknown> for DxgiDecodeSwapChain {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxgiDecodeSwapChain {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

