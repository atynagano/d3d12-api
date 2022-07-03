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
use crate::core::win32::graphics::direct3d::*;
#[repr(C)]
pub struct D3D12PipelineState(pub(crate) D3D12Pageable);

impl Guid for D3D12PipelineState {
	const IID: &'static GUID = &GUID::from_u128(0x765a30f3f6244c6fa828ace948622445u128);
}

impl Clone for D3D12PipelineState {
	fn clone(&self) -> Self { D3D12PipelineState(self.0.clone()) }
}

pub trait ID3D12PipelineState: ID3D12Pageable {
	fn as_pipeline_state(&self) -> &D3D12PipelineState;
	fn into_pipeline_state(self) -> D3D12PipelineState;

	fn GetCachedBlob(&self, ) -> Result<(D3DBlob), HResult> {
		let vt = self.as_param();
		let mut _blob: Option<D3DBlob> = None;
		let f: extern "system" fn(Param<Self>, _blob: &mut Option<D3DBlob>, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, &mut _blob, );
		if ret.is_ok() {
			if let (Some(_blob)) = (_blob) {
				return Ok((_blob));
			}
		}
		Err(ret)
	}
}

impl ID3D12PipelineState for D3D12PipelineState {
	fn as_pipeline_state(&self) -> &D3D12PipelineState { self }
	fn into_pipeline_state(self) -> D3D12PipelineState { self }
}

impl ID3D12Pageable for D3D12PipelineState {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0 }
	fn into_pageable(self) -> D3D12Pageable { self.0 }
}

impl ID3D12DeviceChild for D3D12PipelineState {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0 }
}

impl ID3D12Object for D3D12PipelineState {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12PipelineState {
    fn from(v: Unknown) -> Self {
        Self(D3D12Pageable::from(v))
    }
}

impl IUnknown for D3D12PipelineState {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

