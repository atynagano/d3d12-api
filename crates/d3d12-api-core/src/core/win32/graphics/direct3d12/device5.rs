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
pub struct D3D12Device5(pub(crate) D3D12Device4);

impl Deref for D3D12Device5 {
	type Target = D3D12Device4;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Device5 {
	const IID: &'static GUID = &GUID::from_u128(0x8b4f173b2fea4b808f584307191ab95du128);
}

impl Com for D3D12Device5 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Device5 {
	pub fn CreateLifetimeTracker<T: IUnknown + From<UnknownWrapper>>(&self, owner: &D3D12LifetimeOwner) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut tracker_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, VTable, &GUID, *mut c_void) -> HResult
				= transmute(vt[57]);
			let _ret_ = f(vt, owner.vtable(), T::IID, transmute(&mut tracker_out_));
			if _ret_.is_ok() { if let Some(tracker_out_) = tracker_out_ { return Ok(T::from(UnknownWrapper(tracker_out_))); } }
			Err(_ret_)
		}
	}

	pub fn RemoveDevice(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[58]);
			let _ret_ = f(vt);
		}
	}

	pub fn CreateMetaCommand<T: IUnknown + From<UnknownWrapper>>(&self, command_id: &GUID, node_mask: u32, creation_parameters_data: Option<&[u8]>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let (creation_parameters_data_ptr_, creation_parameters_data_len_) = creation_parameters_data.deconstruct();
			let mut meta_command_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, u32, *const u8, usize, &GUID, *mut c_void) -> HResult
				= transmute(vt[61]);
			let _ret_ = f(vt, command_id, node_mask, creation_parameters_data_ptr_, creation_parameters_data_len_ as usize, T::IID, transmute(&mut meta_command_out_));
			if _ret_.is_ok() { if let Some(meta_command_out_) = meta_command_out_ { return Ok(T::from(UnknownWrapper(meta_command_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateStateObject<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12StateObjectDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut state_object_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12StateObjectDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[62]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut state_object_out_));
			if _ret_.is_ok() { if let Some(state_object_out_) = state_object_out_ { return Ok(T::from(UnknownWrapper(state_object_out_))); } }
			Err(_ret_)
		}
	}

	pub fn GetRaytracingAccelerationStructurePrebuildInfo(&self, desc: &D3D12BuildRaytracingAccelerationStructureInputs) -> D3D12RaytracingAccelerationStructurePrebuildInfo {
		unsafe {
			let vt = self.as_param();
			let mut info_out_: MaybeUninit<D3D12RaytracingAccelerationStructurePrebuildInfo> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, &D3D12BuildRaytracingAccelerationStructureInputs, *mut D3D12RaytracingAccelerationStructurePrebuildInfo) -> ()
				= transmute(vt[63]);
			let _ret_ = f(vt, desc, info_out_.as_mut_ptr());
			info_out_.assume_init()
		}
	}

	pub fn CheckDriverMatchingIdentifier(&self, serialized_data_type: D3D12SerializedDataType, identifier_to_check: &D3D12SerializedDataDriverMatchingIdentifier) -> D3D12DriverMatchingIdentifierStatus {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12SerializedDataType, &D3D12SerializedDataDriverMatchingIdentifier) -> D3D12DriverMatchingIdentifierStatus
				= transmute(vt[64]);
			let _ret_ = f(vt, serialized_data_type, identifier_to_check);
			_ret_
		}
	}
}

pub trait ID3D12Device5: ID3D12Device4 {
	fn as_device5(&self) -> &D3D12Device5;
	fn into_device5(self) -> D3D12Device5;
}

impl ID3D12Device5 for D3D12Device5 {
	fn as_device5(&self) -> &D3D12Device5 { self }
	fn into_device5(self) -> D3D12Device5 { self }
}
impl ID3D12Device4 for D3D12Device5 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0.as_device4() }
	fn into_device4(self) -> D3D12Device4 { self.0.into_device4() }
}

impl ID3D12Device3 for D3D12Device5 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.as_device3() }
	fn into_device3(self) -> D3D12Device3 { self.0.into_device3() }
}

impl ID3D12Device2 for D3D12Device5 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.as_device2() }
	fn into_device2(self) -> D3D12Device2 { self.0.into_device2() }
}

impl ID3D12Device1 for D3D12Device5 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D3D12Device1 { self.0.into_device1() }
}

impl ID3D12Device for D3D12Device5 {
	fn as_device(&self) -> &D3D12Device { &self.0.as_device() }
	fn into_device(self) -> D3D12Device { self.0.into_device() }
}

impl ID3D12Object for D3D12Device5 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Device5 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Device5 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Device4::from(v))
    }
}

