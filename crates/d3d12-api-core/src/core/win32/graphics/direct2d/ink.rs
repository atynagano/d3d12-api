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

use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Ink(pub(crate) D2D1Resource);

impl Deref for D2D1Ink {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Ink {
	const IID: &'static GUID = &GUID::from_u128(0xb499923b7029478fa8b3432c7c5f5312u128);
}

impl Com for D2D1Ink {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Ink {
	pub fn SetStartPoint(&self, start_point: &D2D1InkPoint) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1InkPoint) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, start_point);
		}
	}

	pub fn GetStartPoint(&self) -> D2D1InkPoint {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1InkPoint
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn AddSegments(&self, segments: &[D2D1InkBezierSegment]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (segments_ptr_, segments_len_) = segments.deconstruct();
			let f: extern "system" fn(Param<Self>, *const D2D1InkBezierSegment, u32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, segments_ptr_, segments_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn RemoveSegmentsAtEnd(&self, segments_count: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, segments_count);
			_ret_.ok()
		}
	}

	pub fn SetSegments(&self, start_segment: u32, segments: &[D2D1InkBezierSegment]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (segments_ptr_, segments_len_) = segments.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const D2D1InkBezierSegment, u32) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, start_segment, segments_ptr_, segments_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn SetSegmentAtEnd(&self, segment: &D2D1InkBezierSegment) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1InkBezierSegment) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, segment);
			_ret_.ok()
		}
	}

	pub fn GetSegmentCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[10]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetSegments(&self) { todo!() }

	pub fn StreamAsGeometry(&self, ink_style: Option<&D2D1InkStyle>, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32, geometry_sink: &D2D1SimplifiedGeometrySink) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, f32, VTable) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, option_to_vtable(ink_style), transmute(world_transform), flattening_tolerance, geometry_sink.vtable());
			_ret_.ok()
		}
	}

	pub fn GetBounds(&self, ink_style: Option<&D2D1InkStyle>, world_transform: Option<&D2DMatrix3x2F>) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, *mut D2DRectF) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, option_to_vtable(ink_style), transmute(world_transform), bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1Ink: ID2D1Resource {
	fn as_ink(&self) -> &D2D1Ink;
	fn into_ink(self) -> D2D1Ink;
}

impl ID2D1Ink for D2D1Ink {
	fn as_ink(&self) -> &D2D1Ink { self }
	fn into_ink(self) -> D2D1Ink { self }
}
impl ID2D1Resource for D2D1Ink {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Ink {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Ink {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

