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
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1PathGeometry1(pub(crate) D2D1PathGeometry);

impl Deref for D2D1PathGeometry1 {
	type Target = D2D1PathGeometry;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1PathGeometry1 {
	const IID: &'static GUID = &GUID::from_u128(0x62baa2d2ab5441b7b872787e0106a421u128);
}

impl Com for D2D1PathGeometry1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1PathGeometry1 {
	pub fn ComputePointAndSegmentAtLength(&self, length: f32, start_segment: u32, world_transform: Option<&D2DMatrix3x2F>, flattening_tolerance: f32) -> Result<D2D1PointDescription, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut point_description_out_: MaybeUninit<D2D1PointDescription> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, f32, u32, *const c_void, f32, *mut D2D1PointDescription) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, length, start_segment, transmute(world_transform), flattening_tolerance, point_description_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(point_description_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1PathGeometry1: ID2D1PathGeometry {
	fn as_path_geometry1(&self) -> &D2D1PathGeometry1;
	fn into_path_geometry1(self) -> D2D1PathGeometry1;
}

impl ID2D1PathGeometry1 for D2D1PathGeometry1 {
	fn as_path_geometry1(&self) -> &D2D1PathGeometry1 { self }
	fn into_path_geometry1(self) -> D2D1PathGeometry1 { self }
}
impl ID2D1PathGeometry for D2D1PathGeometry1 {
	fn as_path_geometry(&self) -> &D2D1PathGeometry { &self.0.as_path_geometry() }
	fn into_path_geometry(self) -> D2D1PathGeometry { self.0.into_path_geometry() }
}

impl ID2D1Geometry for D2D1PathGeometry1 {
	fn as_geometry(&self) -> &D2D1Geometry { &self.0.as_geometry() }
	fn into_geometry(self) -> D2D1Geometry { self.0.into_geometry() }
}

impl ID2D1Resource for D2D1PathGeometry1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1PathGeometry1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1PathGeometry1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1PathGeometry::from(v))
    }
}

