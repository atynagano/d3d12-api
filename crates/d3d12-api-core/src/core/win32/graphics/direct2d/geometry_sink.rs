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
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1GeometrySink(pub(crate) D2D1SimplifiedGeometrySink);

impl Deref for D2D1GeometrySink {
	type Target = D2D1SimplifiedGeometrySink;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GeometrySink {
	const IID: &'static GUID = &GUID::from_u128(0x2cd9069f12e211dc9fed001143a055f9u128);
}

impl Com for D2D1GeometrySink {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GeometrySink {
	pub fn AddLine(&self, point: D2DPoint2F) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, point);
		}
	}

	pub fn AddBezier(&self, bezier: &D2D1BezierSegment) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1BezierSegment) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, bezier);
		}
	}

	pub fn AddQuadraticBezier(&self, bezier: &D2D1QuadraticBezierSegment) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1QuadraticBezierSegment) -> ()
				= transmute(vt[12]);
			let _ret_ = f(vt, bezier);
		}
	}

	pub fn AddQuadraticBeziers(&self, beziers: &[D2D1QuadraticBezierSegment]) -> () {
		unsafe {
			let vt = self.as_param();
			let (beziers_ptr_, beziers_len_) = beziers.deconstruct();
			let f: extern "system" fn(Param<Self>, *const D2D1QuadraticBezierSegment, u32) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt, beziers_ptr_, beziers_len_ as u32);
		}
	}

	pub fn AddArc(&self, arc: &D2D1ArcSegment) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1ArcSegment) -> ()
				= transmute(vt[14]);
			let _ret_ = f(vt, arc);
		}
	}
}

pub trait ID2D1GeometrySink: ID2D1SimplifiedGeometrySink {
	fn as_geometry_sink(&self) -> &D2D1GeometrySink;
	fn into_geometry_sink(self) -> D2D1GeometrySink;
}

impl ID2D1GeometrySink for D2D1GeometrySink {
	fn as_geometry_sink(&self) -> &D2D1GeometrySink { self }
	fn into_geometry_sink(self) -> D2D1GeometrySink { self }
}
impl ID2D1SimplifiedGeometrySink for D2D1GeometrySink {
	fn as_simplified_geometry_sink(&self) -> &D2D1SimplifiedGeometrySink { &self.0.as_simplified_geometry_sink() }
	fn into_simplified_geometry_sink(self) -> D2D1SimplifiedGeometrySink { self.0.into_simplified_geometry_sink() }
}

impl IUnknown for D2D1GeometrySink {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GeometrySink {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1SimplifiedGeometrySink::from(v))
    }
}

