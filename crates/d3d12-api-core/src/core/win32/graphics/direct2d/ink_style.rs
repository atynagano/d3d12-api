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
pub struct D2D1InkStyle(pub(crate) D2D1Resource);

impl Deref for D2D1InkStyle {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1InkStyle {
	const IID: &'static GUID = &GUID::from_u128(0xbae8b34423fc40718cb5d05d6f073848u128);
}

impl Com for D2D1InkStyle {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1InkStyle {
	pub fn SetNibTransform(&self, transform: &D2DMatrix3x2F) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DMatrix3x2F) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, transform);
		}
	}

	pub fn GetNibTransform(&self) -> D2DMatrix3x2F {
		unsafe {
			let vt = self.as_param();
			let mut transform_out_: MaybeUninit<D2DMatrix3x2F> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2DMatrix3x2F) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, transform_out_.as_mut_ptr());
			transform_out_.assume_init()
		}
	}

	pub fn SetNibShape(&self, nib_shape: D2D1InkNibShape) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1InkNibShape) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, nib_shape);
		}
	}

	pub fn GetNibShape(&self) -> D2D1InkNibShape {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1InkNibShape
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1InkStyle: ID2D1Resource {
	fn as_ink_style(&self) -> &D2D1InkStyle;
	fn into_ink_style(self) -> D2D1InkStyle;
}

impl ID2D1InkStyle for D2D1InkStyle {
	fn as_ink_style(&self) -> &D2D1InkStyle { self }
	fn into_ink_style(self) -> D2D1InkStyle { self }
}
impl ID2D1Resource for D2D1InkStyle {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1InkStyle {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1InkStyle {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

