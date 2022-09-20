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
use crate::core::win32::graphics::gdi::*;
use crate::core::win32::ui::windows_and_messaging::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICImagingFactory(pub(crate) Unknown);

impl Deref for WICImagingFactory {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICImagingFactory {
	const IID: &'static GUID = &GUID::from_u128(0xec5ec8a9c39543149c7754d7a935ff70u128);
}

impl Com for WICImagingFactory {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICImagingFactory {
	pub fn CreateDecoderFromFilename(&self, wz_filename: &str, pguid_vendor: &GUID, desired_access: u32, metadata_options: WicDecodeOptions) -> Result<WICBitmapDecoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_decoder_out_: Option<WICBitmapDecoder> = None;
			let f: extern "system" fn(Param<Self>, *const u16, &GUID, u32, WicDecodeOptions, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, wz_filename.to_utf16().as_ptr_or_null(), pguid_vendor, desired_access, metadata_options, transmute(&mut i_decoder_out_));
			if _ret_.is_ok() { if let Some(i_decoder_out_) = i_decoder_out_ { return Ok(i_decoder_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDecoderFromStream(&self, i_stream: &Stream, pguid_vendor: &GUID, metadata_options: WicDecodeOptions) -> Result<WICBitmapDecoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_decoder_out_: Option<WICBitmapDecoder> = None;
			let f: extern "system" fn(Param<Self>, VTable, &GUID, WicDecodeOptions, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, i_stream.vtable(), pguid_vendor, metadata_options, transmute(&mut i_decoder_out_));
			if _ret_.is_ok() { if let Some(i_decoder_out_) = i_decoder_out_ { return Ok(i_decoder_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDecoderFromFileHandle(&self, file: usize, pguid_vendor: &GUID, metadata_options: WicDecodeOptions) -> Result<WICBitmapDecoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_decoder_out_: Option<WICBitmapDecoder> = None;
			let f: extern "system" fn(Param<Self>, usize, &GUID, WicDecodeOptions, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, file, pguid_vendor, metadata_options, transmute(&mut i_decoder_out_));
			if _ret_.is_ok() { if let Some(i_decoder_out_) = i_decoder_out_ { return Ok(i_decoder_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateComponentInfo(&self, clsid_component: &GUID) -> Result<WICComponentInfo, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_info_out_: Option<WICComponentInfo> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, clsid_component, transmute(&mut i_info_out_));
			if _ret_.is_ok() { if let Some(i_info_out_) = i_info_out_ { return Ok(i_info_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDecoder(&self, guid_container_format: &GUID, pguid_vendor: &GUID) -> Result<WICBitmapDecoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_decoder_out_: Option<WICBitmapDecoder> = None;
			let f: extern "system" fn(Param<Self>, &GUID, &GUID, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, guid_container_format, pguid_vendor, transmute(&mut i_decoder_out_));
			if _ret_.is_ok() { if let Some(i_decoder_out_) = i_decoder_out_ { return Ok(i_decoder_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateEncoder(&self, guid_container_format: &GUID, pguid_vendor: &GUID) -> Result<WICBitmapEncoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_encoder_out_: Option<WICBitmapEncoder> = None;
			let f: extern "system" fn(Param<Self>, &GUID, &GUID, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, guid_container_format, pguid_vendor, transmute(&mut i_encoder_out_));
			if _ret_.is_ok() { if let Some(i_encoder_out_) = i_encoder_out_ { return Ok(i_encoder_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreatePalette(&self) -> Result<WICPalette, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_palette_out_: Option<WICPalette> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transmute(&mut i_palette_out_));
			if _ret_.is_ok() { if let Some(i_palette_out_) = i_palette_out_ { return Ok(i_palette_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateFormatConverter(&self) -> Result<WICFormatConverter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_format_converter_out_: Option<WICFormatConverter> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, transmute(&mut i_format_converter_out_));
			if _ret_.is_ok() { if let Some(i_format_converter_out_) = i_format_converter_out_ { return Ok(i_format_converter_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapScaler(&self) -> Result<WICBitmapScaler, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_scaler_out_: Option<WICBitmapScaler> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, transmute(&mut i_bitmap_scaler_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_scaler_out_) = i_bitmap_scaler_out_ { return Ok(i_bitmap_scaler_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapClipper(&self) -> Result<WICBitmapClipper, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_clipper_out_: Option<WICBitmapClipper> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(&mut i_bitmap_clipper_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_clipper_out_) = i_bitmap_clipper_out_ { return Ok(i_bitmap_clipper_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFlipRotator(&self) -> Result<WICBitmapFlipRotator, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_flip_rotator_out_: Option<WICBitmapFlipRotator> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(&mut i_bitmap_flip_rotator_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_flip_rotator_out_) = i_bitmap_flip_rotator_out_ { return Ok(i_bitmap_flip_rotator_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateStream(&self) -> Result<WICStream, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut iwic_stream_out_: Option<WICStream> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, transmute(&mut iwic_stream_out_));
			if _ret_.is_ok() { if let Some(iwic_stream_out_) = iwic_stream_out_ { return Ok(iwic_stream_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContext(&self) -> Result<WICColorContext, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut iwic_color_context_out_: Option<WICColorContext> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, transmute(&mut iwic_color_context_out_));
			if _ret_.is_ok() { if let Some(iwic_color_context_out_) = iwic_color_context_out_ { return Ok(iwic_color_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorTransformer(&self) -> Result<WICColorTransform, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut iwic_color_transform_out_: Option<WICColorTransform> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, transmute(&mut iwic_color_transform_out_));
			if _ret_.is_ok() { if let Some(iwic_color_transform_out_) = iwic_color_transform_out_ { return Ok(iwic_color_transform_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmap(&self, ui_width: u32, ui_height: u32, pixel_format: &GUID, option: WicBitmapCreateCacheOption) -> Result<WICBitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_out_: Option<WICBitmap> = None;
			let f: extern "system" fn(Param<Self>, u32, u32, &GUID, WicBitmapCreateCacheOption, *mut c_void) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, ui_width, ui_height, pixel_format, option, transmute(&mut i_bitmap_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_out_) = i_bitmap_out_ { return Ok(i_bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFromSource(&self, i_bitmap_source: &WICBitmapSource, option: WicBitmapCreateCacheOption) -> Result<WICBitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_out_: Option<WICBitmap> = None;
			let f: extern "system" fn(Param<Self>, VTable, WicBitmapCreateCacheOption, *mut c_void) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, i_bitmap_source.vtable(), option, transmute(&mut i_bitmap_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_out_) = i_bitmap_out_ { return Ok(i_bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFromSourceRect(&self, i_bitmap_source: &WICBitmapSource, x: u32, y: u32, width: u32, height: u32) -> Result<WICBitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_out_: Option<WICBitmap> = None;
			let f: extern "system" fn(Param<Self>, VTable, u32, u32, u32, u32, *mut c_void) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, i_bitmap_source.vtable(), x, y, width, height, transmute(&mut i_bitmap_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_out_) = i_bitmap_out_ { return Ok(i_bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFromMemory(&self, ui_width: u32, ui_height: u32, pixel_format: &GUID, stride: u32, pb_buffer: &[u8]) -> Result<WICBitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let (pb_buffer_ptr_, pb_buffer_len_) = pb_buffer.deconstruct();
			let mut i_bitmap_out_: Option<WICBitmap> = None;
			let f: extern "system" fn(Param<Self>, u32, u32, &GUID, u32, u32, *const u8, *mut c_void) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, ui_width, ui_height, pixel_format, stride, pb_buffer_len_ as u32, pb_buffer_ptr_, transmute(&mut i_bitmap_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_out_) = i_bitmap_out_ { return Ok(i_bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFromHBITMAP(&self, bitmap: HBitmap, palette: HPalette, options: WicBitmapAlphaChannelOption) -> Result<WICBitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_out_: Option<WICBitmap> = None;
			let f: extern "system" fn(Param<Self>, HBitmap, HPalette, WicBitmapAlphaChannelOption, *mut c_void) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, bitmap, palette, options, transmute(&mut i_bitmap_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_out_) = i_bitmap_out_ { return Ok(i_bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFromHICON(&self, icon: HIcon) -> Result<WICBitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_bitmap_out_: Option<WICBitmap> = None;
			let f: extern "system" fn(Param<Self>, HIcon, *mut c_void) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, icon, transmute(&mut i_bitmap_out_));
			if _ret_.is_ok() { if let Some(i_bitmap_out_) = i_bitmap_out_ { return Ok(i_bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateComponentEnumerator(&self, component_types: u32, options: u32) -> Result<EnumUnknown, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_enum_unknown_out_: Option<EnumUnknown> = None;
			let f: extern "system" fn(Param<Self>, u32, u32, *mut c_void) -> HResult
				= transmute(vt[23]);
			let _ret_ = f(vt, component_types, options, transmute(&mut i_enum_unknown_out_));
			if _ret_.is_ok() { if let Some(i_enum_unknown_out_) = i_enum_unknown_out_ { return Ok(i_enum_unknown_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateFastMetadataEncoderFromDecoder(&self, i_decoder: &WICBitmapDecoder) -> Result<WICFastMetadataEncoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_fast_encoder_out_: Option<WICFastMetadataEncoder> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, i_decoder.vtable(), transmute(&mut i_fast_encoder_out_));
			if _ret_.is_ok() { if let Some(i_fast_encoder_out_) = i_fast_encoder_out_ { return Ok(i_fast_encoder_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateFastMetadataEncoderFromFrameDecode(&self, i_frame_decoder: &WICBitmapFrameDecode) -> Result<WICFastMetadataEncoder, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_fast_encoder_out_: Option<WICFastMetadataEncoder> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, i_frame_decoder.vtable(), transmute(&mut i_fast_encoder_out_));
			if _ret_.is_ok() { if let Some(i_fast_encoder_out_) = i_fast_encoder_out_ { return Ok(i_fast_encoder_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateQueryWriter(&self, guid_metadata_format: &GUID, pguid_vendor: &GUID) -> Result<WICMetadataQueryWriter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_query_writer_out_: Option<WICMetadataQueryWriter> = None;
			let f: extern "system" fn(Param<Self>, &GUID, &GUID, *mut c_void) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, guid_metadata_format, pguid_vendor, transmute(&mut i_query_writer_out_));
			if _ret_.is_ok() { if let Some(i_query_writer_out_) = i_query_writer_out_ { return Ok(i_query_writer_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateQueryWriterFromReader(&self, i_query_reader: &WICMetadataQueryReader, pguid_vendor: &GUID) -> Result<WICMetadataQueryWriter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_query_writer_out_: Option<WICMetadataQueryWriter> = None;
			let f: extern "system" fn(Param<Self>, VTable, &GUID, *mut c_void) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, i_query_reader.vtable(), pguid_vendor, transmute(&mut i_query_writer_out_));
			if _ret_.is_ok() { if let Some(i_query_writer_out_) = i_query_writer_out_ { return Ok(i_query_writer_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICImagingFactory: IUnknown {
	fn as_imaging_factory(&self) -> &WICImagingFactory;
	fn into_imaging_factory(self) -> WICImagingFactory;
}

impl IWICImagingFactory for WICImagingFactory {
	fn as_imaging_factory(&self) -> &WICImagingFactory { self }
	fn into_imaging_factory(self) -> WICImagingFactory { self }
}
impl IUnknown for WICImagingFactory {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICImagingFactory {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

