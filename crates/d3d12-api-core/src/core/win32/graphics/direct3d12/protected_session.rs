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
pub struct D3D12ProtectedSession(pub(crate) D3D12DeviceChild);

impl Guid for D3D12ProtectedSession {
	const IID: &'static GUID = &GUID::from_u128(0xa1533d180ac1408485b989a96116806bu128);
}

impl Clone for D3D12ProtectedSession {
	fn clone(&self) -> Self { D3D12ProtectedSession(self.0.clone()) }
}

pub trait ID3D12ProtectedSession: ID3D12DeviceChild {
	fn as_protected_session(&self) -> &D3D12ProtectedSession;
	fn into_protected_session(self) -> D3D12ProtectedSession;

	fn GetStatusFence<T: IUnknown>(&self, fence: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_fence: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, riid: &GUID, fence: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, T::IID, if fence.is_some() { Some(&mut out_fence) } else { None }, );
		if let (Some(fence), Some(out_fence)) = (fence, out_fence) { *fence = Some(T::from(out_fence)); }
		ret.ok()
	}

	fn GetSessionStatus(&self, ) -> (D3D12ProtectedSessionStatus) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> D3D12ProtectedSessionStatus
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl ID3D12ProtectedSession for D3D12ProtectedSession {
	fn as_protected_session(&self) -> &D3D12ProtectedSession { self }
	fn into_protected_session(self) -> D3D12ProtectedSession { self }
}

impl ID3D12DeviceChild for D3D12ProtectedSession {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0 }
}

impl ID3D12Object for D3D12ProtectedSession {
	fn as_object(&self) -> &D3D12Object { &self.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0 }
}

impl From<Unknown> for D3D12ProtectedSession {
    fn from(v: Unknown) -> Self {
        Self(D3D12DeviceChild::from(v))
    }
}

impl IUnknown for D3D12ProtectedSession {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

