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
pub struct D2D1RadialGradientBrush(pub(crate) D2D1Brush);

impl Deref for D2D1RadialGradientBrush {
	type Target = D2D1Brush;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1RadialGradientBrush {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906ac12e211dc9fed001143a055f9u128);
}

impl Com for D2D1RadialGradientBrush {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1RadialGradientBrush {
	pub fn SetCenter(&self, center: D2DPoint2F) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, center);
		}
	}

	pub fn SetGradientOriginOffset(&self, gradient_origin_offset: D2DPoint2F) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, gradient_origin_offset);
		}
	}

	pub fn SetRadiusX(&self, radius_x: f32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, radius_x);
		}
	}

	pub fn SetRadiusY(&self, radius_y: f32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, radius_y);
		}
	}

	pub fn GetCenter(&self) -> D2DPoint2F {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DPoint2F
				= transmute(vt[12]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetGradientOriginOffset(&self) -> D2DPoint2F {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DPoint2F
				= transmute(vt[13]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetRadiusX(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[14]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetRadiusY(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[15]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetGradientStopCollection(&self) -> Option<D2D1GradientStopCollection> {
		unsafe {
			let vt = self.as_param();
			let mut gradient_stop_collection_out_: Option<D2D1GradientStopCollection> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[16]);
			let _ret_ = f(vt, transmute(&mut gradient_stop_collection_out_));
			gradient_stop_collection_out_
		}
	}
}

pub trait ID2D1RadialGradientBrush: ID2D1Brush {
	fn as_radial_gradient_brush(&self) -> &D2D1RadialGradientBrush;
	fn into_radial_gradient_brush(self) -> D2D1RadialGradientBrush;
}

impl ID2D1RadialGradientBrush for D2D1RadialGradientBrush {
	fn as_radial_gradient_brush(&self) -> &D2D1RadialGradientBrush { self }
	fn into_radial_gradient_brush(self) -> D2D1RadialGradientBrush { self }
}
impl ID2D1Brush for D2D1RadialGradientBrush {
	fn as_brush(&self) -> &D2D1Brush { &self.0.as_brush() }
	fn into_brush(self) -> D2D1Brush { self.0.into_brush() }
}

impl ID2D1Resource for D2D1RadialGradientBrush {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1RadialGradientBrush {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1RadialGradientBrush {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Brush::from(v))
    }
}

