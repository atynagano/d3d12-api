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
pub struct D3D12Device5(pub(crate) D3D12Device4);

impl Guid for D3D12Device5 {
	const IID: &'static GUID = &GUID::from_u128(0x8b4f173b2fea4b808f584307191ab95du128);
}

impl Clone for D3D12Device5 {
	fn clone(&self) -> Self { D3D12Device5(self.0.clone()) }
}

pub trait ID3D12Device5: ID3D12Device4 {
	fn as_device5(&self) -> &D3D12Device5;
	fn into_device5(self) -> D3D12Device5;

	fn CreateLifetimeTracker<T: IUnknown>(&self, owner: &(impl ID3D12LifetimeOwner + ?Sized), ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_tracker: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, owner: VTable, riid: &GUID, _out_tracker: *mut c_void, ) -> HResult
				= transmute(vt[57]);
			let _ret_ = f(vt, owner.vtable(), T::IID, transmute(&mut _out_tracker), );
			if _ret_.is_ok() {
				if let Some(_out_tracker) = _out_tracker {
					return Ok(T::from(_out_tracker));
				}
			}
			Err(_ret_)
		}
	}

	fn RemoveDevice(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[58]);
			let _ret_ = f(vt, );
		}
	}

	fn CreateStateObject<T: IUnknown>(&self, desc: &D3D12StateObjectDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_state_object: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12StateObjectDesc, riid: &GUID, _out_state_object: *mut c_void, ) -> HResult
				= transmute(vt[62]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut _out_state_object), );
			if _ret_.is_ok() {
				if let Some(_out_state_object) = _out_state_object {
					return Ok(T::from(_out_state_object));
				}
			}
			Err(_ret_)
		}
	}

	fn GetRaytracingAccelerationStructurePrebuildInfo(&self, desc: &D3D12BuildRaytracingAccelerationStructureInputs, ) -> D3D12RaytracingAccelerationStructurePrebuildInfo {
		unsafe {
			let vt = self.as_param();
			let mut _out_info: MaybeUninit<D3D12RaytracingAccelerationStructurePrebuildInfo> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, desc: &D3D12BuildRaytracingAccelerationStructureInputs, _out_info: *mut D3D12RaytracingAccelerationStructurePrebuildInfo, ) -> ()
				= transmute(vt[63]);
			let _ret_ = f(vt, desc, _out_info.as_mut_ptr(), );
			_out_info.assume_init()
		}
	}

	fn CheckDriverMatchingIdentifier(&self, serialized_data_type: D3D12SerializedDataType, identifier_to_check: &D3D12SerializedDataDriverMatchingIdentifier, ) -> D3D12DriverMatchingIdentifierStatus {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, serialized_data_type: D3D12SerializedDataType, identifier_to_check: &D3D12SerializedDataDriverMatchingIdentifier, ) -> D3D12DriverMatchingIdentifierStatus
				= transmute(vt[64]);
			let _ret_ = f(vt, serialized_data_type, identifier_to_check, );
			_ret_
		}
	}
}

impl ID3D12Device5 for D3D12Device5 {
	fn as_device5(&self) -> &D3D12Device5 { self }
	fn into_device5(self) -> D3D12Device5 { self }
}

impl ID3D12Device4 for D3D12Device5 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0 }
	fn into_device4(self) -> D3D12Device4 { self.0 }
}

impl ID3D12Device3 for D3D12Device5 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.0 }
	fn into_device3(self) -> D3D12Device3 { self.0.0 }
}

impl ID3D12Device2 for D3D12Device5 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.0.0 }
	fn into_device2(self) -> D3D12Device2 { self.0.0.0 }
}

impl ID3D12Device1 for D3D12Device5 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.0.0.0 }
	fn into_device1(self) -> D3D12Device1 { self.0.0.0.0 }
}

impl ID3D12Device for D3D12Device5 {
	fn as_device(&self) -> &D3D12Device { &self.0.0.0.0.0 }
	fn into_device(self) -> D3D12Device { self.0.0.0.0.0 }
}

impl ID3D12Object for D3D12Device5 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0.0 }
}

impl From<Unknown> for D3D12Device5 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device4::from(v))
    }
}

impl IUnknown for D3D12Device5 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0 }
}

