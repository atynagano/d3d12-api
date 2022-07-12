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
use crate::core::win32::system::com::*;

#[repr(C)]
pub struct DxgiFactory(pub(crate) DxgiObject);

impl Guid for DxgiFactory {
	const IID: &'static GUID = &GUID::from_u128(0x7b7166ec21c744aeb21ac9ae321ae369u128);
}

impl Clone for DxgiFactory {
	fn clone(&self) -> Self { DxgiFactory(self.0.clone()) }
}

pub trait IDxgiFactory: IDxgiObject {
	fn as_factory(&self) -> &DxgiFactory;
	fn into_factory(self) -> DxgiFactory;

	fn EnumAdapters(&self, adapter: u32, ) -> Result<DxgiAdapter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_adapter: Option<DxgiAdapter> = None;
			let f: extern "system" fn(Param<Self>, adapter: u32, adapter: *mut c_void, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, adapter, transmute(&mut _out_adapter), );
			if _ret_.is_ok() {
				if let Some(_out_adapter) = _out_adapter {
					return Ok(_out_adapter);
				}
			}
			Err(_ret_)
		}
	}

	fn MakeWindowAssociation(&self, window_handle: HWnd, flags: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, window_handle: HWnd, flags: u32, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, window_handle, flags, );
			_ret_.ok()
		}
	}

	fn GetWindowAssociation(&self, ) -> Result<HWnd, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_window_handle: Option<HWnd> = None;
			let f: extern "system" fn(Param<Self>, _out_window_handle: *mut c_void, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transmute(&mut _out_window_handle), );
			if _ret_.is_ok() {
				if let Some(_out_window_handle) = _out_window_handle {
					return Ok(_out_window_handle);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateSwapChain(&self, device: &(impl IUnknown + ?Sized), desc: &DxgiSwapChainDesc, ) -> Result<DxgiSwapChain, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_swap_chain: Option<DxgiSwapChain> = None;
			let f: extern "system" fn(Param<Self>, device: VTable, desc: &DxgiSwapChainDesc, swap_chain: *mut c_void, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, device.vtable(), desc, transmute(&mut _out_swap_chain), );
			if _ret_.is_ok() {
				if let Some(_out_swap_chain) = _out_swap_chain {
					return Ok(_out_swap_chain);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateSoftwareAdapter(&self, module: HInstance, ) -> Result<DxgiAdapter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_adapter: Option<DxgiAdapter> = None;
			let f: extern "system" fn(Param<Self>, module: HInstance, adapter: *mut c_void, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, module, transmute(&mut _out_adapter), );
			if _ret_.is_ok() {
				if let Some(_out_adapter) = _out_adapter {
					return Ok(_out_adapter);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxgiFactory for DxgiFactory {
	fn as_factory(&self) -> &DxgiFactory { self }
	fn into_factory(self) -> DxgiFactory { self }
}

impl IDxgiObject for DxgiFactory {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiFactory {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiFactory {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

