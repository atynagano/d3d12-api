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

	fn CreateCommittedResource2<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc1, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc1, initial_resource_state: D3D12ResourceStates, optimized_clear_value: *const c_void, protected_session: *const c_void, riid_resource: &GUID, resource: *mut c_void, ) -> HResult
				= transmute(vt[69]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(if resource.is_some() { Some(&mut _out_resource) } else { None }), );
			if let Some(_out_resource) = _out_resource { *resource.unwrap_unchecked() = Some(T::from(_out_resource)); }
			_ret_.ok()
		}
	}

	fn create_committed_resource2<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc1, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, protected_session: Option<&D3D12ProtectedResourceSession>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc1, initial_resource_state: D3D12ResourceStates, optimized_clear_value: *const c_void, protected_session: *const c_void, riid_resource: &GUID, _out_resource: *mut c_void, ) -> HResult
				= transmute(vt[69]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), option_to_vtable(protected_session), T::IID, transmute(&mut _out_resource), );
			if _ret_.is_ok() {
				if let Some(_out_resource) = _out_resource {
					return Ok(T::from(_out_resource));
				}
			}
			Err(_ret_)
		}
	}

	fn CreatePlacedResource1<T: IUnknown>(&self, heap: &(impl ID3D12Heap + ?Sized), heap_offset: u64, desc: &D3D12ResourceDesc1, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap: VTable, heap_offset: u64, desc: &D3D12ResourceDesc1, initial_state: D3D12ResourceStates, optimized_clear_value: *const c_void, riid: &GUID, resource: *mut c_void, ) -> HResult
				= transmute(vt[70]);
			let _ret_ = f(vt, heap.vtable(), heap_offset, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(if resource.is_some() { Some(&mut _out_resource) } else { None }), );
			if let Some(_out_resource) = _out_resource { *resource.unwrap_unchecked() = Some(T::from(_out_resource)); }
			_ret_.ok()
		}
	}

	fn create_placed_resource1<T: IUnknown>(&self, heap: &(impl ID3D12Heap + ?Sized), heap_offset: u64, desc: &D3D12ResourceDesc1, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap: VTable, heap_offset: u64, desc: &D3D12ResourceDesc1, initial_state: D3D12ResourceStates, optimized_clear_value: *const c_void, riid: &GUID, _out_resource: *mut c_void, ) -> HResult
				= transmute(vt[70]);
			let _ret_ = f(vt, heap.vtable(), heap_offset, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(&mut _out_resource), );
			if _ret_.is_ok() {
				if let Some(_out_resource) = _out_resource {
					return Ok(T::from(_out_resource));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateSamplerFeedbackUnorderedAccessView(&self, targeted_resource: Option<&D3D12Resource>, feedback_resource: Option<&D3D12Resource>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, targeted_resource: *const c_void, feedback_resource: *const c_void, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
				= transmute(vt[71]);
			let _ret_ = f(vt, option_to_vtable(targeted_resource), option_to_vtable(feedback_resource), dest_descriptor, );
		}
	}
}

impl ID3D12Device8 for D3D12Device8 {
	fn as_device8(&self) -> &D3D12Device8 { self }
	fn into_device8(self) -> D3D12Device8 { self }
}

impl ID3D12Device7 for D3D12Device8 {
	fn as_device7(&self) -> &D3D12Device7 { &self.0.as_device7() }
	fn into_device7(self) -> D3D12Device7 { self.0.into_device7() }
}

impl ID3D12Device6 for D3D12Device8 {
	fn as_device6(&self) -> &D3D12Device6 { &self.0.as_device6() }
	fn into_device6(self) -> D3D12Device6 { self.0.into_device6() }
}

impl ID3D12Device5 for D3D12Device8 {
	fn as_device5(&self) -> &D3D12Device5 { &self.0.as_device5() }
	fn into_device5(self) -> D3D12Device5 { self.0.into_device5() }
}

impl ID3D12Device4 for D3D12Device8 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0.as_device4() }
	fn into_device4(self) -> D3D12Device4 { self.0.into_device4() }
}

impl ID3D12Device3 for D3D12Device8 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.as_device3() }
	fn into_device3(self) -> D3D12Device3 { self.0.into_device3() }
}

impl ID3D12Device2 for D3D12Device8 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.as_device2() }
	fn into_device2(self) -> D3D12Device2 { self.0.into_device2() }
}

impl ID3D12Device1 for D3D12Device8 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D3D12Device1 { self.0.into_device1() }
}

impl ID3D12Device for D3D12Device8 {
	fn as_device(&self) -> &D3D12Device { &self.0.as_device() }
	fn into_device(self) -> D3D12Device { self.0.into_device() }
}

impl ID3D12Object for D3D12Device8 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl From<Unknown> for D3D12Device8 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device7::from(v))
    }
}

impl IUnknown for D3D12Device8 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

