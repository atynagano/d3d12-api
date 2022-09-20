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
pub struct WICBitmapSource(pub(crate) Unknown);

impl Deref for WICBitmapSource {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapSource {
	const IID: &'static GUID = &GUID::from_u128(0x00000120a8f24877ba0afd2b6645fb94u128);
}

impl Com for WICBitmapSource {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapSource {
	pub fn GetSize(&self) -> Result<(u32, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pui_width_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut pui_height_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32, *mut u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, pui_width_out_.as_mut_ptr(), pui_height_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((pui_width_out_.assume_init(), pui_height_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetPixelFormat(&self) { todo!() }

	pub fn GetResolution(&self) -> Result<(f64, f64), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut dpi_x_out_: MaybeUninit<f64> = MaybeUninit::zeroed();
			let mut dpi_y_out_: MaybeUninit<f64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut f64, *mut f64) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, dpi_x_out_.as_mut_ptr(), dpi_y_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((dpi_x_out_.assume_init(), dpi_y_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn CopyPalette(&self, i_palette: &WICPalette) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, i_palette.vtable());
			_ret_.ok()
		}
	}

	pub unsafe fn CopyPixels(&self) { todo!() }
}

pub trait IWICBitmapSource: IUnknown {
	fn as_bitmap_source(&self) -> &WICBitmapSource;
	fn into_bitmap_source(self) -> WICBitmapSource;
}

impl IWICBitmapSource for WICBitmapSource {
	fn as_bitmap_source(&self) -> &WICBitmapSource { self }
	fn into_bitmap_source(self) -> WICBitmapSource { self }
}
impl IUnknown for WICBitmapSource {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapSource {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

