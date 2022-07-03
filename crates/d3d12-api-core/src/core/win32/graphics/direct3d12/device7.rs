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
pub struct D3D12Device7(pub(crate) D3D12Device6);

impl Guid for D3D12Device7 {
	const IID: &'static GUID = &GUID::from_u128(0x5c014b5368a14b9b8bd1dd6046b9358bu128);
}

impl Clone for D3D12Device7 {
	fn clone(&self) -> Self { D3D12Device7(self.0.clone()) }
}

pub trait ID3D12Device7: ID3D12Device6 {
	fn as_device7(&self) -> &D3D12Device7;
	fn into_device7(self) -> D3D12Device7;

	fn AddToStateObject<T: IUnknown>(&self, addition: &D3D12StateObjectDesc, state_object_to_grow_from: &(impl ID3D12StateObject + ?Sized), ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _new_state_object: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, addition: &D3D12StateObjectDesc, state_object_to_grow_from: VTable, riid: &GUID, _new_state_object: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[66]) };
		let ret = f(vt, addition, state_object_to_grow_from.vtable(), T::IID, &mut _new_state_object, );
		if ret.is_ok() {
			if let (Some(_new_state_object)) = (_new_state_object) {
				return Ok((T::from(_new_state_object)));
			}
		}
		Err(ret)
	}

	fn CreateProtectedResourceSession1<T: IUnknown>(&self, desc: &D3D12ProtectedResourceSessionDesc1, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _session: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12ProtectedResourceSessionDesc1, riid: &GUID, _session: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[67]) };
		let ret = f(vt, desc, T::IID, &mut _session, );
		if ret.is_ok() {
			if let (Some(_session)) = (_session) {
				return Ok((T::from(_session)));
			}
		}
		Err(ret)
	}
}

impl ID3D12Device7 for D3D12Device7 {
	fn as_device7(&self) -> &D3D12Device7 { self }
	fn into_device7(self) -> D3D12Device7 { self }
}

impl ID3D12Device6 for D3D12Device7 {
	fn as_device6(&self) -> &D3D12Device6 { &self.0 }
	fn into_device6(self) -> D3D12Device6 { self.0 }
}

impl ID3D12Device5 for D3D12Device7 {
	fn as_device5(&self) -> &D3D12Device5 { &self.0.0 }
	fn into_device5(self) -> D3D12Device5 { self.0.0 }
}

impl ID3D12Device4 for D3D12Device7 {
	fn as_device4(&self) -> &D3D12Device4 { &self.0.0.0 }
	fn into_device4(self) -> D3D12Device4 { self.0.0.0 }
}

impl ID3D12Device3 for D3D12Device7 {
	fn as_device3(&self) -> &D3D12Device3 { &self.0.0.0.0 }
	fn into_device3(self) -> D3D12Device3 { self.0.0.0.0 }
}

impl ID3D12Device2 for D3D12Device7 {
	fn as_device2(&self) -> &D3D12Device2 { &self.0.0.0.0.0 }
	fn into_device2(self) -> D3D12Device2 { self.0.0.0.0.0 }
}

impl ID3D12Device1 for D3D12Device7 {
	fn as_device1(&self) -> &D3D12Device1 { &self.0.0.0.0.0.0 }
	fn into_device1(self) -> D3D12Device1 { self.0.0.0.0.0.0 }
}

impl ID3D12Device for D3D12Device7 {
	fn as_device(&self) -> &D3D12Device { &self.0.0.0.0.0.0.0 }
	fn into_device(self) -> D3D12Device { self.0.0.0.0.0.0.0 }
}

impl ID3D12Object for D3D12Device7 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0.0.0.0 }
}

impl From<Unknown> for D3D12Device7 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Device6::from(v))
    }
}

impl IUnknown for D3D12Device7 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0.0 }
}

