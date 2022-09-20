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
pub struct D2D1DeviceContext1(pub(crate) D2D1DeviceContext);

impl Deref for D2D1DeviceContext1 {
	type Target = D2D1DeviceContext;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DeviceContext1 {
	const IID: &'static GUID = &GUID::from_u128(0xd37f57e46908459fa199e72f24f79987u128);
}

impl Com for D2D1DeviceContext1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DeviceContext1 {
	pub fn CreateFilledGeometryRealization(&self, geometry: &D2D1Geometry, flattening_tolerance: f32) -> Result<D2D1GeometryRealization, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut geometry_realization_out_: Option<D2D1GeometryRealization> = None;
			let f: extern "system" fn(Param<Self>, VTable, f32, *mut c_void) -> HResult
				= transmute(vt[92]);
			let _ret_ = f(vt, geometry.vtable(), flattening_tolerance, transmute(&mut geometry_realization_out_));
			if _ret_.is_ok() { if let Some(geometry_realization_out_) = geometry_realization_out_ { return Ok(geometry_realization_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateStrokedGeometryRealization(&self, geometry: &D2D1Geometry, flattening_tolerance: f32, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>) -> Result<D2D1GeometryRealization, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut geometry_realization_out_: Option<D2D1GeometryRealization> = None;
			let f: extern "system" fn(Param<Self>, VTable, f32, f32, *const c_void, *mut c_void) -> HResult
				= transmute(vt[93]);
			let _ret_ = f(vt, geometry.vtable(), flattening_tolerance, stroke_width, option_to_vtable(stroke_style), transmute(&mut geometry_realization_out_));
			if _ret_.is_ok() { if let Some(geometry_realization_out_) = geometry_realization_out_ { return Ok(geometry_realization_out_); } }
			Err(_ret_)
		}
	}

	pub fn DrawGeometryRealization(&self, geometry_realization: &D2D1GeometryRealization, brush: &D2D1Brush) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable) -> ()
				= transmute(vt[94]);
			let _ret_ = f(vt, geometry_realization.vtable(), brush.vtable());
		}
	}
}

pub trait ID2D1DeviceContext1: ID2D1DeviceContext {
	fn as_device_context1(&self) -> &D2D1DeviceContext1;
	fn into_device_context1(self) -> D2D1DeviceContext1;
}

impl ID2D1DeviceContext1 for D2D1DeviceContext1 {
	fn as_device_context1(&self) -> &D2D1DeviceContext1 { self }
	fn into_device_context1(self) -> D2D1DeviceContext1 { self }
}
impl ID2D1DeviceContext for D2D1DeviceContext1 {
	fn as_device_context(&self) -> &D2D1DeviceContext { &self.0.as_device_context() }
	fn into_device_context(self) -> D2D1DeviceContext { self.0.into_device_context() }
}

impl ID2D1RenderTarget for D2D1DeviceContext1 {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1DeviceContext1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DeviceContext1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DeviceContext1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1DeviceContext::from(v))
    }
}

