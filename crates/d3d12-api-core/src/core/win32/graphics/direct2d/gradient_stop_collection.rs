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
pub struct D2D1GradientStopCollection(pub(crate) D2D1Resource);

impl Deref for D2D1GradientStopCollection {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GradientStopCollection {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906a712e211dc9fed001143a055f9u128);
}

impl Com for D2D1GradientStopCollection {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GradientStopCollection {
	pub fn GetGradientStopCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetGradientStops(&self) { todo!() }

	pub fn GetColorInterpolationGamma(&self) -> D2D1Gamma {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1Gamma
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetExtendMode(&self) -> D2D1ExtendMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ExtendMode
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1GradientStopCollection: ID2D1Resource {
	fn as_gradient_stop_collection(&self) -> &D2D1GradientStopCollection;
	fn into_gradient_stop_collection(self) -> D2D1GradientStopCollection;
}

impl ID2D1GradientStopCollection for D2D1GradientStopCollection {
	fn as_gradient_stop_collection(&self) -> &D2D1GradientStopCollection { self }
	fn into_gradient_stop_collection(self) -> D2D1GradientStopCollection { self }
}
impl ID2D1Resource for D2D1GradientStopCollection {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1GradientStopCollection {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GradientStopCollection {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

