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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1BitmapBrush(pub(crate) D2D1Brush);

impl Deref for D2D1BitmapBrush {
	type Target = D2D1Brush;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1BitmapBrush {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906aa12e211dc9fed001143a055f9u128);
}

impl Com for D2D1BitmapBrush {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1BitmapBrush {
	pub fn SetExtendModeX(&self, extend_mode_x: D2D1ExtendMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1ExtendMode) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, extend_mode_x);
		}
	}

	pub fn SetExtendModeY(&self, extend_mode_y: D2D1ExtendMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1ExtendMode) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, extend_mode_y);
		}
	}

	pub fn SetInterpolationMode(&self, interpolation_mode: D2D1BitmapInterpolationMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1BitmapInterpolationMode) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, interpolation_mode);
		}
	}

	pub fn SetBitmap(&self, bitmap: Option<&D2D1Bitmap>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, option_to_vtable(bitmap));
		}
	}

	pub fn GetExtendModeX(&self) -> D2D1ExtendMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ExtendMode
				= transmute(vt[12]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetExtendModeY(&self) -> D2D1ExtendMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ExtendMode
				= transmute(vt[13]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetInterpolationMode(&self) -> D2D1BitmapInterpolationMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1BitmapInterpolationMode
				= transmute(vt[14]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetBitmap(&self, mut bitmap: Option<&mut Option<D2D1Bitmap>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut bitmap) = bitmap { **bitmap = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[15]);
			let _ret_ = f(vt, transmute(bitmap));
		}
	}
}

pub trait ID2D1BitmapBrush: ID2D1Brush {
	fn as_bitmap_brush(&self) -> &D2D1BitmapBrush;
	fn into_bitmap_brush(self) -> D2D1BitmapBrush;
}

impl ID2D1BitmapBrush for D2D1BitmapBrush {
	fn as_bitmap_brush(&self) -> &D2D1BitmapBrush { self }
	fn into_bitmap_brush(self) -> D2D1BitmapBrush { self }
}
impl ID2D1Brush for D2D1BitmapBrush {
	fn as_brush(&self) -> &D2D1Brush { &self.0.as_brush() }
	fn into_brush(self) -> D2D1Brush { self.0.into_brush() }
}

impl ID2D1Resource for D2D1BitmapBrush {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1BitmapBrush {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1BitmapBrush {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Brush::from(v))
    }
}

