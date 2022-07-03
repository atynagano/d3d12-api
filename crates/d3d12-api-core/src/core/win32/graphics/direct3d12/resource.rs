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

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct3d12::*;
#[repr(C)]
pub struct D3D12Resource(pub(crate) D3D12Pageable);

impl Guid for D3D12Resource {
	const IID: &'static GUID = &GUID::from_u128(0x696442bea72e4059bc795b5c98040fadu128);
}

impl Clone for D3D12Resource {
	fn clone(&self) -> Self { D3D12Resource(self.0.clone()) }
}

pub trait ID3D12Resource: ID3D12Pageable {
	fn as_resource(&self) -> &D3D12Resource;
	fn into_resource(self) -> D3D12Resource;

	fn Map(&self, subresource: u32, read_range: Option<&D3D12Range>, data: Option<&mut *const c_void>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, subresource: u32, read_range: Option<&D3D12Range>, data: Option<&mut *const c_void>, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, subresource, read_range, data, );
		ret.ok()
	}

	fn Unmap(&self, subresource: u32, written_range: Option<&D3D12Range>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, subresource: u32, written_range: Option<&D3D12Range>, ) -> ()
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, subresource, written_range, );
	}

	fn GetDesc(&self, ) -> (D3D12ResourceDesc) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> D3D12ResourceDesc
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetGPUVirtualAddress(&self, ) -> (u64) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u64
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, );
		return (ret);
	}

	fn WriteToSubresource(&self, dst_subresource: u32, dst_box: Option<&D3D12Box>, src_data: *const c_void, src_row_pitch: u32, src_depth_pitch: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dst_subresource: u32, dst_box: Option<&D3D12Box>, src_data: *const c_void, src_row_pitch: u32, src_depth_pitch: u32, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, dst_subresource, dst_box, src_data, src_row_pitch, src_depth_pitch, );
		ret.ok()
	}

	fn GetHeapProperties(&self, heap_properties: Option<&mut D3D12HeapProperties>, heap_flags: Option<&mut D3D12HeapFlags>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, heap_properties: Option<&mut D3D12HeapProperties>, heap_flags: Option<&mut D3D12HeapFlags>, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, heap_properties, heap_flags, );
		ret.ok()
	}
}

impl ID3D12Resource for D3D12Resource {
	fn as_resource(&self) -> &D3D12Resource { self }
	fn into_resource(self) -> D3D12Resource { self }
}

impl ID3D12Pageable for D3D12Resource {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0 }
	fn into_pageable(self) -> D3D12Pageable { self.0 }
}

impl ID3D12DeviceChild for D3D12Resource {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0 }
}

impl ID3D12Object for D3D12Resource {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12Resource {
    fn from(v: Unknown) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

impl IUnknown for D3D12Resource {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

