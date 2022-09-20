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
pub struct DWriteFont(pub(crate) Unknown);

impl Deref for DWriteFont {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteFont {
	const IID: &'static GUID = &GUID::from_u128(0xacd166968c144f5d877efe3fc1d32737u128);
}

impl Com for DWriteFont {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteFont {
	pub fn GetFontFamily(&self) -> Result<DWriteFontFamily, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_family_out_: Option<DWriteFontFamily> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, transmute(&mut font_family_out_));
			if _ret_.is_ok() { if let Some(font_family_out_) = font_family_out_ { return Ok(font_family_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetWeight(&self) -> DWriteFontWeight {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontWeight
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetStretch(&self) -> DWriteFontStretch {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontStretch
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetStyle(&self) -> DWriteFontStyle {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontStyle
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn IsSymbolFont(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}

	pub fn GetFaceNames(&self) -> Result<DWriteLocalizedStrings, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut names_out_: Option<DWriteLocalizedStrings> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, transmute(&mut names_out_));
			if _ret_.is_ok() { if let Some(names_out_) = names_out_ { return Ok(names_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetInformationalStrings(&self, informational_string_id: DWriteInformationalStringId, mut informational_strings: Option<&mut Option<DWriteLocalizedStrings>>) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut informational_strings) = informational_strings { **informational_strings = None; }
			let mut exists_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, DWriteInformationalStringId, *mut c_void, &mut Bool) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, informational_string_id, transmute(informational_strings), &mut exists_out_);
			if _ret_.is_ok() { Ok(exists_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn get_informational_strings(&self) { todo!() }

	pub fn GetSimulations(&self) -> DWriteFontSimulations {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontSimulations
				= transmute(vt[10]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetMetrics(&self) -> DWriteFontMetrics {
		unsafe {
			let vt = self.as_param();
			let mut font_metrics_out_: MaybeUninit<DWriteFontMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DWriteFontMetrics) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, font_metrics_out_.as_mut_ptr());
			font_metrics_out_.assume_init()
		}
	}

	pub fn HasCharacter(&self, unicode_value: u32) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut exists_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, u32, &mut Bool) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, unicode_value, &mut exists_out_);
			if _ret_.is_ok() { Ok(exists_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn CreateFontFace(&self) -> Result<DWriteFontFace, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_face_out_: Option<DWriteFontFace> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(&mut font_face_out_));
			if _ret_.is_ok() { if let Some(font_face_out_) = font_face_out_ { return Ok(font_face_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDWriteFont: IUnknown {
	fn as_font(&self) -> &DWriteFont;
	fn into_font(self) -> DWriteFont;
}

impl IDWriteFont for DWriteFont {
	fn as_font(&self) -> &DWriteFont { self }
	fn into_font(self) -> DWriteFont { self }
}
impl IUnknown for DWriteFont {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteFont {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

