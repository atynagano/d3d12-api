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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICBitmapDecoderInfo(pub(crate) WICBitmapCodecInfo);

impl Deref for WICBitmapDecoderInfo {
	type Target = WICBitmapCodecInfo;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapDecoderInfo {
	const IID: &'static GUID = &GUID::from_u128(0xd8cd007fd08f41919bfc236ea7f0e4b5u128);
}

impl Com for WICBitmapDecoderInfo {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapDecoderInfo {
	pub unsafe fn GetPatterns(&self) { todo!() }

	pub fn MatchesPattern(&self, i_stream: &Stream) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_matches_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, VTable, &mut Bool) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, i_stream.vtable(), &mut pf_matches_out_);
			if _ret_.is_ok() { Ok(pf_matches_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn CreateInstance(&self) -> Result<WICBitmapDecoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_decoder_out_: Option<WICBitmapDecoder> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, transmute(&mut i_bitmap_decoder_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_decoder_out_) = i_bitmap_decoder_out_ { return Ok(i_bitmap_decoder_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICBitmapDecoderInfo: IWICBitmapCodecInfo {
	fn as_bitmap_decoder_info(&self) -> &WICBitmapDecoderInfo;
	fn into_bitmap_decoder_info(self) -> WICBitmapDecoderInfo;
}

impl IWICBitmapDecoderInfo for WICBitmapDecoderInfo {
	fn as_bitmap_decoder_info(&self) -> &WICBitmapDecoderInfo { self }
	fn into_bitmap_decoder_info(self) -> WICBitmapDecoderInfo { self }
}
impl IWICBitmapCodecInfo for WICBitmapDecoderInfo {
	fn as_bitmap_codec_info(&self) -> &WICBitmapCodecInfo { &self.0.as_bitmap_codec_info() }
	fn into_bitmap_codec_info(self) -> WICBitmapCodecInfo { self.0.into_bitmap_codec_info() }
}

impl IWICComponentInfo for WICBitmapDecoderInfo {
	fn as_component_info(&self) -> &WICComponentInfo { &self.0.as_component_info() }
	fn into_component_info(self) -> WICComponentInfo { self.0.into_component_info() }
}

impl IUnknown for WICBitmapDecoderInfo {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapDecoderInfo {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapCodecInfo::from(v))
    }
}

