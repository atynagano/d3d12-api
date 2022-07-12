#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{MaybeUninit, size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;

#[repr(C)]
pub struct DxgiKeyedMutex(pub(crate) DxgiDeviceSubObject);

impl Guid for DxgiKeyedMutex {
	const IID: &'static GUID = &GUID::from_u128(0x9d8e1289d7b3465f8126250e349af85du128);
}

impl Clone for DxgiKeyedMutex {
	fn clone(&self) -> Self { DxgiKeyedMutex(self.0.clone()) }
}

pub trait IDxgiKeyedMutex: IDxgiDeviceSubObject {
	fn as_keyed_mutex(&self) -> &DxgiKeyedMutex;
	fn into_keyed_mutex(self) -> DxgiKeyedMutex;

	fn AcquireSync(&self, key: u64, milliseconds: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, key: u64, milliseconds: u32, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, key, milliseconds, );
			_ret_.ok()
		}
	}

	fn ReleaseSync(&self, key: u64, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, key: u64, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, key, );
			_ret_.ok()
		}
	}
}

impl IDxgiKeyedMutex for DxgiKeyedMutex {
	fn as_keyed_mutex(&self) -> &DxgiKeyedMutex { self }
	fn into_keyed_mutex(self) -> DxgiKeyedMutex { self }
}

impl IDxgiDeviceSubObject for DxgiKeyedMutex {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiKeyedMutex {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiKeyedMutex {
    fn from(v: Unknown) -> Self {
        Self(DxgiDeviceSubObject::from(v))
    }
}

impl IUnknown for DxgiKeyedMutex {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

