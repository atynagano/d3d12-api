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

use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12CommandAllocator(pub(crate) D3D12Pageable);

impl Deref for D3D12CommandAllocator {
	type Target = D3D12Pageable;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12CommandAllocator {
	const IID: &'static GUID = &GUID::from_u128(0x6102dee4af594b09b999b44d73f09b24u128);
}

impl Com for D3D12CommandAllocator {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12CommandAllocator {
	pub fn Reset(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait ID3D12CommandAllocator: ID3D12Pageable {
	fn as_command_allocator(&self) -> &D3D12CommandAllocator;
	fn into_command_allocator(self) -> D3D12CommandAllocator;
}

impl ID3D12CommandAllocator for D3D12CommandAllocator {
	fn as_command_allocator(&self) -> &D3D12CommandAllocator { self }
	fn into_command_allocator(self) -> D3D12CommandAllocator { self }
}
impl ID3D12Pageable for D3D12CommandAllocator {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12CommandAllocator {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12CommandAllocator {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12CommandAllocator {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12CommandAllocator {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

