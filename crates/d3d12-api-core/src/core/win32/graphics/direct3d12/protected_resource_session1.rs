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

use crate::core::win32::graphics::direct3d12::*;
#[repr(C)]
pub struct D3D12ProtectedResourceSession1(pub(crate) D3D12ProtectedResourceSession);

impl Guid for D3D12ProtectedResourceSession1 {
	const IID: &'static GUID = &GUID::from_u128(0xd6f12dd676fb406e89614296eefc0409u128);
}

impl Clone for D3D12ProtectedResourceSession1 {
	fn clone(&self) -> Self { D3D12ProtectedResourceSession1(self.0.clone()) }
}

pub trait ID3D12ProtectedResourceSession1: ID3D12ProtectedResourceSession {
	fn as_protected_resource_session1(&self) -> &D3D12ProtectedResourceSession1;
	fn into_protected_resource_session1(self) -> D3D12ProtectedResourceSession1;

	fn GetDesc1(&self, ) -> (D3D12ProtectedResourceSessionDesc1) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> D3D12ProtectedResourceSessionDesc1
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl ID3D12ProtectedResourceSession1 for D3D12ProtectedResourceSession1 {
	fn as_protected_resource_session1(&self) -> &D3D12ProtectedResourceSession1 { self }
	fn into_protected_resource_session1(self) -> D3D12ProtectedResourceSession1 { self }
}

impl ID3D12ProtectedResourceSession for D3D12ProtectedResourceSession1 {
	fn as_protected_resource_session(&self) -> &D3D12ProtectedResourceSession { &self.0 }
	fn into_protected_resource_session(self) -> D3D12ProtectedResourceSession { self.0 }
}

impl ID3D12ProtectedSession for D3D12ProtectedResourceSession1 {
	fn as_protected_session(&self) -> &D3D12ProtectedSession { &self.0.0 }
	fn into_protected_session(self) -> D3D12ProtectedSession { self.0.0 }
}

impl ID3D12DeviceChild for D3D12ProtectedResourceSession1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0.0 }
}

impl ID3D12Object for D3D12ProtectedResourceSession1 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0 }
}

impl From<Unknown> for D3D12ProtectedResourceSession1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12ProtectedResourceSession::from(v))
    }
}

impl IUnknown for D3D12ProtectedResourceSession1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

