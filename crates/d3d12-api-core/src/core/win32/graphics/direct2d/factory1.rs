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
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::graphics::direct_write::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Factory1(pub(crate) D2D1Factory);

impl Deref for D2D1Factory1 {
	type Target = D2D1Factory;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Factory1 {
	const IID: &'static GUID = &GUID::from_u128(0xbb12d362daee4b9aaa1d14ba401cfa1fu128);
}

impl Com for D2D1Factory1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Factory1 {
	pub fn CreateDevice(&self, dxgi_device: &DxgiDevice) -> Result<D2D1Device, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut d2d_device_out_: Option<D2D1Device> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, dxgi_device.vtable(), transmute(&mut d2d_device_out_));
			if _ret_.is_ok() { if let Some(d2d_device_out_) = d2d_device_out_ { return Ok(d2d_device_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateStrokeStyle(&self, stroke_style_properties: &D2D1StrokeStyleProperties1, dashes: Option<&[f32]>) -> Result<D2D1StrokeStyle1, HResult> {
		unsafe {
			let vt = self.as_param();
			let (dashes_ptr_, dashes_len_) = dashes.deconstruct();
			let mut stroke_style_out_: Option<D2D1StrokeStyle1> = None;
			let f: extern "system" fn(Param<Self>, &D2D1StrokeStyleProperties1, *const f32, u32, *mut c_void) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, stroke_style_properties, dashes_ptr_, dashes_len_ as u32, transmute(&mut stroke_style_out_));
			if _ret_.is_ok() { if let Some(stroke_style_out_) = stroke_style_out_ { return Ok(stroke_style_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreatePathGeometry(&self) -> Result<D2D1PathGeometry1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut path_geometry_out_: Option<D2D1PathGeometry1> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, transmute(&mut path_geometry_out_));
			if _ret_.is_ok() { if let Some(path_geometry_out_) = path_geometry_out_ { return Ok(path_geometry_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDrawingStateBlock(&self, drawing_state_description: Option<&D2D1DrawingStateDescription1>, text_rendering_params: Option<&DWriteRenderingParams>) -> Result<D2D1DrawingStateBlock1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut drawing_state_block_out_: Option<D2D1DrawingStateBlock1> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, *mut c_void) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, transmute(drawing_state_description), option_to_vtable(text_rendering_params), transmute(&mut drawing_state_block_out_));
			if _ret_.is_ok() { if let Some(drawing_state_block_out_) = drawing_state_block_out_ { return Ok(drawing_state_block_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateGdiMetafile(&self, metafile_stream: &Stream) -> Result<D2D1GdiMetafile, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut metafile_out_: Option<D2D1GdiMetafile> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, metafile_stream.vtable(), transmute(&mut metafile_out_));
			if _ret_.is_ok() { if let Some(metafile_out_) = metafile_out_ { return Ok(metafile_out_); } }
			Err(_ret_)
		}
	}

	pub fn RegisterEffectFromStream(&self, class_id: &GUID, property_xml: &Stream, bindings: Option<&[D2D1PropertyBinding]>, effect_factory: PD2d1EffectFactory) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (bindings_ptr_, bindings_len_) = bindings.deconstruct();
			let f: extern "system" fn(Param<Self>, &GUID, VTable, *const D2D1PropertyBinding, u32, PD2d1EffectFactory) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, class_id, property_xml.vtable(), bindings_ptr_, bindings_len_ as u32, effect_factory);
			_ret_.ok()
		}
	}

	pub fn RegisterEffectFromString(&self, class_id: &GUID, property_xml: &str, bindings: Option<&[D2D1PropertyBinding]>, effect_factory: PD2d1EffectFactory) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (bindings_ptr_, bindings_len_) = bindings.deconstruct();
			let f: extern "system" fn(Param<Self>, &GUID, *const u16, *const D2D1PropertyBinding, u32, PD2d1EffectFactory) -> HResult
				= transmute(vt[23]);
			let _ret_ = f(vt, class_id, property_xml.to_utf16().as_ptr_or_null(), bindings_ptr_, bindings_len_ as u32, effect_factory);
			_ret_.ok()
		}
	}

	pub fn UnregisterEffect(&self, class_id: &GUID) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &GUID) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, class_id);
			_ret_.ok()
		}
	}

	pub unsafe fn GetRegisteredEffects(&self) { todo!() }

	pub fn GetEffectProperties(&self, effect_id: &GUID) -> Result<D2D1Properties, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut properties_out_: Option<D2D1Properties> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, effect_id, transmute(&mut properties_out_));
			if _ret_.is_ok() { if let Some(properties_out_) = properties_out_ { return Ok(properties_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1Factory1: ID2D1Factory {
	fn as_factory1(&self) -> &D2D1Factory1;
	fn into_factory1(self) -> D2D1Factory1;
}

impl ID2D1Factory1 for D2D1Factory1 {
	fn as_factory1(&self) -> &D2D1Factory1 { self }
	fn into_factory1(self) -> D2D1Factory1 { self }
}
impl ID2D1Factory for D2D1Factory1 {
	fn as_factory(&self) -> &D2D1Factory { &self.0.as_factory() }
	fn into_factory(self) -> D2D1Factory { self.0.into_factory() }
}

impl IUnknown for D2D1Factory1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Factory1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Factory::from(v))
    }
}

