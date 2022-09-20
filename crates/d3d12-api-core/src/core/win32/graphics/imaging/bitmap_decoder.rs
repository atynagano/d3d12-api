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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICBitmapDecoder(pub(crate) Unknown);

impl Deref for WICBitmapDecoder {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapDecoder {
	const IID: &'static GUID = &GUID::from_u128(0x9edde9e78dee47ea99dfe6faf2ed44bfu128);
}

impl Com for WICBitmapDecoder {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapDecoder {
	pub fn QueryCapability(&self, i_stream: &Stream) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pdw_capability_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, VTable, *mut u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, i_stream.vtable(), pdw_capability_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pdw_capability_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn Initialize(&self, i_stream: &Stream, cache_options: WicDecodeOptions) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, WicDecodeOptions) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, i_stream.vtable(), cache_options);
			_ret_.ok()
		}
	}

	pub unsafe fn GetContainerFormat(&self) { todo!() }

	pub fn GetDecoderInfo(&self) -> Result<WICBitmapDecoderInfo, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_decoder_info_out_: Option<WICBitmapDecoderInfo> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, transmute(&mut i_decoder_info_out_));
			if _ret_.is_ok() { if let Some(i_decoder_info_out_) = i_decoder_info_out_ { return Ok(i_decoder_info_out_); } }
			Err(_ret_)
		}
	}

	pub fn CopyPalette(&self, i_palette: &WICPalette) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, i_palette.vtable());
			_ret_.ok()
		}
	}

	pub fn GetMetadataQueryReader(&self) -> Result<WICMetadataQueryReader, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_metadata_query_reader_out_: Option<WICMetadataQueryReader> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, transmute(&mut i_metadata_query_reader_out_));
			if _ret_.is_ok() { if let Some(i_metadata_query_reader_out_) = i_metadata_query_reader_out_ { return Ok(i_metadata_query_reader_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetPreview(&self) -> Result<WICBitmapSource, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_source_out_: Option<WICBitmapSource> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transmute(&mut i_bitmap_source_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_source_out_) = i_bitmap_source_out_ { return Ok(i_bitmap_source_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetColorContexts(&self, i_color_contexts: &mut [Param<WICColorContext>]) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let (i_color_contexts_ptr_, i_color_contexts_len_) = i_color_contexts.deconstruct();
			let mut pc_actual_count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut Param<WICColorContext>, *mut u32) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, i_color_contexts_len_ as u32, i_color_contexts_ptr_, pc_actual_count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pc_actual_count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetThumbnail(&self) -> Result<WICBitmapSource, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_thumbnail_out_: Option<WICBitmapSource> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, transmute(&mut i_thumbnail_out_));
			if _ret_.is_ok() { if let Some(i_thumbnail_out_) = i_thumbnail_out_ { return Ok(i_thumbnail_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetFrameCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetFrame(&self, index: u32) -> Result<WICBitmapFrameDecode, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_frame_out_: Option<WICBitmapFrameDecode> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, index, transmute(&mut i_bitmap_frame_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_frame_out_) = i_bitmap_frame_out_ { return Ok(i_bitmap_frame_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICBitmapDecoder: IUnknown {
	fn as_bitmap_decoder(&self) -> &WICBitmapDecoder;
	fn into_bitmap_decoder(self) -> WICBitmapDecoder;
}

impl IWICBitmapDecoder for WICBitmapDecoder {
	fn as_bitmap_decoder(&self) -> &WICBitmapDecoder { self }
	fn into_bitmap_decoder(self) -> WICBitmapDecoder { self }
}
impl IUnknown for WICBitmapDecoder {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapDecoder {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

