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

	fn CreateCommandList1<T: IUnknown>(&self, node_mask: u32, ty: D3D12CommandListType, flags: D3D12CommandListFlags, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_command_list: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, node_mask: u32, ty: D3D12CommandListType, flags: D3D12CommandListFlags, riid: &GUID, _out_command_list: *mut c_void, ) -> HResult
				= transmute(vt[51]);
			let _ret_ = f(vt, node_mask, ty, flags, T::IID, transmute(&mut _out_command_list), );
			if _ret_.is_ok() {
				if let Some(_out_command_list) = _out_command_list {
					return Ok(T::from(_out_command_list));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateProtectedResourceSession<T: IUnknown>(&self, desc: &D3D12ProtectedResourceSessionDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_session: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12ProtectedResourceSessionDesc, riid: &GUID, _out_session: *mut c_void, ) -> HResult
				= transmute(vt[52]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut _out_session), );
			if _ret_.is_ok() {
				if let Some(_out_session) = _out_session {
					return Ok(T::from(_out_session));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateCommittedResource1<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: *const c_void, protected_session: *const c_void, riid_resource: &GUID, resource: *mut c_void, ) -> HResult
				= transmute(vt[53]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(if resource.is_some() { Some(&mut _out_resource) } else { None }), );
			if let Some(_out_resource) = _out_resource { *resource.unwrap_unchecked() = Some(T::from(_out_resource)); }
			_ret_.ok()
		}
	}

	fn create_committed_resource1<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: *const c_void, protected_session: *const c_void, riid_resource: &GUID, _out_resource: *mut c_void, ) -> HResult
				= transmute(vt[53]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(&mut _out_resource), );
			if _ret_.is_ok() {
				if let Some(_out_resource) = _out_resource {
					return Ok(T::from(_out_resource));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateHeap1<T: IUnknown>(&self, desc: &D3D12HeapDesc, protected_session: Option<&D3D12ProtectedResourceSession>, heap: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_heap: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12HeapDesc, protected_session: *const c_void, riid: &GUID, heap: *mut c_void, ) -> HResult
				= transmute(vt[54]);
			let _ret_ = f(vt, desc, option_to_vtable(protected_session), T::IID, transmute(if heap.is_some() { Some(&mut _out_heap) } else { None }), );
			if let Some(_out_heap) = _out_heap { *heap.unwrap_unchecked() = Some(T::from(_out_heap)); }
			_ret_.ok()
		}
	}

	fn create_heap1<T: IUnknown>(&self, desc: &D3D12HeapDesc, protected_session: Option<&D3D12ProtectedResourceSession>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_heap: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12HeapDesc, protected_session: *const c_void, riid: &GUID, _out_heap: *mut c_void, ) -> HResult
				= transmute(vt[54]);
			let _ret_ = f(vt, desc, option_to_vtable(protected_session), T::IID, transmute(&mut _out_heap), );
			if _ret_.is_ok() {
				if let Some(_out_heap) = _out_heap {
					return Ok(T::from(_out_heap));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateReservedResource1<T: IUnknown>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: *const c_void, protected_session: *const c_void, riid: &GUID, resource: *mut c_void, ) -> HResult
				= transmute(vt[55]);
			let _ret_ = f(vt, desc, initial_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(if resource.is_some() { Some(&mut _out_resource) } else { None }), );
			if let Some(_out_resource) = _out_resource { *resource.unwrap_unchecked() = Some(T::from(_out_resource)); }
			_ret_.ok()
		}
	}

	fn create_reserved_resource1<T: IUnknown>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: *const c_void, protected_session: *const c_void, riid: &GUID, _out_resource: *mut c_void, ) -> HResult
				= transmute(vt[55]);
			let _ret_ = f(vt, desc, initial_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(&mut _out_resource), );
			if _ret_.is_ok() {
				if let Some(_out_resource) = _out_resource {
					return Ok(T::from(_out_resource));
				}
			}
			Err(_ret_)
		}
	}
}

impl ID3D12Device4 for D3D12Device4 {
	fn as_device4(&self) -> &D3D12Device4 { self }
	fn into_device4(self) -> D3D12Device4 { self }
}

impl ID3D12Device3 for D3D12Device4 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.as_device3() }
	fn into_device3(self) -> D3D12Device3 { self.0.into_device3() }
}

impl ID3D12Device2 for D3D12Device4 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.as_device2() }
	fn into_device2(self) -> D3D12Device2 { self.0.into_device2() }
}

impl ID3D12Device1 for D3D12Device4 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D3D12Device1 { self.0.into_device1() }
}

impl ID3D12Device for D3D12Device4 {
	fn as_device(&self) -> &D3D12Device { &self.0.as_device() }
	fn into_device(self) -> D3D12Device { self.0.into_device() }
}

impl ID3D12Object for D3D12Device4 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl From<Unknown> for D3D12Device4 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device3::from(v))
    }
}

impl IUnknown for D3D12Device4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

