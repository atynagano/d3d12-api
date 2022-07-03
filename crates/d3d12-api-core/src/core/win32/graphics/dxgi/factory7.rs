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
pub struct DxgiFactory7(pub(crate) DxgiFactory6);

impl Guid for DxgiFactory7 {
	const IID: &'static GUID = &GUID::from_u128(0xa4966eed76db44da84c1ee9a7afb20a8u128);
}

impl Clone for DxgiFactory7 {
	fn clone(&self) -> Self { DxgiFactory7(self.0.clone()) }
}

pub trait IDxgiFactory7: IDxgiFactory6 {
	fn as_factory7(&self) -> &DxgiFactory7;
	fn into_factory7(self) -> DxgiFactory7;

	fn RegisterAdaptersChangedEvent(&self, event: Handle, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _pdw_cookie: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, event: Handle, _pdw_cookie: &mut u32, ) -> HResult
			= unsafe { transmute(vt[30]) };
		let ret = f(vt, event, &mut _pdw_cookie, );
		if ret.is_ok() {
			return Ok((_pdw_cookie));
		}
		Err(ret)
	}

	fn UnregisterAdaptersChangedEvent(&self, cookie: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, cookie: u32, ) -> HResult
			= unsafe { transmute(vt[31]) };
		let ret = f(vt, cookie, );
		ret.ok()
	}
}

impl IDxgiFactory7 for DxgiFactory7 {
	fn as_factory7(&self) -> &DxgiFactory7 { self }
	fn into_factory7(self) -> DxgiFactory7 { self }
}

impl IDxgiFactory6 for DxgiFactory7 {
	fn as_factory6(&self) -> &DxgiFactory6 { &self.0 }
	fn into_factory6(self) -> DxgiFactory6 { self.0 }
}

impl IDxgiFactory5 for DxgiFactory7 {
	fn as_factory5(&self) -> &DxgiFactory5 { &self.0.0 }
	fn into_factory5(self) -> DxgiFactory5 { self.0.0 }
}

impl IDxgiFactory4 for DxgiFactory7 {
	fn as_factory4(&self) -> &DxgiFactory4 { &self.0.0.0 }
	fn into_factory4(self) -> DxgiFactory4 { self.0.0.0 }
}

impl IDxgiFactory3 for DxgiFactory7 {
	fn as_factory3(&self) -> &DxgiFactory3 { &self.0.0.0.0 }
	fn into_factory3(self) -> DxgiFactory3 { self.0.0.0.0 }
}

impl IDxgiFactory2 for DxgiFactory7 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0.0.0.0.0 }
	fn into_factory2(self) -> DxgiFactory2 { self.0.0.0.0.0 }
}

impl IDxgiFactory1 for DxgiFactory7 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.0.0.0.0.0 }
	fn into_factory1(self) -> DxgiFactory1 { self.0.0.0.0.0.0 }
}

impl IDxgiFactory for DxgiFactory7 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.0.0.0.0.0.0 }
	fn into_factory(self) -> DxgiFactory { self.0.0.0.0.0.0.0 }
}

impl IDxgiObject for DxgiFactory7 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0.0.0.0.0 }
}

impl From<Unknown> for DxgiFactory7 {
    fn from(v: Unknown) -> Self {
        Self(DxgiFactory6::from(v))
    }
}

impl IUnknown for DxgiFactory7 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0.0 }
}

