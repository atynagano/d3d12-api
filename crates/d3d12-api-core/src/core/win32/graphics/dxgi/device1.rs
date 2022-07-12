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
pub struct DxgiDevice1(pub(crate) DxgiDevice);

impl Guid for DxgiDevice1 {
	const IID: &'static GUID = &GUID::from_u128(0x77db970f627648baba28070143b4392cu128);
}

impl Clone for DxgiDevice1 {
	fn clone(&self) -> Self { DxgiDevice1(self.0.clone()) }
}

pub trait IDxgiDevice1: IDxgiDevice {
	fn as_device1(&self) -> &DxgiDevice1;
	fn into_device1(self) -> DxgiDevice1;

	fn SetMaximumFrameLatency(&self, max_latency: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, max_latency: u32, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, max_latency, );
			_ret_.ok()
		}
	}

	fn GetMaximumFrameLatency(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_max_latency: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_max_latency: *mut u32, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, _out_max_latency.as_mut_ptr(), );
			Ok(_out_max_latency.assume_init())
		}
	}
}

impl IDxgiDevice1 for DxgiDevice1 {
	fn as_device1(&self) -> &DxgiDevice1 { self }
	fn into_device1(self) -> DxgiDevice1 { self }
}

impl IDxgiDevice for DxgiDevice1 {
	fn as_device(&self) -> &DxgiDevice { &self.0.as_device() }
	fn into_device(self) -> DxgiDevice { self.0.into_device() }
}

impl IDxgiObject for DxgiDevice1 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiDevice1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiDevice::from(v))
    }
}

impl IUnknown for DxgiDevice1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

