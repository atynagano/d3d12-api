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

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1DrawTransform(pub(crate) D2D1Transform);

impl Deref for D2D1DrawTransform {
	type Target = D2D1Transform;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DrawTransform {
	const IID: &'static GUID = &GUID::from_u128(0x36bfdcb69739435da30da653beff6a6fu128);
}

impl Com for D2D1DrawTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DrawTransform {
	pub fn SetDrawInfo(&self, draw_info: &D2D1DrawInfo) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, draw_info.vtable());
			_ret_.ok()
		}
	}
}

pub trait ID2D1DrawTransform: ID2D1Transform {
	fn as_draw_transform(&self) -> &D2D1DrawTransform;
	fn into_draw_transform(self) -> D2D1DrawTransform;
}

impl ID2D1DrawTransform for D2D1DrawTransform {
	fn as_draw_transform(&self) -> &D2D1DrawTransform { self }
	fn into_draw_transform(self) -> D2D1DrawTransform { self }
}
impl ID2D1Transform for D2D1DrawTransform {
	fn as_transform(&self) -> &D2D1Transform { &self.0.as_transform() }
	fn into_transform(self) -> D2D1Transform { self.0.into_transform() }
}

impl ID2D1TransformNode for D2D1DrawTransform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1DrawTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DrawTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Transform::from(v))
    }
}

