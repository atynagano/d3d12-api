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
pub struct D3D12DebugDevice(pub(crate) Unknown);

impl Guid for D3D12DebugDevice {
	const IID: &'static GUID = &GUID::from_u128(0x3febd6dd497347878194e45f9e28923eu128);
}

impl Clone for D3D12DebugDevice {
	fn clone(&self) -> Self { D3D12DebugDevice(self.0.clone()) }
}

pub trait ID3D12DebugDevice: IUnknown {
	fn as_debug_device(&self) -> &D3D12DebugDevice;
	fn into_debug_device(self) -> D3D12DebugDevice;

	fn SetFeatureMask(&self, mask: D3D12DebugFeature, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, mask: D3D12DebugFeature, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, mask, );
		ret.ok()
	}

	fn GetFeatureMask(&self, ) -> (D3D12DebugFeature) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> D3D12DebugFeature
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, );
		return (ret);
	}

	fn ReportLiveDeviceObjects(&self, flags: D3D12RldoFlags, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, flags: D3D12RldoFlags, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, flags, );
		ret.ok()
	}
}

impl ID3D12DebugDevice for D3D12DebugDevice {
	fn as_debug_device(&self) -> &D3D12DebugDevice { self }
	fn into_debug_device(self) -> D3D12DebugDevice { self }
}

impl From<Unknown> for D3D12DebugDevice {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12DebugDevice {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

