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

use crate::core::win32::graphics::direct3d12::*;
#[repr(C)]
pub struct D3D12Fence1(pub(crate) D3D12Fence);

impl Guid for D3D12Fence1 {
	const IID: &'static GUID = &GUID::from_u128(0x433685fee22b4ca0a8dbb5b4f4dd0e4au128);
}

impl Clone for D3D12Fence1 {
	fn clone(&self) -> Self { D3D12Fence1(self.0.clone()) }
}

pub trait ID3D12Fence1: ID3D12Fence {
	fn as_fence1(&self) -> &D3D12Fence1;
	fn into_fence1(self) -> D3D12Fence1;

	fn GetCreationFlags(&self, ) -> (D3D12FenceFlags) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> D3D12FenceFlags
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl ID3D12Fence1 for D3D12Fence1 {
	fn as_fence1(&self) -> &D3D12Fence1 { self }
	fn into_fence1(self) -> D3D12Fence1 { self }
}

impl ID3D12Fence for D3D12Fence1 {
	fn as_fence(&self) -> &D3D12Fence { &self.0 }
	fn into_fence(self) -> D3D12Fence { self.0 }
}

impl ID3D12Pageable for D3D12Fence1 {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.0 }
	fn into_pageable(self) -> D3D12Pageable { self.0.0 }
}

impl ID3D12DeviceChild for D3D12Fence1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0.0 }
}

impl ID3D12Object for D3D12Fence1 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0 }
}

impl From<Unknown> for D3D12Fence1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Fence::from(v))
    }
}

impl IUnknown for D3D12Fence1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

