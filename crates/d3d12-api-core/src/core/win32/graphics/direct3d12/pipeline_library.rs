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
pub struct D3D12PipelineLibrary(pub(crate) D3D12DeviceChild);

impl Guid for D3D12PipelineLibrary {
	const IID: &'static GUID = &GUID::from_u128(0xc64226a8920146afb4cc53fb9ff7414fu128);
}

impl Clone for D3D12PipelineLibrary {
	fn clone(&self) -> Self { D3D12PipelineLibrary(self.0.clone()) }
}

pub trait ID3D12PipelineLibrary: ID3D12DeviceChild {
	fn as_pipeline_library(&self) -> &D3D12PipelineLibrary;
	fn into_pipeline_library(self) -> D3D12PipelineLibrary;

	fn StorePipeline(&self, name: Option<&str>, pipeline: &(impl ID3D12PipelineState + ?Sized), ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, name: *const u16, pipeline: VTable, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, name.map(str::to_utf16).as_ptr_or_null(), pipeline.vtable(), );
		ret.ok()
	}

	fn LoadGraphicsPipeline<T: IUnknown>(&self, name: &str, desc: &D3D12GraphicsPipelineStateDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _pipeline_state: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, name: *const u16, desc: &D3D12GraphicsPipelineStateDesc, riid: &GUID, _pipeline_state: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, name.to_utf16().as_ptr_or_null(), desc, T::IID, &mut _pipeline_state, );
		if ret.is_ok() {
			if let (Some(_pipeline_state)) = (_pipeline_state) {
				return Ok((T::from(_pipeline_state)));
			}
		}
		Err(ret)
	}

	fn LoadComputePipeline<T: IUnknown>(&self, name: &str, desc: &D3D12ComputePipelineStateDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _pipeline_state: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, name: *const u16, desc: &D3D12ComputePipelineStateDesc, riid: &GUID, _pipeline_state: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, name.to_utf16().as_ptr_or_null(), desc, T::IID, &mut _pipeline_state, );
		if ret.is_ok() {
			if let (Some(_pipeline_state)) = (_pipeline_state) {
				return Ok((T::from(_pipeline_state)));
			}
		}
		Err(ret)
	}

	fn GetSerializedSize(&self, ) -> (usize) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> usize
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, );
		return (ret);
	}

	fn Serialize(&self, mut data: &mut [u8], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, data: *mut u8, data_size_in_bytes: usize, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, data.as_mut_ptr_or_null(), data.len() as usize, );
		ret.ok()
	}
}

impl ID3D12PipelineLibrary for D3D12PipelineLibrary {
	fn as_pipeline_library(&self) -> &D3D12PipelineLibrary { self }
	fn into_pipeline_library(self) -> D3D12PipelineLibrary { self }
}

impl ID3D12DeviceChild for D3D12PipelineLibrary {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0 }
}

impl ID3D12Object for D3D12PipelineLibrary {
	fn as_object(&self) -> &D3D12Object { &self.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0 }
}

impl From<Unknown> for D3D12PipelineLibrary {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

impl IUnknown for D3D12PipelineLibrary {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

