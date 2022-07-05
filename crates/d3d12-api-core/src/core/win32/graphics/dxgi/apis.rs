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


pub fn CreateDXGIFactory<T: IUnknown>() -> Result<T, HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory(riid: &GUID, _out_factory: *mut c_void, ) -> HResult;
		}
		let mut _out_factory: Option<Unknown> = None;
		let _ret_ = CreateDXGIFactory(T::IID, transmute(&mut _out_factory), );
		if _ret_.is_ok() {
			if let Some(_out_factory) = _out_factory {
				return Ok(T::from(_out_factory));
			}
		}
		Err(_ret_)
	}
}

pub fn CreateDXGIFactory1<T: IUnknown>() -> Result<T, HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory1(riid: &GUID, _out_factory: *mut c_void, ) -> HResult;
		}
		let mut _out_factory: Option<Unknown> = None;
		let _ret_ = CreateDXGIFactory1(T::IID, transmute(&mut _out_factory), );
		if _ret_.is_ok() {
			if let Some(_out_factory) = _out_factory {
				return Ok(T::from(_out_factory));
			}
		}
		Err(_ret_)
	}
}

pub fn CreateDXGIFactory2<T: IUnknown>(flags: u32, ) -> Result<T, HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory2(flags: u32, riid: &GUID, _out_factory: *mut c_void, ) -> HResult;
		}
		let mut _out_factory: Option<Unknown> = None;
		let _ret_ = CreateDXGIFactory2(flags, T::IID, transmute(&mut _out_factory), );
		if _ret_.is_ok() {
			if let Some(_out_factory) = _out_factory {
				return Ok(T::from(_out_factory));
			}
		}
		Err(_ret_)
	}
}

pub fn DXGIGetDebugInterface1<T: IUnknown>(flags: u32, ) -> Result<T, HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn DXGIGetDebugInterface1(flags: u32, riid: &GUID, _out_debug: *mut c_void, ) -> HResult;
		}
		let mut _out_debug: Option<Unknown> = None;
		let _ret_ = DXGIGetDebugInterface1(flags, T::IID, transmute(&mut _out_debug), );
		if _ret_.is_ok() {
			if let Some(_out_debug) = _out_debug {
				return Ok(T::from(_out_debug));
			}
		}
		Err(_ret_)
	}
}

pub fn DXGIDeclareAdapterRemovalSupport() -> Result<(), HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn DXGIDeclareAdapterRemovalSupport() -> HResult;
		}
		let _ret_ = DXGIDeclareAdapterRemovalSupport();
		_ret_.ok()
	}
}


