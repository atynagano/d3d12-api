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
pub struct D3D12DeviceRemovedExtendedData2(pub(crate) D3D12DeviceRemovedExtendedData1);

impl Deref for D3D12DeviceRemovedExtendedData2 {
	type Target = D3D12DeviceRemovedExtendedData1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DeviceRemovedExtendedData2 {
	const IID: &'static GUID = &GUID::from_u128(0x67fc5816e4ca4915bf1842541272da54u128);
}

impl Com for D3D12DeviceRemovedExtendedData2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DeviceRemovedExtendedData2 {
	pub fn GetPageFaultAllocationOutput2(&self) -> Result<D3D12DredPageFaultOutput2, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut output_out_: MaybeUninit<D3D12DredPageFaultOutput2> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12DredPageFaultOutput2) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, output_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(output_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetDeviceState(&self) -> D3D12DredDeviceState {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D3D12DredDeviceState
				= transmute(vt[8]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID3D12DeviceRemovedExtendedData2: ID3D12DeviceRemovedExtendedData1 {
	fn as_device_removed_extended_data2(&self) -> &D3D12DeviceRemovedExtendedData2;
	fn into_device_removed_extended_data2(self) -> D3D12DeviceRemovedExtendedData2;
}

impl ID3D12DeviceRemovedExtendedData2 for D3D12DeviceRemovedExtendedData2 {
	fn as_device_removed_extended_data2(&self) -> &D3D12DeviceRemovedExtendedData2 { self }
	fn into_device_removed_extended_data2(self) -> D3D12DeviceRemovedExtendedData2 { self }
}
impl ID3D12DeviceRemovedExtendedData1 for D3D12DeviceRemovedExtendedData2 {
	fn as_device_removed_extended_data1(&self) -> &D3D12DeviceRemovedExtendedData1 { &self.0.as_device_removed_extended_data1() }
	fn into_device_removed_extended_data1(self) -> D3D12DeviceRemovedExtendedData1 { self.0.into_device_removed_extended_data1() }
}

impl ID3D12DeviceRemovedExtendedData for D3D12DeviceRemovedExtendedData2 {
	fn as_device_removed_extended_data(&self) -> &D3D12DeviceRemovedExtendedData { &self.0.as_device_removed_extended_data() }
	fn into_device_removed_extended_data(self) -> D3D12DeviceRemovedExtendedData { self.0.into_device_removed_extended_data() }
}

impl IUnknown for D3D12DeviceRemovedExtendedData2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DeviceRemovedExtendedData2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12DeviceRemovedExtendedData1::from(v))
    }
}

