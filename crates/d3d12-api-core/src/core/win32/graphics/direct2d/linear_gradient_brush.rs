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
pub struct D2D1LinearGradientBrush(pub(crate) D2D1Brush);

impl Deref for D2D1LinearGradientBrush {
	type Target = D2D1Brush;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1LinearGradientBrush {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906ab12e211dc9fed001143a055f9u128);
}

impl Com for D2D1LinearGradientBrush {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1LinearGradientBrush {
	pub fn SetStartPoint(&self, start_point: D2DPoint2F) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, start_point);
		}
	}

	pub fn SetEndPoint(&self, end_point: D2DPoint2F) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DPoint2F) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, end_point);
		}
	}

	pub fn GetStartPoint(&self) -> D2DPoint2F {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DPoint2F
				= transmute(vt[10]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetEndPoint(&self) -> D2DPoint2F {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DPoint2F
				= transmute(vt[11]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetGradientStopCollection(&self) -> Option<D2D1GradientStopCollection> {
		unsafe {
			let vt = self.as_param();
			let mut gradient_stop_collection_out_: Option<D2D1GradientStopCollection> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(&mut gradient_stop_collection_out_));
			gradient_stop_collection_out_
		}
	}
}

pub trait ID2D1LinearGradientBrush: ID2D1Brush {
	fn as_linear_gradient_brush(&self) -> &D2D1LinearGradientBrush;
	fn into_linear_gradient_brush(self) -> D2D1LinearGradientBrush;
}

impl ID2D1LinearGradientBrush for D2D1LinearGradientBrush {
	fn as_linear_gradient_brush(&self) -> &D2D1LinearGradientBrush { self }
	fn into_linear_gradient_brush(self) -> D2D1LinearGradientBrush { self }
}
impl ID2D1Brush for D2D1LinearGradientBrush {
	fn as_brush(&self) -> &D2D1Brush { &self.0.as_brush() }
	fn into_brush(self) -> D2D1Brush { self.0.into_brush() }
}

impl ID2D1Resource for D2D1LinearGradientBrush {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1LinearGradientBrush {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1LinearGradientBrush {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Brush::from(v))
    }
}

