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


pub fn CreateDXGIFactory<T: IUnknown + From<UnknownWrapper>>() -> Result<T, HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory(riid: &GUID, ppFactory: *mut c_void) -> HResult;
		} 
		let mut factory_out_: Option<Unknown> = None;
		let _ret_ = CreateDXGIFactory(T::IID, transmute(&mut factory_out_));
		if _ret_.is_ok() { if let Some(factory_out_) = factory_out_ { return Ok(T::from(UnknownWrapper(factory_out_))); } }
		Err(_ret_)
	}
}

pub fn CreateDXGIFactory1<T: IUnknown + From<UnknownWrapper>>() -> Result<T, HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory1(riid: &GUID, ppFactory: *mut c_void) -> HResult;
		} 
		let mut factory_out_: Option<Unknown> = None;
		let _ret_ = CreateDXGIFactory1(T::IID, transmute(&mut factory_out_));
		if _ret_.is_ok() { if let Some(factory_out_) = factory_out_ { return Ok(T::from(UnknownWrapper(factory_out_))); } }
		Err(_ret_)
	}
}

pub fn CreateDXGIFactory2<T: IUnknown + From<UnknownWrapper>>(flags: u32) -> Result<T, HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory2(Flags: u32, riid: &GUID, ppFactory: *mut c_void) -> HResult;
		} 
		let mut factory_out_: Option<Unknown> = None;
		let _ret_ = CreateDXGIFactory2(flags, T::IID, transmute(&mut factory_out_));
		if _ret_.is_ok() { if let Some(factory_out_) = factory_out_ { return Ok(T::from(UnknownWrapper(factory_out_))); } }
		Err(_ret_)
	}
}

pub fn DXGIGetDebugInterface1<T: IUnknown + From<UnknownWrapper>>(flags: u32) -> Result<T, HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn DXGIGetDebugInterface1(Flags: u32, riid: &GUID, pDebug: *mut c_void) -> HResult;
		} 
		let mut debug_out_: Option<Unknown> = None;
		let _ret_ = DXGIGetDebugInterface1(flags, T::IID, transmute(&mut debug_out_));
		if _ret_.is_ok() { if let Some(debug_out_) = debug_out_ { return Ok(T::from(UnknownWrapper(debug_out_))); } }
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


