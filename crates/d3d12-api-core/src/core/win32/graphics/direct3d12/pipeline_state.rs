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
use crate::core::win32::graphics::direct3d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12PipelineState(pub(crate) D3D12Pageable);

impl Deref for D3D12PipelineState {
	type Target = D3D12Pageable;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12PipelineState {
	const IID: &'static GUID = &GUID::from_u128(0x765a30f3f6244c6fa828ace948622445u128);
}

impl Com for D3D12PipelineState {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12PipelineState {
	pub fn GetCachedBlob(&self) -> Result<D3DBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut blob_out_: Option<D3DBlob> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, transmute(&mut blob_out_));
			if _ret_.is_ok() { if let Some(blob_out_) = blob_out_ { return Ok(blob_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID3D12PipelineState: ID3D12Pageable {
	fn as_pipeline_state(&self) -> &D3D12PipelineState;
	fn into_pipeline_state(self) -> D3D12PipelineState;
}

impl ID3D12PipelineState for D3D12PipelineState {
	fn as_pipeline_state(&self) -> &D3D12PipelineState { self }
	fn into_pipeline_state(self) -> D3D12PipelineState { self }
}
impl ID3D12Pageable for D3D12PipelineState {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12PipelineState {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12PipelineState {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12PipelineState {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12PipelineState {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

