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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Brush(pub(crate) D2D1Resource);

impl Deref for D2D1Brush {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Brush {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906a812e211dc9fed001143a055f9u128);
}

impl Com for D2D1Brush {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Brush {
	pub fn SetOpacity(&self, opacity: f32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, f32) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, opacity);
		}
	}

	pub fn SetTransform(&self, transform: &D2DMatrix3x2F) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DMatrix3x2F) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, transform);
		}
	}

	pub fn GetOpacity(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetTransform(&self) -> D2DMatrix3x2F {
		unsafe {
			let vt = self.as_param();
			let mut transform_out_: MaybeUninit<D2DMatrix3x2F> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2DMatrix3x2F) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, transform_out_.as_mut_ptr());
			transform_out_.assume_init()
		}
	}
}

pub trait ID2D1Brush: ID2D1Resource {
	fn as_brush(&self) -> &D2D1Brush;
	fn into_brush(self) -> D2D1Brush;
}

impl ID2D1Brush for D2D1Brush {
	fn as_brush(&self) -> &D2D1Brush { self }
	fn into_brush(self) -> D2D1Brush { self }
}
impl ID2D1Resource for D2D1Brush {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Brush {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Brush {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

