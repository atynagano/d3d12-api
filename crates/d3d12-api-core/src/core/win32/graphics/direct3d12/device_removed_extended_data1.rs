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
pub struct D3D12DeviceRemovedExtendedData1(pub(crate) D3D12DeviceRemovedExtendedData);

impl Guid for D3D12DeviceRemovedExtendedData1 {
	const IID: &'static GUID = &GUID::from_u128(0x9727a022cf1d4dda9ebaeffa653fc506u128);
}

impl Clone for D3D12DeviceRemovedExtendedData1 {
	fn clone(&self) -> Self { D3D12DeviceRemovedExtendedData1(self.0.clone()) }
}

pub trait ID3D12DeviceRemovedExtendedData1: ID3D12DeviceRemovedExtendedData {
	fn as_device_removed_extended_data1(&self) -> &D3D12DeviceRemovedExtendedData1;
	fn into_device_removed_extended_data1(self) -> D3D12DeviceRemovedExtendedData1;

	fn GetPageFaultAllocationOutput1(&self, ) -> Result<(D3D12DredPageFaultOutput1), HResult> {
		let vt = self.as_param();
		let mut _output: D3D12DredPageFaultOutput1 = D3D12DredPageFaultOutput1::zeroed();
		let f: extern "system" fn(Param<Self>, _output: &mut D3D12DredPageFaultOutput1, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, &mut _output, );
		if ret.is_ok() {
			return Ok((_output));
		}
		Err(ret)
	}
}

impl ID3D12DeviceRemovedExtendedData1 for D3D12DeviceRemovedExtendedData1 {
	fn as_device_removed_extended_data1(&self) -> &D3D12DeviceRemovedExtendedData1 { self }
	fn into_device_removed_extended_data1(self) -> D3D12DeviceRemovedExtendedData1 { self }
}

impl ID3D12DeviceRemovedExtendedData for D3D12DeviceRemovedExtendedData1 {
	fn as_device_removed_extended_data(&self) -> &D3D12DeviceRemovedExtendedData { &self.0 }
	fn into_device_removed_extended_data(self) -> D3D12DeviceRemovedExtendedData { self.0 }
}

impl From<Unknown> for D3D12DeviceRemovedExtendedData1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceRemovedExtendedData::from(v))
    }
}

impl IUnknown for D3D12DeviceRemovedExtendedData1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

