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
pub struct D3D12DeviceRemovedExtendedData(pub(crate) Unknown);

impl Guid for D3D12DeviceRemovedExtendedData {
	const IID: &'static GUID = &GUID::from_u128(0x98931d335ae84791aa3c1a73a2934e71u128);
}

impl Clone for D3D12DeviceRemovedExtendedData {
	fn clone(&self) -> Self { D3D12DeviceRemovedExtendedData(self.0.clone()) }
}

pub trait ID3D12DeviceRemovedExtendedData: IUnknown {
	fn as_device_removed_extended_data(&self) -> &D3D12DeviceRemovedExtendedData;
	fn into_device_removed_extended_data(self) -> D3D12DeviceRemovedExtendedData;

	fn GetPageFaultAllocationOutput(&self, ) -> Result<D3D12DredPageFaultOutput, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_output: MaybeUninit<D3D12DredPageFaultOutput> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_output: *mut D3D12DredPageFaultOutput, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, _out_output.as_mut_ptr(), );
			Ok(_out_output.assume_init())
		}
	}
}

impl ID3D12DeviceRemovedExtendedData for D3D12DeviceRemovedExtendedData {
	fn as_device_removed_extended_data(&self) -> &D3D12DeviceRemovedExtendedData { self }
	fn into_device_removed_extended_data(self) -> D3D12DeviceRemovedExtendedData { self }
}

impl From<Unknown> for D3D12DeviceRemovedExtendedData {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12DeviceRemovedExtendedData {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

