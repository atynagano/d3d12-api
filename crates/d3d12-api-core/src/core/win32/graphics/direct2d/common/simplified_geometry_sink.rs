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

use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SimplifiedGeometrySink(pub(crate) Unknown);

impl Deref for D2D1SimplifiedGeometrySink {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SimplifiedGeometrySink {
	const IID: &'static GUID = &GUID::from_u128(0x2cd9069e12e211dc9fed001143a055f9u128);
}

impl Com for D2D1SimplifiedGeometrySink {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SimplifiedGeometrySink {
	pub fn SetFillMode(&self, fill_mode: D2D1FillMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1FillMode) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, fill_mode);
		}
	}

	pub fn SetSegmentFlags(&self, vertex_flags: D2D1PathSegment) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1PathSegment) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, vertex_flags);
		}
	}

	pub fn BeginFigure(&self, start_point: D2DPoint2F, figure_begin: D2D1FigureBegin) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F, D2D1FigureBegin) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, start_point, figure_begin);
		}
	}

	pub fn AddLines(&self, points: &[D2DPoint2F]) -> () {
		unsafe {
			let vt = self.as_param();
			let (points_ptr_, points_len_) = points.deconstruct();
			let f: extern "system" fn(Param<Self>, *const D2DPoint2F, u32) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, points_ptr_, points_len_ as u32);
		}
	}

	pub fn AddBeziers(&self, beziers: &[D2D1BezierSegment]) -> () {
		unsafe {
			let vt = self.as_param();
			let (beziers_ptr_, beziers_len_) = beziers.deconstruct();
			let f: extern "system" fn(Param<Self>, *const D2D1BezierSegment, u32) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, beziers_ptr_, beziers_len_ as u32);
		}
	}

	pub fn EndFigure(&self, figure_end: D2D1FigureEnd) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1FigureEnd) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, figure_end);
		}
	}

	pub fn Close(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait ID2D1SimplifiedGeometrySink: IUnknown {
	fn as_simplified_geometry_sink(&self) -> &D2D1SimplifiedGeometrySink;
	fn into_simplified_geometry_sink(self) -> D2D1SimplifiedGeometrySink;
}

impl ID2D1SimplifiedGeometrySink for D2D1SimplifiedGeometrySink {
	fn as_simplified_geometry_sink(&self) -> &D2D1SimplifiedGeometrySink { self }
	fn into_simplified_geometry_sink(self) -> D2D1SimplifiedGeometrySink { self }
}
impl IUnknown for D2D1SimplifiedGeometrySink {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SimplifiedGeometrySink {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

