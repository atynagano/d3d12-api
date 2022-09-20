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
pub struct D3D12Device3(pub(crate) D3D12Device2);

impl Deref for D3D12Device3 {
	type Target = D3D12Device2;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Device3 {
	const IID: &'static GUID = &GUID::from_u128(0x81dadc152bad439293c5101345c4aa98u128);
}

impl Com for D3D12Device3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Device3 {
	pub fn OpenExistingHeapFromAddress<T: IUnknown + From<UnknownWrapper>>(&self, address: *const impl Sized) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[48]);
			let _ret_ = f(vt, address as _, T::IID, transmute(&mut heap_out_));
			if _ret_.is_ok() { if let Some(heap_out_) = heap_out_ { return Ok(T::from(UnknownWrapper(heap_out_))); } }
			Err(_ret_)
		}
	}

	pub fn OpenExistingHeapFromFileMapping<T: IUnknown + From<UnknownWrapper>>(&self, file_mapping: Handle) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, Handle, &GUID, *mut c_void) -> HResult
				= transmute(vt[49]);
			let _ret_ = f(vt, file_mapping, T::IID, transmute(&mut heap_out_));
			if _ret_.is_ok() { if let Some(heap_out_) = heap_out_ { return Ok(T::from(UnknownWrapper(heap_out_))); } }
			Err(_ret_)
		}
	}

	pub fn EnqueueMakeResident(&self, flags: D3D12ResidencyFlags, objects: &[Param<D3D12Pageable>], fence_to_signal: &D3D12Fence, fence_value_to_signal: u64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (objects_ptr_, objects_len_) = objects.deconstruct();
			let f: extern "system" fn(Param<Self>, D3D12ResidencyFlags, u32, *const Param<D3D12Pageable>, VTable, u64) -> HResult
				= transmute(vt[50]);
			let _ret_ = f(vt, flags, objects_len_ as u32, objects_ptr_, fence_to_signal.vtable(), fence_value_to_signal);
			_ret_.ok()
		}
	}
}

pub trait ID3D12Device3: ID3D12Device2 {
	fn as_device3(&self) -> &D3D12Device3;
	fn into_device3(self) -> D3D12Device3;
}

impl ID3D12Device3 for D3D12Device3 {
	fn as_device3(&self) -> &D3D12Device3 { self }
	fn into_device3(self) -> D3D12Device3 { self }
}
impl ID3D12Device2 for D3D12Device3 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.as_device2() }
	fn into_device2(self) -> D3D12Device2 { self.0.into_device2() }
}

impl ID3D12Device1 for D3D12Device3 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D3D12Device1 { self.0.into_device1() }
}

impl ID3D12Device for D3D12Device3 {
	fn as_device(&self) -> &D3D12Device { &self.0.as_device() }
	fn into_device(self) -> D3D12Device { self.0.into_device() }
}

impl ID3D12Object for D3D12Device3 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Device3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Device3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Device2::from(v))
    }
}

