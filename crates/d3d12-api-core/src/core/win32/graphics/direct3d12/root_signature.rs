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
pub struct D3D12RootSignature(pub(crate) D3D12DeviceChild);

impl Deref for D3D12RootSignature {
	type Target = D3D12DeviceChild;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12RootSignature {
	const IID: &'static GUID = &GUID::from_u128(0xc54a6b6672df4ee88be5a946a1429214u128);
}

impl Com for D3D12RootSignature {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12RootSignature {}

pub trait ID3D12RootSignature: ID3D12DeviceChild {
	fn as_root_signature(&self) -> &D3D12RootSignature;
	fn into_root_signature(self) -> D3D12RootSignature;
}

impl ID3D12RootSignature for D3D12RootSignature {
	fn as_root_signature(&self) -> &D3D12RootSignature { self }
	fn into_root_signature(self) -> D3D12RootSignature { self }
}
impl ID3D12DeviceChild for D3D12RootSignature {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12RootSignature {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12RootSignature {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12RootSignature {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

