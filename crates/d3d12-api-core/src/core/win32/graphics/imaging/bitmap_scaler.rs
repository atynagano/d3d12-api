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
pub struct WICBitmapScaler(pub(crate) WICBitmapSource);

impl Deref for WICBitmapScaler {
	type Target = WICBitmapSource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapScaler {
	const IID: &'static GUID = &GUID::from_u128(0x00000302a8f24877ba0afd2b6645fb94u128);
}

impl Com for WICBitmapScaler {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapScaler {
	pub fn Initialize(&self, i_source: &WICBitmapSource, ui_width: u32, ui_height: u32, mode: WicBitmapInterpolationMode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, u32, WicBitmapInterpolationMode) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, i_source.vtable(), ui_width, ui_height, mode);
			_ret_.ok()
		}
	}
}

pub trait IWICBitmapScaler: IWICBitmapSource {
	fn as_bitmap_scaler(&self) -> &WICBitmapScaler;
	fn into_bitmap_scaler(self) -> WICBitmapScaler;
}

impl IWICBitmapScaler for WICBitmapScaler {
	fn as_bitmap_scaler(&self) -> &WICBitmapScaler { self }
	fn into_bitmap_scaler(self) -> WICBitmapScaler { self }
}
impl IWICBitmapSource for WICBitmapScaler {
	fn as_bitmap_source(&self) -> &WICBitmapSource { &self.0.as_bitmap_source() }
	fn into_bitmap_source(self) -> WICBitmapSource { self.0.into_bitmap_source() }
}

impl IUnknown for WICBitmapScaler {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapScaler {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapSource::from(v))
    }
}

