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
use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Resource(pub(crate) D3D12Pageable);

impl Deref for D3D12Resource {
	type Target = D3D12Pageable;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Resource {
	const IID: &'static GUID = &GUID::from_u128(0x696442bea72e4059bc795b5c98040fadu128);
}

impl Com for D3D12Resource {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Resource {
	pub fn Map(&self, subresource: u32, read_range: Option<&D3D12Range>, data: Option<&mut Option<NonNull<()>>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, *const c_void, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, subresource, transmute(read_range), transmute(data));
			_ret_.ok()
		}
	}

	pub fn Unmap(&self, subresource: u32, written_range: Option<&D3D12Range>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, *const c_void) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, subresource, transmute(written_range));
		}
	}

	pub fn GetDesc(&self) -> D3D12ResourceDesc {
		unsafe {
			let vt = self.as_param();
			let mut _out_: MaybeUninit<D3D12ResourceDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12ResourceDesc) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, _out_.as_mut_ptr());
			_out_.assume_init()
		}
	}

	pub fn GetGPUVirtualAddress(&self, ) -> Option<D3D12GpuVirtualAddress> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Option<D3D12GpuVirtualAddress>
				= transmute(vt[11]);
			f(vt)
		}
	}

	pub fn WriteToSubresource(&self, dst_subresource: u32, dst_box: Option<&D3D12Box>, src_data: *const impl Sized, src_row_pitch: u32, src_depth_pitch: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, *const c_void, *const c_void, u32, u32) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, dst_subresource, transmute(dst_box), src_data as _, src_row_pitch, src_depth_pitch);
			_ret_.ok()
		}
	}

	pub fn GetHeapProperties(&self, heap_properties: Option<&mut MaybeUninit<D3D12HeapProperties>>, heap_flags: Option<&mut MaybeUninit<D3D12HeapFlags>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Option<&mut MaybeUninit<D3D12HeapProperties>>, Option<&mut MaybeUninit<D3D12HeapFlags>>) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, heap_properties, heap_flags);
			_ret_.ok()
		}
	}
}

pub trait ID3D12Resource: ID3D12Pageable {
	fn as_resource(&self) -> &D3D12Resource;
	fn into_resource(self) -> D3D12Resource;
}

impl ID3D12Resource for D3D12Resource {
	fn as_resource(&self) -> &D3D12Resource { self }
	fn into_resource(self) -> D3D12Resource { self }
}
impl ID3D12Pageable for D3D12Resource {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12Resource {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12Resource {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Resource {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Resource {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

