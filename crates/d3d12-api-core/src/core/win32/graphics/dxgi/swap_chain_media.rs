#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::num::NonZeroUsize;
use std::mem::{MaybeUninit, size_of_val, transmute};
use std::ops::Deref;
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiSwapChainMedia(pub(crate) Unknown);

impl Deref for DxgiSwapChainMedia {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiSwapChainMedia {
	const IID: &'static GUID = &GUID::from_u128(0xdd95b90bf05f4f6abd6525bfb264bd84u128);
}

impl Com for DxgiSwapChainMedia {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiSwapChainMedia {
	pub fn GetFrameStatisticsMedia(&self) -> Result<DxgiFrameStatisticsMedia, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut stats_out_: MaybeUninit<DxgiFrameStatisticsMedia> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiFrameStatisticsMedia) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, stats_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(stats_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn SetPresentDuration(&self, duration: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, duration);
			_ret_.ok()
		}
	}

	pub fn CheckPresentDurationSupport(&self, desired_present_duration: u32) -> Result<(u32, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut closest_smaller_present_duration_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut closest_larger_present_duration_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u32, *mut u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, desired_present_duration, closest_smaller_present_duration_out_.as_mut_ptr(), closest_larger_present_duration_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((closest_smaller_present_duration_out_.assume_init(), closest_larger_present_duration_out_.assume_init())) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiSwapChainMedia: IUnknown {
	fn as_swap_chain_media(&self) -> &DxgiSwapChainMedia;
	fn into_swap_chain_media(self) -> DxgiSwapChainMedia;
}

impl IDxgiSwapChainMedia for DxgiSwapChainMedia {
	fn as_swap_chain_media(&self) -> &DxgiSwapChainMedia { self }
	fn into_swap_chain_media(self) -> DxgiSwapChainMedia { self }
}
impl IUnknown for DxgiSwapChainMedia {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiSwapChainMedia {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

