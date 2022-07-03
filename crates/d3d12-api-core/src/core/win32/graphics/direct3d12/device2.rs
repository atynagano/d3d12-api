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
pub struct D3D12Device2(pub(crate) D3D12Device1);

impl Guid for D3D12Device2 {
	const IID: &'static GUID = &GUID::from_u128(0x30baa41eb15b475ca0bb1af5c5b64328u128);
}

impl Clone for D3D12Device2 {
	fn clone(&self) -> Self { D3D12Device2(self.0.clone()) }
}

pub trait ID3D12Device2: ID3D12Device1 {
	fn as_device2(&self) -> &D3D12Device2;
	fn into_device2(self) -> D3D12Device2;

	fn CreatePipelineState<T: IUnknown>(&self, desc: &D3D12PipelineStateStreamDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _pipeline_state: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12PipelineStateStreamDesc, riid: &GUID, _pipeline_state: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[47]) };
		let ret = f(vt, desc, T::IID, &mut _pipeline_state, );
		if ret.is_ok() {
			if let (Some(_pipeline_state)) = (_pipeline_state) {
				return Ok((T::from(_pipeline_state)));
			}
		}
		Err(ret)
	}
}

impl ID3D12Device2 for D3D12Device2 {
	fn as_device2(&self) -> &D3D12Device2 { self }
	fn into_device2(self) -> D3D12Device2 { self }
}

impl ID3D12Device1 for D3D12Device2 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0 }
	fn into_device1(self) -> D3D12Device1 { self.0 }
}

impl ID3D12Device for D3D12Device2 {
	fn as_device(&self) -> &D3D12Device { &self.0.0 }
	fn into_device(self) -> D3D12Device { self.0.0 }
}

impl ID3D12Object for D3D12Device2 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12Device2 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device1::from(v))
    }
}

impl IUnknown for D3D12Device2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

