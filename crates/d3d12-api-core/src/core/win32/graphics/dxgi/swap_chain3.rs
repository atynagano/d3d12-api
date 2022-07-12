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

	fn GetCurrentBackBufferIndex(&self, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u32
				= transmute(vt[36]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn CheckColorSpaceSupport(&self, color_space: DxgiColorSpaceType, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_color_space_support: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, color_space: DxgiColorSpaceType, _out_color_space_support: *mut u32, ) -> HResult
				= transmute(vt[37]);
			let _ret_ = f(vt, color_space, _out_color_space_support.as_mut_ptr(), );
			Ok(_out_color_space_support.assume_init())
		}
	}

	fn SetColorSpace1(&self, color_space: DxgiColorSpaceType, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, color_space: DxgiColorSpaceType, ) -> HResult
				= transmute(vt[38]);
			let _ret_ = f(vt, color_space, );
			_ret_.ok()
		}
	}
}

impl IDxgiSwapChain3 for DxgiSwapChain3 {
	fn as_swap_chain3(&self) -> &DxgiSwapChain3 { self }
	fn into_swap_chain3(self) -> DxgiSwapChain3 { self }
}

impl IDxgiSwapChain2 for DxgiSwapChain3 {
	fn as_swap_chain2(&self) -> &DxgiSwapChain2 { &self.0.as_swap_chain2() }
	fn into_swap_chain2(self) -> DxgiSwapChain2 { self.0.into_swap_chain2() }
}

impl IDxgiSwapChain1 for DxgiSwapChain3 {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1 { &self.0.as_swap_chain1() }
	fn into_swap_chain1(self) -> DxgiSwapChain1 { self.0.into_swap_chain1() }
}

impl IDxgiSwapChain for DxgiSwapChain3 {
	fn as_swap_chain(&self) -> &DxgiSwapChain { &self.0.as_swap_chain() }
	fn into_swap_chain(self) -> DxgiSwapChain { self.0.into_swap_chain() }
}

impl IDxgiDeviceSubObject for DxgiSwapChain3 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiSwapChain3 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiSwapChain3 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSwapChain2::from(v))
    }
}

impl IUnknown for DxgiSwapChain3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

