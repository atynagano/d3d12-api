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
use crate::core::win32::graphics::dxgi::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiSwapChain(pub(crate) DxgiDeviceSubObject);

impl Deref for DxgiSwapChain {
	type Target = DxgiDeviceSubObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiSwapChain {
	const IID: &'static GUID = &GUID::from_u128(0x310d36a0d2e74c0aaa046a9d23b8886au128);
}

impl Com for DxgiSwapChain {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiSwapChain {
	pub fn Present(&self, sync_interval: u32, flags: DxgiPresent, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, sync_interval: u32, flags: DxgiPresent, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, sync_interval, flags, );
			_ret_.ok()
		}
	}

	pub fn GetBuffer<T: IUnknown + From<UnknownWrapper>>(&self, buffer: u32) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut surface_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, u32, &GUID, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, buffer, T::IID, transmute(&mut surface_out_));
			if _ret_.is_ok() { if let Some(surface_out_) = surface_out_ { return Ok(T::from(UnknownWrapper(surface_out_))); } }
			Err(_ret_)
		}
	}

	pub fn SetFullscreenState(&self, fullscreen: bool, target: Option<&DxgiOutput>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool, *const c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, fullscreen.to_bool(), option_to_vtable(target));
			_ret_.ok()
		}
	}

	pub fn GetFullscreenState(&self, fullscreen: Option<&mut MaybeUninit<Bool>>, mut target: Option<&mut Option<DxgiOutput>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut target) = target { **target = None; }
			let f: extern "system" fn(Param<Self>, Option<&mut MaybeUninit<Bool>>, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, fullscreen, transmute(target));
			_ret_.ok()
		}
	}

	pub fn GetDesc(&self) -> Result<DxgiSwapChainDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiSwapChainDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiSwapChainDesc) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn ResizeBuffers(&self, buffer_count: u32, width: u32, height: u32, new_format: DxgiFormat, swap_chain_flags: DxgiSwapChainFlag, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, buffer_count: u32, width: u32, height: u32, new_format: DxgiFormat, swap_chain_flags: DxgiSwapChainFlag, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, buffer_count, width, height, new_format, swap_chain_flags, );
			_ret_.ok()
		}
	}

	pub fn ResizeTarget(&self, new_target_parameters: &DxgiModeDesc) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &DxgiModeDesc) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, new_target_parameters);
			_ret_.ok()
		}
	}

	pub fn GetContainingOutput(&self) -> Result<DxgiOutput, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut output_out_: Option<DxgiOutput> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, transmute(&mut output_out_));
			if _ret_.is_ok() { if let Some(output_out_) = output_out_ { return Ok(output_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetFrameStatistics(&self) -> Result<DxgiFrameStatistics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut stats_out_: MaybeUninit<DxgiFrameStatistics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiFrameStatistics) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, stats_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(stats_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetLastPresentCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut last_present_count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, last_present_count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(last_present_count_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiSwapChain: IDxgiDeviceSubObject {
	fn as_swap_chain(&self) -> &DxgiSwapChain;
	fn into_swap_chain(self) -> DxgiSwapChain;
}

impl IDxgiSwapChain for DxgiSwapChain {
	fn as_swap_chain(&self) -> &DxgiSwapChain { self }
	fn into_swap_chain(self) -> DxgiSwapChain { self }
}
impl IDxgiDeviceSubObject for DxgiSwapChain {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiSwapChain {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiSwapChain {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiSwapChain {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiDeviceSubObject::from(v))
    }
}

