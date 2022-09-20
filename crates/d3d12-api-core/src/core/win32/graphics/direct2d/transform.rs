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
pub struct D2D1Transform(pub(crate) D2D1TransformNode);

impl Deref for D2D1Transform {
	type Target = D2D1TransformNode;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Transform {
	const IID: &'static GUID = &GUID::from_u128(0xef1a287d342a4f768fdbda0d6ea9f92bu128);
}

impl Com for D2D1Transform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Transform {
	pub unsafe fn MapOutputRectToInputRects(&self) { todo!() }

	pub unsafe fn MapInputRectsToOutputRect() { todo!() }

	pub fn MapInvalidRect(&self, input_index: u32, invalid_input_rect: Rect) -> Result<Rect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut invalid_output_rect_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, Rect, *mut Rect) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, input_index, invalid_input_rect, invalid_output_rect_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(invalid_output_rect_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1Transform: ID2D1TransformNode {
	fn as_transform(&self) -> &D2D1Transform;
	fn into_transform(self) -> D2D1Transform;
}

impl ID2D1Transform for D2D1Transform {
	fn as_transform(&self) -> &D2D1Transform { self }
	fn into_transform(self) -> D2D1Transform { self }
}
impl ID2D1TransformNode for D2D1Transform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1Transform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Transform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1TransformNode::from(v))
    }
}

