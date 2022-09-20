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


#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1LookupTable3D(pub(crate) D2D1Resource);

impl Deref for D2D1LookupTable3D {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1LookupTable3D {
	const IID: &'static GUID = &GUID::from_u128(0x53dd9855a3b04d5b82e126e25c5e5797u128);
}

impl Com for D2D1LookupTable3D {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1LookupTable3D {}

pub trait ID2D1LookupTable3D: ID2D1Resource {
	fn as_lookup_table3d(&self) -> &D2D1LookupTable3D;
	fn into_lookup_table3d(self) -> D2D1LookupTable3D;
}

impl ID2D1LookupTable3D for D2D1LookupTable3D {
	fn as_lookup_table3d(&self) -> &D2D1LookupTable3D { self }
	fn into_lookup_table3d(self) -> D2D1LookupTable3D { self }
}
impl ID2D1Resource for D2D1LookupTable3D {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1LookupTable3D {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1LookupTable3D {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

