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
pub struct D2D1CommandSink(pub(crate) Unknown);

impl Deref for D2D1CommandSink {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1CommandSink {
	const IID: &'static GUID = &GUID::from_u128(0x54d7898aa06140a7bec7e465bcba2c4fu128);
}

impl Com for D2D1CommandSink {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1CommandSink {
	pub fn BeginDraw(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn EndDraw(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn SetAntialiasMode(&self, antialias_mode: D2D1AntiAliasMode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1AntiAliasMode) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, antialias_mode);
			_ret_.ok()
		}
	}

	pub fn SetTags(&self, tag1: u64, tag2: u64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u64, u64) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, tag1, tag2);
			_ret_.ok()
		}
	}

	pub fn SetTextAntialiasMode(&self, text_antialias_mode: D2D1TextAntiAliasMode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1TextAntiAliasMode) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, text_antialias_mode);
			_ret_.ok()
		}
	}

	pub fn SetTextRenderingParams(&self, text_rendering_params: Option<&DWriteRenderingParams>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, option_to_vtable(text_rendering_params));
			_ret_.ok()
		}
	}

	pub fn SetTransform(&self, transform: &D2DMatrix3x2F) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DMatrix3x2F) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transform);
			_ret_.ok()
		}
	}

	pub fn SetPrimitiveBlend(&self, primitive_blend: D2D1PrimitiveBlend) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1PrimitiveBlend) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, primitive_blend);
			_ret_.ok()
		}
	}

	pub fn SetUnitMode(&self, unit_mode: D2D1UnitMode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1UnitMode) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, unit_mode);
			_ret_.ok()
		}
	}

	pub fn Clear(&self, color: Option<&D2D1ColorF>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(color));
			_ret_.ok()
		}
	}

	pub fn DrawGlyphRun(&self, baseline_origin: D2DPoint2F, glyph_run: &DWriteGlyphRun, glyph_run_description: Option<&DWriteGlyphRunDescription>, foreground_brush: &D2D1Brush, measuring_mode: DWriteMeasuringMode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, &DWriteGlyphRun, *const c_void, VTable, DWriteMeasuringMode) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, baseline_origin, glyph_run, transmute(glyph_run_description), foreground_brush.vtable(), measuring_mode);
			_ret_.ok()
		}
	}

	pub fn DrawLine(&self, point0: D2DPoint2F, point1: D2DPoint2F, brush: &D2D1Brush, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, D2DPoint2F, VTable, f32, *const c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, point0, point1, brush.vtable(), stroke_width, option_to_vtable(stroke_style));
			_ret_.ok()
		}
	}

	pub fn DrawGeometry(&self, geometry: &D2D1Geometry, brush: &D2D1Brush, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, f32, *const c_void) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, geometry.vtable(), brush.vtable(), stroke_width, option_to_vtable(stroke_style));
			_ret_.ok()
		}
	}

	pub fn DrawRectangle(&self, rect: &D2DRectF, brush: &D2D1Brush, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DRectF, VTable, f32, *const c_void) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, rect, brush.vtable(), stroke_width, option_to_vtable(stroke_style));
			_ret_.ok()
		}
	}

	pub fn DrawBitmap(&self, bitmap: &D2D1Bitmap, destination_rectangle: Option<&D2DRectF>, opacity: f32, interpolation_mode: D2D1InterpolationMode, source_rectangle: Option<&D2DRectF>, perspective_transform: Option<&D2DMatrix4x4F>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, f32, D2D1InterpolationMode, *const c_void, *const c_void) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, bitmap.vtable(), transmute(destination_rectangle), opacity, interpolation_mode, transmute(source_rectangle), transmute(perspective_transform));
			_ret_.ok()
		}
	}

	pub fn DrawImage(&self, image: &D2D1Image, target_offset: Option<&D2DPoint2F>, image_rectangle: Option<&D2DRectF>, interpolation_mode: D2D1InterpolationMode, composite_mode: D2D1CompositeMode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, *const c_void, D2D1InterpolationMode, D2D1CompositeMode) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, image.vtable(), transmute(target_offset), transmute(image_rectangle), interpolation_mode, composite_mode);
			_ret_.ok()
		}
	}

	pub fn DrawGdiMetafile(&self, gdi_metafile: &D2D1GdiMetafile, target_offset: Option<&D2DPoint2F>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, gdi_metafile.vtable(), transmute(target_offset));
			_ret_.ok()
		}
	}

	pub fn FillMesh(&self, mesh: &D2D1Mesh, brush: &D2D1Brush) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, mesh.vtable(), brush.vtable());
			_ret_.ok()
		}
	}

	pub fn FillOpacityMask(&self, opacity_mask: &D2D1Bitmap, brush: &D2D1Brush, destination_rectangle: Option<&D2DRectF>, source_rectangle: Option<&D2DRectF>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, *const c_void, *const c_void) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, opacity_mask.vtable(), brush.vtable(), transmute(destination_rectangle), transmute(source_rectangle));
			_ret_.ok()
		}
	}

	pub fn FillGeometry(&self, geometry: &D2D1Geometry, brush: &D2D1Brush, opacity_brush: Option<&D2D1Brush>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, *const c_void) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, geometry.vtable(), brush.vtable(), option_to_vtable(opacity_brush));
			_ret_.ok()
		}
	}

	pub fn FillRectangle(&self, rect: &D2DRectF, brush: &D2D1Brush) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DRectF, VTable) -> HResult
				= transmute(vt[23]);
			let _ret_ = f(vt, rect, brush.vtable());
			_ret_.ok()
		}
	}

	pub fn PushAxisAlignedClip(&self, clip_rect: &D2DRectF, antialias_mode: D2D1AntiAliasMode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DRectF, D2D1AntiAliasMode) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, clip_rect, antialias_mode);
			_ret_.ok()
		}
	}

	pub fn PushLayer(&self, layer_parameters1: &D2D1LayerParameters1, layer: Option<&D2D1Layer>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1LayerParameters1, *const c_void) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, layer_parameters1, option_to_vtable(layer));
			_ret_.ok()
		}
	}

	pub fn PopAxisAlignedClip(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn PopLayer(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait ID2D1CommandSink: IUnknown {
	fn as_command_sink(&self) -> &D2D1CommandSink;
	fn into_command_sink(self) -> D2D1CommandSink;
}

impl ID2D1CommandSink for D2D1CommandSink {
	fn as_command_sink(&self) -> &D2D1CommandSink { self }
	fn into_command_sink(self) -> D2D1CommandSink { self }
}
impl IUnknown for D2D1CommandSink {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1CommandSink {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

