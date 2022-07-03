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
pub struct DxgiAdapter1(pub(crate) DxgiAdapter);

impl Guid for DxgiAdapter1 {
	const IID: &'static GUID = &GUID::from_u128(0x29038f613839462691fd086879011a05u128);
}

impl Clone for DxgiAdapter1 {
	fn clone(&self) -> Self { DxgiAdapter1(self.0.clone()) }
}

pub trait IDxgiAdapter1: IDxgiAdapter {
	fn as_adapter1(&self) -> &DxgiAdapter1;
	fn into_adapter1(self) -> DxgiAdapter1;

	fn GetDesc1(&self, ) -> Result<(DxgiAdapterDesc1), HResult> {
		let vt = self.as_param();
		let mut _desc: DxgiAdapterDesc1 = DxgiAdapterDesc1::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiAdapterDesc1, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, &mut _desc, );
		if ret.is_ok() {
			return Ok((_desc));
		}
		Err(ret)
	}
}

impl IDxgiAdapter1 for DxgiAdapter1 {
	fn as_adapter1(&self) -> &DxgiAdapter1 { self }
	fn into_adapter1(self) -> DxgiAdapter1 { self }
}

impl IDxgiAdapter for DxgiAdapter1 {
	fn as_adapter(&self) -> &DxgiAdapter { &self.0 }
	fn into_adapter(self) -> DxgiAdapter { self.0 }
}

impl IDxgiObject for DxgiAdapter1 {
	fn as_object(&self) -> &DxgiObject { &self.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0 }
}

impl From<Unknown> for DxgiAdapter1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiAdapter::from(v))
    }
}

impl IUnknown for DxgiAdapter1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}
