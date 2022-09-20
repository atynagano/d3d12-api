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
pub struct D2D1BlendTransform(pub(crate) D2D1ConcreteTransform);

impl Deref for D2D1BlendTransform {
	type Target = D2D1ConcreteTransform;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1BlendTransform {
	const IID: &'static GUID = &GUID::from_u128(0x63ac0b32ba44450f88067f4ca1ff2f1bu128);
}

impl Com for D2D1BlendTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1BlendTransform {
	pub fn SetDescription(&self, description: &D2D1BlendDescription) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1BlendDescription) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, description);
		}
	}

	pub fn GetDescription(&self) -> D2D1BlendDescription {
		unsafe {
			let vt = self.as_param();
			let mut description_out_: MaybeUninit<D2D1BlendDescription> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2D1BlendDescription) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, description_out_.as_mut_ptr());
			description_out_.assume_init()
		}
	}
}

pub trait ID2D1BlendTransform: ID2D1ConcreteTransform {
	fn as_blend_transform(&self) -> &D2D1BlendTransform;
	fn into_blend_transform(self) -> D2D1BlendTransform;
}

impl ID2D1BlendTransform for D2D1BlendTransform {
	fn as_blend_transform(&self) -> &D2D1BlendTransform { self }
	fn into_blend_transform(self) -> D2D1BlendTransform { self }
}
impl ID2D1ConcreteTransform for D2D1BlendTransform {
	fn as_concrete_transform(&self) -> &D2D1ConcreteTransform { &self.0.as_concrete_transform() }
	fn into_concrete_transform(self) -> D2D1ConcreteTransform { self.0.into_concrete_transform() }
}

impl ID2D1TransformNode for D2D1BlendTransform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1BlendTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1BlendTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1ConcreteTransform::from(v))
    }
}

