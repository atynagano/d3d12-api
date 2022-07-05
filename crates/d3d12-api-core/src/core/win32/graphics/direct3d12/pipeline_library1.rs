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
pub struct D3D12PipelineLibrary1(pub(crate) D3D12PipelineLibrary);

impl Guid for D3D12PipelineLibrary1 {
	const IID: &'static GUID = &GUID::from_u128(0x80eabf4225684e5ebd82c37f86961dc3u128);
}

impl Clone for D3D12PipelineLibrary1 {
	fn clone(&self) -> Self { D3D12PipelineLibrary1(self.0.clone()) }
}

pub trait ID3D12PipelineLibrary1: ID3D12PipelineLibrary {
	fn as_pipeline_library1(&self) -> &D3D12PipelineLibrary1;
	fn into_pipeline_library1(self) -> D3D12PipelineLibrary1;

	fn LoadPipeline<T: IUnknown>(&self, name: &str, desc: &D3D12PipelineStateStreamDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pipeline_state: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, name: *const u16, desc: &D3D12PipelineStateStreamDesc, riid: &GUID, _out_pipeline_state: *mut c_void, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), desc, T::IID, transmute(&mut _out_pipeline_state), );
			if _ret_.is_ok() {
				if let Some(_out_pipeline_state) = _out_pipeline_state {
					return Ok(T::from(_out_pipeline_state));
				}
			}
			Err(_ret_)
		}
	}
}

impl ID3D12PipelineLibrary1 for D3D12PipelineLibrary1 {
	fn as_pipeline_library1(&self) -> &D3D12PipelineLibrary1 { self }
	fn into_pipeline_library1(self) -> D3D12PipelineLibrary1 { self }
}

impl ID3D12PipelineLibrary for D3D12PipelineLibrary1 {
	fn as_pipeline_library(&self) -> &D3D12PipelineLibrary { &self.0 }
	fn into_pipeline_library(self) -> D3D12PipelineLibrary { self.0 }
}

impl ID3D12DeviceChild for D3D12PipelineLibrary1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0 }
}

impl ID3D12Object for D3D12PipelineLibrary1 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12PipelineLibrary1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12PipelineLibrary::from(v))
    }
}

impl IUnknown for D3D12PipelineLibrary1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

