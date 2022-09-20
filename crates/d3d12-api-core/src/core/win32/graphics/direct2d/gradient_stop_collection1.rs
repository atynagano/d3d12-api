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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1GradientStopCollection1(pub(crate) D2D1GradientStopCollection);

impl Deref for D2D1GradientStopCollection1 {
	type Target = D2D1GradientStopCollection;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GradientStopCollection1 {
	const IID: &'static GUID = &GUID::from_u128(0xae1572f45dd04777998b9279472ae63bu128);
}

impl Com for D2D1GradientStopCollection1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GradientStopCollection1 {
	pub unsafe fn GetGradientStops1(&self) { todo!() }

	pub fn GetPreInterpolationSpace(&self) -> D2D1ColorSpace {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ColorSpace
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetPostInterpolationSpace(&self) -> D2D1ColorSpace {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ColorSpace
				= transmute(vt[10]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetBufferPrecision(&self) -> D2D1BufferPrecision {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1BufferPrecision
				= transmute(vt[11]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetColorInterpolationMode(&self) -> D2D1ColorInterpolationMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ColorInterpolationMode
				= transmute(vt[12]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1GradientStopCollection1: ID2D1GradientStopCollection {
	fn as_gradient_stop_collection1(&self) -> &D2D1GradientStopCollection1;
	fn into_gradient_stop_collection1(self) -> D2D1GradientStopCollection1;
}

impl ID2D1GradientStopCollection1 for D2D1GradientStopCollection1 {
	fn as_gradient_stop_collection1(&self) -> &D2D1GradientStopCollection1 { self }
	fn into_gradient_stop_collection1(self) -> D2D1GradientStopCollection1 { self }
}
impl ID2D1GradientStopCollection for D2D1GradientStopCollection1 {
	fn as_gradient_stop_collection(&self) -> &D2D1GradientStopCollection { &self.0.as_gradient_stop_collection() }
	fn into_gradient_stop_collection(self) -> D2D1GradientStopCollection { self.0.into_gradient_stop_collection() }
}

impl ID2D1Resource for D2D1GradientStopCollection1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1GradientStopCollection1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GradientStopCollection1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1GradientStopCollection::from(v))
    }
}

