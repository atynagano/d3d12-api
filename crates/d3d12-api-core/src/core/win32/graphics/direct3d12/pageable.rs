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


#[repr(C)]
pub struct D3D12Pageable(pub(crate) D3D12DeviceChild);

impl Guid for D3D12Pageable {
	const IID: &'static GUID = &GUID::from_u128(0x63ee58fb1268483586daf008ce62f0d6u128);
}

impl Clone for D3D12Pageable {
	fn clone(&self) -> Self { D3D12Pageable(self.0.clone()) }
}

pub trait ID3D12Pageable: ID3D12DeviceChild {
	fn as_pageable(&self) -> &D3D12Pageable;
	fn into_pageable(self) -> D3D12Pageable;
}

impl ID3D12Pageable for D3D12Pageable {
	fn as_pageable(&self) -> &D3D12Pageable { self }
	fn into_pageable(self) -> D3D12Pageable { self }
}

impl ID3D12DeviceChild for D3D12Pageable {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0 }
}

impl ID3D12Object for D3D12Pageable {
	fn as_object(&self) -> &D3D12Object { &self.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0 }
}

impl From<Unknown> for D3D12Pageable {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

impl IUnknown for D3D12Pageable {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

