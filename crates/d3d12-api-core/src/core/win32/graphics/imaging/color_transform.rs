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
pub struct WICColorTransform(pub(crate) WICBitmapSource);

impl Deref for WICColorTransform {
	type Target = WICBitmapSource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICColorTransform {
	const IID: &'static GUID = &GUID::from_u128(0xb66f034fd0e240abb4366de39e321a94u128);
}

impl Com for WICColorTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICColorTransform {
	pub fn Initialize(&self, i_bitmap_source: &WICBitmapSource, i_context_source: &WICColorContext, i_context_dest: &WICColorContext, pixel_fmt_dest: &GUID) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, VTable, &GUID) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, i_bitmap_source.vtable(), i_context_source.vtable(), i_context_dest.vtable(), pixel_fmt_dest);
			_ret_.ok()
		}
	}
}

pub trait IWICColorTransform: IWICBitmapSource {
	fn as_color_transform(&self) -> &WICColorTransform;
	fn into_color_transform(self) -> WICColorTransform;
}

impl IWICColorTransform for WICColorTransform {
	fn as_color_transform(&self) -> &WICColorTransform { self }
	fn into_color_transform(self) -> WICColorTransform { self }
}
impl IWICBitmapSource for WICColorTransform {
	fn as_bitmap_source(&self) -> &WICBitmapSource { &self.0.as_bitmap_source() }
	fn into_bitmap_source(self) -> WICBitmapSource { self.0.into_bitmap_source() }
}

impl IUnknown for WICColorTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICColorTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapSource::from(v))
    }
}

