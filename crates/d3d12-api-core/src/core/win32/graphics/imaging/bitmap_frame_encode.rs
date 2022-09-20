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
use crate::core::win32::system::com::structured_storage::*;
use crate::core::win32::graphics::imaging::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICBitmapFrameEncode(pub(crate) Unknown);

impl Deref for WICBitmapFrameEncode {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapFrameEncode {
	const IID: &'static GUID = &GUID::from_u128(0x00000105a8f24877ba0afd2b6645fb94u128);
}

impl Com for WICBitmapFrameEncode {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapFrameEncode {
	pub fn Initialize(&self, i_encoder_options: &PropertyBag2) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, i_encoder_options.vtable());
			_ret_.ok()
		}
	}

	pub fn SetSize(&self, ui_width: u32, ui_height: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, ui_width, ui_height);
			_ret_.ok()
		}
	}

	pub fn SetResolution(&self, dpi_x: f64, dpi_y: f64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f64, f64) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, dpi_x, dpi_y);
			_ret_.ok()
		}
	}

	pub unsafe fn SetPixelFormat(&self) { todo!() }

	pub fn SetColorContexts(&self, i_color_context: &[Param<WICColorContext>]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (i_color_context_ptr_, i_color_context_len_) = i_color_context.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Param<WICColorContext>) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, i_color_context_len_ as u32, i_color_context_ptr_);
			_ret_.ok()
		}
	}

	pub fn SetPalette(&self, i_palette: &WICPalette) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, i_palette.vtable());
			_ret_.ok()
		}
	}

	pub fn SetThumbnail(&self, i_thumbnail: &WICBitmapSource) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, i_thumbnail.vtable());
			_ret_.ok()
		}
	}

	pub fn WritePixels(&self, line_count: u32, stride: u32, pb_pixels: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (pb_pixels_ptr_, pb_pixels_len_) = pb_pixels.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, u32, u32, *const u8) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, line_count, stride, pb_pixels_len_ as u32, pb_pixels_ptr_);
			_ret_.ok()
		}
	}

	pub fn WriteSource(&self, i_bitmap_source: &WICBitmapSource, prc: &WicRect) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, &WicRect) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, i_bitmap_source.vtable(), prc);
			_ret_.ok()
		}
	}

	pub fn Commit(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn GetMetadataQueryWriter(&self) -> Result<WICMetadataQueryWriter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_metadata_query_writer_out_: Option<WICMetadataQueryWriter> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(&mut i_metadata_query_writer_out_));
			if _ret_.is_ok() { if let Some(i_metadata_query_writer_out_) = i_metadata_query_writer_out_ { return Ok(i_metadata_query_writer_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICBitmapFrameEncode: IUnknown {
	fn as_bitmap_frame_encode(&self) -> &WICBitmapFrameEncode;
	fn into_bitmap_frame_encode(self) -> WICBitmapFrameEncode;
}

impl IWICBitmapFrameEncode for WICBitmapFrameEncode {
	fn as_bitmap_frame_encode(&self) -> &WICBitmapFrameEncode { self }
	fn into_bitmap_frame_encode(self) -> WICBitmapFrameEncode { self }
}
impl IUnknown for WICBitmapFrameEncode {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapFrameEncode {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

