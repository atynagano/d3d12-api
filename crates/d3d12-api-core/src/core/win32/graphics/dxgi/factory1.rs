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
pub struct DxgiFactory1(pub(crate) DxgiFactory);

impl Guid for DxgiFactory1 {
	const IID: &'static GUID = &GUID::from_u128(0x770aae78f26f4dbaa829253c83d1b387u128);
}

impl Clone for DxgiFactory1 {
	fn clone(&self) -> Self { DxgiFactory1(self.0.clone()) }
}

pub trait IDxgiFactory1: IDxgiFactory {
	fn as_factory1(&self) -> &DxgiFactory1;
	fn into_factory1(self) -> DxgiFactory1;

	fn EnumAdapters1(&self, adapter: u32, ) -> Result<(DxgiAdapter1), HResult> {
		let vt = self.as_param();
		let mut _adapter: Option<DxgiAdapter1> = None;
		let f: extern "system" fn(Param<Self>, adapter: u32, _adapter: &mut Option<DxgiAdapter1>, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, adapter, &mut _adapter, );
		if ret.is_ok() {
			if let (Some(_adapter)) = (_adapter) {
				return Ok((_adapter));
			}
		}
		Err(ret)
	}

	fn IsCurrent(&self, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Bool
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, );
		return (ret.to_bool());
	}
}

impl IDxgiFactory1 for DxgiFactory1 {
	fn as_factory1(&self) -> &DxgiFactory1 { self }
	fn into_factory1(self) -> DxgiFactory1 { self }
}

impl IDxgiFactory for DxgiFactory1 {
	fn as_factory(&self) -> &DxgiFactory { &self.0 }
	fn into_factory(self) -> DxgiFactory { self.0 }
}

impl IDxgiObject for DxgiFactory1 {
	fn as_object(&self) -> &DxgiObject { &self.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0 }
}

impl From<Unknown> for DxgiFactory1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiFactory::from(v))
    }
}

impl IUnknown for DxgiFactory1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

