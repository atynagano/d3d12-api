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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
pub struct DxgiFactory5(pub(crate) DxgiFactory4);

impl Guid for DxgiFactory5 {
	const IID: &'static GUID = &GUID::from_u128(0x7632e1f5ee654dca87fd84cd75f8838du128);
}

impl Clone for DxgiFactory5 {
	fn clone(&self) -> Self { DxgiFactory5(self.0.clone()) }
}

pub trait IDxgiFactory5: IDxgiFactory4 {
	fn as_factory5(&self) -> &DxgiFactory5;
	fn into_factory5(self) -> DxgiFactory5;
}

impl IDxgiFactory5 for DxgiFactory5 {
	fn as_factory5(&self) -> &DxgiFactory5 { self }
	fn into_factory5(self) -> DxgiFactory5 { self }
}

impl IDxgiFactory4 for DxgiFactory5 {
	fn as_factory4(&self) -> &DxgiFactory4 { &self.0 }
	fn into_factory4(self) -> DxgiFactory4 { self.0 }
}

impl IDxgiFactory3 for DxgiFactory5 {
	fn as_factory3(&self) -> &DxgiFactory3 { &self.0.0 }
	fn into_factory3(self) -> DxgiFactory3 { self.0.0 }
}

impl IDxgiFactory2 for DxgiFactory5 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0.0.0 }
	fn into_factory2(self) -> DxgiFactory2 { self.0.0.0 }
}

impl IDxgiFactory1 for DxgiFactory5 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.0.0.0 }
	fn into_factory1(self) -> DxgiFactory1 { self.0.0.0.0 }
}

impl IDxgiFactory for DxgiFactory5 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.0.0.0.0 }
	fn into_factory(self) -> DxgiFactory { self.0.0.0.0.0 }
}

impl IDxgiObject for DxgiFactory5 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0.0.0 }
}

impl From<Unknown> for DxgiFactory5 {
    fn from(v: Unknown) -> Self {
        Self(DxgiFactory4::from(v))
    }
}

impl IUnknown for DxgiFactory5 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0 }
}

