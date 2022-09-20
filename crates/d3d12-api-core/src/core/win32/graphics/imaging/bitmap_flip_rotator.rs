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
pub struct WICBitmapFlipRotator(pub(crate) WICBitmapSource);

impl Deref for WICBitmapFlipRotator {
	type Target = WICBitmapSource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapFlipRotator {
	const IID: &'static GUID = &GUID::from_u128(0x5009834f2d6a41ce9e1b17c5aff7a782u128);
}

impl Com for WICBitmapFlipRotator {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapFlipRotator {
	pub fn Initialize(&self, i_source: &WICBitmapSource, options: WicBitmapTransformOptions) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, WicBitmapTransformOptions) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, i_source.vtable(), options);
			_ret_.ok()
		}
	}
}

pub trait IWICBitmapFlipRotator: IWICBitmapSource {
	fn as_bitmap_flip_rotator(&self) -> &WICBitmapFlipRotator;
	fn into_bitmap_flip_rotator(self) -> WICBitmapFlipRotator;
}

impl IWICBitmapFlipRotator for WICBitmapFlipRotator {
	fn as_bitmap_flip_rotator(&self) -> &WICBitmapFlipRotator { self }
	fn into_bitmap_flip_rotator(self) -> WICBitmapFlipRotator { self }
}
impl IWICBitmapSource for WICBitmapFlipRotator {
	fn as_bitmap_source(&self) -> &WICBitmapSource { &self.0.as_bitmap_source() }
	fn into_bitmap_source(self) -> WICBitmapSource { self.0.into_bitmap_source() }
}

impl IUnknown for WICBitmapFlipRotator {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapFlipRotator {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapSource::from(v))
    }
}

