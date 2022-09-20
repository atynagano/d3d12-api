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
pub struct D3D12DeviceRemovedExtendedData(pub(crate) Unknown);

impl Deref for D3D12DeviceRemovedExtendedData {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DeviceRemovedExtendedData {
	const IID: &'static GUID = &GUID::from_u128(0x98931d335ae84791aa3c1a73a2934e71u128);
}

impl Com for D3D12DeviceRemovedExtendedData {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DeviceRemovedExtendedData {
	pub fn GetPageFaultAllocationOutput(&self) -> Result<D3D12DredPageFaultOutput, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut output_out_: MaybeUninit<D3D12DredPageFaultOutput> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12DredPageFaultOutput) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, output_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(output_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID3D12DeviceRemovedExtendedData: IUnknown {
	fn as_device_removed_extended_data(&self) -> &D3D12DeviceRemovedExtendedData;
	fn into_device_removed_extended_data(self) -> D3D12DeviceRemovedExtendedData;
}

impl ID3D12DeviceRemovedExtendedData for D3D12DeviceRemovedExtendedData {
	fn as_device_removed_extended_data(&self) -> &D3D12DeviceRemovedExtendedData { self }
	fn into_device_removed_extended_data(self) -> D3D12DeviceRemovedExtendedData { self }
}
impl IUnknown for D3D12DeviceRemovedExtendedData {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DeviceRemovedExtendedData {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

