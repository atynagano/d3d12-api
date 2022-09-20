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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWriteTextRenderer(pub(crate) DWritePixelSnapping);

impl Deref for DWriteTextRenderer {
	type Target = DWritePixelSnapping;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteTextRenderer {
	const IID: &'static GUID = &GUID::from_u128(0xef8a81355cc645fe8825c5a0724eb819u128);
}

impl Com for DWriteTextRenderer {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteTextRenderer {
	pub fn DrawGlyphRun(&self, client_drawing_context: *const (), baseline_origin_x: f32, baseline_origin_y: f32, measuring_mode: DWriteMeasuringMode, glyph_run: &DWriteGlyphRun, glyph_run_description: &DWriteGlyphRunDescription, client_drawing_effect: Option<&Unknown>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, f32, DWriteMeasuringMode, &DWriteGlyphRun, &DWriteGlyphRunDescription, *const c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, client_drawing_context as _, baseline_origin_x, baseline_origin_y, measuring_mode, glyph_run, glyph_run_description, option_to_vtable(client_drawing_effect));
			_ret_.ok()
		}
	}

	pub fn DrawUnderline(&self, client_drawing_context: *const (), baseline_origin_x: f32, baseline_origin_y: f32, underline: &DWriteUnderline, client_drawing_effect: Option<&Unknown>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, f32, &DWriteUnderline, *const c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, client_drawing_context as _, baseline_origin_x, baseline_origin_y, underline, option_to_vtable(client_drawing_effect));
			_ret_.ok()
		}
	}

	pub fn DrawStrikethrough(&self, client_drawing_context: *const (), baseline_origin_x: f32, baseline_origin_y: f32, strikethrough: &DWriteStrikeThrough, client_drawing_effect: Option<&Unknown>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, f32, &DWriteStrikeThrough, *const c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, client_drawing_context as _, baseline_origin_x, baseline_origin_y, strikethrough, option_to_vtable(client_drawing_effect));
			_ret_.ok()
		}
	}

	pub fn DrawInlineObject(&self, client_drawing_context: *const (), origin_x: f32, origin_y: f32, inline_object: &DWriteInlineObject, is_sideways: bool, is_right_to_left: bool, client_drawing_effect: Option<&Unknown>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, f32, VTable, Bool, Bool, *const c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, client_drawing_context as _, origin_x, origin_y, inline_object.vtable(), is_sideways.to_bool(), is_right_to_left.to_bool(), option_to_vtable(client_drawing_effect));
			_ret_.ok()
		}
	}
}

pub trait IDWriteTextRenderer: IDWritePixelSnapping {
	fn as_text_renderer(&self) -> &DWriteTextRenderer;
	fn into_text_renderer(self) -> DWriteTextRenderer;
}

impl IDWriteTextRenderer for DWriteTextRenderer {
	fn as_text_renderer(&self) -> &DWriteTextRenderer { self }
	fn into_text_renderer(self) -> DWriteTextRenderer { self }
}
impl IDWritePixelSnapping for DWriteTextRenderer {
	fn as_pixel_snapping(&self) -> &DWritePixelSnapping { &self.0.as_pixel_snapping() }
	fn into_pixel_snapping(self) -> DWritePixelSnapping { self.0.into_pixel_snapping() }
}

impl IUnknown for DWriteTextRenderer {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteTextRenderer {
    fn from(v: UnknownWrapper) -> Self {
        Self(DWritePixelSnapping::from(v))
    }
}

