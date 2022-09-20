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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1ResourceTexture(pub(crate) Unknown);

impl Deref for D2D1ResourceTexture {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1ResourceTexture {
	const IID: &'static GUID = &GUID::from_u128(0x688d15c302b0438db13ad1b44c32c39au128);
}

impl Com for D2D1ResourceTexture {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1ResourceTexture {
	pub unsafe fn Update() { todo!() }
}

pub trait ID2D1ResourceTexture: IUnknown {
	fn as_resource_texture(&self) -> &D2D1ResourceTexture;
	fn into_resource_texture(self) -> D2D1ResourceTexture;
}

impl ID2D1ResourceTexture for D2D1ResourceTexture {
	fn as_resource_texture(&self) -> &D2D1ResourceTexture { self }
	fn into_resource_texture(self) -> D2D1ResourceTexture { self }
}
impl IUnknown for D2D1ResourceTexture {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1ResourceTexture {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

