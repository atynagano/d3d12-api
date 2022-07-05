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

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
pub struct D3D12Resource2(pub(crate) D3D12Resource1);

impl Guid for D3D12Resource2 {
	const IID: &'static GUID = &GUID::from_u128(0xbe36ec3bea854aeba45ae9d76404a495u128);
}

impl Clone for D3D12Resource2 {
	fn clone(&self) -> Self { D3D12Resource2(self.0.clone()) }
}

pub trait ID3D12Resource2: ID3D12Resource1 {
	fn as_resource2(&self) -> &D3D12Resource2;
	fn into_resource2(self) -> D3D12Resource2;

	fn GetDesc1(&self, ) -> D3D12ResourceDesc1 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> D3D12ResourceDesc1
				= transmute(vt[16]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl ID3D12Resource2 for D3D12Resource2 {
	fn as_resource2(&self) -> &D3D12Resource2 { self }
	fn into_resource2(self) -> D3D12Resource2 { self }
}

impl ID3D12Resource1 for D3D12Resource2 {
	fn as_resource1(&self) -> &D3D12Resource1 { &self.0 }
	fn into_resource1(self) -> D3D12Resource1 { self.0 }
}

impl ID3D12Resource for D3D12Resource2 {
	fn as_resource(&self) -> &D3D12Resource { &self.0.0 }
	fn into_resource(self) -> D3D12Resource { self.0.0 }
}

impl ID3D12Pageable for D3D12Resource2 {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.0.0 }
	fn into_pageable(self) -> D3D12Pageable { self.0.0.0 }
}

impl ID3D12DeviceChild for D3D12Resource2 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0.0.0 }
}

impl ID3D12Object for D3D12Resource2 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0 }
}

impl From<Unknown> for D3D12Resource2 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Resource1::from(v))
    }
}

impl IUnknown for D3D12Resource2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0 }
}

