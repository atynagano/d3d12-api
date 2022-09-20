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
use crate::core::win32::graphics::direct_write::*;
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1DeviceContext4(pub(crate) D2D1DeviceContext3);

impl Deref for D2D1DeviceContext4 {
	type Target = D2D1DeviceContext3;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DeviceContext4 {
	const IID: &'static GUID = &GUID::from_u128(0x8c4278313d904476b647c4fae349e4dbu128);
}

impl Com for D2D1DeviceContext4 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DeviceContext4 {
	pub fn CreateSvgGlyphStyle(&self) -> Result<D2D1SvgGlyphStyle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut svg_glyph_style_out_: Option<D2D1SvgGlyphStyle> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[108]);
			let _ret_ = f(vt, transmute(&mut svg_glyph_style_out_));
			if _ret_.is_ok() { if let Some(svg_glyph_style_out_) = svg_glyph_style_out_ { return Ok(svg_glyph_style_out_); } }
			Err(_ret_)
		}
	}

	pub fn DrawText(&self, string: &str, text_format: &DWriteTextFormat, layout_rect: &D2DRectF, default_fill_brush: Option<&D2D1Brush>, svg_glyph_style: Option<&D2D1SvgGlyphStyle>, color_palette_index: u32, options: D2D1DrawTextOptions, measuring_mode: DWriteMeasuringMode) -> () {
		unsafe {
			let vt = self.as_param();
			let string_utf16_ = string.encode_utf16().collect::<Vec<_>>();
			let (string_ptr_, string_len_) = string_utf16_.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u16, u32, VTable, &D2DRectF, *const c_void, *const c_void, u32, D2D1DrawTextOptions, DWriteMeasuringMode) -> ()
				= transmute(vt[109]);
			let _ret_ = f(vt, string_ptr_, string_len_ as u32, text_format.vtable(), layout_rect, option_to_vtable(default_fill_brush), option_to_vtable(svg_glyph_style), color_palette_index, options, measuring_mode);
		}
	}

	pub fn DrawTextLayout(&self, origin: D2DPoint2F, text_layout: &DWriteTextLayout, default_fill_brush: Option<&D2D1Brush>, svg_glyph_style: Option<&D2D1SvgGlyphStyle>, color_palette_index: u32, options: D2D1DrawTextOptions) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, VTable, *const c_void, *const c_void, u32, D2D1DrawTextOptions) -> ()
				= transmute(vt[110]);
			let _ret_ = f(vt, origin, text_layout.vtable(), option_to_vtable(default_fill_brush), option_to_vtable(svg_glyph_style), color_palette_index, options);
		}
	}

	pub fn DrawColorBitmapGlyphRun(&self, glyph_image_format: DWriteGlyphImageFormats, baseline_origin: D2DPoint2F, glyph_run: &DWriteGlyphRun, measuring_mode: DWriteMeasuringMode, bitmap_snap_option: D2D1ColorBitmapGlyphSnapOption) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DWriteGlyphImageFormats, D2DPoint2F, &DWriteGlyphRun, DWriteMeasuringMode, D2D1ColorBitmapGlyphSnapOption) -> ()
				= transmute(vt[111]);
			let _ret_ = f(vt, glyph_image_format, baseline_origin, glyph_run, measuring_mode, bitmap_snap_option);
		}
	}

	pub fn DrawSvgGlyphRun(&self, baseline_origin: D2DPoint2F, glyph_run: &DWriteGlyphRun, default_fill_brush: Option<&D2D1Brush>, svg_glyph_style: Option<&D2D1SvgGlyphStyle>, color_palette_index: u32, measuring_mode: DWriteMeasuringMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, &DWriteGlyphRun, *const c_void, *const c_void, u32, DWriteMeasuringMode) -> ()
				= transmute(vt[112]);
			let _ret_ = f(vt, baseline_origin, glyph_run, option_to_vtable(default_fill_brush), option_to_vtable(svg_glyph_style), color_palette_index, measuring_mode);
		}
	}

	pub unsafe fn GetColorBitmapGlyphImage(&self) { todo!() }

	pub unsafe fn GetSvgGlyphImage(&self) { todo!() }
}

pub trait ID2D1DeviceContext4: ID2D1DeviceContext3 {
	fn as_device_context4(&self) -> &D2D1DeviceContext4;
	fn into_device_context4(self) -> D2D1DeviceContext4;
}

impl ID2D1DeviceContext4 for D2D1DeviceContext4 {
	fn as_device_context4(&self) -> &D2D1DeviceContext4 { self }
	fn into_device_context4(self) -> D2D1DeviceContext4 { self }
}
impl ID2D1DeviceContext3 for D2D1DeviceContext4 {
	fn as_device_context3(&self) -> &D2D1DeviceContext3 { &self.0.as_device_context3() }
	fn into_device_context3(self) -> D2D1DeviceContext3 { self.0.into_device_context3() }
}

impl ID2D1DeviceContext2 for D2D1DeviceContext4 {
	fn as_device_context2(&self) -> &D2D1DeviceContext2 { &self.0.as_device_context2() }
	fn into_device_context2(self) -> D2D1DeviceContext2 { self.0.into_device_context2() }
}

impl ID2D1DeviceContext1 for D2D1DeviceContext4 {
	fn as_device_context1(&self) -> &D2D1DeviceContext1 { &self.0.as_device_context1() }
	fn into_device_context1(self) -> D2D1DeviceContext1 { self.0.into_device_context1() }
}

impl ID2D1DeviceContext for D2D1DeviceContext4 {
	fn as_device_context(&self) -> &D2D1DeviceContext { &self.0.as_device_context() }
	fn into_device_context(self) -> D2D1DeviceContext { self.0.into_device_context() }
}

impl ID2D1RenderTarget for D2D1DeviceContext4 {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1DeviceContext4 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DeviceContext4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DeviceContext4 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1DeviceContext3::from(v))
    }
}

