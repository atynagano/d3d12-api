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
pub struct D2D1BorderTransform(pub(crate) D2D1ConcreteTransform);

impl Deref for D2D1BorderTransform {
	type Target = D2D1ConcreteTransform;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1BorderTransform {
	const IID: &'static GUID = &GUID::from_u128(0x4998735c3a19473c9781656847e3a347u128);
}

impl Com for D2D1BorderTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1BorderTransform {
	pub fn SetExtendModeX(&self, extend_mode: D2D1ExtendMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1ExtendMode) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, extend_mode);
		}
	}

	pub fn SetExtendModeY(&self, extend_mode: D2D1ExtendMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1ExtendMode) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, extend_mode);
		}
	}

	pub fn GetExtendModeX(&self) -> D2D1ExtendMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ExtendMode
				= transmute(vt[8]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetExtendModeY(&self) -> D2D1ExtendMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ExtendMode
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1BorderTransform: ID2D1ConcreteTransform {
	fn as_border_transform(&self) -> &D2D1BorderTransform;
	fn into_border_transform(self) -> D2D1BorderTransform;
}

impl ID2D1BorderTransform for D2D1BorderTransform {
	fn as_border_transform(&self) -> &D2D1BorderTransform { self }
	fn into_border_transform(self) -> D2D1BorderTransform { self }
}
impl ID2D1ConcreteTransform for D2D1BorderTransform {
	fn as_concrete_transform(&self) -> &D2D1ConcreteTransform { &self.0.as_concrete_transform() }
	fn into_concrete_transform(self) -> D2D1ConcreteTransform { self.0.into_concrete_transform() }
}

impl ID2D1TransformNode for D2D1BorderTransform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1BorderTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1BorderTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1ConcreteTransform::from(v))
    }
}

