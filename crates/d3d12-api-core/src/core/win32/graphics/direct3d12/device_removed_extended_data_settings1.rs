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
pub struct D3D12DeviceRemovedExtendedDataSettings1(pub(crate) D3D12DeviceRemovedExtendedDataSettings);

impl Deref for D3D12DeviceRemovedExtendedDataSettings1 {
	type Target = D3D12DeviceRemovedExtendedDataSettings;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DeviceRemovedExtendedDataSettings1 {
	const IID: &'static GUID = &GUID::from_u128(0xdbd5ae5133174f0aadf91d7cedcaae0bu128);
}

impl Com for D3D12DeviceRemovedExtendedDataSettings1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DeviceRemovedExtendedDataSettings1 {
	pub fn SetBreadcrumbContextEnablement(&self, enablement: D3D12DredEnablement) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12DredEnablement) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, enablement);
		}
	}
}

pub trait ID3D12DeviceRemovedExtendedDataSettings1: ID3D12DeviceRemovedExtendedDataSettings {
	fn as_device_removed_extended_data_settings1(&self) -> &D3D12DeviceRemovedExtendedDataSettings1;
	fn into_device_removed_extended_data_settings1(self) -> D3D12DeviceRemovedExtendedDataSettings1;
}

impl ID3D12DeviceRemovedExtendedDataSettings1 for D3D12DeviceRemovedExtendedDataSettings1 {
	fn as_device_removed_extended_data_settings1(&self) -> &D3D12DeviceRemovedExtendedDataSettings1 { self }
	fn into_device_removed_extended_data_settings1(self) -> D3D12DeviceRemovedExtendedDataSettings1 { self }
}
impl ID3D12DeviceRemovedExtendedDataSettings for D3D12DeviceRemovedExtendedDataSettings1 {
	fn as_device_removed_extended_data_settings(&self) -> &D3D12DeviceRemovedExtendedDataSettings { &self.0.as_device_removed_extended_data_settings() }
	fn into_device_removed_extended_data_settings(self) -> D3D12DeviceRemovedExtendedDataSettings { self.0.into_device_removed_extended_data_settings() }
}

impl IUnknown for D3D12DeviceRemovedExtendedDataSettings1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DeviceRemovedExtendedDataSettings1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12DeviceRemovedExtendedDataSettings::from(v))
    }
}

