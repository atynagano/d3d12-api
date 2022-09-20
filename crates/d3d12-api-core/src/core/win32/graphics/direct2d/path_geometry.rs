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
pub struct D2D1PathGeometry(pub(crate) D2D1Geometry);

impl Deref for D2D1PathGeometry {
	type Target = D2D1Geometry;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1PathGeometry {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906a512e211dc9fed001143a055f9u128);
}

impl Com for D2D1PathGeometry {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1PathGeometry {
	pub fn Open(&self) -> Result<D2D1GeometrySink, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut geometry_sink_out_: Option<D2D1GeometrySink> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, transmute(&mut geometry_sink_out_));
			if _ret_.is_ok() { if let Some(geometry_sink_out_) = geometry_sink_out_ { return Ok(geometry_sink_out_); } }
			Err(_ret_)
		}
	}

	pub fn Stream(&self, geometry_sink: &D2D1GeometrySink) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, geometry_sink.vtable());
			_ret_.ok()
		}
	}

	pub fn GetSegmentCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetFigureCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1PathGeometry: ID2D1Geometry {
	fn as_path_geometry(&self) -> &D2D1PathGeometry;
	fn into_path_geometry(self) -> D2D1PathGeometry;
}

impl ID2D1PathGeometry for D2D1PathGeometry {
	fn as_path_geometry(&self) -> &D2D1PathGeometry { self }
	fn into_path_geometry(self) -> D2D1PathGeometry { self }
}
impl ID2D1Geometry for D2D1PathGeometry {
	fn as_geometry(&self) -> &D2D1Geometry { &self.0.as_geometry() }
	fn into_geometry(self) -> D2D1Geometry { self.0.into_geometry() }
}

impl ID2D1Resource for D2D1PathGeometry {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1PathGeometry {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1PathGeometry {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Geometry::from(v))
    }
}

