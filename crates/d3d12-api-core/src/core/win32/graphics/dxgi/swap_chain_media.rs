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
pub struct DxgiSwapChainMedia(pub(crate) Unknown);

impl Guid for DxgiSwapChainMedia {
	const IID: &'static GUID = &GUID::from_u128(0xdd95b90bf05f4f6abd6525bfb264bd84u128);
}

impl Clone for DxgiSwapChainMedia {
	fn clone(&self) -> Self { DxgiSwapChainMedia(self.0.clone()) }
}

pub trait IDxgiSwapChainMedia: IUnknown {
	fn as_swap_chain_media(&self) -> &DxgiSwapChainMedia;
	fn into_swap_chain_media(self) -> DxgiSwapChainMedia;

	fn GetFrameStatisticsMedia(&self, ) -> Result<(DxgiFrameStatisticsMedia), HResult> {
		let vt = self.as_param();
		let mut _stats: DxgiFrameStatisticsMedia = DxgiFrameStatisticsMedia::zeroed();
		let f: extern "system" fn(Param<Self>, _stats: &mut DxgiFrameStatisticsMedia, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, &mut _stats, );
		if ret.is_ok() {
			return Ok((_stats));
		}
		Err(ret)
	}

	fn SetPresentDuration(&self, duration: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, duration: u32, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, duration, );
		ret.ok()
	}

	fn CheckPresentDurationSupport(&self, desired_present_duration: u32, ) -> Result<(u32, u32), HResult> {
		let vt = self.as_param();
		let mut _closest_smaller_present_duration: u32 = u32::zeroed();
		let mut _closest_larger_present_duration: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, desired_present_duration: u32, _closest_smaller_present_duration: &mut u32, _closest_larger_present_duration: &mut u32, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, desired_present_duration, &mut _closest_smaller_present_duration, &mut _closest_larger_present_duration, );
		if ret.is_ok() {
			return Ok((_closest_smaller_present_duration, _closest_larger_present_duration));
		}
		Err(ret)
	}
}

impl IDxgiSwapChainMedia for DxgiSwapChainMedia {
	fn as_swap_chain_media(&self) -> &DxgiSwapChainMedia { self }
	fn into_swap_chain_media(self) -> DxgiSwapChainMedia { self }
}

impl From<Unknown> for DxgiSwapChainMedia {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxgiSwapChainMedia {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

