#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;

pub fn CreateDXGIFactory<T: IUnknown>() -> Result<(T), HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory(riid: &GUID, _factory: &mut Option<Unknown>, ) -> HResult;
		}
		let mut _factory: Option<Unknown> = None;
		let ret = CreateDXGIFactory(T::IID, &mut _factory, );
		if ret.is_ok() {
			if let (Some(_factory)) = (_factory) {
				return Ok((T::from(_factory)));
			}
		}
		Err(ret)
	}
}

pub fn CreateDXGIFactory1<T: IUnknown>() -> Result<(T), HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory1(riid: &GUID, _factory: &mut Option<Unknown>, ) -> HResult;
		}
		let mut _factory: Option<Unknown> = None;
		let ret = CreateDXGIFactory1(T::IID, &mut _factory, );
		if ret.is_ok() {
			if let (Some(_factory)) = (_factory) {
				return Ok((T::from(_factory)));
			}
		}
		Err(ret)
	}
}

pub fn CreateDXGIFactory2<T: IUnknown>(flags: u32, ) -> Result<(T), HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn CreateDXGIFactory2(flags: u32, riid: &GUID, _factory: &mut Option<Unknown>, ) -> HResult;
		}
		let mut _factory: Option<Unknown> = None;
		let ret = CreateDXGIFactory2(flags, T::IID, &mut _factory, );
		if ret.is_ok() {
			if let (Some(_factory)) = (_factory) {
				return Ok((T::from(_factory)));
			}
		}
		Err(ret)
	}
}

pub fn DXGIGetDebugInterface1<T: IUnknown>(flags: u32, ) -> Result<(T), HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn DXGIGetDebugInterface1(flags: u32, riid: &GUID, _debug: &mut Option<Unknown>, ) -> HResult;
		}
		let mut _debug: Option<Unknown> = None;
		let ret = DXGIGetDebugInterface1(flags, T::IID, &mut _debug, );
		if ret.is_ok() {
			if let (Some(_debug)) = (_debug) {
				return Ok((T::from(_debug)));
			}
		}
		Err(ret)
	}
}

pub fn DXGIDeclareAdapterRemovalSupport() -> Result<(), HResult> {
	unsafe {
		#[link(name = "dxgi")]
		extern "system" {
			fn DXGIDeclareAdapterRemovalSupport() -> HResult;
		}
		let ret = DXGIDeclareAdapterRemovalSupport();
		ret.ok()
	}
}


