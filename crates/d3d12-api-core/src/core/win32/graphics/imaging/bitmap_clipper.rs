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
pub struct WICBitmapClipper(pub(crate) WICBitmapSource);

impl Deref for WICBitmapClipper {
	type Target = WICBitmapSource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapClipper {
	const IID: &'static GUID = &GUID::from_u128(0xe4fbcf03223d4e819333d635556dd1b5u128);
}

impl Com for WICBitmapClipper {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapClipper {
	pub fn Initialize(&self, i_source: &WICBitmapSource, prc: &WicRect) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, &WicRect) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, i_source.vtable(), prc);
			_ret_.ok()
		}
	}
}

pub trait IWICBitmapClipper: IWICBitmapSource {
	fn as_bitmap_clipper(&self) -> &WICBitmapClipper;
	fn into_bitmap_clipper(self) -> WICBitmapClipper;
}

impl IWICBitmapClipper for WICBitmapClipper {
	fn as_bitmap_clipper(&self) -> &WICBitmapClipper { self }
	fn into_bitmap_clipper(self) -> WICBitmapClipper { self }
}
impl IWICBitmapSource for WICBitmapClipper {
	fn as_bitmap_source(&self) -> &WICBitmapSource { &self.0.as_bitmap_source() }
	fn into_bitmap_source(self) -> WICBitmapSource { self.0.into_bitmap_source() }
}

impl IUnknown for WICBitmapClipper {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapClipper {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapSource::from(v))
    }
}

