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

#[repr(C)]
pub struct D3D12Resource1(pub(crate) D3D12Resource);

impl Guid for D3D12Resource1 {
	const IID: &'static GUID = &GUID::from_u128(0x9d5e227a4430416188b33eca6bb16e19u128);
}

impl Clone for D3D12Resource1 {
	fn clone(&self) -> Self { D3D12Resource1(self.0.clone()) }
}

pub trait ID3D12Resource1: ID3D12Resource {
	fn as_resource1(&self) -> &D3D12Resource1;
	fn into_resource1(self) -> D3D12Resource1;

	fn GetProtectedResourceSession<T: IUnknown>(&self, protected_session: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_protected_session: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, riid: &GUID, protected_session: *mut c_void, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, T::IID, transmute(if protected_session.is_some() { Some(&mut _out_protected_session) } else { None }), );
			if let Some(_out_protected_session) = _out_protected_session { *protected_session.unwrap_unchecked() = Some(T::from(_out_protected_session)); }
			_ret_.ok()
		}
	}

	fn get_protected_resource_session<T: IUnknown>(&self, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_protected_session: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, riid: &GUID, _out_protected_session: *mut c_void, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, T::IID, transmute(&mut _out_protected_session), );
			if _ret_.is_ok() {
				if let Some(_out_protected_session) = _out_protected_session {
					return Ok(T::from(_out_protected_session));
				}
			}
			Err(_ret_)
		}
	}
}

impl ID3D12Resource1 for D3D12Resource1 {
	fn as_resource1(&self) -> &D3D12Resource1 { self }
	fn into_resource1(self) -> D3D12Resource1 { self }
}

impl ID3D12Resource for D3D12Resource1 {
	fn as_resource(&self) -> &D3D12Resource { &self.0.as_resource() }
	fn into_resource(self) -> D3D12Resource { self.0.into_resource() }
}

impl ID3D12Pageable for D3D12Resource1 {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12Resource1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12Resource1 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl From<Unknown> for D3D12Resource1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Resource::from(v))
    }
}

impl IUnknown for D3D12Resource1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

