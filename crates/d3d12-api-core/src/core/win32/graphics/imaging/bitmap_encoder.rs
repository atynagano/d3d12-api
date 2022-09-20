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
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::imaging::*;
use crate::core::win32::system::com::structured_storage::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICBitmapEncoder(pub(crate) Unknown);

impl Deref for WICBitmapEncoder {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapEncoder {
	const IID: &'static GUID = &GUID::from_u128(0x00000103a8f24877ba0afd2b6645fb94u128);
}

impl Com for WICBitmapEncoder {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapEncoder {
	pub fn Initialize(&self, i_stream: &Stream, cache_option: WicBitmapEncoderCacheOption) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, WicBitmapEncoderCacheOption) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, i_stream.vtable(), cache_option);
			_ret_.ok()
		}
	}

	pub unsafe fn GetContainerFormat(&self) { todo!() }

	pub fn GetEncoderInfo(&self) -> Result<WICBitmapEncoderInfo, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_encoder_info_out_: Option<WICBitmapEncoderInfo> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, transmute(&mut i_encoder_info_out_));
			if _ret_.is_ok() { if let Some(i_encoder_info_out_) = i_encoder_info_out_ { return Ok(i_encoder_info_out_); } }
			Err(_ret_)
		}
	}

	pub fn SetColorContexts(&self, i_color_context: &[Param<WICColorContext>]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (i_color_context_ptr_, i_color_context_len_) = i_color_context.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Param<WICColorContext>) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, i_color_context_len_ as u32, i_color_context_ptr_);
			_ret_.ok()
		}
	}

	pub fn SetPalette(&self, i_palette: &WICPalette) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, i_palette.vtable());
			_ret_.ok()
		}
	}

	pub fn SetThumbnail(&self, i_thumbnail: &WICBitmapSource) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, i_thumbnail.vtable());
			_ret_.ok()
		}
	}

	pub fn SetPreview(&self, i_preview: &WICBitmapSource) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, i_preview.vtable());
			_ret_.ok()
		}
	}

	pub fn CreateNewFrame(&self, i_encoder_options: &mut PropertyBag2) -> Result<WICBitmapFrameEncode, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_frame_encode_out_: Option<WICBitmapFrameEncode> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void, &mut PropertyBag2) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, transmute(&mut i_frame_encode_out_), i_encoder_options);
			if _ret_.is_ok() { if let Some(i_frame_encode_out_) = i_frame_encode_out_ { return Ok(i_frame_encode_out_); } }
			Err(_ret_)
		}
	}

	pub fn Commit(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn GetMetadataQueryWriter(&self) -> Result<WICMetadataQueryWriter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_metadata_query_writer_out_: Option<WICMetadataQueryWriter> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(&mut i_metadata_query_writer_out_));
			if _ret_.is_ok() { if let Some(i_metadata_query_writer_out_) = i_metadata_query_writer_out_ { return Ok(i_metadata_query_writer_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICBitmapEncoder: IUnknown {
	fn as_bitmap_encoder(&self) -> &WICBitmapEncoder;
	fn into_bitmap_encoder(self) -> WICBitmapEncoder;
}

impl IWICBitmapEncoder for WICBitmapEncoder {
	fn as_bitmap_encoder(&self) -> &WICBitmapEncoder { self }
	fn into_bitmap_encoder(self) -> WICBitmapEncoder { self }
}
impl IUnknown for WICBitmapEncoder {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapEncoder {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

