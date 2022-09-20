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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1OffsetTransform(pub(crate) D2D1TransformNode);

impl Deref for D2D1OffsetTransform {
	type Target = D2D1TransformNode;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1OffsetTransform {
	const IID: &'static GUID = &GUID::from_u128(0x3fe6adea76434f53bd14a0ce63f24042u128);
}

impl Com for D2D1OffsetTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1OffsetTransform {
	pub fn SetOffset(&self, offset: Point) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Point) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, offset);
		}
	}

	pub fn GetOffset(&self) -> Point {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Point
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1OffsetTransform: ID2D1TransformNode {
	fn as_offset_transform(&self) -> &D2D1OffsetTransform;
	fn into_offset_transform(self) -> D2D1OffsetTransform;
}

impl ID2D1OffsetTransform for D2D1OffsetTransform {
	fn as_offset_transform(&self) -> &D2D1OffsetTransform { self }
	fn into_offset_transform(self) -> D2D1OffsetTransform { self }
}
impl ID2D1TransformNode for D2D1OffsetTransform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1OffsetTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1OffsetTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1TransformNode::from(v))
    }
}

