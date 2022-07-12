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
pub struct DxgiSwapChain1(pub(crate) DxgiSwapChain);

impl Guid for DxgiSwapChain1 {
	const IID: &'static GUID = &GUID::from_u128(0x790a45f70d424876983a0a55cfe6f4aau128);
}

impl Clone for DxgiSwapChain1 {
	fn clone(&self) -> Self { DxgiSwapChain1(self.0.clone()) }
}

pub trait IDxgiSwapChain1: IDxgiSwapChain {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1;
	fn into_swap_chain1(self) -> DxgiSwapChain1;

	fn GetDesc1(&self, ) -> Result<DxgiSwapChainDesc1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiSwapChainDesc1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiSwapChainDesc1, ) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			Ok(_out_desc.assume_init())
		}
	}

	fn GetFullscreenDesc(&self, ) -> Result<DxgiSwapChainFullScreenDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiSwapChainFullScreenDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiSwapChainFullScreenDesc, ) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			Ok(_out_desc.assume_init())
		}
	}

	fn GetHwnd(&self, ) -> Result<HWnd, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_hwnd: Option<HWnd> = None;
			let f: extern "system" fn(Param<Self>, _out_hwnd: *mut c_void, ) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, transmute(&mut _out_hwnd), );
			if _ret_.is_ok() {
				if let Some(_out_hwnd) = _out_hwnd {
					return Ok(_out_hwnd);
				}
			}
			Err(_ret_)
		}
	}

	fn GetCoreWindow<T: IUnknown>(&self, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_unk: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, refiid: &GUID, _out_unk: *mut c_void, ) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, T::IID, transmute(&mut _out_unk), );
			if _ret_.is_ok() {
				if let Some(_out_unk) = _out_unk {
					return Ok(T::from(_out_unk));
				}
			}
			Err(_ret_)
		}
	}

	fn Present1(&self, sync_interval: u32, present_flags: u32, present_parameters: &DxgiPresentParameters, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, sync_interval: u32, present_flags: u32, present_parameters: &DxgiPresentParameters, ) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, sync_interval, present_flags, present_parameters, );
			_ret_.ok()
		}
	}

	fn IsTemporaryMonoSupported(&self, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Bool
				= transmute(vt[23]);
			let _ret_ = f(vt, );
			_ret_.to_bool()
		}
	}

	fn GetRestrictToOutput(&self, ) -> Result<DxgiOutput, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_restrict_to_output: Option<DxgiOutput> = None;
			let f: extern "system" fn(Param<Self>, restrict_to_output: *mut c_void, ) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, transmute(&mut _out_restrict_to_output), );
			if _ret_.is_ok() {
				if let Some(_out_restrict_to_output) = _out_restrict_to_output {
					return Ok(_out_restrict_to_output);
				}
			}
			Err(_ret_)
		}
	}

	fn SetBackgroundColor(&self, color: &DxgiRgba, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, color: &DxgiRgba, ) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, color, );
			_ret_.ok()
		}
	}

	fn GetBackgroundColor(&self, ) -> Result<DxgiRgba, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_color: MaybeUninit<DxgiRgba> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_color: *mut DxgiRgba, ) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, _out_color.as_mut_ptr(), );
			Ok(_out_color.assume_init())
		}
	}

	fn SetRotation(&self, rotation: DxgiModeRotation, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, rotation: DxgiModeRotation, ) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, rotation, );
			_ret_.ok()
		}
	}

	fn GetRotation(&self, ) -> Result<DxgiModeRotation, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_rotation: MaybeUninit<DxgiModeRotation> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_rotation: *mut DxgiModeRotation, ) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, _out_rotation.as_mut_ptr(), );
			Ok(_out_rotation.assume_init())
		}
	}
}

impl IDxgiSwapChain1 for DxgiSwapChain1 {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1 { self }
	fn into_swap_chain1(self) -> DxgiSwapChain1 { self }
}

impl IDxgiSwapChain for DxgiSwapChain1 {
	fn as_swap_chain(&self) -> &DxgiSwapChain { &self.0.as_swap_chain() }
	fn into_swap_chain(self) -> DxgiSwapChain { self.0.into_swap_chain() }
}

impl IDxgiDeviceSubObject for DxgiSwapChain1 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiSwapChain1 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiSwapChain1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSwapChain::from(v))
    }
}

impl IUnknown for DxgiSwapChain1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

