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

#[repr(C)]
pub struct DxgiFactory3(pub(crate) DxgiFactory2);

impl Guid for DxgiFactory3 {
	const IID: &'static GUID = &GUID::from_u128(0x25483823cd464c7d86ca47aa95b837bdu128);
}

impl Clone for DxgiFactory3 {
	fn clone(&self) -> Self { DxgiFactory3(self.0.clone()) }
}

pub trait IDxgiFactory3: IDxgiFactory2 {
	fn as_factory3(&self) -> &DxgiFactory3;
	fn into_factory3(self) -> DxgiFactory3;

	fn GetCreationFlags(&self, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u32
			= unsafe { transmute(vt[25]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl IDxgiFactory3 for DxgiFactory3 {
	fn as_factory3(&self) -> &DxgiFactory3 { self }
	fn into_factory3(self) -> DxgiFactory3 { self }
}

impl IDxgiFactory2 for DxgiFactory3 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0 }
	fn into_factory2(self) -> DxgiFactory2 { self.0 }
}

impl IDxgiFactory1 for DxgiFactory3 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.0 }
	fn into_factory1(self) -> DxgiFactory1 { self.0.0 }
}

impl IDxgiFactory for DxgiFactory3 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.0.0 }
	fn into_factory(self) -> DxgiFactory { self.0.0.0 }
}

impl IDxgiObject for DxgiFactory3 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0 }
}

impl From<Unknown> for DxgiFactory3 {
    fn from(v: Unknown) -> Self {
        Self(DxgiFactory2::from(v))
    }
}

impl IUnknown for DxgiFactory3 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

