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
pub struct D3D12Heap(pub(crate) D3D12Pageable);

impl Guid for D3D12Heap {
	const IID: &'static GUID = &GUID::from_u128(0x6b3b25026e5145b390ee9884265e8df3u128);
}

impl Clone for D3D12Heap {
	fn clone(&self) -> Self { D3D12Heap(self.0.clone()) }
}

pub trait ID3D12Heap: ID3D12Pageable {
	fn as_heap(&self) -> &D3D12Heap;
	fn into_heap(self) -> D3D12Heap;

	fn GetDesc(&self, ) -> D3D12HeapDesc {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> D3D12HeapDesc
				= transmute(vt[8]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl ID3D12Heap for D3D12Heap {
	fn as_heap(&self) -> &D3D12Heap { self }
	fn into_heap(self) -> D3D12Heap { self }
}

impl ID3D12Pageable for D3D12Heap {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0 }
	fn into_pageable(self) -> D3D12Pageable { self.0 }
}

impl ID3D12DeviceChild for D3D12Heap {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0 }
}

impl ID3D12Object for D3D12Heap {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12Heap {
    fn from(v: Unknown) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

impl IUnknown for D3D12Heap {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

