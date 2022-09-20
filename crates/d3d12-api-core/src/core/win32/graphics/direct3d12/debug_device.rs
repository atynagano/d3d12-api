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
pub struct D3D12DebugDevice(pub(crate) Unknown);

impl Deref for D3D12DebugDevice {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DebugDevice {
	const IID: &'static GUID = &GUID::from_u128(0x3febd6dd497347878194e45f9e28923eu128);
}

impl Com for D3D12DebugDevice {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DebugDevice {
	pub fn SetFeatureMask(&self, mask: D3D12DebugFeature) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12DebugFeature) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, mask);
			_ret_.ok()
		}
	}

	pub fn GetFeatureMask(&self) -> D3D12DebugFeature {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D3D12DebugFeature
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

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

pub trait ID3D12DebugDevice: IUnknown {
	fn as_debug_device(&self) -> &D3D12DebugDevice;
	fn into_debug_device(self) -> D3D12DebugDevice;
}

impl ID3D12DebugDevice for D3D12DebugDevice {
	fn as_debug_device(&self) -> &D3D12DebugDevice { self }
	fn into_debug_device(self) -> D3D12DebugDevice { self }
}
impl IUnknown for D3D12DebugDevice {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DebugDevice {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

