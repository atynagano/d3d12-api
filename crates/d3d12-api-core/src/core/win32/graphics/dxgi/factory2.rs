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
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
pub struct DxgiFactory2(pub(crate) DxgiFactory1);

impl Guid for DxgiFactory2 {
	const IID: &'static GUID = &GUID::from_u128(0x50c83a1ce0724c4887b03630fa36a6d0u128);
}

impl Clone for DxgiFactory2 {
	fn clone(&self) -> Self { DxgiFactory2(self.0.clone()) }
}

pub trait IDxgiFactory2: IDxgiFactory1 {
	fn as_factory2(&self) -> &DxgiFactory2;
	fn into_factory2(self) -> DxgiFactory2;

	fn IsWindowedStereoEnabled(&self, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Bool
				= transmute(vt[14]);
			let _ret_ = f(vt, );
			_ret_.to_bool()
		}
	}

	fn CreateSwapChainForHwnd(&self, device: &(impl IUnknown + ?Sized), wnd: HWnd, desc: &DxgiSwapChainDesc1, fullscreen_desc: Option<&DxgiSwapChainFullScreenDesc>, restrict_to_output: Option<&DxgiOutput>, ) -> Result<DxgiSwapChain1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_swap_chain: Option<DxgiSwapChain1> = None;
			let f: extern "system" fn(Param<Self>, device: VTable, wnd: HWnd, desc: &DxgiSwapChainDesc1, fullscreen_desc: *const c_void, restrict_to_output: *const c_void, swap_chain: *mut c_void, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, device.vtable(), wnd, desc, transmute(fullscreen_desc), option_to_vtable(restrict_to_output), transmute(&mut _out_swap_chain), );
			if _ret_.is_ok() {
				if let Some(_out_swap_chain) = _out_swap_chain {
					return Ok(_out_swap_chain);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateSwapChainForCoreWindow(&self, device: &(impl IUnknown + ?Sized), window: &(impl IUnknown + ?Sized), desc: &DxgiSwapChainDesc1, restrict_to_output: Option<&DxgiOutput>, ) -> Result<DxgiSwapChain1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_swap_chain: Option<DxgiSwapChain1> = None;
			let f: extern "system" fn(Param<Self>, device: VTable, window: VTable, desc: &DxgiSwapChainDesc1, restrict_to_output: *const c_void, swap_chain: *mut c_void, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, device.vtable(), window.vtable(), desc, option_to_vtable(restrict_to_output), transmute(&mut _out_swap_chain), );
			if _ret_.is_ok() {
				if let Some(_out_swap_chain) = _out_swap_chain {
					return Ok(_out_swap_chain);
				}
			}
			Err(_ret_)
		}
	}

	fn GetSharedResourceAdapterLuid(&self, resource: Handle, ) -> Result<Luid, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_luid: MaybeUninit<Luid> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, resource: Handle, _out_luid: *mut Luid, ) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, resource, _out_luid.as_mut_ptr(), );
			Ok(_out_luid.assume_init())
		}
	}

	fn RegisterStereoStatusWindow(&self, window_handle: HWnd, msg: u32, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pdw_cookie: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, window_handle: HWnd, msg: u32, _out_pdw_cookie: *mut u32, ) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, window_handle, msg, _out_pdw_cookie.as_mut_ptr(), );
			Ok(_out_pdw_cookie.assume_init())
		}
	}

	fn RegisterStereoStatusEvent(&self, event: Handle, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pdw_cookie: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, event: Handle, _out_pdw_cookie: *mut u32, ) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, event, _out_pdw_cookie.as_mut_ptr(), );
			Ok(_out_pdw_cookie.assume_init())
		}
	}

	fn UnregisterStereoStatus(&self, cookie: u32, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, cookie: u32, ) -> ()
				= transmute(vt[20]);
			let _ret_ = f(vt, cookie, );
		}
	}

	fn RegisterOcclusionStatusWindow(&self, window_handle: HWnd, msg: u32, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pdw_cookie: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, window_handle: HWnd, msg: u32, _out_pdw_cookie: *mut u32, ) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, window_handle, msg, _out_pdw_cookie.as_mut_ptr(), );
			Ok(_out_pdw_cookie.assume_init())
		}
	}

	fn RegisterOcclusionStatusEvent(&self, event: Handle, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pdw_cookie: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, event: Handle, _out_pdw_cookie: *mut u32, ) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, event, _out_pdw_cookie.as_mut_ptr(), );
			Ok(_out_pdw_cookie.assume_init())
		}
	}

	fn UnregisterOcclusionStatus(&self, cookie: u32, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, cookie: u32, ) -> ()
				= transmute(vt[23]);
			let _ret_ = f(vt, cookie, );
		}
	}

	fn CreateSwapChainForComposition(&self, device: &(impl IUnknown + ?Sized), desc: &DxgiSwapChainDesc1, restrict_to_output: Option<&DxgiOutput>, ) -> Result<DxgiSwapChain1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_swap_chain: Option<DxgiSwapChain1> = None;
			let f: extern "system" fn(Param<Self>, device: VTable, desc: &DxgiSwapChainDesc1, restrict_to_output: *const c_void, swap_chain: *mut c_void, ) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, device.vtable(), desc, option_to_vtable(restrict_to_output), transmute(&mut _out_swap_chain), );
			if _ret_.is_ok() {
				if let Some(_out_swap_chain) = _out_swap_chain {
					return Ok(_out_swap_chain);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxgiFactory2 for DxgiFactory2 {
	fn as_factory2(&self) -> &DxgiFactory2 { self }
	fn into_factory2(self) -> DxgiFactory2 { self }
}

impl IDxgiFactory1 for DxgiFactory2 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0 }
	fn into_factory1(self) -> DxgiFactory1 { self.0 }
}

impl IDxgiFactory for DxgiFactory2 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.0 }
	fn into_factory(self) -> DxgiFactory { self.0.0 }
}

impl IDxgiObject for DxgiFactory2 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0 }
}

impl From<Unknown> for DxgiFactory2 {
    fn from(v: Unknown) -> Self {
        Self(DxgiFactory1::from(v))
    }
}

impl IUnknown for DxgiFactory2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

