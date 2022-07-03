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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::system::com::*;
#[repr(C)]
pub struct DxgiSwapChain3(pub(crate) DxgiSwapChain2);

impl Guid for DxgiSwapChain3 {
	const IID: &'static GUID = &GUID::from_u128(0x94d99bdbf1f84ab0b2367da0170edab1u128);
}

impl Clone for DxgiSwapChain3 {
	fn clone(&self) -> Self { DxgiSwapChain3(self.0.clone()) }
}

pub trait IDxgiSwapChain3: IDxgiSwapChain2 {
	fn as_swap_chain3(&self) -> &DxgiSwapChain3;
	fn into_swap_chain3(self) -> DxgiSwapChain3;

	fn GetCurrentBackBufferIndex(&self, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u32
			= unsafe { transmute(vt[36]) };
		let ret = f(vt, );
		return (ret);
	}

	fn CheckColorSpaceSupport(&self, color_space: DxgiColorSpaceType, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _color_space_support: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, color_space: DxgiColorSpaceType, _color_space_support: &mut u32, ) -> HResult
			= unsafe { transmute(vt[37]) };
		let ret = f(vt, color_space, &mut _color_space_support, );
		if ret.is_ok() {
			return Ok((_color_space_support));
		}
		Err(ret)
	}

	fn SetColorSpace1(&self, color_space: DxgiColorSpaceType, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, color_space: DxgiColorSpaceType, ) -> HResult
			= unsafe { transmute(vt[38]) };
		let ret = f(vt, color_space, );
		ret.ok()
	}

	fn ResizeBuffers1(&self, width: u32, height: u32, format: DxgiFormat, swap_chain_flags: u32, creation_node_mask: &[u32], present_queue: &[Param<Unknown>], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, buffer_count: u32, width: u32, height: u32, format: DxgiFormat, swap_chain_flags: u32, creation_node_mask: *const u32, present_queue: *const Param<Unknown>, ) -> HResult
			= unsafe { transmute(vt[39]) };
		let ret = f(vt, creation_node_mask.len() as u32, width, height, format, swap_chain_flags, creation_node_mask.as_ptr_or_null(), present_queue.as_ptr_or_null(), );
		ret.ok()
	}
}

impl IDxgiSwapChain3 for DxgiSwapChain3 {
	fn as_swap_chain3(&self) -> &DxgiSwapChain3 { self }
	fn into_swap_chain3(self) -> DxgiSwapChain3 { self }
}

impl IDxgiSwapChain2 for DxgiSwapChain3 {
	fn as_swap_chain2(&self) -> &DxgiSwapChain2 { &self.0 }
	fn into_swap_chain2(self) -> DxgiSwapChain2 { self.0 }
}

impl IDxgiSwapChain1 for DxgiSwapChain3 {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1 { &self.0.0 }
	fn into_swap_chain1(self) -> DxgiSwapChain1 { self.0.0 }
}

impl IDxgiSwapChain for DxgiSwapChain3 {
	fn as_swap_chain(&self) -> &DxgiSwapChain { &self.0.0.0 }
	fn into_swap_chain(self) -> DxgiSwapChain { self.0.0.0 }
}

impl IDxgiDeviceSubObject for DxgiSwapChain3 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.0.0.0 }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.0.0.0 }
}

impl IDxgiObject for DxgiSwapChain3 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0.0 }
}

impl From<Unknown> for DxgiSwapChain3 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSwapChain2::from(v))
    }
}

impl IUnknown for DxgiSwapChain3 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0 }
}

