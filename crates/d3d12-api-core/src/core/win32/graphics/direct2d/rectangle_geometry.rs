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

use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1RectangleGeometry(pub(crate) D2D1Geometry);

impl Deref for D2D1RectangleGeometry {
	type Target = D2D1Geometry;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1RectangleGeometry {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906a212e211dc9fed001143a055f9u128);
}

impl Com for D2D1RectangleGeometry {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1RectangleGeometry {
	pub fn GetRect(&self) -> D2DRectF {
		unsafe {
			let vt = self.as_param();
			let mut rect_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2DRectF) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, rect_out_.as_mut_ptr());
			rect_out_.assume_init()
		}
	}
}

pub trait ID2D1RectangleGeometry: ID2D1Geometry {
	fn as_rectangle_geometry(&self) -> &D2D1RectangleGeometry;
	fn into_rectangle_geometry(self) -> D2D1RectangleGeometry;
}

impl ID2D1RectangleGeometry for D2D1RectangleGeometry {
	fn as_rectangle_geometry(&self) -> &D2D1RectangleGeometry { self }
	fn into_rectangle_geometry(self) -> D2D1RectangleGeometry { self }
}
impl ID2D1Geometry for D2D1RectangleGeometry {
	fn as_geometry(&self) -> &D2D1Geometry { &self.0.as_geometry() }
	fn into_geometry(self) -> D2D1Geometry { self.0.into_geometry() }
}

impl ID2D1Resource for D2D1RectangleGeometry {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1RectangleGeometry {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1RectangleGeometry {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Geometry::from(v))
    }
}

