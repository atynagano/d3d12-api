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
pub struct DWriteFontFamily(pub(crate) DWriteFontList);

impl Deref for DWriteFontFamily {
	type Target = DWriteFontList;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteFontFamily {
	const IID: &'static GUID = &GUID::from_u128(0xda20d8ef812a4c43980262ec4abd7addu128);
}

impl Com for DWriteFontFamily {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteFontFamily {
	pub fn GetFamilyNames(&self) -> Result<DWriteLocalizedStrings, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut names_out_: Option<DWriteLocalizedStrings> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, transmute(&mut names_out_));
			if _ret_.is_ok() { if let Some(names_out_) = names_out_ { return Ok(names_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetFirstMatchingFont(&self, weight: DWriteFontWeight, stretch: DWriteFontStretch, style: DWriteFontStyle) -> Result<DWriteFont, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut matching_font_out_: Option<DWriteFont> = None;
			let f: extern "system" fn(Param<Self>, DWriteFontWeight, DWriteFontStretch, DWriteFontStyle, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, weight, stretch, style, transmute(&mut matching_font_out_));
			if _ret_.is_ok() { if let Some(matching_font_out_) = matching_font_out_ { return Ok(matching_font_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetMatchingFonts(&self, weight: DWriteFontWeight, stretch: DWriteFontStretch, style: DWriteFontStyle) -> Result<DWriteFontList, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut matching_fonts_out_: Option<DWriteFontList> = None;
			let f: extern "system" fn(Param<Self>, DWriteFontWeight, DWriteFontStretch, DWriteFontStyle, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, weight, stretch, style, transmute(&mut matching_fonts_out_));
			if _ret_.is_ok() { if let Some(matching_fonts_out_) = matching_fonts_out_ { return Ok(matching_fonts_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDWriteFontFamily: IDWriteFontList {
	fn as_font_family(&self) -> &DWriteFontFamily;
	fn into_font_family(self) -> DWriteFontFamily;
}

impl IDWriteFontFamily for DWriteFontFamily {
	fn as_font_family(&self) -> &DWriteFontFamily { self }
	fn into_font_family(self) -> DWriteFontFamily { self }
}
impl IDWriteFontList for DWriteFontFamily {
	fn as_font_list(&self) -> &DWriteFontList { &self.0.as_font_list() }
	fn into_font_list(self) -> DWriteFontList { self.0.into_font_list() }
}

impl IUnknown for DWriteFontFamily {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteFontFamily {
    fn from(v: UnknownWrapper) -> Self {
        Self(DWriteFontList::from(v))
    }
}

