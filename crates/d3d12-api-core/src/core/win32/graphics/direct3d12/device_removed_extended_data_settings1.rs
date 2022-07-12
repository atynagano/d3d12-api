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
pub struct D3D12DeviceRemovedExtendedDataSettings1(pub(crate) D3D12DeviceRemovedExtendedDataSettings);

impl Guid for D3D12DeviceRemovedExtendedDataSettings1 {
	const IID: &'static GUID = &GUID::from_u128(0xdbd5ae5133174f0aadf91d7cedcaae0bu128);
}

impl Clone for D3D12DeviceRemovedExtendedDataSettings1 {
	fn clone(&self) -> Self { D3D12DeviceRemovedExtendedDataSettings1(self.0.clone()) }
}

pub trait ID3D12DeviceRemovedExtendedDataSettings1: ID3D12DeviceRemovedExtendedDataSettings {
	fn as_device_removed_extended_data_settings1(&self) -> &D3D12DeviceRemovedExtendedDataSettings1;
	fn into_device_removed_extended_data_settings1(self) -> D3D12DeviceRemovedExtendedDataSettings1;

	fn SetBreadcrumbContextEnablement(&self, enablement: D3D12DredEnablement, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, enablement: D3D12DredEnablement, ) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, enablement, );
		}
	}
}

impl ID3D12DeviceRemovedExtendedDataSettings1 for D3D12DeviceRemovedExtendedDataSettings1 {
	fn as_device_removed_extended_data_settings1(&self) -> &D3D12DeviceRemovedExtendedDataSettings1 { self }
	fn into_device_removed_extended_data_settings1(self) -> D3D12DeviceRemovedExtendedDataSettings1 { self }
}

impl ID3D12DeviceRemovedExtendedDataSettings for D3D12DeviceRemovedExtendedDataSettings1 {
	fn as_device_removed_extended_data_settings(&self) -> &D3D12DeviceRemovedExtendedDataSettings { &self.0.as_device_removed_extended_data_settings() }
	fn into_device_removed_extended_data_settings(self) -> D3D12DeviceRemovedExtendedDataSettings { self.0.into_device_removed_extended_data_settings() }
}

impl From<Unknown> for D3D12DeviceRemovedExtendedDataSettings1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceRemovedExtendedDataSettings::from(v))
    }
}

impl IUnknown for D3D12DeviceRemovedExtendedDataSettings1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

