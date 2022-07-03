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
pub struct D3D12Device3(pub(crate) D3D12Device2);

impl Guid for D3D12Device3 {
	const IID: &'static GUID = &GUID::from_u128(0x81dadc152bad439293c5101345c4aa98u128);
}

impl Clone for D3D12Device3 {
	fn clone(&self) -> Self { D3D12Device3(self.0.clone()) }
}

pub trait ID3D12Device3: ID3D12Device2 {
	fn as_device3(&self) -> &D3D12Device3;
	fn into_device3(self) -> D3D12Device3;

	fn OpenExistingHeapFromAddress<T: IUnknown>(&self, address: *const c_void, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _heap: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, address: *const c_void, riid: &GUID, _heap: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[48]) };
		let ret = f(vt, address, T::IID, &mut _heap, );
		if ret.is_ok() {
			if let (Some(_heap)) = (_heap) {
				return Ok((T::from(_heap)));
			}
		}
		Err(ret)
	}

	fn OpenExistingHeapFromFileMapping<T: IUnknown>(&self, file_mapping: Handle, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _heap: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, file_mapping: Handle, riid: &GUID, _heap: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[49]) };
		let ret = f(vt, file_mapping, T::IID, &mut _heap, );
		if ret.is_ok() {
			if let (Some(_heap)) = (_heap) {
				return Ok((T::from(_heap)));
			}
		}
		Err(ret)
	}

	fn EnqueueMakeResident(&self, flags: D3D12ResidencyFlags, objects: &[Param<D3D12Pageable>], fence_to_signal: &(impl ID3D12Fence + ?Sized), fence_value_to_signal: u64, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, flags: D3D12ResidencyFlags, num_objects: u32, objects: *const Param<D3D12Pageable>, fence_to_signal: VTable, fence_value_to_signal: u64, ) -> HResult
			= unsafe { transmute(vt[50]) };
		let ret = f(vt, flags, objects.len() as u32, objects.as_ptr_or_null(), fence_to_signal.vtable(), fence_value_to_signal, );
		ret.ok()
	}
}

impl ID3D12Device3 for D3D12Device3 {
	fn as_device3(&self) -> &D3D12Device3 { self }
	fn into_device3(self) -> D3D12Device3 { self }
}

impl ID3D12Device2 for D3D12Device3 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0 }
	fn into_device2(self) -> D3D12Device2 { self.0 }
}

impl ID3D12Device1 for D3D12Device3 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.0 }
	fn into_device1(self) -> D3D12Device1 { self.0.0 }
}

impl ID3D12Device for D3D12Device3 {
	fn as_device(&self) -> &D3D12Device { &self.0.0.0 }
	fn into_device(self) -> D3D12Device { self.0.0.0 }
}

impl ID3D12Object for D3D12Device3 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0 }
}

impl From<Unknown> for D3D12Device3 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device2::from(v))
    }
}

impl IUnknown for D3D12Device3 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

