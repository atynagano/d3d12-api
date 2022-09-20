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
use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::graphics::imaging::*;
use crate::core::win32::graphics::direct_write::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1RenderTarget(pub(crate) D2D1Resource);

impl Deref for D2D1RenderTarget {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1RenderTarget {
	const IID: &'static GUID = &GUID::from_u128(0x2cd9069412e211dc9fed001143a055f9u128);
}

impl Com for D2D1RenderTarget {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1RenderTarget {
	pub fn CreateBitmap(&self, size: D2DSizeU, src_data: *const (), pitch: u32, bitmap_properties: &D2D1BitmapProperties) -> Result<D2D1Bitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_out_: Option<D2D1Bitmap> = None;
			let f: extern "system" fn(Param<Self>, D2DSizeU, *const c_void, u32, &D2D1BitmapProperties, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, size, src_data as _, pitch, bitmap_properties, transmute(&mut bitmap_out_));
			if _ret_.is_ok() { if let Some(bitmap_out_) = bitmap_out_ { return Ok(bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFromWicBitmap(&self, wic_bitmap_source: &WICBitmapSource, bitmap_properties: Option<&D2D1BitmapProperties>) -> Result<D2D1Bitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_out_: Option<D2D1Bitmap> = None;
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, wic_bitmap_source.vtable(), transmute(bitmap_properties), transmute(&mut bitmap_out_));
			if _ret_.is_ok() { if let Some(bitmap_out_) = bitmap_out_ { return Ok(bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateSharedBitmap(&self, riid: &GUID, data: &mut (), bitmap_properties: Option<&D2D1BitmapProperties>) -> Result<D2D1Bitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_out_: Option<D2D1Bitmap> = None;
			let f: extern "system" fn(Param<Self>, &GUID, &mut (), *const c_void, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, riid, data, transmute(bitmap_properties), transmute(&mut bitmap_out_));
			if _ret_.is_ok() { if let Some(bitmap_out_) = bitmap_out_ { return Ok(bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapBrush(&self, bitmap: Option<&D2D1Bitmap>, bitmap_brush_properties: Option<&D2D1BitmapBrushProperties>, brush_properties: Option<&D2D1BrushProperties>) -> Result<D2D1BitmapBrush, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_brush_out_: Option<D2D1BitmapBrush> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, *const c_void, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, option_to_vtable(bitmap), transmute(bitmap_brush_properties), transmute(brush_properties), transmute(&mut bitmap_brush_out_));
			if _ret_.is_ok() { if let Some(bitmap_brush_out_) = bitmap_brush_out_ { return Ok(bitmap_brush_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateSolidColorBrush(&self, color: &D2D1ColorF, brush_properties: Option<&D2D1BrushProperties>) -> Result<D2D1SolidColorBrush, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut solid_color_brush_out_: Option<D2D1SolidColorBrush> = None;
			let f: extern "system" fn(Param<Self>, &D2D1ColorF, *const c_void, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, color, transmute(brush_properties), transmute(&mut solid_color_brush_out_));
			if _ret_.is_ok() { if let Some(solid_color_brush_out_) = solid_color_brush_out_ { return Ok(solid_color_brush_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateGradientStopCollection(&self, gradient_stops: &[D2D1GradientStop], color_interpolation_gamma: D2D1Gamma, extend_mode: D2D1ExtendMode) -> Result<D2D1GradientStopCollection, HResult> {
		unsafe {
			let vt = self.as_param();
			let (gradient_stops_ptr_, gradient_stops_len_) = gradient_stops.deconstruct();
			let mut gradient_stop_collection_out_: Option<D2D1GradientStopCollection> = None;
			let f: extern "system" fn(Param<Self>, *const D2D1GradientStop, u32, D2D1Gamma, D2D1ExtendMode, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, gradient_stops_ptr_, gradient_stops_len_ as u32, color_interpolation_gamma, extend_mode, transmute(&mut gradient_stop_collection_out_));
			if _ret_.is_ok() { if let Some(gradient_stop_collection_out_) = gradient_stop_collection_out_ { return Ok(gradient_stop_collection_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateLinearGradientBrush(&self, linear_gradient_brush_properties: &D2D1LinearGradientBrushProperties, brush_properties: Option<&D2D1BrushProperties>, gradient_stop_collection: &D2D1GradientStopCollection) -> Result<D2D1LinearGradientBrush, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut linear_gradient_brush_out_: Option<D2D1LinearGradientBrush> = None;
			let f: extern "system" fn(Param<Self>, &D2D1LinearGradientBrushProperties, *const c_void, VTable, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, linear_gradient_brush_properties, transmute(brush_properties), gradient_stop_collection.vtable(), transmute(&mut linear_gradient_brush_out_));
			if _ret_.is_ok() { if let Some(linear_gradient_brush_out_) = linear_gradient_brush_out_ { return Ok(linear_gradient_brush_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateRadialGradientBrush(&self, radial_gradient_brush_properties: &D2D1RadialGradientBrushProperties, brush_properties: Option<&D2D1BrushProperties>, gradient_stop_collection: &D2D1GradientStopCollection) -> Result<D2D1RadialGradientBrush, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut radial_gradient_brush_out_: Option<D2D1RadialGradientBrush> = None;
			let f: extern "system" fn(Param<Self>, &D2D1RadialGradientBrushProperties, *const c_void, VTable, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, radial_gradient_brush_properties, transmute(brush_properties), gradient_stop_collection.vtable(), transmute(&mut radial_gradient_brush_out_));
			if _ret_.is_ok() { if let Some(radial_gradient_brush_out_) = radial_gradient_brush_out_ { return Ok(radial_gradient_brush_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateCompatibleRenderTarget(&self, desired_size: Option<&D2DSizeF>, desired_pixel_size: Option<&D2DSizeU>, desired_format: Option<&D2D1PixelFormat>, options: D2D1CompatibleRenderTargetOptions) -> Result<D2D1BitmapRenderTarget, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_render_target_out_: Option<D2D1BitmapRenderTarget> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, *const c_void, D2D1CompatibleRenderTargetOptions, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(desired_size), transmute(desired_pixel_size), transmute(desired_format), options, transmute(&mut bitmap_render_target_out_));
			if _ret_.is_ok() { if let Some(bitmap_render_target_out_) = bitmap_render_target_out_ { return Ok(bitmap_render_target_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateLayer(&self, size: Option<&D2DSizeF>) -> Result<D2D1Layer, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut layer_out_: Option<D2D1Layer> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(size), transmute(&mut layer_out_));
			if _ret_.is_ok() { if let Some(layer_out_) = layer_out_ { return Ok(layer_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateMesh(&self) -> Result<D2D1Mesh, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut mesh_out_: Option<D2D1Mesh> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, transmute(&mut mesh_out_));
			if _ret_.is_ok() { if let Some(mesh_out_) = mesh_out_ { return Ok(mesh_out_); } }
			Err(_ret_)
		}
	}

	pub fn DrawLine(&self, point0: D2DPoint2F, point1: D2DPoint2F, brush: &D2D1Brush, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, D2DPoint2F, VTable, f32, *const c_void) -> ()
				= transmute(vt[15]);
			let _ret_ = f(vt, point0, point1, brush.vtable(), stroke_width, option_to_vtable(stroke_style));
		}
	}

	pub fn DrawRectangle(&self, rect: &D2DRectF, brush: &D2D1Brush, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DRectF, VTable, f32, *const c_void) -> ()
				= transmute(vt[16]);
			let _ret_ = f(vt, rect, brush.vtable(), stroke_width, option_to_vtable(stroke_style));
		}
	}

	pub fn FillRectangle(&self, rect: &D2DRectF, brush: &D2D1Brush) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DRectF, VTable) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, rect, brush.vtable());
		}
	}

	pub fn DrawRoundedRectangle(&self, rounded_rect: &D2D1RoundedRect, brush: &D2D1Brush, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1RoundedRect, VTable, f32, *const c_void) -> ()
				= transmute(vt[18]);
			let _ret_ = f(vt, rounded_rect, brush.vtable(), stroke_width, option_to_vtable(stroke_style));
		}
	}

	pub fn FillRoundedRectangle(&self, rounded_rect: &D2D1RoundedRect, brush: &D2D1Brush) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1RoundedRect, VTable) -> ()
				= transmute(vt[19]);
			let _ret_ = f(vt, rounded_rect, brush.vtable());
		}
	}

	pub fn DrawEllipse(&self, ellipse: &D2D1Ellipse, brush: &D2D1Brush, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1Ellipse, VTable, f32, *const c_void) -> ()
				= transmute(vt[20]);
			let _ret_ = f(vt, ellipse, brush.vtable(), stroke_width, option_to_vtable(stroke_style));
		}
	}

	pub fn FillEllipse(&self, ellipse: &D2D1Ellipse, brush: &D2D1Brush) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1Ellipse, VTable) -> ()
				= transmute(vt[21]);
			let _ret_ = f(vt, ellipse, brush.vtable());
		}
	}

	pub fn DrawGeometry(&self, geometry: &D2D1Geometry, brush: &D2D1Brush, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, f32, *const c_void) -> ()
				= transmute(vt[22]);
			let _ret_ = f(vt, geometry.vtable(), brush.vtable(), stroke_width, option_to_vtable(stroke_style));
		}
	}

	pub fn FillGeometry(&self, geometry: &D2D1Geometry, brush: &D2D1Brush, opacity_brush: Option<&D2D1Brush>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, *const c_void) -> ()
				= transmute(vt[23]);
			let _ret_ = f(vt, geometry.vtable(), brush.vtable(), option_to_vtable(opacity_brush));
		}
	}

	pub fn FillMesh(&self, mesh: &D2D1Mesh, brush: &D2D1Brush) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable) -> ()
				= transmute(vt[24]);
			let _ret_ = f(vt, mesh.vtable(), brush.vtable());
		}
	}

	pub fn FillOpacityMask(&self, opacity_mask: &D2D1Bitmap, brush: &D2D1Brush, content: D2D1OpacityMaskContent, destination_rectangle: Option<&D2DRectF>, source_rectangle: Option<&D2DRectF>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, D2D1OpacityMaskContent, *const c_void, *const c_void) -> ()
				= transmute(vt[25]);
			let _ret_ = f(vt, opacity_mask.vtable(), brush.vtable(), content, transmute(destination_rectangle), transmute(source_rectangle));
		}
	}

	pub fn DrawBitmap(&self, bitmap: &D2D1Bitmap, destination_rectangle: Option<&D2DRectF>, opacity: f32, interpolation_mode: D2D1BitmapInterpolationMode, source_rectangle: Option<&D2DRectF>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, f32, D2D1BitmapInterpolationMode, *const c_void) -> ()
				= transmute(vt[26]);
			let _ret_ = f(vt, bitmap.vtable(), transmute(destination_rectangle), opacity, interpolation_mode, transmute(source_rectangle));
		}
	}

	pub fn DrawText(&self, string: &str, text_format: &DWriteTextFormat, layout_rect: &D2DRectF, default_fill_brush: &D2D1Brush, options: D2D1DrawTextOptions, measuring_mode: DWriteMeasuringMode) -> () {
		unsafe {
			let vt = self.as_param();
			let string_utf16_ = string.encode_utf16().collect::<Vec<_>>();
			let (string_ptr_, string_len_) = string_utf16_.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u16, u32, VTable, &D2DRectF, VTable, D2D1DrawTextOptions, DWriteMeasuringMode) -> ()
				= transmute(vt[27]);
			let _ret_ = f(vt, string_ptr_, string_len_ as u32, text_format.vtable(), layout_rect, default_fill_brush.vtable(), options, measuring_mode);
		}
	}

	pub fn DrawTextLayout(&self, origin: D2DPoint2F, text_layout: &DWriteTextLayout, default_fill_brush: &D2D1Brush, options: D2D1DrawTextOptions) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, VTable, VTable, D2D1DrawTextOptions) -> ()
				= transmute(vt[28]);
			let _ret_ = f(vt, origin, text_layout.vtable(), default_fill_brush.vtable(), options);
		}
	}

	pub fn DrawGlyphRun(&self, baseline_origin: D2DPoint2F, glyph_run: &DWriteGlyphRun, foreground_brush: &D2D1Brush, measuring_mode: DWriteMeasuringMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, &DWriteGlyphRun, VTable, DWriteMeasuringMode) -> ()
				= transmute(vt[29]);
			let _ret_ = f(vt, baseline_origin, glyph_run, foreground_brush.vtable(), measuring_mode);
		}
	}

	pub fn SetTransform(&self, transform: &D2DMatrix3x2F) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DMatrix3x2F) -> ()
				= transmute(vt[30]);
			let _ret_ = f(vt, transform);
		}
	}

	pub fn GetTransform(&self) -> D2DMatrix3x2F {
		unsafe {
			let vt = self.as_param();
			let mut transform_out_: MaybeUninit<D2DMatrix3x2F> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2DMatrix3x2F) -> ()
				= transmute(vt[31]);
			let _ret_ = f(vt, transform_out_.as_mut_ptr());
			transform_out_.assume_init()
		}
	}

	pub fn SetAntialiasMode(&self, antialias_mode: D2D1AntiAliasMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1AntiAliasMode) -> ()
				= transmute(vt[32]);
			let _ret_ = f(vt, antialias_mode);
		}
	}

	pub fn GetAntialiasMode(&self) -> D2D1AntiAliasMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1AntiAliasMode
				= transmute(vt[33]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetTextAntialiasMode(&self, text_antialias_mode: D2D1TextAntiAliasMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1TextAntiAliasMode) -> ()
				= transmute(vt[34]);
			let _ret_ = f(vt, text_antialias_mode);
		}
	}

	pub fn GetTextAntialiasMode(&self) -> D2D1TextAntiAliasMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1TextAntiAliasMode
				= transmute(vt[35]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetTextRenderingParams(&self, text_rendering_params: Option<&DWriteRenderingParams>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[36]);
			let _ret_ = f(vt, option_to_vtable(text_rendering_params));
		}
	}

	pub fn GetTextRenderingParams(&self, mut text_rendering_params: Option<&mut Option<DWriteRenderingParams>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut text_rendering_params) = text_rendering_params { **text_rendering_params = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[37]);
			let _ret_ = f(vt, transmute(text_rendering_params));
		}
	}

	pub fn SetTags(&self, tag1: u64, tag2: u64) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u64, u64) -> ()
				= transmute(vt[38]);
			let _ret_ = f(vt, tag1, tag2);
		}
	}

	pub fn GetTags(&self, tag1: Option<&mut MaybeUninit<u64>>, tag2: Option<&mut MaybeUninit<u64>>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Option<&mut MaybeUninit<u64>>, Option<&mut MaybeUninit<u64>>) -> ()
				= transmute(vt[39]);
			let _ret_ = f(vt, tag1, tag2);
		}
	}

	pub fn PushLayer(&self, layer_parameters: &D2D1LayerParameters, layer: Option<&D2D1Layer>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1LayerParameters, *const c_void) -> ()
				= transmute(vt[40]);
			let _ret_ = f(vt, layer_parameters, option_to_vtable(layer));
		}
	}

	pub fn PopLayer(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[41]);
			let _ret_ = f(vt);
		}
	}

	pub fn Flush(&self, tag1: Option<&mut MaybeUninit<u64>>, tag2: Option<&mut MaybeUninit<u64>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Option<&mut MaybeUninit<u64>>, Option<&mut MaybeUninit<u64>>) -> HResult
				= transmute(vt[42]);
			let _ret_ = f(vt, tag1, tag2);
			_ret_.ok()
		}
	}

	pub fn SaveDrawingState(&self, drawing_state_block: &D2D1DrawingStateBlock) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> ()
				= transmute(vt[43]);
			let _ret_ = f(vt, drawing_state_block.vtable());
		}
	}

	pub fn RestoreDrawingState(&self, drawing_state_block: &D2D1DrawingStateBlock) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> ()
				= transmute(vt[44]);
			let _ret_ = f(vt, drawing_state_block.vtable());
		}
	}

	pub fn PushAxisAlignedClip(&self, clip_rect: &D2DRectF, antialias_mode: D2D1AntiAliasMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DRectF, D2D1AntiAliasMode) -> ()
				= transmute(vt[45]);
			let _ret_ = f(vt, clip_rect, antialias_mode);
		}
	}

	pub fn PopAxisAlignedClip(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[46]);
			let _ret_ = f(vt);
		}
	}

	pub fn Clear(&self, clear_color: Option<&D2D1ColorF>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[47]);
			let _ret_ = f(vt, transmute(clear_color));
		}
	}

	pub fn BeginDraw(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[48]);
			let _ret_ = f(vt);
		}
	}

	pub fn EndDraw(&self, tag1: Option<&mut MaybeUninit<u64>>, tag2: Option<&mut MaybeUninit<u64>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Option<&mut MaybeUninit<u64>>, Option<&mut MaybeUninit<u64>>) -> HResult
				= transmute(vt[49]);
			let _ret_ = f(vt, tag1, tag2);
			_ret_.ok()
		}
	}

	pub fn GetPixelFormat(&self) -> D2D1PixelFormat {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1PixelFormat
				= transmute(vt[50]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetDpi(&self, dpi_x: f32, dpi_y: f32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32, f32) -> ()
				= transmute(vt[51]);
			let _ret_ = f(vt, dpi_x, dpi_y);
		}
	}

	pub fn GetDpi(&self) -> (f32, f32) {
		unsafe {
			let vt = self.as_param();
			let mut dpi_x_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut dpi_y_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut f32, *mut f32) -> ()
				= transmute(vt[52]);
			let _ret_ = f(vt, dpi_x_out_.as_mut_ptr(), dpi_y_out_.as_mut_ptr());
			(dpi_x_out_.assume_init(), dpi_y_out_.assume_init())
		}
	}

	pub fn GetSize(&self) -> D2DSizeF {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DSizeF
				= transmute(vt[53]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetPixelSize(&self) -> D2DSizeU {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DSizeU
				= transmute(vt[54]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetMaximumBitmapSize(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[55]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn IsSupported(&self, render_target_properties: &D2D1RenderTargetProperties) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1RenderTargetProperties) -> Bool
				= transmute(vt[56]);
			let _ret_ = f(vt, render_target_properties);
			_ret_.to_bool()
		}
	}
}

pub trait ID2D1RenderTarget: ID2D1Resource {
	fn as_render_target(&self) -> &D2D1RenderTarget;
	fn into_render_target(self) -> D2D1RenderTarget;
}

impl ID2D1RenderTarget for D2D1RenderTarget {
	fn as_render_target(&self) -> &D2D1RenderTarget { self }
	fn into_render_target(self) -> D2D1RenderTarget { self }
}
impl ID2D1Resource for D2D1RenderTarget {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1RenderTarget {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1RenderTarget {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

