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
pub struct D3D12DebugDevice1(pub(crate) Unknown);

impl Deref for D3D12DebugDevice1 {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DebugDevice1 {
	const IID: &'static GUID = &GUID::from_u128(0xa9b71770d0994a65a6983dee10020f88u128);
}

impl Com for D3D12DebugDevice1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DebugDevice1 {
	pub fn SetDebugParameter(&self, r#type: D3D12DebugDeviceParameterType, data: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, D3D12DebugDeviceParameterType, *const u8, u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, r#type, data_ptr_, data_len_ as u32);
			_ret_.ok()
		}
	}

	pub unsafe fn GetDebugParameter(&self) { todo!() }

	pub fn ReportLiveDeviceObjects(&self, flags: D3D12RldoFlags) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12RldoFlags) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, flags);
			_ret_.ok()
		}
	}
}

pub trait ID3D12DebugDevice1: IUnknown {
	fn as_debug_device1(&self) -> &D3D12DebugDevice1;
	fn into_debug_device1(self) -> D3D12DebugDevice1;
}

impl ID3D12DebugDevice1 for D3D12DebugDevice1 {
	fn as_debug_device1(&self) -> &D3D12DebugDevice1 { self }
	fn into_debug_device1(self) -> D3D12DebugDevice1 { self }
}
impl IUnknown for D3D12DebugDevice1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DebugDevice1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

