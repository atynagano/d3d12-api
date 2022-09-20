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
pub struct D2D1GradientMesh(pub(crate) D2D1Resource);

impl Deref for D2D1GradientMesh {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GradientMesh {
	const IID: &'static GUID = &GUID::from_u128(0xf292e401c0504cde83d704962d3b23c2u128);
}

impl Com for D2D1GradientMesh {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GradientMesh {
	pub fn GetPatchCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetPatches(&self) { todo!() }
}

pub trait ID2D1GradientMesh: ID2D1Resource {
	fn as_gradient_mesh(&self) -> &D2D1GradientMesh;
	fn into_gradient_mesh(self) -> D2D1GradientMesh;
}

impl ID2D1GradientMesh for D2D1GradientMesh {
	fn as_gradient_mesh(&self) -> &D2D1GradientMesh { self }
	fn into_gradient_mesh(self) -> D2D1GradientMesh { self }
}
impl ID2D1Resource for D2D1GradientMesh {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1GradientMesh {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GradientMesh {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

