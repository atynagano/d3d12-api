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
pub struct DxgiFactoryMedia(pub(crate) Unknown);

impl Deref for DxgiFactoryMedia {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactoryMedia {
	const IID: &'static GUID = &GUID::from_u128(0x41e7d1f2a5914f7ba2e5fa9c843e1c12u128);
}

impl Com for DxgiFactoryMedia {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactoryMedia {
	pub fn CreateSwapChainForCompositionSurfaceHandle(&self, device: &Unknown, surface: Option<Handle>, desc: &DxgiSwapChainDesc1, restrict_to_output: Option<&DxgiOutput>) -> Result<DxgiSwapChain1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut swap_chain_out_: Option<DxgiSwapChain1> = None;
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, &DxgiSwapChainDesc1, *const c_void, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, device.vtable(), transmute(surface), desc, option_to_vtable(restrict_to_output), transmute(&mut swap_chain_out_));
			if _ret_.is_ok() { if let Some(swap_chain_out_) = swap_chain_out_ { return Ok(swap_chain_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDecodeSwapChainForCompositionSurfaceHandle(&self, device: &Unknown, surface: Option<Handle>, desc: &DxgiDecodeSwapChainDesc, yuv_decode_buffers: &DxgiResource, restrict_to_output: Option<&DxgiOutput>) -> Result<DxgiDecodeSwapChain, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut swap_chain_out_: Option<DxgiDecodeSwapChain> = None;
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, &DxgiDecodeSwapChainDesc, VTable, *const c_void, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, device.vtable(), transmute(surface), desc, yuv_decode_buffers.vtable(), option_to_vtable(restrict_to_output), transmute(&mut swap_chain_out_));
			if _ret_.is_ok() { if let Some(swap_chain_out_) = swap_chain_out_ { return Ok(swap_chain_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiFactoryMedia: IUnknown {
	fn as_factory_media(&self) -> &DxgiFactoryMedia;
	fn into_factory_media(self) -> DxgiFactoryMedia;
}

impl IDxgiFactoryMedia for DxgiFactoryMedia {
	fn as_factory_media(&self) -> &DxgiFactoryMedia { self }
	fn into_factory_media(self) -> DxgiFactoryMedia { self }
}
impl IUnknown for DxgiFactoryMedia {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactoryMedia {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

