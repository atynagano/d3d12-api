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
pub struct D2D1ComputeTransform(pub(crate) D2D1Transform);

impl Deref for D2D1ComputeTransform {
	type Target = D2D1Transform;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1ComputeTransform {
	const IID: &'static GUID = &GUID::from_u128(0x0d85573c01e34f7dbfd90d60608bf3c3u128);
}

impl Com for D2D1ComputeTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1ComputeTransform {
	pub fn SetComputeInfo(&self, compute_info: &D2D1ComputeInfo) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, compute_info.vtable());
			_ret_.ok()
		}
	}

	pub fn CalculateThreadgroups(&self, output_rect: &Rect) -> Result<(u32, u32, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut dimension_x_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut dimension_y_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut dimension_z_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, &Rect, *mut u32, *mut u32, *mut u32) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, output_rect, dimension_x_out_.as_mut_ptr(), dimension_y_out_.as_mut_ptr(), dimension_z_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((dimension_x_out_.assume_init(), dimension_y_out_.assume_init(), dimension_z_out_.assume_init())) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1ComputeTransform: ID2D1Transform {
	fn as_compute_transform(&self) -> &D2D1ComputeTransform;
	fn into_compute_transform(self) -> D2D1ComputeTransform;
}

impl ID2D1ComputeTransform for D2D1ComputeTransform {
	fn as_compute_transform(&self) -> &D2D1ComputeTransform { self }
	fn into_compute_transform(self) -> D2D1ComputeTransform { self }
}
impl ID2D1Transform for D2D1ComputeTransform {
	fn as_transform(&self) -> &D2D1Transform { &self.0.as_transform() }
	fn into_transform(self) -> D2D1Transform { self.0.into_transform() }
}

impl ID2D1TransformNode for D2D1ComputeTransform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1ComputeTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1ComputeTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Transform::from(v))
    }
}

