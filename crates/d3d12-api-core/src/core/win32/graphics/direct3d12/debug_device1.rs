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
pub struct D3D12DebugDevice1(pub(crate) Unknown);

impl Guid for D3D12DebugDevice1 {
	const IID: &'static GUID = &GUID::from_u128(0xa9b71770d0994a65a6983dee10020f88u128);
}

impl Clone for D3D12DebugDevice1 {
	fn clone(&self) -> Self { D3D12DebugDevice1(self.0.clone()) }
}

pub trait ID3D12DebugDevice1: IUnknown {
	fn as_debug_device1(&self) -> &D3D12DebugDevice1;
	fn into_debug_device1(self) -> D3D12DebugDevice1;

	fn ReportLiveDeviceObjects(&self, flags: D3D12RldoFlags, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, flags: D3D12RldoFlags, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, flags, );
			_ret_.ok()
		}
	}
}

impl ID3D12DebugDevice1 for D3D12DebugDevice1 {
	fn as_debug_device1(&self) -> &D3D12DebugDevice1 { self }
	fn into_debug_device1(self) -> D3D12DebugDevice1 { self }
}

impl From<Unknown> for D3D12DebugDevice1 {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12DebugDevice1 {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

