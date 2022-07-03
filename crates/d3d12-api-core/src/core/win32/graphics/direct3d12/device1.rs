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
pub struct D3D12Device1(pub(crate) D3D12Device);

impl Guid for D3D12Device1 {
	const IID: &'static GUID = &GUID::from_u128(0x77acce80638e4e658895c1f23386863eu128);
}

impl Clone for D3D12Device1 {
	fn clone(&self) -> Self { D3D12Device1(self.0.clone()) }
}

pub trait ID3D12Device1: ID3D12Device {
	fn as_device1(&self) -> &D3D12Device1;
	fn into_device1(self) -> D3D12Device1;

	fn CreatePipelineLibrary<T: IUnknown>(&self, library_blob: &[u8], ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _pipeline_library: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, library_blob: *const u8, blob_length: usize, riid: &GUID, _pipeline_library: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[44]) };
		let ret = f(vt, library_blob.as_ptr_or_null(), library_blob.len() as usize, T::IID, &mut _pipeline_library, );
		if ret.is_ok() {
			if let (Some(_pipeline_library)) = (_pipeline_library) {
				return Ok((T::from(_pipeline_library)));
			}
		}
		Err(ret)
	}

	fn SetEventOnMultipleFenceCompletion(&self, fences: &[Param<D3D12Fence>], fence_values: &[u64], flags: D3D12MultipleFenceWaitFlags, event: Handle, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, fences: *const Param<D3D12Fence>, fence_values: *const u64, num_fences: u32, flags: D3D12MultipleFenceWaitFlags, event: Handle, ) -> HResult
			= unsafe { transmute(vt[45]) };
		let ret = f(vt, fences.as_ptr_or_null(), fence_values.as_ptr_or_null(), fences.len() as u32, flags, event, );
		ret.ok()
	}

	fn SetResidencyPriority(&self, objects: &[Param<D3D12Pageable>], priorities: &[D3D12ResidencyPriority], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_objects: u32, objects: *const Param<D3D12Pageable>, priorities: *const D3D12ResidencyPriority, ) -> HResult
			= unsafe { transmute(vt[46]) };
		let ret = f(vt, objects.len() as u32, objects.as_ptr_or_null(), priorities.as_ptr_or_null(), );
		ret.ok()
	}
}

impl ID3D12Device1 for D3D12Device1 {
	fn as_device1(&self) -> &D3D12Device1 { self }
	fn into_device1(self) -> D3D12Device1 { self }
}

impl ID3D12Device for D3D12Device1 {
	fn as_device(&self) -> &D3D12Device { &self.0 }
	fn into_device(self) -> D3D12Device { self.0 }
}

impl ID3D12Object for D3D12Device1 {
	fn as_object(&self) -> &D3D12Object { &self.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0 }
}

impl From<Unknown> for D3D12Device1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device::from(v))
    }
}

impl IUnknown for D3D12Device1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

