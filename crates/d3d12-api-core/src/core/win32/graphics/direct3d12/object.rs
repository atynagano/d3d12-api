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
use crate::core::win32::system::com::*;

#[repr(C)]
pub struct D3D12Object(pub(crate) Unknown);

impl Guid for D3D12Object {
	const IID: &'static GUID = &GUID::from_u128(0xc4fec28f79664e959f94f431cb56c3b8u128);
}

impl Clone for D3D12Object {
	fn clone(&self) -> Self { D3D12Object(self.0.clone()) }
}

pub trait ID3D12Object: IUnknown {
	fn as_object(&self) -> &D3D12Object;
	fn into_object(self) -> D3D12Object;

	fn SetPrivateDataInterface(&self, guid: &GUID, data: Option<&Unknown>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, guid: &GUID, data: *const c_void, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, guid, option_to_vtable(data), );
			_ret_.ok()
		}
	}

	fn SetName(&self, name: &str, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, name: *const u16, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), );
			_ret_.ok()
		}
	}
}

impl ID3D12Object for D3D12Object {
	fn as_object(&self) -> &D3D12Object { self }
	fn into_object(self) -> D3D12Object { self }
}

impl From<Unknown> for D3D12Object {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12Object {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

