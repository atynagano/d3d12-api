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


#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1TransformNode(pub(crate) Unknown);

impl Deref for D2D1TransformNode {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1TransformNode {
	const IID: &'static GUID = &GUID::from_u128(0xb2efe1e7729f4102949f505fa21bf666u128);
}

impl Com for D2D1TransformNode {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1TransformNode {
	pub fn GetInputCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1TransformNode: IUnknown {
	fn as_transform_node(&self) -> &D2D1TransformNode;
	fn into_transform_node(self) -> D2D1TransformNode;
}

impl ID2D1TransformNode for D2D1TransformNode {
	fn as_transform_node(&self) -> &D2D1TransformNode { self }
	fn into_transform_node(self) -> D2D1TransformNode { self }
}
impl IUnknown for D2D1TransformNode {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1TransformNode {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

