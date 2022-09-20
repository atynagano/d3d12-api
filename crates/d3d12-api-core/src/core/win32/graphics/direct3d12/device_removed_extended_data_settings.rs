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

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12DeviceRemovedExtendedDataSettings(pub(crate) Unknown);

impl Deref for D3D12DeviceRemovedExtendedDataSettings {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DeviceRemovedExtendedDataSettings {
	const IID: &'static GUID = &GUID::from_u128(0x82bc481c6b9b4030aedb7ee3d1df1e63u128);
}

impl Com for D3D12DeviceRemovedExtendedDataSettings {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DeviceRemovedExtendedDataSettings {
	pub fn SetAutoBreadcrumbsEnablement(&self, enablement: D3D12DredEnablement) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12DredEnablement) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, enablement);
		}
	}

	pub fn SetPageFaultEnablement(&self, enablement: D3D12DredEnablement) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12DredEnablement) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, enablement);
		}
	}

	pub fn SetWatsonDumpEnablement(&self, enablement: D3D12DredEnablement) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12DredEnablement) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, enablement);
		}
	}
}

pub trait ID3D12DeviceRemovedExtendedDataSettings: IUnknown {
	fn as_device_removed_extended_data_settings(&self) -> &D3D12DeviceRemovedExtendedDataSettings;
	fn into_device_removed_extended_data_settings(self) -> D3D12DeviceRemovedExtendedDataSettings;
}

impl ID3D12DeviceRemovedExtendedDataSettings for D3D12DeviceRemovedExtendedDataSettings {
	fn as_device_removed_extended_data_settings(&self) -> &D3D12DeviceRemovedExtendedDataSettings { self }
	fn into_device_removed_extended_data_settings(self) -> D3D12DeviceRemovedExtendedDataSettings { self }
}
impl IUnknown for D3D12DeviceRemovedExtendedDataSettings {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DeviceRemovedExtendedDataSettings {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

