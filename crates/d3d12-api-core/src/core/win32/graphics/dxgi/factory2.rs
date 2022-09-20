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
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiFactory2(pub(crate) DxgiFactory1);

impl Deref for DxgiFactory2 {
	type Target = DxgiFactory1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactory2 {
	const IID: &'static GUID = &GUID::from_u128(0x50c83a1ce0724c4887b03630fa36a6d0u128);
}

impl Com for DxgiFactory2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactory2 {
	pub fn IsWindowedStereoEnabled(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[14]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}

	pub fn CreateSwapChainForHwnd(&self, device: &Unknown, wnd: HWnd, desc: &DxgiSwapChainDesc1, fullscreen_desc: Option<&DxgiSwapChainFullScreenDesc>, restrict_to_output: Option<&DxgiOutput>) -> Result<DxgiSwapChain1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut swap_chain_out_: Option<DxgiSwapChain1> = None;
			let f: extern "system" fn(Param<Self>, VTable, HWnd, &DxgiSwapChainDesc1, *const c_void, *const c_void, *mut c_void) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, device.vtable(), wnd, desc, transmute(fullscreen_desc), option_to_vtable(restrict_to_output), transmute(&mut swap_chain_out_));
			if _ret_.is_ok() { if let Some(swap_chain_out_) = swap_chain_out_ { return Ok(swap_chain_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateSwapChainForCoreWindow(&self, device: &Unknown, window: &Unknown, desc: &DxgiSwapChainDesc1, restrict_to_output: Option<&DxgiOutput>) -> Result<DxgiSwapChain1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut swap_chain_out_: Option<DxgiSwapChain1> = None;
			let f: extern "system" fn(Param<Self>, VTable, VTable, &DxgiSwapChainDesc1, *const c_void, *mut c_void) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, device.vtable(), window.vtable(), desc, option_to_vtable(restrict_to_output), transmute(&mut swap_chain_out_));
			if _ret_.is_ok() { if let Some(swap_chain_out_) = swap_chain_out_ { return Ok(swap_chain_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetSharedResourceAdapterLuid(&self, resource: Handle) -> Result<Luid, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut luid_out_: MaybeUninit<Luid> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, Handle, *mut Luid) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, resource, luid_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(luid_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn RegisterStereoStatusWindow(&self, window_handle: HWnd, msg: u32) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pdw_cookie_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, HWnd, u32, *mut u32) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, window_handle, msg, pdw_cookie_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pdw_cookie_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn RegisterStereoStatusEvent(&self, event: Handle) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pdw_cookie_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, Handle, *mut u32) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, event, pdw_cookie_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pdw_cookie_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn UnregisterStereoStatus(&self, cookie: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> ()
				= transmute(vt[20]);
			let _ret_ = f(vt, cookie);
		}
	}

	pub fn RegisterOcclusionStatusWindow(&self, window_handle: HWnd, msg: u32) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pdw_cookie_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, HWnd, u32, *mut u32) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, window_handle, msg, pdw_cookie_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pdw_cookie_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn RegisterOcclusionStatusEvent(&self, event: Handle) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pdw_cookie_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, Handle, *mut u32) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, event, pdw_cookie_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pdw_cookie_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn UnregisterOcclusionStatus(&self, cookie: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> ()
				= transmute(vt[23]);
			let _ret_ = f(vt, cookie);
		}
	}

	pub fn CreateSwapChainForComposition(&self, device: &Unknown, desc: &DxgiSwapChainDesc1, restrict_to_output: Option<&DxgiOutput>) -> Result<DxgiSwapChain1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut swap_chain_out_: Option<DxgiSwapChain1> = None;
			let f: extern "system" fn(Param<Self>, VTable, &DxgiSwapChainDesc1, *const c_void, *mut c_void) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, device.vtable(), desc, option_to_vtable(restrict_to_output), transmute(&mut swap_chain_out_));
			if _ret_.is_ok() { if let Some(swap_chain_out_) = swap_chain_out_ { return Ok(swap_chain_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiFactory2: IDxgiFactory1 {
	fn as_factory2(&self) -> &DxgiFactory2;
	fn into_factory2(self) -> DxgiFactory2;
}

impl IDxgiFactory2 for DxgiFactory2 {
	fn as_factory2(&self) -> &DxgiFactory2 { self }
	fn into_factory2(self) -> DxgiFactory2 { self }
}
impl IDxgiFactory1 for DxgiFactory2 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.as_factory1() }
	fn into_factory1(self) -> DxgiFactory1 { self.0.into_factory1() }
}

impl IDxgiFactory for DxgiFactory2 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.as_factory() }
	fn into_factory(self) -> DxgiFactory { self.0.into_factory() }
}

impl IDxgiObject for DxgiFactory2 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiFactory2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactory2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiFactory1::from(v))
    }
}

