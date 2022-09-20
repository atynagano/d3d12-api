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
pub struct D3D12PipelineLibrary1(pub(crate) D3D12PipelineLibrary);

impl Deref for D3D12PipelineLibrary1 {
	type Target = D3D12PipelineLibrary;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12PipelineLibrary1 {
	const IID: &'static GUID = &GUID::from_u128(0x80eabf4225684e5ebd82c37f86961dc3u128);
}

impl Com for D3D12PipelineLibrary1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12PipelineLibrary1 {
	pub fn LoadPipeline<T: IUnknown + From<UnknownWrapper>>(&self, name: &str, desc: &D3D12PipelineStateStreamDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pipeline_state_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, *const u16, &D3D12PipelineStateStreamDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), desc, T::IID, transmute(&mut pipeline_state_out_));
			if _ret_.is_ok() { if let Some(pipeline_state_out_) = pipeline_state_out_ { return Ok(T::from(UnknownWrapper(pipeline_state_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait ID3D12PipelineLibrary1: ID3D12PipelineLibrary {
	fn as_pipeline_library1(&self) -> &D3D12PipelineLibrary1;
	fn into_pipeline_library1(self) -> D3D12PipelineLibrary1;
}

impl ID3D12PipelineLibrary1 for D3D12PipelineLibrary1 {
	fn as_pipeline_library1(&self) -> &D3D12PipelineLibrary1 { self }
	fn into_pipeline_library1(self) -> D3D12PipelineLibrary1 { self }
}
impl ID3D12PipelineLibrary for D3D12PipelineLibrary1 {
	fn as_pipeline_library(&self) -> &D3D12PipelineLibrary { &self.0.as_pipeline_library() }
	fn into_pipeline_library(self) -> D3D12PipelineLibrary { self.0.into_pipeline_library() }
}

impl ID3D12DeviceChild for D3D12PipelineLibrary1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12PipelineLibrary1 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12PipelineLibrary1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12PipelineLibrary1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12PipelineLibrary::from(v))
    }
}

