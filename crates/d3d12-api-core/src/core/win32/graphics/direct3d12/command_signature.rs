#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::num::NonZeroUsize;
use std::mem::{MaybeUninit, size_of_val, transmute};
use std::ops::Deref;
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;


#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12CommandSignature(pub(crate) D3D12Pageable);

impl Deref for D3D12CommandSignature {
	type Target = D3D12Pageable;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12CommandSignature {
	const IID: &'static GUID = &GUID::from_u128(0xc36a797cec804f0a8985a7b2475082d1u128);
}

impl Com for D3D12CommandSignature {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12CommandSignature {}

pub trait ID3D12CommandSignature: ID3D12Pageable {
	fn as_command_signature(&self) -> &D3D12CommandSignature;
	fn into_command_signature(self) -> D3D12CommandSignature;
}

impl ID3D12CommandSignature for D3D12CommandSignature {
	fn as_command_signature(&self) -> &D3D12CommandSignature { self }
	fn into_command_signature(self) -> D3D12CommandSignature { self }
}
impl ID3D12Pageable for D3D12CommandSignature {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12CommandSignature {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12CommandSignature {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12CommandSignature {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12CommandSignature {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

