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
use crate::core::win32::graphics::imaging::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICBitmapFrameDecode(pub(crate) WICBitmapSource);

impl Deref for WICBitmapFrameDecode {
	type Target = WICBitmapSource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapFrameDecode {
	const IID: &'static GUID = &GUID::from_u128(0x3b16811b6a434ec9a8133d930c13b940u128);
}

impl Com for WICBitmapFrameDecode {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapFrameDecode {
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

	pub fn GetColorContexts(&self, i_color_contexts: &mut [Param<WICColorContext>]) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let (i_color_contexts_ptr_, i_color_contexts_len_) = i_color_contexts.deconstruct();
			let mut pc_actual_count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut Param<WICColorContext>, *mut u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, i_color_contexts_len_ as u32, i_color_contexts_ptr_, pc_actual_count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pc_actual_count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetThumbnail(&self) -> Result<WICBitmapSource, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_thumbnail_out_: Option<WICBitmapSource> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, transmute(&mut i_thumbnail_out_));
			if _ret_.is_ok() { if let Some(i_thumbnail_out_) = i_thumbnail_out_ { return Ok(i_thumbnail_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICBitmapFrameDecode: IWICBitmapSource {
	fn as_bitmap_frame_decode(&self) -> &WICBitmapFrameDecode;
	fn into_bitmap_frame_decode(self) -> WICBitmapFrameDecode;
}

impl IWICBitmapFrameDecode for WICBitmapFrameDecode {
	fn as_bitmap_frame_decode(&self) -> &WICBitmapFrameDecode { self }
	fn into_bitmap_frame_decode(self) -> WICBitmapFrameDecode { self }
}
impl IWICBitmapSource for WICBitmapFrameDecode {
	fn as_bitmap_source(&self) -> &WICBitmapSource { &self.0.as_bitmap_source() }
	fn into_bitmap_source(self) -> WICBitmapSource { self.0.into_bitmap_source() }
}

impl IUnknown for WICBitmapFrameDecode {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapFrameDecode {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapSource::from(v))
    }
}

