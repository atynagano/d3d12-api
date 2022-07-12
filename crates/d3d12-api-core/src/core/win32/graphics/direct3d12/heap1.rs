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
pub struct D3D12Heap1(pub(crate) D3D12Heap);

impl Guid for D3D12Heap1 {
	const IID: &'static GUID = &GUID::from_u128(0x572f7389216849e39693d6df5871bf6du128);
}

impl Clone for D3D12Heap1 {
	fn clone(&self) -> Self { D3D12Heap1(self.0.clone()) }
}

pub trait ID3D12Heap1: ID3D12Heap {
	fn as_heap1(&self) -> &D3D12Heap1;
	fn into_heap1(self) -> D3D12Heap1;

	fn GetProtectedResourceSession<T: IUnknown>(&self, protected_session: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_protected_session: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, riid: &GUID, protected_session: *mut c_void, ) -> HResult
				= transmute(vt[9]);
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
				= transmute(vt[9]);
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

impl ID3D12Heap1 for D3D12Heap1 {
	fn as_heap1(&self) -> &D3D12Heap1 { self }
	fn into_heap1(self) -> D3D12Heap1 { self }
}

impl ID3D12Heap for D3D12Heap1 {
	fn as_heap(&self) -> &D3D12Heap { &self.0.as_heap() }
	fn into_heap(self) -> D3D12Heap { self.0.into_heap() }
}

impl ID3D12Pageable for D3D12Heap1 {
	fn as_pageable(&self) -> &D3D12Pageable { &self.0.as_pageable() }
	fn into_pageable(self) -> D3D12Pageable { self.0.into_pageable() }
}

impl ID3D12DeviceChild for D3D12Heap1 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12Heap1 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl From<Unknown> for D3D12Heap1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12Heap::from(v))
    }
}

impl IUnknown for D3D12Heap1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

