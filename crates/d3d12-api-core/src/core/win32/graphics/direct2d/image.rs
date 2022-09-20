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
pub struct D2D1Image(pub(crate) D2D1Resource);

impl Deref for D2D1Image {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Image {
	const IID: &'static GUID = &GUID::from_u128(0x65019f758da2497cb32cdfa34e48ede6u128);
}

impl Com for D2D1Image {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Image {}

pub trait ID2D1Image: ID2D1Resource {
	fn as_image(&self) -> &D2D1Image;
	fn into_image(self) -> D2D1Image;
}

impl ID2D1Image for D2D1Image {
	fn as_image(&self) -> &D2D1Image { self }
	fn into_image(self) -> D2D1Image { self }
}
impl ID2D1Resource for D2D1Image {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Image {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Image {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

