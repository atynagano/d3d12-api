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
use crate::core::win32::graphics::direct_write::*;
use crate::core::win32::graphics::imaging::*;
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Factory(pub(crate) Unknown);

impl Deref for D2D1Factory {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Factory {
	const IID: &'static GUID = &GUID::from_u128(0x061522476f50465a9245118bfd3b6007u128);
}

impl Com for D2D1Factory {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Factory {
	pub fn ReloadSystemMetrics(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn GetDesktopDpi(&self) -> (f32, f32) {
		unsafe {
			let vt = self.as_param();
			let mut dpi_x_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut dpi_y_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut f32, *mut f32) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, dpi_x_out_.as_mut_ptr(), dpi_y_out_.as_mut_ptr());
			(dpi_x_out_.assume_init(), dpi_y_out_.assume_init())
		}
	}

	pub fn CreateRectangleGeometry(&self, rectangle: &D2DRectF) -> Result<D2D1RectangleGeometry, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut rectangle_geometry_out_: Option<D2D1RectangleGeometry> = None;
			let f: extern "system" fn(Param<Self>, &D2DRectF, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, rectangle, transmute(&mut rectangle_geometry_out_));
			if _ret_.is_ok() { if let Some(rectangle_geometry_out_) = rectangle_geometry_out_ { return Ok(rectangle_geometry_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateRoundedRectangleGeometry(&self, rounded_rectangle: &D2D1RoundedRect) -> Result<D2D1RoundedRectangleGeometry, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut rounded_rectangle_geometry_out_: Option<D2D1RoundedRectangleGeometry> = None;
			let f: extern "system" fn(Param<Self>, &D2D1RoundedRect, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, rounded_rectangle, transmute(&mut rounded_rectangle_geometry_out_));
			if _ret_.is_ok() { if let Some(rounded_rectangle_geometry_out_) = rounded_rectangle_geometry_out_ { return Ok(rounded_rectangle_geometry_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateEllipseGeometry(&self, ellipse: &D2D1Ellipse) -> Result<D2D1EllipseGeometry, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut ellipse_geometry_out_: Option<D2D1EllipseGeometry> = None;
			let f: extern "system" fn(Param<Self>, &D2D1Ellipse, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, ellipse, transmute(&mut ellipse_geometry_out_));
			if _ret_.is_ok() { if let Some(ellipse_geometry_out_) = ellipse_geometry_out_ { return Ok(ellipse_geometry_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateGeometryGroup(&self, fill_mode: D2D1FillMode, geometries: &[Param<D2D1Geometry>]) -> Result<D2D1GeometryGroup, HResult> {
		unsafe {
			let vt = self.as_param();
			let (geometries_ptr_, geometries_len_) = geometries.deconstruct();
			let mut geometry_group_out_: Option<D2D1GeometryGroup> = None;
			let f: extern "system" fn(Param<Self>, D2D1FillMode, *const Param<D2D1Geometry>, u32, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, fill_mode, geometries_ptr_, geometries_len_ as u32, transmute(&mut geometry_group_out_));
			if _ret_.is_ok() { if let Some(geometry_group_out_) = geometry_group_out_ { return Ok(geometry_group_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateTransformedGeometry(&self, source_geometry: &D2D1Geometry, transform: &D2DMatrix3x2F) -> Result<D2D1TransformedGeometry, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut transformed_geometry_out_: Option<D2D1TransformedGeometry> = None;
			let f: extern "system" fn(Param<Self>, VTable, &D2DMatrix3x2F, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, source_geometry.vtable(), transform, transmute(&mut transformed_geometry_out_));
			if _ret_.is_ok() { if let Some(transformed_geometry_out_) = transformed_geometry_out_ { return Ok(transformed_geometry_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreatePathGeometry(&self) -> Result<D2D1PathGeometry, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut path_geometry_out_: Option<D2D1PathGeometry> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, transmute(&mut path_geometry_out_));
			if _ret_.is_ok() { if let Some(path_geometry_out_) = path_geometry_out_ { return Ok(path_geometry_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateStrokeStyle(&self, stroke_style_properties: &D2D1StrokeStyleProperties, dashes: Option<&[f32]>) -> Result<D2D1StrokeStyle, HResult> {
		unsafe {
			let vt = self.as_param();
			let (dashes_ptr_, dashes_len_) = dashes.deconstruct();
			let mut stroke_style_out_: Option<D2D1StrokeStyle> = None;
			let f: extern "system" fn(Param<Self>, &D2D1StrokeStyleProperties, *const f32, u32, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, stroke_style_properties, dashes_ptr_, dashes_len_ as u32, transmute(&mut stroke_style_out_));
			if _ret_.is_ok() { if let Some(stroke_style_out_) = stroke_style_out_ { return Ok(stroke_style_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDrawingStateBlock(&self, drawing_state_description: Option<&D2D1DrawingStateDescription>, text_rendering_params: Option<&DWriteRenderingParams>) -> Result<D2D1DrawingStateBlock, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut drawing_state_block_out_: Option<D2D1DrawingStateBlock> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(drawing_state_description), option_to_vtable(text_rendering_params), transmute(&mut drawing_state_block_out_));
			if _ret_.is_ok() { if let Some(drawing_state_block_out_) = drawing_state_block_out_ { return Ok(drawing_state_block_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateWicBitmapRenderTarget(&self, target: &WICBitmap, render_target_properties: &D2D1RenderTargetProperties) -> Result<D2D1RenderTarget, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut render_target_out_: Option<D2D1RenderTarget> = None;
			let f: extern "system" fn(Param<Self>, VTable, &D2D1RenderTargetProperties, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, target.vtable(), render_target_properties, transmute(&mut render_target_out_));
			if _ret_.is_ok() { if let Some(render_target_out_) = render_target_out_ { return Ok(render_target_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateHwndRenderTarget(&self, render_target_properties: &D2D1RenderTargetProperties, hwnd_render_target_properties: &D2D1HWndRenderTargetProperties) -> Result<D2D1HwndRenderTarget, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut hwnd_render_target_out_: Option<D2D1HwndRenderTarget> = None;
			let f: extern "system" fn(Param<Self>, &D2D1RenderTargetProperties, &D2D1HWndRenderTargetProperties, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, render_target_properties, hwnd_render_target_properties, transmute(&mut hwnd_render_target_out_));
			if _ret_.is_ok() { if let Some(hwnd_render_target_out_) = hwnd_render_target_out_ { return Ok(hwnd_render_target_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDxgiSurfaceRenderTarget(&self, dxgi_surface: &DxgiSurface, render_target_properties: &D2D1RenderTargetProperties) -> Result<D2D1RenderTarget, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut render_target_out_: Option<D2D1RenderTarget> = None;
			let f: extern "system" fn(Param<Self>, VTable, &D2D1RenderTargetProperties, *mut c_void) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, dxgi_surface.vtable(), render_target_properties, transmute(&mut render_target_out_));
			if _ret_.is_ok() { if let Some(render_target_out_) = render_target_out_ { return Ok(render_target_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDCRenderTarget(&self, render_target_properties: &D2D1RenderTargetProperties) -> Result<D2D1DCRenderTarget, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut dc_render_target_out_: Option<D2D1DCRenderTarget> = None;
			let f: extern "system" fn(Param<Self>, &D2D1RenderTargetProperties, *mut c_void) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, render_target_properties, transmute(&mut dc_render_target_out_));
			if _ret_.is_ok() { if let Some(dc_render_target_out_) = dc_render_target_out_ { return Ok(dc_render_target_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1Factory: IUnknown {
	fn as_factory(&self) -> &D2D1Factory;
	fn into_factory(self) -> D2D1Factory;
}

impl ID2D1Factory for D2D1Factory {
	fn as_factory(&self) -> &D2D1Factory { self }
	fn into_factory(self) -> D2D1Factory { self }
}
impl IUnknown for D2D1Factory {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Factory {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

