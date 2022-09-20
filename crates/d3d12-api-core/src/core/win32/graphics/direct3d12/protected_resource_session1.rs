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

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12ProtectedResourceSession1(pub(crate) D3D12ProtectedResourceSession);

impl Deref for D3D12ProtectedResourceSession1 {
	type Target = D3D12ProtectedResourceSession;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12ProtectedResourceSession1 {
	const IID: &'static GUID = &GUID::from_u128(0xd6f12dd676fb406e89614296eefc0409u128);
}

impl Com for D3D12ProtectedResourceSession1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12ProtectedResourceSession1 {
	pub fn GetDesc1(&self) -> D3D12ProtectedResourceSessionDesc1 {
		unsafe {
			let vt = self.as_param();
			let mut _out_: MaybeUninit<D3D12ProtectedResourceSessionDesc1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12ProtectedResourceSessionDesc1) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, _out_.as_mut_ptr());
			_out_.assume_init()
		}
	}
}

pub trait ID3D12ProtectedResourceSession1: ID3D12ProtectedResourceSession {
	fn as_protected_resource_session1(&self) -> &D3D12ProtectedResourceSession1;
	fn into_protected_resource_session1(self) -> D3D12ProtectedResourceSession1;
}

impl ID3D12ProtectedResourceSession1 for D3D12ProtectedResourceSession1 {
	fn as_protected_resource_session1(&self) -> &D3D12ProtectedResourceSession1 { self }
	fn into_protected_resource_session1(self) -> D3D12ProtectedResourceSession1 { self }
}
impl ID3D12ProtectedResourceSession for D3D12ProtectedResourceSession1 {
	fn as_protected_resource_session(&self) -> &D3D12ProtectedResourceSession { &self.0.as_protected_resource_session() }
	fn into_protected_resource_session(self) -> D3D12ProtectedResourceSession { self.0.into_protected_resource_session() }
}

impl ID3D12ProtectedSession for D3D12ProtectedResourceSession1 {
	fn as_protected_session(&self) -> &D3D12ProtectedSession { &self.0.as_protected_session() }
	fn into_protected_session(self) -> D3D12ProtectedSession { self.0.into_protected_session() }
}

impl ID3D12DeviceChild for D3D12ProtectedResourceSession1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12ProtectedResourceSession1 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12ProtectedResourceSession1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12ProtectedResourceSession1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12ProtectedResourceSession::from(v))
    }
}

