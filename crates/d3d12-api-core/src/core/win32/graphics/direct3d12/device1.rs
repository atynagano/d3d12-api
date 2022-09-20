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
pub struct D3D12Device1(pub(crate) D3D12Device);

impl Deref for D3D12Device1 {
	type Target = D3D12Device;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Device1 {
	const IID: &'static GUID = &GUID::from_u128(0x77acce80638e4e658895c1f23386863eu128);
}

impl Com for D3D12Device1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Device1 {
	pub fn CreatePipelineLibrary<T: IUnknown + From<UnknownWrapper>>(&self, library_blob: &[u8]) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let (library_blob_ptr_, library_blob_len_) = library_blob.deconstruct();
			let mut pipeline_library_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, *const u8, usize, &GUID, *mut c_void) -> HResult
				= transmute(vt[44]);
			let _ret_ = f(vt, library_blob_ptr_, library_blob_len_ as usize, T::IID, transmute(&mut pipeline_library_out_));
			if _ret_.is_ok() { if let Some(pipeline_library_out_) = pipeline_library_out_ { return Ok(T::from(UnknownWrapper(pipeline_library_out_))); } }
			Err(_ret_)
		}
	}

	pub unsafe fn SetEventOnMultipleFenceCompletion() { todo!() }

	pub unsafe fn SetResidencyPriority() { todo!() }
}

pub trait ID3D12Device1: ID3D12Device {
	fn as_device1(&self) -> &D3D12Device1;
	fn into_device1(self) -> D3D12Device1;
}

impl ID3D12Device1 for D3D12Device1 {
	fn as_device1(&self) -> &D3D12Device1 { self }
	fn into_device1(self) -> D3D12Device1 { self }
}
impl ID3D12Device for D3D12Device1 {
	fn as_device(&self) -> &D3D12Device { &self.0.as_device() }
	fn into_device(self) -> D3D12Device { self.0.into_device() }
}

impl ID3D12Object for D3D12Device1 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Device1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Device1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Device::from(v))
    }
}

