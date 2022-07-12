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

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
pub struct D3D12DeviceRemovedExtendedDataSettings(pub(crate) Unknown);

impl Guid for D3D12DeviceRemovedExtendedDataSettings {
	const IID: &'static GUID = &GUID::from_u128(0x82bc481c6b9b4030aedb7ee3d1df1e63u128);
}

impl Clone for D3D12DeviceRemovedExtendedDataSettings {
	fn clone(&self) -> Self { D3D12DeviceRemovedExtendedDataSettings(self.0.clone()) }
}

pub trait ID3D12DeviceRemovedExtendedDataSettings: IUnknown {
	fn as_device_removed_extended_data_settings(&self) -> &D3D12DeviceRemovedExtendedDataSettings;
	fn into_device_removed_extended_data_settings(self) -> D3D12DeviceRemovedExtendedDataSettings;

	fn SetAutoBreadcrumbsEnablement(&self, enablement: D3D12DredEnablement, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, enablement: D3D12DredEnablement, ) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, enablement, );
		}
	}

	fn SetPageFaultEnablement(&self, enablement: D3D12DredEnablement, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, enablement: D3D12DredEnablement, ) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, enablement, );
		}
	}

	fn SetWatsonDumpEnablement(&self, enablement: D3D12DredEnablement, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, enablement: D3D12DredEnablement, ) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, enablement, );
		}
	}
}

impl ID3D12DeviceRemovedExtendedDataSettings for D3D12DeviceRemovedExtendedDataSettings {
	fn as_device_removed_extended_data_settings(&self) -> &D3D12DeviceRemovedExtendedDataSettings { self }
	fn into_device_removed_extended_data_settings(self) -> D3D12DeviceRemovedExtendedDataSettings { self }
}

impl From<Unknown> for D3D12DeviceRemovedExtendedDataSettings {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12DeviceRemovedExtendedDataSettings {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

