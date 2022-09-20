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
pub struct D2D1BoundsAdjustmentTransform(pub(crate) D2D1TransformNode);

impl Deref for D2D1BoundsAdjustmentTransform {
	type Target = D2D1TransformNode;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1BoundsAdjustmentTransform {
	const IID: &'static GUID = &GUID::from_u128(0x90f732e250924606a8198651970baccdu128);
}

impl Com for D2D1BoundsAdjustmentTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1BoundsAdjustmentTransform {
	pub fn SetOutputBounds(&self, output_bounds: &Rect) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &Rect) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, output_bounds);
		}
	}

	pub fn GetOutputBounds(&self) -> Rect {
		unsafe {
			let vt = self.as_param();
			let mut output_bounds_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut Rect) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, output_bounds_out_.as_mut_ptr());
			output_bounds_out_.assume_init()
		}
	}
}

pub trait ID2D1BoundsAdjustmentTransform: ID2D1TransformNode {
	fn as_bounds_adjustment_transform(&self) -> &D2D1BoundsAdjustmentTransform;
	fn into_bounds_adjustment_transform(self) -> D2D1BoundsAdjustmentTransform;
}

impl ID2D1BoundsAdjustmentTransform for D2D1BoundsAdjustmentTransform {
	fn as_bounds_adjustment_transform(&self) -> &D2D1BoundsAdjustmentTransform { self }
	fn into_bounds_adjustment_transform(self) -> D2D1BoundsAdjustmentTransform { self }
}
impl ID2D1TransformNode for D2D1BoundsAdjustmentTransform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1BoundsAdjustmentTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1BoundsAdjustmentTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1TransformNode::from(v))
    }
}

