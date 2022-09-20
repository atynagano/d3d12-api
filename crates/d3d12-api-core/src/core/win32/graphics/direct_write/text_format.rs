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
use crate::core::win32::graphics::direct_write::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWriteTextFormat(pub(crate) Unknown);

impl Deref for DWriteTextFormat {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteTextFormat {
	const IID: &'static GUID = &GUID::from_u128(0x9c90681831d74fd3a1517c5e225db55au128);
}

impl Com for DWriteTextFormat {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteTextFormat {
	pub fn SetTextAlignment(&self, text_alignment: DWriteTextAlignment) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteTextAlignment) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, text_alignment);
			_ret_.ok()
		}
	}

	pub fn SetParagraphAlignment(&self, paragraph_alignment: DWriteParagraphAlignment) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteParagraphAlignment) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, paragraph_alignment);
			_ret_.ok()
		}
	}

	pub fn SetWordWrapping(&self, word_wrapping: DWriteWordWrapping) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteWordWrapping) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, word_wrapping);
			_ret_.ok()
		}
	}

	pub fn SetReadingDirection(&self, reading_direction: DWriteReadingDirection) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteReadingDirection) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, reading_direction);
			_ret_.ok()
		}
	}

	pub fn SetFlowDirection(&self, flow_direction: DWriteFlowDirection) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteFlowDirection) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, flow_direction);
			_ret_.ok()
		}
	}

	pub fn SetIncrementalTabStop(&self, incremental_tab_stop: f32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, incremental_tab_stop);
			_ret_.ok()
		}
	}

	pub fn SetTrimming(&self, trimming_options: &DWriteTrimming, trimming_sign: Option<&DWriteInlineObject>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &DWriteTrimming, *const c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, trimming_options, option_to_vtable(trimming_sign));
			_ret_.ok()
		}
	}

	pub fn SetLineSpacing(&self, line_spacing_method: DWriteLineSpacingMethod, line_spacing: f32, baseline: f32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteLineSpacingMethod, f32, f32) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, line_spacing_method, line_spacing, baseline);
			_ret_.ok()
		}
	}

	pub fn GetTextAlignment(&self) -> DWriteTextAlignment {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteTextAlignment
				= transmute(vt[11]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetParagraphAlignment(&self) -> DWriteParagraphAlignment {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteParagraphAlignment
				= transmute(vt[12]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetWordWrapping(&self) -> DWriteWordWrapping {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteWordWrapping
				= transmute(vt[13]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetReadingDirection(&self) -> DWriteReadingDirection {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteReadingDirection
				= transmute(vt[14]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetFlowDirection(&self) -> DWriteFlowDirection {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFlowDirection
				= transmute(vt[15]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetIncrementalTabStop(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[16]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetTrimming(&self) { todo!() }

	pub fn GetLineSpacing(&self) -> Result<(DWriteLineSpacingMethod, f32, f32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut line_spacing_method_out_: MaybeUninit<DWriteLineSpacingMethod> = MaybeUninit::zeroed();
			let mut line_spacing_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut baseline_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DWriteLineSpacingMethod, *mut f32, *mut f32) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, line_spacing_method_out_.as_mut_ptr(), line_spacing_out_.as_mut_ptr(), baseline_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((line_spacing_method_out_.assume_init(), line_spacing_out_.assume_init(), baseline_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetFontCollection(&self) -> Result<DWriteFontCollection, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_collection_out_: Option<DWriteFontCollection> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, transmute(&mut font_collection_out_));
			if _ret_.is_ok() { if let Some(font_collection_out_) = font_collection_out_ { return Ok(font_collection_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetFontFamilyNameLength(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[20]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetFontFamilyName(&self) { todo!() }

	pub fn GetFontWeight(&self) -> DWriteFontWeight {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontWeight
				= transmute(vt[22]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetFontStyle(&self) -> DWriteFontStyle {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontStyle
				= transmute(vt[23]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetFontStretch(&self) -> DWriteFontStretch {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontStretch
				= transmute(vt[24]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetFontSize(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[25]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetLocaleNameLength(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[26]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetLocaleName(&self) { todo!() }
}

pub trait IDWriteTextFormat: IUnknown {
	fn as_text_format(&self) -> &DWriteTextFormat;
	fn into_text_format(self) -> DWriteTextFormat;
}

impl IDWriteTextFormat for DWriteTextFormat {
	fn as_text_format(&self) -> &DWriteTextFormat { self }
	fn into_text_format(self) -> DWriteTextFormat { self }
}
impl IUnknown for DWriteTextFormat {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteTextFormat {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

