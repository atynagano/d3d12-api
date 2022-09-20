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
pub struct D3D12DeviceRemovedExtendedData1(pub(crate) D3D12DeviceRemovedExtendedData);

impl Deref for D3D12DeviceRemovedExtendedData1 {
	type Target = D3D12DeviceRemovedExtendedData;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DeviceRemovedExtendedData1 {
	const IID: &'static GUID = &GUID::from_u128(0x9727a022cf1d4dda9ebaeffa653fc506u128);
}

impl Com for D3D12DeviceRemovedExtendedData1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DeviceRemovedExtendedData1 {
	pub fn GetPageFaultAllocationOutput1(&self) -> Result<D3D12DredPageFaultOutput1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut output_out_: MaybeUninit<D3D12DredPageFaultOutput1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12DredPageFaultOutput1) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, output_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(output_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID3D12DeviceRemovedExtendedData1: ID3D12DeviceRemovedExtendedData {
	fn as_device_removed_extended_data1(&self) -> &D3D12DeviceRemovedExtendedData1;
	fn into_device_removed_extended_data1(self) -> D3D12DeviceRemovedExtendedData1;
}

impl ID3D12DeviceRemovedExtendedData1 for D3D12DeviceRemovedExtendedData1 {
	fn as_device_removed_extended_data1(&self) -> &D3D12DeviceRemovedExtendedData1 { self }
	fn into_device_removed_extended_data1(self) -> D3D12DeviceRemovedExtendedData1 { self }
}
impl ID3D12DeviceRemovedExtendedData for D3D12DeviceRemovedExtendedData1 {
	fn as_device_removed_extended_data(&self) -> &D3D12DeviceRemovedExtendedData { &self.0.as_device_removed_extended_data() }
	fn into_device_removed_extended_data(self) -> D3D12DeviceRemovedExtendedData { self.0.into_device_removed_extended_data() }
}

impl IUnknown for D3D12DeviceRemovedExtendedData1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DeviceRemovedExtendedData1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12DeviceRemovedExtendedData::from(v))
    }
}

