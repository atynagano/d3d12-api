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
pub struct D3D12StateObject(pub(crate) D3D12Pageable);

impl Guid for D3D12StateObject {
	const IID: &'static GUID = &GUID::from_u128(0x47016943fca8459493eaaf258b55346du128);
}

impl Clone for D3D12StateObject {
	fn clone(&self) -> Self { D3D12StateObject(self.0.clone()) }
}

pub trait ID3D12StateObject: ID3D12Pageable {
	fn as_state_object(&self) -> &D3D12StateObject;
	fn into_state_object(self) -> D3D12StateObject;
}

impl ID3D12StateObject for D3D12StateObject {
	fn as_state_object(&self) -> &D3D12StateObject { self }
	fn into_state_object(self) -> D3D12StateObject { self }
}

impl ID3D12Pageable for D3D12StateObject {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0 }
	fn into_pageable(self) -> D3D12Pageable { self.0 }
}

impl ID3D12DeviceChild for D3D12StateObject {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0 }
}

impl ID3D12Object for D3D12StateObject {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12StateObject {
    fn from(v: Unknown) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

impl IUnknown for D3D12StateObject {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

