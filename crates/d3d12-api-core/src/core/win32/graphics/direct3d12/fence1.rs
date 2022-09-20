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

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Fence1(pub(crate) D3D12Fence);

impl Deref for D3D12Fence1 {
	type Target = D3D12Fence;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Fence1 {
	const IID: &'static GUID = &GUID::from_u128(0x433685fee22b4ca0a8dbb5b4f4dd0e4au128);
}

impl Com for D3D12Fence1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Fence1 {
	pub fn GetCreationFlags(&self) -> D3D12FenceFlags {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D3D12FenceFlags
				= transmute(vt[11]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID3D12Fence1: ID3D12Fence {
	fn as_fence1(&self) -> &D3D12Fence1;
	fn into_fence1(self) -> D3D12Fence1;
}

impl ID3D12Fence1 for D3D12Fence1 {
	fn as_fence1(&self) -> &D3D12Fence1 { self }
	fn into_fence1(self) -> D3D12Fence1 { self }
}
impl ID3D12Fence for D3D12Fence1 {
	fn as_fence(&self) -> &D3D12Fence { &self.0.as_fence() }
	fn into_fence(self) -> D3D12Fence { self.0.into_fence() }
}

impl ID3D12Pageable for D3D12Fence1 {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12Fence1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12Fence1 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Fence1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Fence1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Fence::from(v))
    }
}

