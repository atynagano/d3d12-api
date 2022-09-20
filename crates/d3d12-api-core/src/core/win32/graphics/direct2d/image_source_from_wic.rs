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
use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::graphics::imaging::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1ImageSourceFromWic(pub(crate) D2D1ImageSource);

impl Deref for D2D1ImageSourceFromWic {
	type Target = D2D1ImageSource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1ImageSourceFromWic {
	const IID: &'static GUID = &GUID::from_u128(0x773954411c8f45558683f50dab0fe792u128);
}

impl Com for D2D1ImageSourceFromWic {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1ImageSourceFromWic {
	pub fn EnsureCached(&self, rectangle_to_fill: Option<&D2DRectU>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, transmute(rectangle_to_fill));
			_ret_.ok()
		}
	}

	pub fn TrimCache(&self, rectangle_to_preserve: Option<&D2DRectU>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, transmute(rectangle_to_preserve));
			_ret_.ok()
		}
	}

	pub fn GetSource(&self, mut wic_bitmap_source: Option<&mut Option<WICBitmapSource>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut wic_bitmap_source) = wic_bitmap_source { **wic_bitmap_source = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, transmute(wic_bitmap_source));
		}
	}
}

pub trait ID2D1ImageSourceFromWic: ID2D1ImageSource {
	fn as_image_source_from_wic(&self) -> &D2D1ImageSourceFromWic;
	fn into_image_source_from_wic(self) -> D2D1ImageSourceFromWic;
}

impl ID2D1ImageSourceFromWic for D2D1ImageSourceFromWic {
	fn as_image_source_from_wic(&self) -> &D2D1ImageSourceFromWic { self }
	fn into_image_source_from_wic(self) -> D2D1ImageSourceFromWic { self }
}
impl ID2D1ImageSource for D2D1ImageSourceFromWic {
	fn as_image_source(&self) -> &D2D1ImageSource { &self.0.as_image_source() }
	fn into_image_source(self) -> D2D1ImageSource { self.0.into_image_source() }
}

impl ID2D1Image for D2D1ImageSourceFromWic {
	fn as_image(&self) -> &D2D1Image { &self.0.as_image() }
	fn into_image(self) -> D2D1Image { self.0.into_image() }
}

impl ID2D1Resource for D2D1ImageSourceFromWic {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1ImageSourceFromWic {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1ImageSourceFromWic {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1ImageSource::from(v))
    }
}

