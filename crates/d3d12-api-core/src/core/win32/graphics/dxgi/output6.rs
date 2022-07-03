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
use crate::core::win32::graphics::dxgi::*;
#[repr(C)]
pub struct DxgiOutput6(pub(crate) DxgiOutput5);

impl Guid for DxgiOutput6 {
	const IID: &'static GUID = &GUID::from_u128(0x068346e8aaec4b84add7137f513f77a1u128);
}

impl Clone for DxgiOutput6 {
	fn clone(&self) -> Self { DxgiOutput6(self.0.clone()) }
}

pub trait IDxgiOutput6: IDxgiOutput5 {
	fn as_output6(&self) -> &DxgiOutput6;
	fn into_output6(self) -> DxgiOutput6;

	fn GetDesc1(&self, ) -> Result<(DxgiOutputDesc1), HResult> {
		let vt = self.as_param();
		let mut _desc: DxgiOutputDesc1 = DxgiOutputDesc1::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiOutputDesc1, ) -> HResult
			= unsafe { transmute(vt[27]) };
		let ret = f(vt, &mut _desc, );
		if ret.is_ok() {
			return Ok((_desc));
		}
		Err(ret)
	}

	fn CheckHardwareCompositionSupport(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _flags: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _flags: &mut u32, ) -> HResult
			= unsafe { transmute(vt[28]) };
		let ret = f(vt, &mut _flags, );
		if ret.is_ok() {
			return Ok((_flags));
		}
		Err(ret)
	}
}

impl IDxgiOutput6 for DxgiOutput6 {
	fn as_output6(&self) -> &DxgiOutput6 { self }
	fn into_output6(self) -> DxgiOutput6 { self }
}

impl IDxgiOutput5 for DxgiOutput6 {
	fn as_output5(&self) -> &DxgiOutput5 { &self.0 }
	fn into_output5(self) -> DxgiOutput5 { self.0 }
}

impl IDxgiOutput4 for DxgiOutput6 {
	fn as_output4(&self) -> &DxgiOutput4 { &self.0.0 }
	fn into_output4(self) -> DxgiOutput4 { self.0.0 }
}

impl IDxgiOutput3 for DxgiOutput6 {
	fn as_output3(&self) -> &DxgiOutput3 { &self.0.0.0 }
	fn into_output3(self) -> DxgiOutput3 { self.0.0.0 }
}

impl IDxgiOutput2 for DxgiOutput6 {
	fn as_output2(&self) -> &DxgiOutput2 { &self.0.0.0.0 }
	fn into_output2(self) -> DxgiOutput2 { self.0.0.0.0 }
}

impl IDxgiOutput1 for DxgiOutput6 {
	fn as_output1(&self) -> &DxgiOutput1 { &self.0.0.0.0.0 }
	fn into_output1(self) -> DxgiOutput1 { self.0.0.0.0.0 }
}

impl IDxgiOutput for DxgiOutput6 {
	fn as_output(&self) -> &DxgiOutput { &self.0.0.0.0.0.0 }
	fn into_output(self) -> DxgiOutput { self.0.0.0.0.0.0 }
}

impl IDxgiObject for DxgiOutput6 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0.0.0.0 }
}

impl From<Unknown> for DxgiOutput6 {
    fn from(v: Unknown) -> Self {
        Self(DxgiOutput5::from(v))
    }
}

impl IUnknown for DxgiOutput6 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0 }
}

