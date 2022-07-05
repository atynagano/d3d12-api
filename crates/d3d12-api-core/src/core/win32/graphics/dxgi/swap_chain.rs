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
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, sync_interval: u32, flags: u32, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, sync_interval, flags, );
			_ret_.ok()
		}
	}

	fn GetBuffer<T: IUnknown>(&self, buffer: u32, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_surface: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, buffer: u32, riid: &GUID, _out_surface: *mut c_void, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, buffer, T::IID, transmute(&mut _out_surface), );
			if _ret_.is_ok() {
				if let Some(_out_surface) = _out_surface {
					return Ok(T::from(_out_surface));
				}
			}
			Err(_ret_)
		}
	}

	fn SetFullscreenState(&self, fullscreen: bool, target: Option<&DxgiOutput>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, fullscreen: Bool, target: *const c_void, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, fullscreen.to_bool(), option_to_vtable(target), );
			_ret_.ok()
		}
	}

	fn GetFullscreenState(&self, fullscreen: Option<&mut Bool>, mut target: Option<&mut Option<DxgiOutput>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut target) = target { **target = None; }
			let f: extern "system" fn(Param<Self>, fullscreen: Option<&mut Bool>, target: *mut c_void, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, fullscreen, transmute(target), );
			_ret_.ok()
		}
	}

	fn GetDesc(&self, ) -> Result<DxgiSwapChainDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiSwapChainDesc> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiSwapChainDesc, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			Ok(_out_desc.assume_init())
		}
	}

	fn ResizeBuffers(&self, buffer_count: u32, width: u32, height: u32, new_format: DxgiFormat, swap_chain_flags: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, buffer_count: u32, width: u32, height: u32, new_format: DxgiFormat, swap_chain_flags: u32, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, buffer_count, width, height, new_format, swap_chain_flags, );
			_ret_.ok()
		}
	}

	fn ResizeTarget(&self, new_target_parameters: &DxgiModeDesc, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, new_target_parameters: &DxgiModeDesc, ) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, new_target_parameters, );
			_ret_.ok()
		}
	}

	fn GetContainingOutput(&self, ) -> Result<DxgiOutput, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_output: Option<DxgiOutput> = None;
			let f: extern "system" fn(Param<Self>, output: *mut c_void, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, transmute(&mut _out_output), );
			if _ret_.is_ok() {
				if let Some(_out_output) = _out_output {
					return Ok(_out_output);
				}
			}
			Err(_ret_)
		}
	}

	fn GetFrameStatistics(&self, ) -> Result<DxgiFrameStatistics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_stats: MaybeUninit<DxgiFrameStatistics> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_stats: *mut DxgiFrameStatistics, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, _out_stats.as_mut_ptr(), );
			Ok(_out_stats.assume_init())
		}
	}

	fn GetLastPresentCount(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_last_present_count: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_last_present_count: *mut u32, ) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, _out_last_present_count.as_mut_ptr(), );
			Ok(_out_last_present_count.assume_init())
		}
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

