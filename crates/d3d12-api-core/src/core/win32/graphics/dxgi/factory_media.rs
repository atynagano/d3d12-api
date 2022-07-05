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
pub struct DxgiFactoryMedia(pub(crate) Unknown);

impl Guid for DxgiFactoryMedia {
	const IID: &'static GUID = &GUID::from_u128(0x41e7d1f2a5914f7ba2e5fa9c843e1c12u128);
}

impl Clone for DxgiFactoryMedia {
	fn clone(&self) -> Self { DxgiFactoryMedia(self.0.clone()) }
}

pub trait IDxgiFactoryMedia: IUnknown {
	fn as_factory_media(&self) -> &DxgiFactoryMedia;
	fn into_factory_media(self) -> DxgiFactoryMedia;

	fn CreateSwapChainForCompositionSurfaceHandle(&self, device: &(impl IUnknown + ?Sized), surface: Option<Handle>, desc: &DxgiSwapChainDesc1, restrict_to_output: Option<&DxgiOutput>, ) -> Result<DxgiSwapChain1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_swap_chain: Option<DxgiSwapChain1> = None;
			let f: extern "system" fn(Param<Self>, device: VTable, surface: *const c_void, desc: &DxgiSwapChainDesc1, restrict_to_output: *const c_void, swap_chain: *mut c_void, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, device.vtable(), transmute(surface), desc, option_to_vtable(restrict_to_output), transmute(&mut _out_swap_chain), );
			if _ret_.is_ok() {
				if let Some(_out_swap_chain) = _out_swap_chain {
					return Ok(_out_swap_chain);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateDecodeSwapChainForCompositionSurfaceHandle(&self, device: &(impl IUnknown + ?Sized), surface: Option<Handle>, desc: &DxgiDecodeSwapChainDesc, yuv_decode_buffers: &(impl IDxgiResource + ?Sized), restrict_to_output: Option<&DxgiOutput>, ) -> Result<DxgiDecodeSwapChain, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_swap_chain: Option<DxgiDecodeSwapChain> = None;
			let f: extern "system" fn(Param<Self>, device: VTable, surface: *const c_void, desc: &DxgiDecodeSwapChainDesc, yuv_decode_buffers: VTable, restrict_to_output: *const c_void, swap_chain: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, device.vtable(), transmute(surface), desc, yuv_decode_buffers.vtable(), option_to_vtable(restrict_to_output), transmute(&mut _out_swap_chain), );
			if _ret_.is_ok() {
				if let Some(_out_swap_chain) = _out_swap_chain {
					return Ok(_out_swap_chain);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxgiFactoryMedia for DxgiFactoryMedia {
	fn as_factory_media(&self) -> &DxgiFactoryMedia { self }
	fn into_factory_media(self) -> DxgiFactoryMedia { self }
}

impl From<Unknown> for DxgiFactoryMedia {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxgiFactoryMedia {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

