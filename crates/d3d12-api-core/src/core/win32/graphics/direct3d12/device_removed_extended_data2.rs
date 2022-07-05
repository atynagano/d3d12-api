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
pub struct D3D12DeviceRemovedExtendedData2(pub(crate) D3D12DeviceRemovedExtendedData1);

impl Guid for D3D12DeviceRemovedExtendedData2 {
	const IID: &'static GUID = &GUID::from_u128(0x67fc5816e4ca4915bf1842541272da54u128);
}

impl Clone for D3D12DeviceRemovedExtendedData2 {
	fn clone(&self) -> Self { D3D12DeviceRemovedExtendedData2(self.0.clone()) }
}

pub trait ID3D12DeviceRemovedExtendedData2: ID3D12DeviceRemovedExtendedData1 {
	fn as_device_removed_extended_data2(&self) -> &D3D12DeviceRemovedExtendedData2;
	fn into_device_removed_extended_data2(self) -> D3D12DeviceRemovedExtendedData2;

	fn GetPageFaultAllocationOutput2(&self, ) -> Result<D3D12DredPageFaultOutput2, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_output: MaybeUninit<D3D12DredPageFaultOutput2> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_output: *mut D3D12DredPageFaultOutput2, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, _out_output.as_mut_ptr(), );
			Ok(_out_output.assume_init())
		}
	}

	fn GetDeviceState(&self, ) -> D3D12DredDeviceState {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> D3D12DredDeviceState
				= transmute(vt[8]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl ID3D12DeviceRemovedExtendedData2 for D3D12DeviceRemovedExtendedData2 {
	fn as_device_removed_extended_data2(&self) -> &D3D12DeviceRemovedExtendedData2 { self }
	fn into_device_removed_extended_data2(self) -> D3D12DeviceRemovedExtendedData2 { self }
}

impl ID3D12DeviceRemovedExtendedData1 for D3D12DeviceRemovedExtendedData2 {
	fn as_device_removed_extended_data1(&self) -> &D3D12DeviceRemovedExtendedData1 { &self.0 }
	fn into_device_removed_extended_data1(self) -> D3D12DeviceRemovedExtendedData1 { self.0 }
}

impl ID3D12DeviceRemovedExtendedData for D3D12DeviceRemovedExtendedData2 {
	fn as_device_removed_extended_data(&self) -> &D3D12DeviceRemovedExtendedData { &self.0.0 }
	fn into_device_removed_extended_data(self) -> D3D12DeviceRemovedExtendedData { self.0.0 }
}

impl From<Unknown> for D3D12DeviceRemovedExtendedData2 {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceRemovedExtendedData1::from(v))
    }
}

impl IUnknown for D3D12DeviceRemovedExtendedData2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

