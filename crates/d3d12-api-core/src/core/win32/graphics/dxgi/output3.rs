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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::system::com::*;
#[repr(C)]
pub struct DxgiOutput3(pub(crate) DxgiOutput2);

impl Guid for DxgiOutput3 {
	const IID: &'static GUID = &GUID::from_u128(0x8a6bb3017e7e41f4a8e05b32f7f99b18u128);
}

impl Clone for DxgiOutput3 {
	fn clone(&self) -> Self { DxgiOutput3(self.0.clone()) }
}

pub trait IDxgiOutput3: IDxgiOutput2 {
	fn as_output3(&self) -> &DxgiOutput3;
	fn into_output3(self) -> DxgiOutput3;

	fn CheckOverlaySupport(&self, enum_format: DxgiFormat, concerned_device: &(impl IUnknown + ?Sized), ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _flags: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, enum_format: DxgiFormat, concerned_device: VTable, _flags: &mut u32, ) -> HResult
			= unsafe { transmute(vt[24]) };
		let ret = f(vt, enum_format, concerned_device.vtable(), &mut _flags, );
		if ret.is_ok() {
			return Ok((_flags));
		}
		Err(ret)
	}
}

impl IDxgiOutput3 for DxgiOutput3 {
	fn as_output3(&self) -> &DxgiOutput3 { self }
	fn into_output3(self) -> DxgiOutput3 { self }
}

impl IDxgiOutput2 for DxgiOutput3 {
	fn as_output2(&self) -> &DxgiOutput2 { &self.0 }
	fn into_output2(self) -> DxgiOutput2 { self.0 }
}

impl IDxgiOutput1 for DxgiOutput3 {
	fn as_output1(&self) -> &DxgiOutput1 { &self.0.0 }
	fn into_output1(self) -> DxgiOutput1 { self.0.0 }
}

impl IDxgiOutput for DxgiOutput3 {
	fn as_output(&self) -> &DxgiOutput { &self.0.0.0 }
	fn into_output(self) -> DxgiOutput { self.0.0.0 }
}

impl IDxgiObject for DxgiOutput3 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0 }
}

impl From<Unknown> for DxgiOutput3 {
    fn from(v: Unknown) -> Self {
        Self(DxgiOutput2::from(v))
    }
}

impl IUnknown for DxgiOutput3 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

