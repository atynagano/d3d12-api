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

	fn CreateLifetimeTracker<T: IUnknown>(&self, owner: &(impl ID3D12LifetimeOwner + ?Sized), ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _tracker: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, owner: VTable, riid: &GUID, _tracker: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[57]) };
		let ret = f(vt, owner.vtable(), T::IID, &mut _tracker, );
		if ret.is_ok() {
			if let (Some(_tracker)) = (_tracker) {
				return Ok((T::from(_tracker)));
			}
		}
		Err(ret)
	}

	fn RemoveDevice(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[58]) };
		let ret = f(vt, );
	}

	fn CreateMetaCommand<T: IUnknown>(&self, command_id: &GUID, node_mask: u32, creation_parameters_data: Option<&[u8]>, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _meta_command: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, command_id: &GUID, node_mask: u32, creation_parameters_data: *const u8, creation_parameters_data_size_in_bytes: usize, riid: &GUID, _meta_command: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[61]) };
		let ret = f(vt, command_id, node_mask, creation_parameters_data.as_ptr_or_null(), creation_parameters_data.len() as usize, T::IID, &mut _meta_command, );
		if ret.is_ok() {
			if let (Some(_meta_command)) = (_meta_command) {
				return Ok((T::from(_meta_command)));
			}
		}
		Err(ret)
	}

	fn CreateStateObject<T: IUnknown>(&self, desc: &D3D12StateObjectDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _state_object: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12StateObjectDesc, riid: &GUID, _state_object: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[62]) };
		let ret = f(vt, desc, T::IID, &mut _state_object, );
		if ret.is_ok() {
			if let (Some(_state_object)) = (_state_object) {
				return Ok((T::from(_state_object)));
			}
		}
		Err(ret)
	}

	fn GetRaytracingAccelerationStructurePrebuildInfo(&self, desc: &D3D12BuildRaytracingAccelerationStructureInputs, ) -> (D3D12RaytracingAccelerationStructurePrebuildInfo) {
		let vt = self.as_param();
		let mut _info: D3D12RaytracingAccelerationStructurePrebuildInfo = D3D12RaytracingAccelerationStructurePrebuildInfo::zeroed();
		let f: extern "system" fn(Param<Self>, desc: &D3D12BuildRaytracingAccelerationStructureInputs, _info: &mut D3D12RaytracingAccelerationStructurePrebuildInfo, ) -> ()
			= unsafe { transmute(vt[63]) };
		let ret = f(vt, desc, &mut _info, );
		return (_info);
	}

	fn CheckDriverMatchingIdentifier(&self, serialized_data_type: D3D12SerializedDataType, identifier_to_check: &D3D12SerializedDataDriverMatchingIdentifier, ) -> (D3D12DriverMatchingIdentifierStatus) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, serialized_data_type: D3D12SerializedDataType, identifier_to_check: &D3D12SerializedDataDriverMatchingIdentifier, ) -> D3D12DriverMatchingIdentifierStatus
			= unsafe { transmute(vt[64]) };
		let ret = f(vt, serialized_data_type, identifier_to_check, );
		return (ret);
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

