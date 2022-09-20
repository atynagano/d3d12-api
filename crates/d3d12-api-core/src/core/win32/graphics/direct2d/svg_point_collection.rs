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
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SvgPointCollection(pub(crate) D2D1SvgAttribute);

impl Deref for D2D1SvgPointCollection {
	type Target = D2D1SvgAttribute;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SvgPointCollection {
	const IID: &'static GUID = &GUID::from_u128(0x9dbe4c0d35724dd998255530813bb712u128);
}

impl Com for D2D1SvgPointCollection {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SvgPointCollection {
	pub fn RemovePointsAtEnd(&self, points_count: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, points_count);
			_ret_.ok()
		}
	}

	pub fn UpdatePoints(&self, points: &[D2DPoint2F], start_index: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (points_ptr_, points_len_) = points.deconstruct();
			let f: extern "system" fn(Param<Self>, *const D2DPoint2F, u32, u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, points_ptr_, points_len_ as u32, start_index);
			_ret_.ok()
		}
	}

	pub unsafe fn GetPoints(&self) { todo!() }

	pub fn GetPointsCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1SvgPointCollection: ID2D1SvgAttribute {
	fn as_svg_point_collection(&self) -> &D2D1SvgPointCollection;
	fn into_svg_point_collection(self) -> D2D1SvgPointCollection;
}

impl ID2D1SvgPointCollection for D2D1SvgPointCollection {
	fn as_svg_point_collection(&self) -> &D2D1SvgPointCollection { self }
	fn into_svg_point_collection(self) -> D2D1SvgPointCollection { self }
}
impl ID2D1SvgAttribute for D2D1SvgPointCollection {
	fn as_svg_attribute(&self) -> &D2D1SvgAttribute { &self.0.as_svg_attribute() }
	fn into_svg_attribute(self) -> D2D1SvgAttribute { self.0.into_svg_attribute() }
}

impl ID2D1Resource for D2D1SvgPointCollection {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SvgPointCollection {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SvgPointCollection {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1SvgAttribute::from(v))
    }
}

