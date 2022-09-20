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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Object(pub(crate) Unknown);

impl Deref for D3D12Object {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Object {
	const IID: &'static GUID = &GUID::from_u128(0xc4fec28f79664e959f94f431cb56c3b8u128);
}

impl Com for D3D12Object {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Object {
	pub fn SetPrivateData(&self, guid: &GUID, data: Option<&[u8]>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, &GUID, u32, *const u8) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, guid, data_len_ as u32, data_ptr_);
			_ret_.ok()
		}
	}

	pub fn SetPrivateDataInterface(&self, guid: &GUID, data: Option<&Unknown>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &GUID, *const c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, guid, option_to_vtable(data));
			_ret_.ok()
		}
	}

	pub fn SetName(&self, name: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null());
			_ret_.ok()
		}
	}
}

pub trait ID3D12Object: IUnknown {
	fn as_object(&self) -> &D3D12Object;
	fn into_object(self) -> D3D12Object;
}

impl ID3D12Object for D3D12Object {
	fn as_object(&self) -> &D3D12Object { self }
	fn into_object(self) -> D3D12Object { self }
}
impl IUnknown for D3D12Object {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Object {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

