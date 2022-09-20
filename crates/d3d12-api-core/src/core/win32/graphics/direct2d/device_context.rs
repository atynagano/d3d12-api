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
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::direct_write::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1DeviceContext(pub(crate) D2D1RenderTarget);

impl Deref for D2D1DeviceContext {
	type Target = D2D1RenderTarget;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DeviceContext {
	const IID: &'static GUID = &GUID::from_u128(0xe8f7fe7a191c466dad95975678bda998u128);
}

impl Com for D2D1DeviceContext {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DeviceContext {
	pub fn CreateBitmap(&self, size: D2DSizeU, source_data: *const (), pitch: u32, bitmap_properties: &D2D1BitmapProperties1) -> Result<D2D1Bitmap1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_out_: Option<D2D1Bitmap1> = None;
			let f: extern "system" fn(Param<Self>, D2DSizeU, *const c_void, u32, &D2D1BitmapProperties1, *mut c_void) -> HResult
				= transmute(vt[57]);
			let _ret_ = f(vt, size, source_data as _, pitch, bitmap_properties, transmute(&mut bitmap_out_));
			if _ret_.is_ok() { if let Some(bitmap_out_) = bitmap_out_ { return Ok(bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFromWicBitmap(&self, wic_bitmap_source: &WICBitmapSource, bitmap_properties: Option<&D2D1BitmapProperties1>) -> Result<D2D1Bitmap1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_out_: Option<D2D1Bitmap1> = None;
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, *mut c_void) -> HResult
				= transmute(vt[58]);
			let _ret_ = f(vt, wic_bitmap_source.vtable(), transmute(bitmap_properties), transmute(&mut bitmap_out_));
			if _ret_.is_ok() { if let Some(bitmap_out_) = bitmap_out_ { return Ok(bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContext(&self, space: D2D1ColorSpace, profile: Option<&[u8]>) -> Result<D2D1ColorContext, HResult> {
		unsafe {
			let vt = self.as_param();
			let (profile_ptr_, profile_len_) = profile.deconstruct();
			let mut color_context_out_: Option<D2D1ColorContext> = None;
			let f: extern "system" fn(Param<Self>, D2D1ColorSpace, *const u8, u32, *mut c_void) -> HResult
				= transmute(vt[59]);
			let _ret_ = f(vt, space, profile_ptr_, profile_len_ as u32, transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContextFromFilename(&self, filename: &str) -> Result<D2D1ColorContext, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_context_out_: Option<D2D1ColorContext> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *mut c_void) -> HResult
				= transmute(vt[60]);
			let _ret_ = f(vt, filename.to_utf16().as_ptr_or_null(), transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContextFromWicColorContext(&self, wic_color_context: &WICColorContext) -> Result<D2D1ColorContext, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_context_out_: Option<D2D1ColorContext> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[61]);
			let _ret_ = f(vt, wic_color_context.vtable(), transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapFromDxgiSurface(&self, surface: &DxgiSurface, bitmap_properties: Option<&D2D1BitmapProperties1>) -> Result<D2D1Bitmap1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_out_: Option<D2D1Bitmap1> = None;
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, *mut c_void) -> HResult
				= transmute(vt[62]);
			let _ret_ = f(vt, surface.vtable(), transmute(bitmap_properties), transmute(&mut bitmap_out_));
			if _ret_.is_ok() { if let Some(bitmap_out_) = bitmap_out_ { return Ok(bitmap_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateEffect(&self, effect_id: &GUID) -> Result<D2D1Effect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut effect_out_: Option<D2D1Effect> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[63]);
			let _ret_ = f(vt, effect_id, transmute(&mut effect_out_));
			if _ret_.is_ok() { if let Some(effect_out_) = effect_out_ { return Ok(effect_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateGradientStopCollection(&self, straight_alpha_gradient_stops: &[D2D1GradientStop], pre_interpolation_space: D2D1ColorSpace, post_interpolation_space: D2D1ColorSpace, buffer_precision: D2D1BufferPrecision, extend_mode: D2D1ExtendMode, color_interpolation_mode: D2D1ColorInterpolationMode) -> Result<D2D1GradientStopCollection1, HResult> {
		unsafe {
			let vt = self.as_param();
			let (straight_alpha_gradient_stops_ptr_, straight_alpha_gradient_stops_len_) = straight_alpha_gradient_stops.deconstruct();
			let mut gradient_stop_collection1_out_: Option<D2D1GradientStopCollection1> = None;
			let f: extern "system" fn(Param<Self>, *const D2D1GradientStop, u32, D2D1ColorSpace, D2D1ColorSpace, D2D1BufferPrecision, D2D1ExtendMode, D2D1ColorInterpolationMode, *mut c_void) -> HResult
				= transmute(vt[64]);
			let _ret_ = f(vt, straight_alpha_gradient_stops_ptr_, straight_alpha_gradient_stops_len_ as u32, pre_interpolation_space, post_interpolation_space, buffer_precision, extend_mode, color_interpolation_mode, transmute(&mut gradient_stop_collection1_out_));
			if _ret_.is_ok() { if let Some(gradient_stop_collection1_out_) = gradient_stop_collection1_out_ { return Ok(gradient_stop_collection1_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateImageBrush(&self, image: Option<&D2D1Image>, image_brush_properties: &D2D1ImageBrushProperties, brush_properties: Option<&D2D1BrushProperties>) -> Result<D2D1ImageBrush, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut image_brush_out_: Option<D2D1ImageBrush> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, &D2D1ImageBrushProperties, *const c_void, *mut c_void) -> HResult
				= transmute(vt[65]);
			let _ret_ = f(vt, option_to_vtable(image), image_brush_properties, transmute(brush_properties), transmute(&mut image_brush_out_));
			if _ret_.is_ok() { if let Some(image_brush_out_) = image_brush_out_ { return Ok(image_brush_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBitmapBrush(&self, bitmap: Option<&D2D1Bitmap>, bitmap_brush_properties: Option<&D2D1BitmapBrushProperties1>, brush_properties: Option<&D2D1BrushProperties>) -> Result<D2D1BitmapBrush1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_brush_out_: Option<D2D1BitmapBrush1> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, *const c_void, *mut c_void) -> HResult
				= transmute(vt[66]);
			let _ret_ = f(vt, option_to_vtable(bitmap), transmute(bitmap_brush_properties), transmute(brush_properties), transmute(&mut bitmap_brush_out_));
			if _ret_.is_ok() { if let Some(bitmap_brush_out_) = bitmap_brush_out_ { return Ok(bitmap_brush_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateCommandList(&self) -> Result<D2D1CommandList, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut command_list_out_: Option<D2D1CommandList> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[67]);
			let _ret_ = f(vt, transmute(&mut command_list_out_));
			if _ret_.is_ok() { if let Some(command_list_out_) = command_list_out_ { return Ok(command_list_out_); } }
			Err(_ret_)
		}
	}

	pub fn IsDxgiFormatSupported(&self, format: DxgiFormat) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DxgiFormat) -> Bool
				= transmute(vt[68]);
			let _ret_ = f(vt, format);
			_ret_.to_bool()
		}
	}

	pub fn IsBufferPrecisionSupported(&self, buffer_precision: D2D1BufferPrecision) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1BufferPrecision) -> Bool
				= transmute(vt[69]);
			let _ret_ = f(vt, buffer_precision);
			_ret_.to_bool()
		}
	}

	pub fn GetImageLocalBounds(&self, image: &D2D1Image) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut local_bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, VTable, *mut D2DRectF) -> HResult
				= transmute(vt[70]);
			let _ret_ = f(vt, image.vtable(), local_bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(local_bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetImageWorldBounds(&self, image: &D2D1Image) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut world_bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, VTable, *mut D2DRectF) -> HResult
				= transmute(vt[71]);
			let _ret_ = f(vt, image.vtable(), world_bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(world_bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetGlyphRunWorldBounds(&self, baseline_origin: D2DPoint2F, glyph_run: &DWriteGlyphRun, measuring_mode: DWriteMeasuringMode) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, &DWriteGlyphRun, DWriteMeasuringMode, *mut D2DRectF) -> HResult
				= transmute(vt[72]);
			let _ret_ = f(vt, baseline_origin, glyph_run, measuring_mode, bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetDevice(&self) -> Option<D2D1Device> {
		unsafe {
			let vt = self.as_param();
			let mut device_out_: Option<D2D1Device> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[73]);
			let _ret_ = f(vt, transmute(&mut device_out_));
			device_out_
		}
	}

	pub fn SetTarget(&self, image: Option<&D2D1Image>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[74]);
			let _ret_ = f(vt, option_to_vtable(image));
		}
	}

	pub fn GetTarget(&self, mut image: Option<&mut Option<D2D1Image>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut image) = image { **image = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[75]);
			let _ret_ = f(vt, transmute(image));
		}
	}

	pub fn SetRenderingControls(&self, rendering_controls: &D2D1RenderingControls) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1RenderingControls) -> ()
				= transmute(vt[76]);
			let _ret_ = f(vt, rendering_controls);
		}
	}

	pub fn GetRenderingControls(&self) -> D2D1RenderingControls {
		unsafe {
			let vt = self.as_param();
			let mut rendering_controls_out_: MaybeUninit<D2D1RenderingControls> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2D1RenderingControls) -> ()
				= transmute(vt[77]);
			let _ret_ = f(vt, rendering_controls_out_.as_mut_ptr());
			rendering_controls_out_.assume_init()
		}
	}

	pub fn SetPrimitiveBlend(&self, primitive_blend: D2D1PrimitiveBlend) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1PrimitiveBlend) -> ()
				= transmute(vt[78]);
			let _ret_ = f(vt, primitive_blend);
		}
	}

	pub fn GetPrimitiveBlend(&self) -> D2D1PrimitiveBlend {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1PrimitiveBlend
				= transmute(vt[79]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetUnitMode(&self, unit_mode: D2D1UnitMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1UnitMode) -> ()
				= transmute(vt[80]);
			let _ret_ = f(vt, unit_mode);
		}
	}

	pub fn GetUnitMode(&self) -> D2D1UnitMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1UnitMode
				= transmute(vt[81]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn DrawGlyphRun(&self, baseline_origin: D2DPoint2F, glyph_run: &DWriteGlyphRun, glyph_run_description: Option<&DWriteGlyphRunDescription>, foreground_brush: &D2D1Brush, measuring_mode: DWriteMeasuringMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, &DWriteGlyphRun, *const c_void, VTable, DWriteMeasuringMode) -> ()
				= transmute(vt[82]);
			let _ret_ = f(vt, baseline_origin, glyph_run, transmute(glyph_run_description), foreground_brush.vtable(), measuring_mode);
		}
	}

	pub fn DrawImage(&self, image: &D2D1Image, target_offset: Option<&D2DPoint2F>, image_rectangle: Option<&D2DRectF>, interpolation_mode: D2D1InterpolationMode, composite_mode: D2D1CompositeMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, *const c_void, D2D1InterpolationMode, D2D1CompositeMode) -> ()
				= transmute(vt[83]);
			let _ret_ = f(vt, image.vtable(), transmute(target_offset), transmute(image_rectangle), interpolation_mode, composite_mode);
		}
	}

	pub fn DrawGdiMetafile(&self, gdi_metafile: &D2D1GdiMetafile, target_offset: Option<&D2DPoint2F>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void) -> ()
				= transmute(vt[84]);
			let _ret_ = f(vt, gdi_metafile.vtable(), transmute(target_offset));
		}
	}

	pub fn DrawBitmap(&self, bitmap: &D2D1Bitmap, destination_rectangle: Option<&D2DRectF>, opacity: f32, interpolation_mode: D2D1InterpolationMode, source_rectangle: Option<&D2DRectF>, perspective_transform: Option<&D2DMatrix4x4F>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, f32, D2D1InterpolationMode, *const c_void, *const c_void) -> ()
				= transmute(vt[85]);
			let _ret_ = f(vt, bitmap.vtable(), transmute(destination_rectangle), opacity, interpolation_mode, transmute(source_rectangle), transmute(perspective_transform));
		}
	}

	pub fn PushLayer(&self, layer_parameters: &D2D1LayerParameters1, layer: Option<&D2D1Layer>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1LayerParameters1, *const c_void) -> ()
				= transmute(vt[86]);
			let _ret_ = f(vt, layer_parameters, option_to_vtable(layer));
		}
	}

	pub fn InvalidateEffectInputRectangle(&self, effect: &D2D1Effect, input: u32, input_rectangle: &D2DRectF) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, &D2DRectF) -> HResult
				= transmute(vt[87]);
			let _ret_ = f(vt, effect.vtable(), input, input_rectangle);
			_ret_.ok()
		}
	}

	pub fn GetEffectInvalidRectangleCount(&self, effect: &D2D1Effect) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut rectangle_count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, VTable, *mut u32) -> HResult
				= transmute(vt[88]);
			let _ret_ = f(vt, effect.vtable(), rectangle_count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(rectangle_count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetEffectInvalidRectangles(&self) { todo!() }

	pub unsafe fn GetEffectRequiredInputRectangles() { todo!() }

	pub fn FillOpacityMask(&self, opacity_mask: &D2D1Bitmap, brush: &D2D1Brush, destination_rectangle: Option<&D2DRectF>, source_rectangle: Option<&D2DRectF>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, *const c_void, *const c_void) -> ()
				= transmute(vt[91]);
			let _ret_ = f(vt, opacity_mask.vtable(), brush.vtable(), transmute(destination_rectangle), transmute(source_rectangle));
		}
	}
}

pub trait ID2D1DeviceContext: ID2D1RenderTarget {
	fn as_device_context(&self) -> &D2D1DeviceContext;
	fn into_device_context(self) -> D2D1DeviceContext;
}

impl ID2D1DeviceContext for D2D1DeviceContext {
	fn as_device_context(&self) -> &D2D1DeviceContext { self }
	fn into_device_context(self) -> D2D1DeviceContext { self }
}
impl ID2D1RenderTarget for D2D1DeviceContext {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1DeviceContext {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DeviceContext {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DeviceContext {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1RenderTarget::from(v))
    }
}

