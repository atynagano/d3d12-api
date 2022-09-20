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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICBitmapCodecInfo(pub(crate) WICComponentInfo);

impl Deref for WICBitmapCodecInfo {
	type Target = WICComponentInfo;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapCodecInfo {
	const IID: &'static GUID = &GUID::from_u128(0xe87a44c4b76e4c478b09298eb12a2714u128);
}

impl Com for WICBitmapCodecInfo {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapCodecInfo {
	pub unsafe fn GetContainerFormat(&self) { todo!() }

	pub fn GetPixelFormats(&self, pguid_pixel_formats: &mut [GUID]) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let (pguid_pixel_formats_ptr_, pguid_pixel_formats_len_) = pguid_pixel_formats.deconstruct();
			let mut pc_actual_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut GUID, *mut u32) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, pguid_pixel_formats_len_ as u32, pguid_pixel_formats_ptr_, pc_actual_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pc_actual_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetColorManagementVersion(&self) { todo!() }

	pub unsafe fn GetDeviceManufacturer(&self) { todo!() }

	pub unsafe fn GetDeviceModels(&self) { todo!() }

	pub unsafe fn GetMimeTypes(&self) { todo!() }

	pub unsafe fn GetFileExtensions(&self) { todo!() }

	pub fn DoesSupportAnimation(&self) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_support_animation_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &mut Bool) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, &mut pf_support_animation_out_);
			if _ret_.is_ok() { Ok(pf_support_animation_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn DoesSupportChromakey(&self) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_support_chromakey_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &mut Bool) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, &mut pf_support_chromakey_out_);
			if _ret_.is_ok() { Ok(pf_support_chromakey_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn DoesSupportLossless(&self) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_support_lossless_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &mut Bool) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, &mut pf_support_lossless_out_);
			if _ret_.is_ok() { Ok(pf_support_lossless_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn DoesSupportMultiframe(&self) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_support_multiframe_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &mut Bool) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, &mut pf_support_multiframe_out_);
			if _ret_.is_ok() { Ok(pf_support_multiframe_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn MatchesMimeType(&self, wz_mime_type: &str) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_matches_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, *const u16, &mut Bool) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, wz_mime_type.to_utf16().as_ptr_or_null(), &mut pf_matches_out_);
			if _ret_.is_ok() { Ok(pf_matches_out_.to_bool()) } else { Err(_ret_) }
		}
	}
}

pub trait IWICBitmapCodecInfo: IWICComponentInfo {
	fn as_bitmap_codec_info(&self) -> &WICBitmapCodecInfo;
	fn into_bitmap_codec_info(self) -> WICBitmapCodecInfo;
}

impl IWICBitmapCodecInfo for WICBitmapCodecInfo {
	fn as_bitmap_codec_info(&self) -> &WICBitmapCodecInfo { self }
	fn into_bitmap_codec_info(self) -> WICBitmapCodecInfo { self }
}
impl IWICComponentInfo for WICBitmapCodecInfo {
	fn as_component_info(&self) -> &WICComponentInfo { &self.0.as_component_info() }
	fn into_component_info(self) -> WICComponentInfo { self.0.into_component_info() }
}

impl IUnknown for WICBitmapCodecInfo {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapCodecInfo {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICComponentInfo::from(v))
    }
}

