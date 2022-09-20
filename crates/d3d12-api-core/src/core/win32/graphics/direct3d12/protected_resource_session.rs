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
pub struct D3D12ProtectedResourceSession(pub(crate) D3D12ProtectedSession);

impl Deref for D3D12ProtectedResourceSession {
	type Target = D3D12ProtectedSession;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12ProtectedResourceSession {
	const IID: &'static GUID = &GUID::from_u128(0x6cd696f4f28940cc80915a6c0a099c3du128);
}

impl Com for D3D12ProtectedResourceSession {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12ProtectedResourceSession {
	pub fn GetDesc(&self) -> D3D12ProtectedResourceSessionDesc {
		unsafe {
			let vt = self.as_param();
			let mut _out_: MaybeUninit<D3D12ProtectedResourceSessionDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D3D12ProtectedResourceSessionDesc) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, _out_.as_mut_ptr());
			_out_.assume_init()
		}
	}
}

pub trait ID3D12ProtectedResourceSession: ID3D12ProtectedSession {
	fn as_protected_resource_session(&self) -> &D3D12ProtectedResourceSession;
	fn into_protected_resource_session(self) -> D3D12ProtectedResourceSession;
}

impl ID3D12ProtectedResourceSession for D3D12ProtectedResourceSession {
	fn as_protected_resource_session(&self) -> &D3D12ProtectedResourceSession { self }
	fn into_protected_resource_session(self) -> D3D12ProtectedResourceSession { self }
}
impl ID3D12ProtectedSession for D3D12ProtectedResourceSession {
	fn as_protected_session(&self) -> &D3D12ProtectedSession { &self.0.as_protected_session() }
	fn into_protected_session(self) -> D3D12ProtectedSession { self.0.into_protected_session() }
}

impl ID3D12DeviceChild for D3D12ProtectedResourceSession {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12ProtectedResourceSession {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12ProtectedResourceSession {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12ProtectedResourceSession {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12ProtectedSession::from(v))
    }
}

