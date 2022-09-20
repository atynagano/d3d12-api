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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Heap1(pub(crate) D3D12Heap);

impl Deref for D3D12Heap1 {
	type Target = D3D12Heap;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Heap1 {
	const IID: &'static GUID = &GUID::from_u128(0x572f7389216849e39693d6df5871bf6du128);
}

impl Com for D3D12Heap1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Heap1 {
	pub fn GetProtectedResourceSession<T: IUnknown + From<UnknownWrapper>>(&self, protected_session: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut protected_session_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, T::IID, transmute(if protected_session_out_.is_some() { Some(&mut protected_session_out_) } else { None }));
			if let Some(protected_session_out_) = protected_session_out_ { *protected_session.unwrap_unchecked() = Some(T::from(UnknownWrapper(protected_session_out_))); }
			_ret_.ok()
		}
	}

	pub fn get_protected_resource_session<T: IUnknown + From<UnknownWrapper>>(&self) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut protected_session_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, T::IID, transmute(&mut protected_session_out_));
			if _ret_.is_ok() { if let Some(protected_session_out_) = protected_session_out_ { return Ok(T::from(UnknownWrapper(protected_session_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait ID3D12Heap1: ID3D12Heap {
	fn as_heap1(&self) -> &D3D12Heap1;
	fn into_heap1(self) -> D3D12Heap1;
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

impl IUnknown for D3D12Heap1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Heap1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Heap::from(v))
    }
}

