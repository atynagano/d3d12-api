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
use crate::core::win32::graphics::imaging::*;
use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::graphics::dxgi::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1DeviceContext2(pub(crate) D2D1DeviceContext1);

impl Deref for D2D1DeviceContext2 {
	type Target = D2D1DeviceContext1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DeviceContext2 {
	const IID: &'static GUID = &GUID::from_u128(0x394ea6a30c344321950b6ca20f0be6c7u128);
}

impl Com for D2D1DeviceContext2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DeviceContext2 {
	pub fn CreateInk(&self, start_point: &D2D1InkPoint) -> Result<D2D1Ink, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut ink_out_: Option<D2D1Ink> = None;
			let f: extern "system" fn(Param<Self>, &D2D1InkPoint, *mut c_void) -> HResult
				= transmute(vt[95]);
			let _ret_ = f(vt, start_point, transmute(&mut ink_out_));
			if _ret_.is_ok() { if let Some(ink_out_) = ink_out_ { return Ok(ink_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateInkStyle(&self, ink_style_properties: Option<&D2D1InkStyleProperties>) -> Result<D2D1InkStyle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut ink_style_out_: Option<D2D1InkStyle> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, *mut c_void) -> HResult
				= transmute(vt[96]);
			let _ret_ = f(vt, transmute(ink_style_properties), transmute(&mut ink_style_out_));
			if _ret_.is_ok() { if let Some(ink_style_out_) = ink_style_out_ { return Ok(ink_style_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateGradientMesh(&self, patches: &[D2D1GradientMeshPatch]) -> Result<D2D1GradientMesh, HResult> {
		unsafe {
			let vt = self.as_param();
			let (patches_ptr_, patches_len_) = patches.deconstruct();
			let mut gradient_mesh_out_: Option<D2D1GradientMesh> = None;
			let f: extern "system" fn(Param<Self>, *const D2D1GradientMeshPatch, u32, *mut c_void) -> HResult
				= transmute(vt[97]);
			let _ret_ = f(vt, patches_ptr_, patches_len_ as u32, transmute(&mut gradient_mesh_out_));
			if _ret_.is_ok() { if let Some(gradient_mesh_out_) = gradient_mesh_out_ { return Ok(gradient_mesh_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateImageSourceFromWic(&self, wic_bitmap_source: &WICBitmapSource, loading_options: D2D1ImageSourceLoadingOptions, alpha_mode: D2D1AlphaMode) -> Result<D2D1ImageSourceFromWic, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut image_source_out_: Option<D2D1ImageSourceFromWic> = None;
			let f: extern "system" fn(Param<Self>, VTable, D2D1ImageSourceLoadingOptions, D2D1AlphaMode, *mut c_void) -> HResult
				= transmute(vt[98]);
			let _ret_ = f(vt, wic_bitmap_source.vtable(), loading_options, alpha_mode, transmute(&mut image_source_out_));
			if _ret_.is_ok() { if let Some(image_source_out_) = image_source_out_ { return Ok(image_source_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateLookupTable3D(&self, precision: D2D1BufferPrecision, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> Result<D2D1LookupTable3D, HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let mut lookup_table_out_: Option<D2D1LookupTable3D> = None;
			let f: extern "system" fn(Param<Self>, D2D1BufferPrecision, *const u32, *const u8, u32, *const u32, *mut c_void) -> HResult
				= transmute(vt[99]);
			let _ret_ = f(vt, precision, extents.as_ptr_or_null(), data_ptr_, data_len_ as u32, strides.as_ptr_or_null(), transmute(&mut lookup_table_out_));
			if _ret_.is_ok() { if let Some(lookup_table_out_) = lookup_table_out_ { return Ok(lookup_table_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateImageSourceFromDxgi(&self, surfaces: &[Param<DxgiSurface>], color_space: DxgiColorSpaceType, options: D2D1ImageSourceFromDxgiOptions) -> Result<D2D1ImageSource, HResult> {
		unsafe {
			let vt = self.as_param();
			let (surfaces_ptr_, surfaces_len_) = surfaces.deconstruct();
			let mut image_source_out_: Option<D2D1ImageSource> = None;
			let f: extern "system" fn(Param<Self>, *const Param<DxgiSurface>, u32, DxgiColorSpaceType, D2D1ImageSourceFromDxgiOptions, *mut c_void) -> HResult
				= transmute(vt[100]);
			let _ret_ = f(vt, surfaces_ptr_, surfaces_len_ as u32, color_space, options, transmute(&mut image_source_out_));
			if _ret_.is_ok() { if let Some(image_source_out_) = image_source_out_ { return Ok(image_source_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetGradientMeshWorldBounds(&self, gradient_mesh: &D2D1GradientMesh) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, VTable, *mut D2DRectF) -> HResult
				= transmute(vt[101]);
			let _ret_ = f(vt, gradient_mesh.vtable(), bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn DrawInk(&self, ink: &D2D1Ink, brush: &D2D1Brush, ink_style: Option<&D2D1InkStyle>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable, *const c_void) -> ()
				= transmute(vt[102]);
			let _ret_ = f(vt, ink.vtable(), brush.vtable(), option_to_vtable(ink_style));
		}
	}

	pub fn DrawGradientMesh(&self, gradient_mesh: &D2D1GradientMesh) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> ()
				= transmute(vt[103]);
			let _ret_ = f(vt, gradient_mesh.vtable());
		}
	}

	pub fn DrawGdiMetafile(&self, gdi_metafile: &D2D1GdiMetafile, destination_rectangle: Option<&D2DRectF>, source_rectangle: Option<&D2DRectF>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, *const c_void) -> ()
				= transmute(vt[104]);
			let _ret_ = f(vt, gdi_metafile.vtable(), transmute(destination_rectangle), transmute(source_rectangle));
		}
	}

	pub fn CreateTransformedImageSource(&self, image_source: &D2D1ImageSource, properties: &D2D1TransformedImageSourceProperties) -> Result<D2D1TransformedImageSource, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut transformed_image_source_out_: Option<D2D1TransformedImageSource> = None;
			let f: extern "system" fn(Param<Self>, VTable, &D2D1TransformedImageSourceProperties, *mut c_void) -> HResult
				= transmute(vt[105]);
			let _ret_ = f(vt, image_source.vtable(), properties, transmute(&mut transformed_image_source_out_));
			if _ret_.is_ok() { if let Some(transformed_image_source_out_) = transformed_image_source_out_ { return Ok(transformed_image_source_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1DeviceContext2: ID2D1DeviceContext1 {
	fn as_device_context2(&self) -> &D2D1DeviceContext2;
	fn into_device_context2(self) -> D2D1DeviceContext2;
}

impl ID2D1DeviceContext2 for D2D1DeviceContext2 {
	fn as_device_context2(&self) -> &D2D1DeviceContext2 { self }
	fn into_device_context2(self) -> D2D1DeviceContext2 { self }
}
impl ID2D1DeviceContext1 for D2D1DeviceContext2 {
	fn as_device_context1(&self) -> &D2D1DeviceContext1 { &self.0.as_device_context1() }
	fn into_device_context1(self) -> D2D1DeviceContext1 { self.0.into_device_context1() }
}

impl ID2D1DeviceContext for D2D1DeviceContext2 {
	fn as_device_context(&self) -> &D2D1DeviceContext { &self.0.as_device_context() }
	fn into_device_context(self) -> D2D1DeviceContext { self.0.into_device_context() }
}

impl ID2D1RenderTarget for D2D1DeviceContext2 {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1DeviceContext2 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DeviceContext2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DeviceContext2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1DeviceContext1::from(v))
    }
}

