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
pub struct DWriteFontList(pub(crate) Unknown);

impl Deref for DWriteFontList {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteFontList {
	const IID: &'static GUID = &GUID::from_u128(0x1a0d84381d974ec1aef9a2fb86ed6acbu128);
}

impl Com for DWriteFontList {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteFontList {
	pub fn GetFontCollection(&self) -> Result<DWriteFontCollection, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_collection_out_: Option<DWriteFontCollection> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, transmute(&mut font_collection_out_));
			if _ret_.is_ok() { if let Some(font_collection_out_) = font_collection_out_ { return Ok(font_collection_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetFontCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetFont(&self, index: u32) -> Result<DWriteFont, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_out_: Option<DWriteFont> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, index, transmute(&mut font_out_));
			if _ret_.is_ok() { if let Some(font_out_) = font_out_ { return Ok(font_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDWriteFontList: IUnknown {
	fn as_font_list(&self) -> &DWriteFontList;
	fn into_font_list(self) -> DWriteFontList;
}

impl IDWriteFontList for DWriteFontList {
	fn as_font_list(&self) -> &DWriteFontList { self }
	fn into_font_list(self) -> DWriteFontList { self }
}
impl IUnknown for DWriteFontList {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteFontList {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

