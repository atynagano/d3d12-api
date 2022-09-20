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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiFactory(pub(crate) DxgiObject);

impl Deref for DxgiFactory {
	type Target = DxgiObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactory {
	const IID: &'static GUID = &GUID::from_u128(0x7b7166ec21c744aeb21ac9ae321ae369u128);
}

impl Com for DxgiFactory {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactory {
	pub fn EnumAdapters(&self, adapter: u32) -> Result<DxgiAdapter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut adapter_out_: Option<DxgiAdapter> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, adapter, transmute(&mut adapter_out_));
			if _ret_.is_ok() { if let Some(adapter_out_) = adapter_out_ { return Ok(adapter_out_); } }
			Err(_ret_)
		}
	}

	pub fn MakeWindowAssociation(&self, window_handle: HWnd, flags: DxgiMwa, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, window_handle: HWnd, flags: DxgiMwa, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, window_handle, flags, );
			_ret_.ok()
		}
	}

	pub fn GetWindowAssociation(&self) -> Result<HWnd, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut window_handle_out_: Option<HWnd> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transmute(&mut window_handle_out_));
			if _ret_.is_ok() { if let Some(window_handle_out_) = window_handle_out_ { return Ok(window_handle_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateSwapChain(&self, device: &Unknown, desc: &DxgiSwapChainDesc) -> Result<DxgiSwapChain, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut swap_chain_out_: Option<DxgiSwapChain> = None;
			let f: extern "system" fn(Param<Self>, VTable, &DxgiSwapChainDesc, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, device.vtable(), desc, transmute(&mut swap_chain_out_));
			if _ret_.is_ok() { if let Some(swap_chain_out_) = swap_chain_out_ { return Ok(swap_chain_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateSoftwareAdapter(&self, module: HInstance) -> Result<DxgiAdapter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut adapter_out_: Option<DxgiAdapter> = None;
			let f: extern "system" fn(Param<Self>, HInstance, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, module, transmute(&mut adapter_out_));
			if _ret_.is_ok() { if let Some(adapter_out_) = adapter_out_ { return Ok(adapter_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiFactory: IDxgiObject {
	fn as_factory(&self) -> &DxgiFactory;
	fn into_factory(self) -> DxgiFactory;
}

impl IDxgiFactory for DxgiFactory {
	fn as_factory(&self) -> &DxgiFactory { self }
	fn into_factory(self) -> DxgiFactory { self }
}
impl IDxgiObject for DxgiFactory {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiFactory {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactory {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiObject::from(v))
    }
}

