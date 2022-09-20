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
pub struct WICBitmap(pub(crate) WICBitmapSource);

impl Deref for WICBitmap {
	type Target = WICBitmapSource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmap {
	const IID: &'static GUID = &GUID::from_u128(0x00000121a8f24877ba0afd2b6645fb94u128);
}

impl Com for WICBitmap {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmap {
	pub fn Lock(&self, prc_lock: &WicRect, flags: u32) -> Result<WICBitmapLock, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_lock_out_: Option<WICBitmapLock> = None;
			let f: extern "system" fn(Param<Self>, &WicRect, u32, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, prc_lock, flags, transmute(&mut i_lock_out_));
			if _ret_.is_ok() { if let Some(i_lock_out_) = i_lock_out_ { return Ok(i_lock_out_); } }
			Err(_ret_)
		}
	}

	pub fn SetPalette(&self, i_palette: &WICPalette) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, i_palette.vtable());
			_ret_.ok()
		}
	}

	pub fn SetResolution(&self, dpi_x: f64, dpi_y: f64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f64, f64) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, dpi_x, dpi_y);
			_ret_.ok()
		}
	}
}

pub trait IWICBitmap: IWICBitmapSource {
	fn as_bitmap(&self) -> &WICBitmap;
	fn into_bitmap(self) -> WICBitmap;
}

impl IWICBitmap for WICBitmap {
	fn as_bitmap(&self) -> &WICBitmap { self }
	fn into_bitmap(self) -> WICBitmap { self }
}
impl IWICBitmapSource for WICBitmap {
	fn as_bitmap_source(&self) -> &WICBitmapSource { &self.0.as_bitmap_source() }
	fn into_bitmap_source(self) -> WICBitmapSource { self.0.into_bitmap_source() }
}

impl IUnknown for WICBitmap {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmap {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICBitmapSource::from(v))
    }
}

