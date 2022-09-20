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
pub struct D3D12ProtectedSession(pub(crate) D3D12DeviceChild);

impl Deref for D3D12ProtectedSession {
	type Target = D3D12DeviceChild;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12ProtectedSession {
	const IID: &'static GUID = &GUID::from_u128(0xa1533d180ac1408485b989a96116806bu128);
}

impl Com for D3D12ProtectedSession {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12ProtectedSession {
	pub fn GetStatusFence<T: IUnknown + From<UnknownWrapper>>(&self, fence: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut fence_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, T::IID, transmute(if fence_out_.is_some() { Some(&mut fence_out_) } else { None }));
			if let Some(fence_out_) = fence_out_ { *fence.unwrap_unchecked() = Some(T::from(UnknownWrapper(fence_out_))); }
			_ret_.ok()
		}
	}

	pub fn get_status_fence<T: IUnknown + From<UnknownWrapper>>(&self) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut fence_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, T::IID, transmute(&mut fence_out_));
			if _ret_.is_ok() { if let Some(fence_out_) = fence_out_ { return Ok(T::from(UnknownWrapper(fence_out_))); } }
			Err(_ret_)
		}
	}

	pub fn GetSessionStatus(&self) -> D3D12ProtectedSessionStatus {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D3D12ProtectedSessionStatus
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID3D12ProtectedSession: ID3D12DeviceChild {
	fn as_protected_session(&self) -> &D3D12ProtectedSession;
	fn into_protected_session(self) -> D3D12ProtectedSession;
}

impl ID3D12ProtectedSession for D3D12ProtectedSession {
	fn as_protected_session(&self) -> &D3D12ProtectedSession { self }
	fn into_protected_session(self) -> D3D12ProtectedSession { self }
}
impl ID3D12DeviceChild for D3D12ProtectedSession {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12ProtectedSession {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12ProtectedSession {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12ProtectedSession {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

