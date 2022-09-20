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
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1GeometryGroup(pub(crate) D2D1Geometry);

impl Deref for D2D1GeometryGroup {
	type Target = D2D1Geometry;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GeometryGroup {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906a612e211dc9fed001143a055f9u128);
}

impl Com for D2D1GeometryGroup {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GeometryGroup {
	pub fn GetFillMode(&self) -> D2D1FillMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1FillMode
				= transmute(vt[17]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetSourceGeometryCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[18]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetSourceGeometries(&self) { todo!() }
}

pub trait ID2D1GeometryGroup: ID2D1Geometry {
	fn as_geometry_group(&self) -> &D2D1GeometryGroup;
	fn into_geometry_group(self) -> D2D1GeometryGroup;
}

impl ID2D1GeometryGroup for D2D1GeometryGroup {
	fn as_geometry_group(&self) -> &D2D1GeometryGroup { self }
	fn into_geometry_group(self) -> D2D1GeometryGroup { self }
}
impl ID2D1Geometry for D2D1GeometryGroup {
	fn as_geometry(&self) -> &D2D1Geometry { &self.0.as_geometry() }
	fn into_geometry(self) -> D2D1Geometry { self.0.into_geometry() }
}

impl ID2D1Resource for D2D1GeometryGroup {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1GeometryGroup {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GeometryGroup {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Geometry::from(v))
    }
}

