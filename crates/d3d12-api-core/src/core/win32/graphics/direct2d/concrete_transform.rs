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
pub struct D2D1ConcreteTransform(pub(crate) D2D1TransformNode);

impl Deref for D2D1ConcreteTransform {
	type Target = D2D1TransformNode;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1ConcreteTransform {
	const IID: &'static GUID = &GUID::from_u128(0x1a799d8a69f74e4c9fed437ccc6684ccu128);
}

impl Com for D2D1ConcreteTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1ConcreteTransform {
	pub fn SetOutputBuffer(&self, buffer_precision: D2D1BufferPrecision, channel_depth: D2D1ChannelDepth) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1BufferPrecision, D2D1ChannelDepth) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, buffer_precision, channel_depth);
			_ret_.ok()
		}
	}

	pub fn SetCached(&self, is_cached: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, is_cached.to_bool());
		}
	}
}

pub trait ID2D1ConcreteTransform: ID2D1TransformNode {
	fn as_concrete_transform(&self) -> &D2D1ConcreteTransform;
	fn into_concrete_transform(self) -> D2D1ConcreteTransform;
}

impl ID2D1ConcreteTransform for D2D1ConcreteTransform {
	fn as_concrete_transform(&self) -> &D2D1ConcreteTransform { self }
	fn into_concrete_transform(self) -> D2D1ConcreteTransform { self }
}
impl ID2D1TransformNode for D2D1ConcreteTransform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1ConcreteTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1ConcreteTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1TransformNode::from(v))
    }
}

