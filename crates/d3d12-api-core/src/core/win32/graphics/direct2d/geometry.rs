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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Geometry(pub(crate) D2D1Resource);

impl Deref for D2D1Geometry {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Geometry {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906a112e211dc9fed001143a055f9u128);
}

impl Com for D2D1Geometry {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Geometry {
	pub fn GetBounds(&self, world_transform: Option<&D2DMatrix3x2F>) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const c_void, *mut D2DRectF) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(world_transform), bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetWidenedBounds(&self, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, f32, *const c_void, *const c_void, f32, *mut D2DRectF) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, stroke_width, option_to_vtable(stroke_style), transmute(world_transform), flattening_tolerance, bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn StrokeContainsPoint(&self, point: D2DPoint2F, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut contains_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, D2DPoint2F, f32, *const c_void, *const c_void, f32, &mut Bool) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, point, stroke_width, option_to_vtable(stroke_style), transmute(world_transform), flattening_tolerance, &mut contains_out_);
			if _ret_.is_ok() { Ok(contains_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn FillContainsPoint(&self, point: D2DPoint2F, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut contains_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, D2DPoint2F, *const c_void, f32, &mut Bool) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, point, transmute(world_transform), flattening_tolerance, &mut contains_out_);
			if _ret_.is_ok() { Ok(contains_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn CompareWithGeometry(&self, input_geometry: &D2D1Geometry, input_geometry_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32) -> Result<D2D1GeometryRelation, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut relation_out_: MaybeUninit<D2D1GeometryRelation> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, f32, *mut D2D1GeometryRelation) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, input_geometry.vtable(), transmute(input_geometry_transform), flattening_tolerance, relation_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(relation_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn Simplify(&self, simplification_option: D2D1GeometrySimplificationOption, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32, geometry_sink: &D2D1SimplifiedGeometrySink) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1GeometrySimplificationOption, *const c_void, f32, VTable) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, simplification_option, transmute(world_transform), flattening_tolerance, geometry_sink.vtable());
			_ret_.ok()
		}
	}

	pub fn Tessellate(&self, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32, tessellation_sink: &D2D1TessellationSink) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, VTable) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, transmute(world_transform), flattening_tolerance, tessellation_sink.vtable());
			_ret_.ok()
		}
	}

	pub fn CombineWithGeometry(&self, input_geometry: &D2D1Geometry, combine_mode: D2D1CombineMode, input_geometry_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32, geometry_sink: &D2D1SimplifiedGeometrySink) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, D2D1CombineMode, *const c_void, f32, VTable) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, input_geometry.vtable(), combine_mode, transmute(input_geometry_transform), flattening_tolerance, geometry_sink.vtable());
			_ret_.ok()
		}
	}

	pub fn Outline(&self, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32, geometry_sink: &D2D1SimplifiedGeometrySink) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, VTable) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(world_transform), flattening_tolerance, geometry_sink.vtable());
			_ret_.ok()
		}
	}

	pub fn ComputeArea(&self, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32) -> Result<f32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut area_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, *mut f32) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(world_transform), flattening_tolerance, area_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(area_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn ComputeLength(&self, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32) -> Result<f32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut length_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const c_void, f32, *mut f32) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, transmute(world_transform), flattening_tolerance, length_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(length_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn ComputePointAtLength(&self, length: f32, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32, point: Option<&mut MaybeUninit<D2DPoint2F>>, unit_tangent_vector: Option<&mut MaybeUninit<D2DPoint2F>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32, *const c_void, f32, Option<&mut MaybeUninit<D2DPoint2F>>, Option<&mut MaybeUninit<D2DPoint2F>>) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, length, transmute(world_transform), flattening_tolerance, point, unit_tangent_vector);
			_ret_.ok()
		}
	}

	pub fn Widen(&self, stroke_width: f32, stroke_style: Option<&D2D1StrokeStyle>, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32, geometry_sink: &D2D1SimplifiedGeometrySink) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32, *const c_void, *const c_void, f32, VTable) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, stroke_width, option_to_vtable(stroke_style), transmute(world_transform), flattening_tolerance, geometry_sink.vtable());
			_ret_.ok()
		}
	}
}

pub trait ID2D1Geometry: ID2D1Resource {
	fn as_geometry(&self) -> &D2D1Geometry;
	fn into_geometry(self) -> D2D1Geometry;
}

impl ID2D1Geometry for D2D1Geometry {
	fn as_geometry(&self) -> &D2D1Geometry { self }
	fn into_geometry(self) -> D2D1Geometry { self }
}
impl ID2D1Resource for D2D1Geometry {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Geometry {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Geometry {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

