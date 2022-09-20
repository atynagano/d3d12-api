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
pub struct D2D1EllipseGeometry(pub(crate) D2D1Geometry);

impl Deref for D2D1EllipseGeometry {
	type Target = D2D1Geometry;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1EllipseGeometry {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906a412e211dc9fed001143a055f9u128);
}

impl Com for D2D1EllipseGeometry {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1EllipseGeometry {
	pub fn GetEllipse(&self) -> D2D1Ellipse {
		unsafe {
			let vt = self.as_param();
			let mut ellipse_out_: MaybeUninit<D2D1Ellipse> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2D1Ellipse) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, ellipse_out_.as_mut_ptr());
			ellipse_out_.assume_init()
		}
	}
}

pub trait ID2D1EllipseGeometry: ID2D1Geometry {
	fn as_ellipse_geometry(&self) -> &D2D1EllipseGeometry;
	fn into_ellipse_geometry(self) -> D2D1EllipseGeometry;
}

impl ID2D1EllipseGeometry for D2D1EllipseGeometry {
	fn as_ellipse_geometry(&self) -> &D2D1EllipseGeometry { self }
	fn into_ellipse_geometry(self) -> D2D1EllipseGeometry { self }
}
impl ID2D1Geometry for D2D1EllipseGeometry {
	fn as_geometry(&self) -> &D2D1Geometry { &self.0.as_geometry() }
	fn into_geometry(self) -> D2D1Geometry { self.0.into_geometry() }
}

impl ID2D1Resource for D2D1EllipseGeometry {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1EllipseGeometry {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1EllipseGeometry {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Geometry::from(v))
    }
}

