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

use crate::core::win32::graphics::direct_write::*;
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWriteFontFace(pub(crate) Unknown);

impl Deref for DWriteFontFace {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteFontFace {
	const IID: &'static GUID = &GUID::from_u128(0x5f49804d70244d43bfa9d25984f53849u128);
}

impl Com for DWriteFontFace {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteFontFace {
	pub fn GetType(&self) -> DWriteFontFaceType {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontFaceType
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetFiles(&self) { todo!() }

	pub fn GetIndex(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetSimulations(&self) -> DWriteFontSimulations {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteFontSimulations
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn IsSymbolFont(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}

	pub fn GetMetrics(&self) -> DWriteFontMetrics {
		unsafe {
			let vt = self.as_param();
			let mut font_face_metrics_out_: MaybeUninit<DWriteFontMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DWriteFontMetrics) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, font_face_metrics_out_.as_mut_ptr());
			font_face_metrics_out_.assume_init()
		}
	}

	pub fn GetGlyphCount(&self) -> u16 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u16
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetDesignGlyphMetrics() { todo!() }

	pub unsafe fn GetGlyphIndices() { todo!() }

	pub unsafe fn TryGetFontTable(&self) { todo!() }

	pub fn ReleaseFontTable(&self, table_context: *const impl Sized) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt, table_context as _);
		}
	}

	pub unsafe fn GetGlyphRunOutline() { todo!() }

	pub fn GetRecommendedRenderingMode(&self, em_size: f32, pixels_per_dip: f32, measuring_mode: DWriteMeasuringMode, rendering_params: &DWriteRenderingParams) -> Result<DWriteRenderingMode, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut rendering_mode_out_: MaybeUninit<DWriteRenderingMode> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, f32, f32, DWriteMeasuringMode, VTable, *mut DWriteRenderingMode) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, em_size, pixels_per_dip, measuring_mode, rendering_params.vtable(), rendering_mode_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(rendering_mode_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetGdiCompatibleMetrics(&self, em_size: f32, pixels_per_dip: f32, transform: Option<&DWriteMatrix>) -> Result<DWriteFontMetrics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut font_face_metrics_out_: MaybeUninit<DWriteFontMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, f32, f32, *const c_void, *mut DWriteFontMetrics) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, em_size, pixels_per_dip, transmute(transform), font_face_metrics_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(font_face_metrics_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetGdiCompatibleGlyphMetrics() { todo!() }
}

pub trait IDWriteFontFace: IUnknown {
	fn as_font_face(&self) -> &DWriteFontFace;
	fn into_font_face(self) -> DWriteFontFace;
}

impl IDWriteFontFace for DWriteFontFace {
	fn as_font_face(&self) -> &DWriteFontFace { self }
	fn into_font_face(self) -> DWriteFontFace { self }
}
impl IUnknown for DWriteFontFace {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteFontFace {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

