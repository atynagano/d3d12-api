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
pub struct WICBitmapEncoderInfo(pub(crate) WICBitmapCodecInfo);

impl Deref for WICBitmapEncoderInfo {
	type Target = WICBitmapCodecInfo;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapEncoderInfo {
	const IID: &'static GUID = &GUID::from_u128(0x94c9b4eea09f4f928a1e4a9bce7e76fbu128);
}

impl Com for WICBitmapEncoderInfo {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapEncoderInfo {
	pub fn CreateInstance(&self) -> Result<WICBitmapEncoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_encoder_out_: Option<WICBitmapEncoder> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[23]);
			let _ret_ = f(vt, transmute(&mut i_bitmap_encoder_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_encoder_out_) = i_bitmap_encoder_out_ { return Ok(i_bitmap_encoder_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICBitmapEncoderInfo: IWICBitmapCodecInfo {
	fn as_bitmap_encoder_info(&self) -> &WICBitmapEncoderInfo;
	fn into_bitmap_encoder_info(self) -> WICBitmapEncoderInfo;
}

impl IWICBitmapEncoderInfo for WICBitmapEncoderInfo {
	fn as_bitmap_encoder_info(&self) -> &WICBitmapEncoderInfo { self }
	fn into_bitmap_encoder_info(self) -> WICBitmapEncoderInfo { self }
}
impl IWICBitmapCodecInfo for WICBitmapEncoderInfo {
	fn as_bitmap_codec_info(&self) -> &WICBitmapCodecInfo { &self.0.as_bitmap_codec_info() }
	fn into_bitmap_codec_info(self) -> WICBitmapCodecInfo { self.0.into_bitmap_codec_info() }
}

impl IWICComponentInfo for WICBitmapEncoderInfo {
	fn as_component_info(&self) -> &WICComponentInfo { &self.0.as_component_info() }
	fn into_component_info(self) -> WICComponentInfo { self.0.into_component_info() }
}

impl IUnknown for WICBitmapEncoderInfo {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapEncoderInfo {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapCodecInfo::from(v))
    }
}

