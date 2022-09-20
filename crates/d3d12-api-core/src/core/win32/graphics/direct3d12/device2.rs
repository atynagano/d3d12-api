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
pub struct D3D12Device2(pub(crate) D3D12Device1);

impl Deref for D3D12Device2 {
	type Target = D3D12Device1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Device2 {
	const IID: &'static GUID = &GUID::from_u128(0x30baa41eb15b475ca0bb1af5c5b64328u128);
}

impl Com for D3D12Device2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Device2 {
	pub fn CreatePipelineState<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12PipelineStateStreamDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pipeline_state_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12PipelineStateStreamDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[47]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut pipeline_state_out_));
			if _ret_.is_ok() { if let Some(pipeline_state_out_) = pipeline_state_out_ { return Ok(T::from(UnknownWrapper(pipeline_state_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait ID3D12Device2: ID3D12Device1 {
	fn as_device2(&self) -> &D3D12Device2;
	fn into_device2(self) -> D3D12Device2;
}

impl ID3D12Device2 for D3D12Device2 {
	fn as_device2(&self) -> &D3D12Device2 { self }
	fn into_device2(self) -> D3D12Device2 { self }
}
impl ID3D12Device1 for D3D12Device2 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D3D12Device1 { self.0.into_device1() }
}

impl ID3D12Device for D3D12Device2 {
	fn as_device(&self) -> &D3D12Device { &self.0.as_device() }
	fn into_device(self) -> D3D12Device { self.0.into_device() }
}

impl ID3D12Object for D3D12Device2 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Device2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Device2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Device1::from(v))
    }
}

