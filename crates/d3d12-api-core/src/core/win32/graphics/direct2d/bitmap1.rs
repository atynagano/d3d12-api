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

use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Bitmap1(pub(crate) D2D1Bitmap);

impl Deref for D2D1Bitmap1 {
	type Target = D2D1Bitmap;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Bitmap1 {
	const IID: &'static GUID = &GUID::from_u128(0xa898a84c38734588b08bebbf978df041u128);
}

impl Com for D2D1Bitmap1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Bitmap1 {
	pub fn GetColorContext(&self, mut color_context: Option<&mut Option<D2D1ColorContext>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut color_context) = color_context { **color_context = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, transmute(color_context));
		}
	}

	pub fn GetOptions(&self) -> D2D1BitmapOptions {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1BitmapOptions
				= transmute(vt[12]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetSurface(&self, mut dxgi_surface: Option<&mut Option<DxgiSurface>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut dxgi_surface) = dxgi_surface { **dxgi_surface = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(dxgi_surface));
			_ret_.ok()
		}
	}

	pub fn get_surface(&self) -> Result<DxgiSurface, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut dxgi_surface_out_: Option<DxgiSurface> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(&mut dxgi_surface_out_));
			if _ret_.is_ok() { if let Some(dxgi_surface_out_) = dxgi_surface_out_ { return Ok(dxgi_surface_out_); } }
			Err(_ret_)
		}
	}

	pub fn Map(&self, options: D2D1MapOptions) -> Result<D2D1MappedRect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut mapped_rect_out_: MaybeUninit<D2D1MappedRect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, D2D1MapOptions, *mut D2D1MappedRect) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, options, mapped_rect_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(mapped_rect_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn Unmap(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait ID2D1Bitmap1: ID2D1Bitmap {
	fn as_bitmap1(&self) -> &D2D1Bitmap1;
	fn into_bitmap1(self) -> D2D1Bitmap1;
}

impl ID2D1Bitmap1 for D2D1Bitmap1 {
	fn as_bitmap1(&self) -> &D2D1Bitmap1 { self }
	fn into_bitmap1(self) -> D2D1Bitmap1 { self }
}
impl ID2D1Bitmap for D2D1Bitmap1 {
	fn as_bitmap(&self) -> &D2D1Bitmap { &self.0.as_bitmap() }
	fn into_bitmap(self) -> D2D1Bitmap { self.0.into_bitmap() }
}

impl ID2D1Image for D2D1Bitmap1 {
	fn as_image(&self) -> &D2D1Image { &self.0.as_image() }
	fn into_image(self) -> D2D1Image { self.0.into_image() }
}

impl ID2D1Resource for D2D1Bitmap1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Bitmap1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Bitmap1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Bitmap::from(v))
    }
}

