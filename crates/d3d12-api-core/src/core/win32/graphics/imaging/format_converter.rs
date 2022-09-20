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
pub struct WICFormatConverter(pub(crate) WICBitmapSource);

impl Deref for WICFormatConverter {
	type Target = WICBitmapSource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICFormatConverter {
	const IID: &'static GUID = &GUID::from_u128(0x00000301a8f24877ba0afd2b6645fb94u128);
}

impl Com for WICFormatConverter {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICFormatConverter {
	pub fn Initialize(&self, i_source: &WICBitmapSource, dst_format: &GUID, dither: WicBitmapDitherType, i_palette: &WICPalette, alpha_threshold_percent: f64, palette_translate: WicBitmapPaletteType) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, &GUID, WicBitmapDitherType, VTable, f64, WicBitmapPaletteType) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, i_source.vtable(), dst_format, dither, i_palette.vtable(), alpha_threshold_percent, palette_translate);
			_ret_.ok()
		}
	}

	pub fn CanConvert(&self, src_pixel_format: &GUID, dst_pixel_format: &GUID) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_can_convert_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &GUID, &GUID, &mut Bool) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, src_pixel_format, dst_pixel_format, &mut pf_can_convert_out_);
			if _ret_.is_ok() { Ok(pf_can_convert_out_.to_bool()) } else { Err(_ret_) }
		}
	}
}

pub trait IWICFormatConverter: IWICBitmapSource {
	fn as_format_converter(&self) -> &WICFormatConverter;
	fn into_format_converter(self) -> WICFormatConverter;
}

impl IWICFormatConverter for WICFormatConverter {
	fn as_format_converter(&self) -> &WICFormatConverter { self }
	fn into_format_converter(self) -> WICFormatConverter { self }
}
impl IWICBitmapSource for WICFormatConverter {
	fn as_bitmap_source(&self) -> &WICBitmapSource { &self.0.as_bitmap_source() }
	fn into_bitmap_source(self) -> WICBitmapSource { self.0.into_bitmap_source() }
}

impl IUnknown for WICFormatConverter {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICFormatConverter {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapSource::from(v))
    }
}

