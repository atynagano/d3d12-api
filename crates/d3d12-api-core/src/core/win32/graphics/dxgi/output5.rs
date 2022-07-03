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
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::dxgi::*;
#[repr(C)]
pub struct DxgiOutput5(pub(crate) DxgiOutput4);

impl Guid for DxgiOutput5 {
	const IID: &'static GUID = &GUID::from_u128(0x80a07424ab5242eb833c0c42fd282d98u128);
}

impl Clone for DxgiOutput5 {
	fn clone(&self) -> Self { DxgiOutput5(self.0.clone()) }
}

pub trait IDxgiOutput5: IDxgiOutput4 {
	fn as_output5(&self) -> &DxgiOutput5;
	fn into_output5(self) -> DxgiOutput5;

	fn DuplicateOutput1(&self, device: &(impl IUnknown + ?Sized), flags: u32, supported_formats: &[DxgiFormat], ) -> Result<(DxgiOutputDuplication), HResult> {
		let vt = self.as_param();
		let mut _output_duplication: Option<DxgiOutputDuplication> = None;
		let f: extern "system" fn(Param<Self>, device: VTable, flags: u32, supported_formats_count: u32, supported_formats: *const DxgiFormat, _output_duplication: &mut Option<DxgiOutputDuplication>, ) -> HResult
			= unsafe { transmute(vt[26]) };
		let ret = f(vt, device.vtable(), flags, supported_formats.len() as u32, supported_formats.as_ptr_or_null(), &mut _output_duplication, );
		if ret.is_ok() {
			if let (Some(_output_duplication)) = (_output_duplication) {
				return Ok((_output_duplication));
			}
		}
		Err(ret)
	}
}

impl IDxgiOutput5 for DxgiOutput5 {
	fn as_output5(&self) -> &DxgiOutput5 { self }
	fn into_output5(self) -> DxgiOutput5 { self }
}

impl IDxgiOutput4 for DxgiOutput5 {
	fn as_output4(&self) -> &DxgiOutput4 { &self.0 }
	fn into_output4(self) -> DxgiOutput4 { self.0 }
}

impl IDxgiOutput3 for DxgiOutput5 {
	fn as_output3(&self) -> &DxgiOutput3 { &self.0.0 }
	fn into_output3(self) -> DxgiOutput3 { self.0.0 }
}

impl IDxgiOutput2 for DxgiOutput5 {
	fn as_output2(&self) -> &DxgiOutput2 { &self.0.0.0 }
	fn into_output2(self) -> DxgiOutput2 { self.0.0.0 }
}

impl IDxgiOutput1 for DxgiOutput5 {
	fn as_output1(&self) -> &DxgiOutput1 { &self.0.0.0.0 }
	fn into_output1(self) -> DxgiOutput1 { self.0.0.0.0 }
}

impl IDxgiOutput for DxgiOutput5 {
	fn as_output(&self) -> &DxgiOutput { &self.0.0.0.0.0 }
	fn into_output(self) -> DxgiOutput { self.0.0.0.0.0 }
}

impl IDxgiObject for DxgiOutput5 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0.0.0 }
}

impl From<Unknown> for DxgiOutput5 {
    fn from(v: Unknown) -> Self {
        Self(DxgiOutput4::from(v))
    }
}

impl IUnknown for DxgiOutput5 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0 }
}

