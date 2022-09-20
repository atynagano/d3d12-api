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
pub struct DWriteFontCollection(pub(crate) Unknown);

impl Deref for DWriteFontCollection {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteFontCollection {
	const IID: &'static GUID = &GUID::from_u128(0xa84cee023eea4eeea82787c1a02a0fccu128);
}

impl Com for DWriteFontCollection {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteFontCollection {
	pub fn GetFontFamilyCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetFontFamily(&self, index: u32) -> Result<DWriteFontFamily, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_family_out_: Option<DWriteFontFamily> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, index, transmute(&mut font_family_out_));
			if _ret_.is_ok() { if let Some(font_family_out_) = font_family_out_ { return Ok(font_family_out_); } }
			Err(_ret_)
		}
	}

	pub fn FindFamilyName(&self, family_name: &str) -> Result<(u32, bool), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut index_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut exists_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, *const u16, *mut u32, &mut Bool) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, family_name.to_utf16().as_ptr_or_null(), index_out_.as_mut_ptr(), &mut exists_out_);
			if _ret_.is_ok() { Ok((index_out_.assume_init(), exists_out_.to_bool())) } else { Err(_ret_) }
		}
	}

	pub fn GetFontFromFontFace(&self, font_face: &DWriteFontFace) -> Result<DWriteFont, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_out_: Option<DWriteFont> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, font_face.vtable(), transmute(&mut font_out_));
			if _ret_.is_ok() { if let Some(font_out_) = font_out_ { return Ok(font_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDWriteFontCollection: IUnknown {
	fn as_font_collection(&self) -> &DWriteFontCollection;
	fn into_font_collection(self) -> DWriteFontCollection;
}

impl IDWriteFontCollection for DWriteFontCollection {
	fn as_font_collection(&self) -> &DWriteFontCollection { self }
	fn into_font_collection(self) -> DWriteFontCollection { self }
}
impl IUnknown for DWriteFontCollection {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteFontCollection {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

