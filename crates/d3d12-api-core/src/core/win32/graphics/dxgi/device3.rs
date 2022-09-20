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
pub struct DxgiDevice3(pub(crate) DxgiDevice2);

impl Deref for DxgiDevice3 {
	type Target = DxgiDevice2;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiDevice3 {
	const IID: &'static GUID = &GUID::from_u128(0x6007896c32444afdbf18a6d3beda5023u128);
}

impl Com for DxgiDevice3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiDevice3 {
	pub fn Trim(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt);
		}
	}
}

pub trait IDxgiDevice3: IDxgiDevice2 {
	fn as_device3(&self) -> &DxgiDevice3;
	fn into_device3(self) -> DxgiDevice3;
}

impl IDxgiDevice3 for DxgiDevice3 {
	fn as_device3(&self) -> &DxgiDevice3 { self }
	fn into_device3(self) -> DxgiDevice3 { self }
}
impl IDxgiDevice2 for DxgiDevice3 {
	fn as_device2(&self) -> &DxgiDevice2 { &self.0.as_device2() }
	fn into_device2(self) -> DxgiDevice2 { self.0.into_device2() }
}

impl IDxgiDevice1 for DxgiDevice3 {
	fn as_device1(&self) -> &DxgiDevice1 { &self.0.as_device1() }
	fn into_device1(self) -> DxgiDevice1 { self.0.into_device1() }
}

impl IDxgiDevice for DxgiDevice3 {
	fn as_device(&self) -> &DxgiDevice { &self.0.as_device() }
	fn into_device(self) -> DxgiDevice { self.0.into_device() }
}

impl IDxgiObject for DxgiDevice3 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiDevice3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiDevice3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiDevice2::from(v))
    }
}

