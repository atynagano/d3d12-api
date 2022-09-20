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
pub struct D2D1Resource(pub(crate) Unknown);

impl Deref for D2D1Resource {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Resource {
	const IID: &'static GUID = &GUID::from_u128(0x2cd9069112e211dc9fed001143a055f9u128);
}

impl Com for D2D1Resource {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Resource {
	pub fn GetFactory(&self) -> Option<D2D1Factory> {
		unsafe {
			let vt = self.as_param();
			let mut factory_out_: Option<D2D1Factory> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, transmute(&mut factory_out_));
			factory_out_
		}
	}
}

pub trait ID2D1Resource: IUnknown {
	fn as_resource(&self) -> &D2D1Resource;
	fn into_resource(self) -> D2D1Resource;
}

impl ID2D1Resource for D2D1Resource {
	fn as_resource(&self) -> &D2D1Resource { self }
	fn into_resource(self) -> D2D1Resource { self }
}
impl IUnknown for D2D1Resource {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Resource {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

