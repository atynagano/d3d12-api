#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
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
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, max_latency: u32, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, max_latency, );
		ret.ok()
	}

	fn GetMaximumFrameLatency(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _max_latency: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _max_latency: &mut u32, ) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, &mut _max_latency, );
		if ret.is_ok() {
			return Ok((_max_latency));
		}
		Err(ret)
	}
}

impl IDxgiDevice1 for DxgiDevice1 {
	fn as_device1(&self) -> &DxgiDevice1 { self }
	fn into_device1(self) -> DxgiDevice1 { self }
}

impl IDxgiDevice for DxgiDevice1 {
	fn as_device(&self) -> &DxgiDevice { &self.0 }
	fn into_device(self) -> DxgiDevice { self.0 }
}

impl IDxgiObject for DxgiDevice1 {
	fn as_object(&self) -> &DxgiObject { &self.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0 }
}

impl From<Unknown> for DxgiDevice1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiDevice::from(v))
    }
}

impl IUnknown for DxgiDevice1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

