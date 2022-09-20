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

use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Bitmap(pub(crate) D2D1Image);

impl Deref for D2D1Bitmap {
	type Target = D2D1Image;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Bitmap {
	const IID: &'static GUID = &GUID::from_u128(0xa2296057ea424099983b539fb6505426u128);
}

impl Com for D2D1Bitmap {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Bitmap {
	pub fn GetSize(&self) -> D2DSizeF {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DSizeF
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetPixelSize(&self) -> D2DSizeU {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DSizeU
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetPixelFormat(&self) -> D2D1PixelFormat {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1PixelFormat
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetDpi(&self) -> (f32, f32) {
		unsafe {
			let vt = self.as_param();
			let mut dpi_x_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut dpi_y_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut f32, *mut f32) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, dpi_x_out_.as_mut_ptr(), dpi_y_out_.as_mut_ptr());
			(dpi_x_out_.assume_init(), dpi_y_out_.assume_init())
		}
	}

	pub fn CopyFromBitmap(&self, dest_point: Option<&D2DPoint2U>, bitmap: &D2D1Bitmap, src_rect: Option<&D2DRectU>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, VTable, *const c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, transmute(dest_point), bitmap.vtable(), transmute(src_rect));
			_ret_.ok()
		}
	}

	pub fn CopyFromRenderTarget(&self, dest_point: Option<&D2DPoint2U>, render_target: &D2D1RenderTarget, src_rect: Option<&D2DRectU>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, VTable, *const c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transmute(dest_point), render_target.vtable(), transmute(src_rect));
			_ret_.ok()
		}
	}

	pub fn CopyFromMemory(&self, dst_rect: Option<&D2DRectU>, src_data: *const impl Sized, pitch: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, u32) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, transmute(dst_rect), src_data as _, pitch);
			_ret_.ok()
		}
	}
}

pub trait ID2D1Bitmap: ID2D1Image {
	fn as_bitmap(&self) -> &D2D1Bitmap;
	fn into_bitmap(self) -> D2D1Bitmap;
}

impl ID2D1Bitmap for D2D1Bitmap {
	fn as_bitmap(&self) -> &D2D1Bitmap { self }
	fn into_bitmap(self) -> D2D1Bitmap { self }
}
impl ID2D1Image for D2D1Bitmap {
	fn as_image(&self) -> &D2D1Image { &self.0.as_image() }
	fn into_image(self) -> D2D1Image { self.0.into_image() }
}

impl ID2D1Resource for D2D1Bitmap {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Bitmap {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Bitmap {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Image::from(v))
    }
}

