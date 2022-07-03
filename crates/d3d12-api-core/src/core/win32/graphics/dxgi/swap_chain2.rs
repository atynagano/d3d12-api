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
pub struct DxgiSwapChain2(pub(crate) DxgiSwapChain1);

impl Guid for DxgiSwapChain2 {
	const IID: &'static GUID = &GUID::from_u128(0xa8be2ac4199f4946b33179599fb98de7u128);
}

impl Clone for DxgiSwapChain2 {
	fn clone(&self) -> Self { DxgiSwapChain2(self.0.clone()) }
}

pub trait IDxgiSwapChain2: IDxgiSwapChain1 {
	fn as_swap_chain2(&self) -> &DxgiSwapChain2;
	fn into_swap_chain2(self) -> DxgiSwapChain2;

	fn SetSourceSize(&self, width: u32, height: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, width: u32, height: u32, ) -> HResult
			= unsafe { transmute(vt[29]) };
		let ret = f(vt, width, height, );
		ret.ok()
	}

	fn GetSourceSize(&self, ) -> Result<(u32, u32), HResult> {
		let vt = self.as_param();
		let mut _width: u32 = u32::zeroed();
		let mut _height: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _width: &mut u32, _height: &mut u32, ) -> HResult
			= unsafe { transmute(vt[30]) };
		let ret = f(vt, &mut _width, &mut _height, );
		if ret.is_ok() {
			return Ok((_width, _height));
		}
		Err(ret)
	}

	fn SetMaximumFrameLatency(&self, max_latency: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, max_latency: u32, ) -> HResult
			= unsafe { transmute(vt[31]) };
		let ret = f(vt, max_latency, );
		ret.ok()
	}

	fn GetMaximumFrameLatency(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _max_latency: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _max_latency: &mut u32, ) -> HResult
			= unsafe { transmute(vt[32]) };
		let ret = f(vt, &mut _max_latency, );
		if ret.is_ok() {
			return Ok((_max_latency));
		}
		Err(ret)
	}

	fn GetFrameLatencyWaitableObject(&self, ) -> (Handle) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Handle
			= unsafe { transmute(vt[33]) };
		let ret = f(vt, );
		return (ret);
	}

	fn SetMatrixTransform(&self, matrix: &DxgiMatrix3X2F, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, matrix: &DxgiMatrix3X2F, ) -> HResult
			= unsafe { transmute(vt[34]) };
		let ret = f(vt, matrix, );
		ret.ok()
	}

	fn GetMatrixTransform(&self, ) -> Result<(DxgiMatrix3X2F), HResult> {
		let vt = self.as_param();
		let mut _matrix: DxgiMatrix3X2F = DxgiMatrix3X2F::zeroed();
		let f: extern "system" fn(Param<Self>, _matrix: &mut DxgiMatrix3X2F, ) -> HResult
			= unsafe { transmute(vt[35]) };
		let ret = f(vt, &mut _matrix, );
		if ret.is_ok() {
			return Ok((_matrix));
		}
		Err(ret)
	}
}

impl IDxgiSwapChain2 for DxgiSwapChain2 {
	fn as_swap_chain2(&self) -> &DxgiSwapChain2 { self }
	fn into_swap_chain2(self) -> DxgiSwapChain2 { self }
}

impl IDxgiSwapChain1 for DxgiSwapChain2 {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1 { &self.0 }
	fn into_swap_chain1(self) -> DxgiSwapChain1 { self.0 }
}

impl IDxgiSwapChain for DxgiSwapChain2 {
	fn as_swap_chain(&self) -> &DxgiSwapChain { &self.0.0 }
	fn into_swap_chain(self) -> DxgiSwapChain { self.0.0 }
}

impl IDxgiDeviceSubObject for DxgiSwapChain2 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.0.0 }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.0.0 }
}

impl IDxgiObject for DxgiSwapChain2 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0 }
}

impl From<Unknown> for DxgiSwapChain2 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSwapChain1::from(v))
    }
}

impl IUnknown for DxgiSwapChain2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

