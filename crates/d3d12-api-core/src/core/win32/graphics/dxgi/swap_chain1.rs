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
pub struct DxgiSwapChain1(pub(crate) DxgiSwapChain);

impl Deref for DxgiSwapChain1 {
	type Target = DxgiSwapChain;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiSwapChain1 {
	const IID: &'static GUID = &GUID::from_u128(0x790a45f70d424876983a0a55cfe6f4aau128);
}

impl Com for DxgiSwapChain1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiSwapChain1 {
	pub fn GetDesc1(&self) -> Result<DxgiSwapChainDesc1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiSwapChainDesc1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiSwapChainDesc1) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetFullscreenDesc(&self) -> Result<DxgiSwapChainFullScreenDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiSwapChainFullScreenDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiSwapChainFullScreenDesc) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetHwnd(&self) -> Result<HWnd, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut hwnd_out_: Option<HWnd> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, transmute(&mut hwnd_out_));
			if _ret_.is_ok() { if let Some(hwnd_out_) = hwnd_out_ { return Ok(hwnd_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetCoreWindow<T: IUnknown + From<UnknownWrapper>>(&self) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut unk_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, T::IID, transmute(&mut unk_out_));
			if _ret_.is_ok() { if let Some(unk_out_) = unk_out_ { return Ok(T::from(UnknownWrapper(unk_out_))); } }
			Err(_ret_)
		}
	}

	pub fn Present1(&self, sync_interval: u32, present_flags: DxgiPresent, present_parameters: &DxgiPresentParameters, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, sync_interval: u32, present_flags: DxgiPresent, present_parameters: &DxgiPresentParameters, ) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, sync_interval, present_flags, present_parameters, );
			_ret_.ok()
		}
	}

	pub fn IsTemporaryMonoSupported(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[23]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}

	pub fn GetRestrictToOutput(&self) -> Result<DxgiOutput, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut restrict_to_output_out_: Option<DxgiOutput> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, transmute(&mut restrict_to_output_out_));
			if _ret_.is_ok() { if let Some(restrict_to_output_out_) = restrict_to_output_out_ { return Ok(restrict_to_output_out_); } }
			Err(_ret_)
		}
	}

	pub fn SetBackgroundColor(&self, color: &DxgiRgba) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &DxgiRgba) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, color);
			_ret_.ok()
		}
	}

	pub fn GetBackgroundColor(&self) -> Result<DxgiRgba, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_out_: MaybeUninit<DxgiRgba> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiRgba) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, color_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(color_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn SetRotation(&self, rotation: DxgiModeRotation) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DxgiModeRotation) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, rotation);
			_ret_.ok()
		}
	}

	pub fn GetRotation(&self) -> Result<DxgiModeRotation, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut rotation_out_: MaybeUninit<DxgiModeRotation> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiModeRotation) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, rotation_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(rotation_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiSwapChain1: IDxgiSwapChain {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1;
	fn into_swap_chain1(self) -> DxgiSwapChain1;
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

impl IUnknown for DxgiSwapChain1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiSwapChain1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiSwapChain::from(v))
    }
}

