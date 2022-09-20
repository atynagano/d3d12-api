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
pub struct DWriteTypography(pub(crate) Unknown);

impl Deref for DWriteTypography {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteTypography {
	const IID: &'static GUID = &GUID::from_u128(0x55f1112b1dc24b3c9541f46894ed85b6u128);
}

impl Com for DWriteTypography {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteTypography {
	pub fn AddFontFeature(&self, font_feature: DWriteFontFeature) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteFontFeature) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, font_feature);
			_ret_.ok()
		}
	}

	pub fn GetFontFeatureCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetFontFeature(&self, font_feature_index: u32) -> Result<DWriteFontFeature, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_feature_out_: MaybeUninit<DWriteFontFeature> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut DWriteFontFeature) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, font_feature_index, font_feature_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(font_feature_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDWriteTypography: IUnknown {
	fn as_typography(&self) -> &DWriteTypography;
	fn into_typography(self) -> DWriteTypography;
}

impl IDWriteTypography for DWriteTypography {
	fn as_typography(&self) -> &DWriteTypography { self }
	fn into_typography(self) -> DWriteTypography { self }
}
impl IUnknown for DWriteTypography {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteTypography {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

