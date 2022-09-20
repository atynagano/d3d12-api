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
pub struct D3D12Device4(pub(crate) D3D12Device3);

impl Deref for D3D12Device4 {
	type Target = D3D12Device3;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Device4 {
	const IID: &'static GUID = &GUID::from_u128(0xe865df17a9ee46f9a4633098315aa2e5u128);
}

impl Com for D3D12Device4 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Device4 {
	pub fn CreateCommandList1<T: IUnknown + From<UnknownWrapper>>(&self, node_mask: u32, r#type: D3D12CommandListType, flags: D3D12CommandListFlags) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut command_list_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, u32, D3D12CommandListType, D3D12CommandListFlags, &GUID, *mut c_void) -> HResult
				= transmute(vt[51]);
			let _ret_ = f(vt, node_mask, r#type, flags, T::IID, transmute(&mut command_list_out_));
			if _ret_.is_ok() { if let Some(command_list_out_) = command_list_out_ { return Ok(T::from(UnknownWrapper(command_list_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateProtectedResourceSession<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12ProtectedResourceSessionDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut session_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12ProtectedResourceSessionDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[52]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut session_out_));
			if _ret_.is_ok() { if let Some(session_out_) = session_out_ { return Ok(T::from(UnknownWrapper(session_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateCommittedResource1<T: IUnknown + From<UnknownWrapper>>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, resource: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12HeapProperties, D3D12HeapFlags, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[53]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(if resource_out_.is_some() { Some(&mut resource_out_) } else { None }));
			if let Some(resource_out_) = resource_out_ { *resource.unwrap_unchecked() = Some(T::from(UnknownWrapper(resource_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_committed_resource1<T: IUnknown + From<UnknownWrapper>>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12HeapProperties, D3D12HeapFlags, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[53]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(&mut resource_out_));
			if _ret_.is_ok() { if let Some(resource_out_) = resource_out_ { return Ok(T::from(UnknownWrapper(resource_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateHeap1<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12HeapDesc, protected_session: Option<&D3D12ProtectedResourceSession>, heap: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12HeapDesc, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[54]);
			let _ret_ = f(vt, desc, option_to_vtable(protected_session), T::IID, transmute(if heap_out_.is_some() { Some(&mut heap_out_) } else { None }));
			if let Some(heap_out_) = heap_out_ { *heap.unwrap_unchecked() = Some(T::from(UnknownWrapper(heap_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_heap1<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12HeapDesc, protected_session: Option<&D3D12ProtectedResourceSession>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12HeapDesc, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[54]);
			let _ret_ = f(vt, desc, option_to_vtable(protected_session), T::IID, transmute(&mut heap_out_));
			if _ret_.is_ok() { if let Some(heap_out_) = heap_out_ { return Ok(T::from(UnknownWrapper(heap_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateReservedResource1<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, resource: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[55]);
			let _ret_ = f(vt, desc, initial_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(if resource_out_.is_some() { Some(&mut resource_out_) } else { None }));
			if let Some(resource_out_) = resource_out_ { *resource.unwrap_unchecked() = Some(T::from(UnknownWrapper(resource_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_reserved_resource1<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[55]);
			let _ret_ = f(vt, desc, initial_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(&mut resource_out_));
			if _ret_.is_ok() { if let Some(resource_out_) = resource_out_ { return Ok(T::from(UnknownWrapper(resource_out_))); } }
			Err(_ret_)
		}
	}

	pub unsafe fn GetResourceAllocationInfo1() { todo!() }
}

pub trait ID3D12Device4: ID3D12Device3 {
	fn as_device4(&self) -> &D3D12Device4;
	fn into_device4(self) -> D3D12Device4;
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

impl IUnknown for D3D12Device4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Device4 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Device3::from(v))
    }
}

