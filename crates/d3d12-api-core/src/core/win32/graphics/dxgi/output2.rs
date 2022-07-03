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
pub struct DxgiOutput2(pub(crate) DxgiOutput1);

impl Guid for DxgiOutput2 {
	const IID: &'static GUID = &GUID::from_u128(0x595e39d12724466399b1da969de28364u128);
}

impl Clone for DxgiOutput2 {
	fn clone(&self) -> Self { DxgiOutput2(self.0.clone()) }
}

pub trait IDxgiOutput2: IDxgiOutput1 {
	fn as_output2(&self) -> &DxgiOutput2;
	fn into_output2(self) -> DxgiOutput2;

	fn SupportsOverlays(&self, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Bool
			= unsafe { transmute(vt[23]) };
		let ret = f(vt, );
		return (ret.to_bool());
	}
}

impl IDxgiOutput2 for DxgiOutput2 {
	fn as_output2(&self) -> &DxgiOutput2 { self }
	fn into_output2(self) -> DxgiOutput2 { self }
}

impl IDxgiOutput1 for DxgiOutput2 {
	fn as_output1(&self) -> &DxgiOutput1 { &self.0 }
	fn into_output1(self) -> DxgiOutput1 { self.0 }
}

impl IDxgiOutput for DxgiOutput2 {
	fn as_output(&self) -> &DxgiOutput { &self.0.0 }
	fn into_output(self) -> DxgiOutput { self.0.0 }
}

impl IDxgiObject for DxgiOutput2 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0 }
}

impl From<Unknown> for DxgiOutput2 {
    fn from(v: Unknown) -> Self {
        Self(DxgiOutput1::from(v))
    }
}

impl IUnknown for DxgiOutput2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

