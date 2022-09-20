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
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SvgGlyphStyle(pub(crate) D2D1Resource);

impl Deref for D2D1SvgGlyphStyle {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SvgGlyphStyle {
	const IID: &'static GUID = &GUID::from_u128(0xaf671749d2414db88e41dcc2e5c1a438u128);
}

impl Com for D2D1SvgGlyphStyle {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SvgGlyphStyle {
	pub fn SetFill(&self, brush: Option<&D2D1Brush>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, option_to_vtable(brush));
			_ret_.ok()
		}
	}

	pub fn GetFill(&self, mut brush: Option<&mut Option<D2D1Brush>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut brush) = brush { **brush = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, transmute(brush));
		}
	}

	pub fn SetStroke(&self, brush: Option<&D2D1Brush>, stroke_width: f32, dashes: Option<&[f32]>, dash_offset: f32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (dashes_ptr_, dashes_len_) = dashes.deconstruct();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, *const f32, u32, f32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, option_to_vtable(brush), stroke_width, dashes_ptr_, dashes_len_ as u32, dash_offset);
			_ret_.ok()
		}
	}

	pub fn GetStrokeDashesCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetStroke(&self) { todo!() }
}

pub trait ID2D1SvgGlyphStyle: ID2D1Resource {
	fn as_svg_glyph_style(&self) -> &D2D1SvgGlyphStyle;
	fn into_svg_glyph_style(self) -> D2D1SvgGlyphStyle;
}

impl ID2D1SvgGlyphStyle for D2D1SvgGlyphStyle {
	fn as_svg_glyph_style(&self) -> &D2D1SvgGlyphStyle { self }
	fn into_svg_glyph_style(self) -> D2D1SvgGlyphStyle { self }
}
impl ID2D1Resource for D2D1SvgGlyphStyle {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SvgGlyphStyle {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SvgGlyphStyle {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

