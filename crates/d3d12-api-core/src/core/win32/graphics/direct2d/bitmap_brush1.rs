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
pub struct D2D1BitmapBrush1(pub(crate) D2D1BitmapBrush);

impl Deref for D2D1BitmapBrush1 {
	type Target = D2D1BitmapBrush;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1BitmapBrush1 {
	const IID: &'static GUID = &GUID::from_u128(0x41343a53e41a49a291cd21793bbb62e5u128);
}

impl Com for D2D1BitmapBrush1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1BitmapBrush1 {
	pub fn SetInterpolationMode1(&self, interpolation_mode: D2D1InterpolationMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1InterpolationMode) -> ()
				= transmute(vt[16]);
			let _ret_ = f(vt, interpolation_mode);
		}
	}

	pub fn GetInterpolationMode1(&self) -> D2D1InterpolationMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1InterpolationMode
				= transmute(vt[17]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1BitmapBrush1: ID2D1BitmapBrush {
	fn as_bitmap_brush1(&self) -> &D2D1BitmapBrush1;
	fn into_bitmap_brush1(self) -> D2D1BitmapBrush1;
}

impl ID2D1BitmapBrush1 for D2D1BitmapBrush1 {
	fn as_bitmap_brush1(&self) -> &D2D1BitmapBrush1 { self }
	fn into_bitmap_brush1(self) -> D2D1BitmapBrush1 { self }
}
impl ID2D1BitmapBrush for D2D1BitmapBrush1 {
	fn as_bitmap_brush(&self) -> &D2D1BitmapBrush { &self.0.as_bitmap_brush() }
	fn into_bitmap_brush(self) -> D2D1BitmapBrush { self.0.into_bitmap_brush() }
}

impl ID2D1Brush for D2D1BitmapBrush1 {
	fn as_brush(&self) -> &D2D1Brush { &self.0.as_brush() }
	fn into_brush(self) -> D2D1Brush { self.0.into_brush() }
}

impl ID2D1Resource for D2D1BitmapBrush1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1BitmapBrush1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1BitmapBrush1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1BitmapBrush::from(v))
    }
}

