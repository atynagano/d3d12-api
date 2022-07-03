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

use crate::core::win32::graphics::direct3d12::*;
use crate::core::win32::foundation::*;
#[repr(C)]
pub struct D3D12Device8(pub(crate) D3D12Device7);

impl Guid for D3D12Device8 {
	const IID: &'static GUID = &GUID::from_u128(0x9218e6bbf9444f7ea75cb1b2c7b701f3u128);
}

impl Clone for D3D12Device8 {
	fn clone(&self) -> Self { D3D12Device8(self.0.clone()) }
}

pub trait ID3D12Device8: ID3D12Device7 {
	fn as_device8(&self) -> &D3D12Device8;
	fn into_device8(self) -> D3D12Device8;

	fn GetResourceAllocationInfo2(&self, visible_mask: u32, resource_descs: &[D3D12ResourceDesc1], mut resource_allocation_info1: Option<&mut [D3D12ResourceAllocationInfo1]>, ) -> (D3D12ResourceAllocationInfo) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, visible_mask: u32, num_resource_descs: u32, resource_descs: *const D3D12ResourceDesc1, resource_allocation_info1: *mut D3D12ResourceAllocationInfo1, ) -> D3D12ResourceAllocationInfo
			= unsafe { transmute(vt[68]) };
		let ret = f(vt, visible_mask, resource_descs.len() as u32, resource_descs.as_ptr_or_null(), resource_allocation_info1.as_mut_ptr_or_null(), );
		return (ret);
	}

	fn CreateCommittedResource2<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc1, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_resource: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc1, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<VTable>, riid_resource: &GUID, resource: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[69]) };
		let ret = f(vt, heap_properties, heap_flags, desc, initial_resource_state, optimized_clear_value, option_to_vtable(protected_session), T::IID, if resource.is_some() { Some(&mut out_resource) } else { None }, );
		if let (Some(resource), Some(out_resource)) = (resource, out_resource) { *resource = Some(T::from(out_resource)); }
		ret.ok()
	}

	fn CreatePlacedResource1<T: IUnknown>(&self, heap: &(impl ID3D12Heap + ?Sized), heap_offset: u64, desc: &D3D12ResourceDesc1, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_resource: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, heap: VTable, heap_offset: u64, desc: &D3D12ResourceDesc1, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, riid: &GUID, resource: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[70]) };
		let ret = f(vt, heap.vtable(), heap_offset, desc, initial_state, optimized_clear_value, T::IID, if resource.is_some() { Some(&mut out_resource) } else { None }, );
		if let (Some(resource), Some(out_resource)) = (resource, out_resource) { *resource = Some(T::from(out_resource)); }
		ret.ok()
	}

	fn CreateSamplerFeedbackUnorderedAccessView(&self, targeted_resource: Option<&D3D12Resource>, feedback_resource: Option<&D3D12Resource>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, targeted_resource: Option<VTable>, feedback_resource: Option<VTable>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[71]) };
		let ret = f(vt, option_to_vtable(targeted_resource), option_to_vtable(feedback_resource), dest_descriptor, );
	}

	fn GetCopyableFootprints1(&self, resource_desc: &D3D12ResourceDesc1, first_subresource: u32, base_offset: u64, mut layouts: Option<&mut [D3D12PlacedSubresourceFootprint]>, mut num_rows: Option<&mut [u32]>, mut row_size_in_bytes: Option<&mut [u64]>, total_bytes: Option<&mut u64>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resource_desc: &D3D12ResourceDesc1, first_subresource: u32, num_subresources: u32, base_offset: u64, layouts: *mut D3D12PlacedSubresourceFootprint, num_rows: *mut u32, row_size_in_bytes: *mut u64, total_bytes: Option<&mut u64>, ) -> ()
			= unsafe { transmute(vt[72]) };
		let ret = f(vt, resource_desc, first_subresource, layouts.len() as u32, base_offset, layouts.as_mut_ptr_or_null(), num_rows.as_mut_ptr_or_null(), row_size_in_bytes.as_mut_ptr_or_null(), total_bytes, );
	}
}

impl ID3D12Device8 for D3D12Device8 {
	fn as_device8(&self) -> &D3D12Device8 { self }
	fn into_device8(self) -> D3D12Device8 { self }
}

impl ID3D12Device7 for D3D12Device8 {
	fn as_device7(&self) -> &D3D12Device7 { &self.0 }
	fn into_device7(self) -> D3D12Device7 { self.0 }
}

impl ID3D12Device6 for D3D12Device8 {
	fn as_device6(&self) -> &D3D12Device6 { &self.0.0 }
	fn into_device6(self) -> D3D12Device6 { self.0.0 }
}

impl ID3D12Device5 for D3D12Device8 {
	fn as_device5(&self) -> &D3D12Device5 { &self.0.0.0 }
	fn into_device5(self) -> D3D12Device5 { self.0.0.0 }
}

impl ID3D12Device4 for D3D12Device8 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0.0.0.0 }
	fn into_device4(self) -> D3D12Device4 { self.0.0.0.0 }
}

impl ID3D12Device3 for D3D12Device8 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.0.0.0.0 }
	fn into_device3(self) -> D3D12Device3 { self.0.0.0.0.0 }
}

impl ID3D12Device2 for D3D12Device8 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.0.0.0.0.0 }
	fn into_device2(self) -> D3D12Device2 { self.0.0.0.0.0.0 }
}

impl ID3D12Device1 for D3D12Device8 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.0.0.0.0.0.0 }
	fn into_device1(self) -> D3D12Device1 { self.0.0.0.0.0.0.0 }
}

impl ID3D12Device for D3D12Device8 {
	fn as_device(&self) -> &D3D12Device { &self.0.0.0.0.0.0.0.0 }
	fn into_device(self) -> D3D12Device { self.0.0.0.0.0.0.0.0 }
}

impl ID3D12Object for D3D12Device8 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0.0.0.0.0 }
}

impl From<Unknown> for D3D12Device8 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device7::from(v))
    }
}

impl IUnknown for D3D12Device8 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0.0.0 }
}

