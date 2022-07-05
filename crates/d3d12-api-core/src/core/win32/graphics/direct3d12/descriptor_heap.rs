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
pub struct D3D12DescriptorHeap(pub(crate) D3D12Pageable);

impl Guid for D3D12DescriptorHeap {
	const IID: &'static GUID = &GUID::from_u128(0x8efb471d616c4f4990f7127bb763fa51u128);
}

impl Clone for D3D12DescriptorHeap {
	fn clone(&self) -> Self { D3D12DescriptorHeap(self.0.clone()) }
}

pub trait ID3D12DescriptorHeap: ID3D12Pageable {
	fn as_descriptor_heap(&self) -> &D3D12DescriptorHeap;
	fn into_descriptor_heap(self) -> D3D12DescriptorHeap;

	fn GetDesc(&self, ) -> D3D12DescriptorHeapDesc {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> D3D12DescriptorHeapDesc
				= transmute(vt[8]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn GetCPUDescriptorHandleForHeapStart(&self) -> (D3D12CpuDescriptorHandle) {
		let vt = self.as_param();
		let mut out = D3D12CpuDescriptorHandle::zeroed();
		let f: extern "system" fn(Param<Self>, &mut D3D12CpuDescriptorHandle) -> *const D3D12CpuDescriptorHandle
			= unsafe { transmute(vt[9]) };
		let _ = f(vt, &mut out);
		return (out);
	}

	fn GetGPUDescriptorHandleForHeapStart(&self) -> (D3D12CpuDescriptorHandle) {
		let vt = self.as_param();
		let mut out = D3D12CpuDescriptorHandle::zeroed();
		let f: extern "system" fn(Param<Self>, &mut D3D12CpuDescriptorHandle) -> *const D3D12CpuDescriptorHandle
			= unsafe { transmute(vt[10]) };
		let _ = f(vt, &mut out);
		return (out);
	}
}

impl ID3D12DescriptorHeap for D3D12DescriptorHeap {
	fn as_descriptor_heap(&self) -> &D3D12DescriptorHeap { self }
	fn into_descriptor_heap(self) -> D3D12DescriptorHeap { self }
}

impl ID3D12Pageable for D3D12DescriptorHeap {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0 }
	fn into_pageable(self) -> D3D12Pageable { self.0 }
}

impl ID3D12DeviceChild for D3D12DescriptorHeap {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0 }
}

impl ID3D12Object for D3D12DescriptorHeap {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12DescriptorHeap {
    fn from(v: Unknown) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

impl IUnknown for D3D12DescriptorHeap {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

