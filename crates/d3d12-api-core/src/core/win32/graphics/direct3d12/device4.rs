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
pub struct D3D12Device4(pub(crate) D3D12Device3);

impl Guid for D3D12Device4 {
	const IID: &'static GUID = &GUID::from_u128(0xe865df17a9ee46f9a4633098315aa2e5u128);
}

impl Clone for D3D12Device4 {
	fn clone(&self) -> Self { D3D12Device4(self.0.clone()) }
}

pub trait ID3D12Device4: ID3D12Device3 {
	fn as_device4(&self) -> &D3D12Device4;
	fn into_device4(self) -> D3D12Device4;

	fn CreateCommandList1<T: IUnknown>(&self, node_mask: u32, ty: D3D12CommandListType, flags: D3D12CommandListFlags, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _command_list: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, node_mask: u32, ty: D3D12CommandListType, flags: D3D12CommandListFlags, riid: &GUID, _command_list: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[51]) };
		let ret = f(vt, node_mask, ty, flags, T::IID, &mut _command_list, );
		if ret.is_ok() {
			if let (Some(_command_list)) = (_command_list) {
				return Ok((T::from(_command_list)));
			}
		}
		Err(ret)
	}

	fn CreateProtectedResourceSession<T: IUnknown>(&self, desc: &D3D12ProtectedResourceSessionDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _session: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12ProtectedResourceSessionDesc, riid: &GUID, _session: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[52]) };
		let ret = f(vt, desc, T::IID, &mut _session, );
		if ret.is_ok() {
			if let (Some(_session)) = (_session) {
				return Ok((T::from(_session)));
			}
		}
		Err(ret)
	}

	fn CreateCommittedResource1<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_resource: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<VTable>, riid_resource: &GUID, resource: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[53]) };
		let ret = f(vt, heap_properties, heap_flags, desc, initial_resource_state, optimized_clear_value, option_to_vtable(protected_session), T::IID, if resource.is_some() { Some(&mut out_resource) } else { None }, );
		if let (Some(resource), Some(out_resource)) = (resource, out_resource) { *resource = Some(T::from(out_resource)); }
		ret.ok()
	}

	fn CreateHeap1<T: IUnknown>(&self, desc: &D3D12HeapDesc, protected_session: Option<&D3D12ProtectedResourceSession>, heap: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_heap: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12HeapDesc, protected_session: Option<VTable>, riid: &GUID, heap: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[54]) };
		let ret = f(vt, desc, option_to_vtable(protected_session), T::IID, if heap.is_some() { Some(&mut out_heap) } else { None }, );
		if let (Some(heap), Some(out_heap)) = (heap, out_heap) { *heap = Some(T::from(out_heap)); }
		ret.ok()
	}

	fn CreateReservedResource1<T: IUnknown>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_resource: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<VTable>, riid: &GUID, resource: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[55]) };
		let ret = f(vt, desc, initial_state, optimized_clear_value, option_to_vtable(protected_session), T::IID, if resource.is_some() { Some(&mut out_resource) } else { None }, );
		if let (Some(resource), Some(out_resource)) = (resource, out_resource) { *resource = Some(T::from(out_resource)); }
		ret.ok()
	}

	fn GetResourceAllocationInfo1(&self, visible_mask: u32, resource_descs: &[D3D12ResourceDesc], mut resource_allocation_info1: Option<&mut [D3D12ResourceAllocationInfo1]>, ) -> (D3D12ResourceAllocationInfo) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, visible_mask: u32, num_resource_descs: u32, resource_descs: *const D3D12ResourceDesc, resource_allocation_info1: *mut D3D12ResourceAllocationInfo1, ) -> D3D12ResourceAllocationInfo
			= unsafe { transmute(vt[56]) };
		let ret = f(vt, visible_mask, resource_descs.len() as u32, resource_descs.as_ptr_or_null(), resource_allocation_info1.as_mut_ptr_or_null(), );
		return (ret);
	}
}

impl ID3D12Device4 for D3D12Device4 {
	fn as_device4(&self) -> &D3D12Device4 { self }
	fn into_device4(self) -> D3D12Device4 { self }
}

impl ID3D12Device3 for D3D12Device4 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0 }
	fn into_device3(self) -> D3D12Device3 { self.0 }
}

impl ID3D12Device2 for D3D12Device4 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.0 }
	fn into_device2(self) -> D3D12Device2 { self.0.0 }
}

impl ID3D12Device1 for D3D12Device4 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.0.0 }
	fn into_device1(self) -> D3D12Device1 { self.0.0.0 }
}

impl ID3D12Device for D3D12Device4 {
	fn as_device(&self) -> &D3D12Device { &self.0.0.0.0 }
	fn into_device(self) -> D3D12Device { self.0.0.0.0 }
}

impl ID3D12Object for D3D12Device4 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0 }
}

impl From<Unknown> for D3D12Device4 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device3::from(v))
    }
}

impl IUnknown for D3D12Device4 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0 }
}

