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
pub struct WICPalette(pub(crate) Unknown);

impl Deref for WICPalette {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICPalette {
	const IID: &'static GUID = &GUID::from_u128(0x00000040a8f24877ba0afd2b6645fb94u128);
}

impl Com for WICPalette {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICPalette {
	pub fn InitializePredefined(&self, e_palette_type: WicBitmapPaletteType, f_add_transparent_color: bool) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, WicBitmapPaletteType, Bool) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, e_palette_type, f_add_transparent_color.to_bool());
			_ret_.ok()
		}
	}

	pub fn InitializeCustom(&self, colors: &[u32]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (colors_ptr_, colors_len_) = colors.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u32, u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, colors_ptr_, colors_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn InitializeFromBitmap(&self, i_surface: &WICBitmapSource, count: u32, f_add_transparent_color: bool) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, Bool) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, i_surface.vtable(), count, f_add_transparent_color.to_bool());
			_ret_.ok()
		}
	}

	pub fn InitializeFromPalette(&self, i_palette: &WICPalette) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, i_palette.vtable());
			_ret_.ok()
		}
	}

	pub fn GetType(&self) -> Result<WicBitmapPaletteType, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pe_palette_type_out_: MaybeUninit<WicBitmapPaletteType> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut WicBitmapPaletteType) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, pe_palette_type_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pe_palette_type_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetColorCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pc_count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, pc_count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pc_count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetColors(&self) { todo!() }

	pub fn IsBlackWhite(&self) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_is_black_white_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &mut Bool) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, &mut pf_is_black_white_out_);
			if _ret_.is_ok() { Ok(pf_is_black_white_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn IsGrayscale(&self) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_is_grayscale_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &mut Bool) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, &mut pf_is_grayscale_out_);
			if _ret_.is_ok() { Ok(pf_is_grayscale_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn HasAlpha(&self) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pf_has_alpha_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, &mut Bool) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, &mut pf_has_alpha_out_);
			if _ret_.is_ok() { Ok(pf_has_alpha_out_.to_bool()) } else { Err(_ret_) }
		}
	}
}

pub trait IWICPalette: IUnknown {
	fn as_palette(&self) -> &WICPalette;
	fn into_palette(self) -> WICPalette;
}

impl IWICPalette for WICPalette {
	fn as_palette(&self) -> &WICPalette { self }
	fn into_palette(self) -> WICPalette { self }
}
impl IUnknown for WICPalette {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICPalette {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

