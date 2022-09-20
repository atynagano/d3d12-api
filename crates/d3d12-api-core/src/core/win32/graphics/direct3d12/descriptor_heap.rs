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
pub struct D3D12DescriptorHeap(pub(crate) D3D12Pageable);

impl Deref for D3D12DescriptorHeap {
	type Target = D3D12Pageable;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DescriptorHeap {
	const IID: &'static GUID = &GUID::from_u128(0x8efb471d616c4f4990f7127bb763fa51u128);
}

impl Com for D3D12DescriptorHeap {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DescriptorHeap {
	pub fn GetDesc(&self) -> D3D12DescriptorHeapDesc {
		unsafe {
			let vt = self.as_param();
			let mut _out_: MaybeUninit<D3D12DescriptorHeapDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12DescriptorHeapDesc) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, _out_.as_mut_ptr());
			_out_.assume_init()
		}
	}

	pub fn GetCPUDescriptorHandleForHeapStart(&self) -> Option<D3D12CpuDescriptorHandle> {
		let vt = self.as_param();
		let mut out = None;
		let f: extern "system" fn(Param<Self>, *mut Option<D3D12CpuDescriptorHandle>) -> *const D3D12CpuDescriptorHandle
			= unsafe { transmute(vt[9]) };
		let _ = f(vt, &mut out);
		out
	}

	pub fn GetGPUDescriptorHandleForHeapStart(&self) -> Option<D3D12GpuDescriptorHandle> {
		let vt = self.as_param();
		let mut out = None;
		let f: extern "system" fn(Param<Self>, &mut Option<D3D12GpuDescriptorHandle>) -> *const D3D12GpuDescriptorHandle
			= unsafe { transmute(vt[10]) };
		let _ = f(vt, &mut out);
		out
	}
}

pub trait ID3D12DescriptorHeap: ID3D12Pageable {
	fn as_descriptor_heap(&self) -> &D3D12DescriptorHeap;
	fn into_descriptor_heap(self) -> D3D12DescriptorHeap;
}

impl ID3D12DescriptorHeap for D3D12DescriptorHeap {
	fn as_descriptor_heap(&self) -> &D3D12DescriptorHeap { self }
	fn into_descriptor_heap(self) -> D3D12DescriptorHeap { self }
}
impl ID3D12Pageable for D3D12DescriptorHeap {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12DescriptorHeap {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12DescriptorHeap {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12DescriptorHeap {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DescriptorHeap {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

