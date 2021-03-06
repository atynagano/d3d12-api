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


#[repr(C)]
pub struct DxcBlob(pub(crate) Unknown);

impl Guid for DxcBlob {
	const IID: &'static GUID = &GUID::from_u128(0x8ba5fb08519540e2ac580d989c3a0102u128);
}

impl Clone for DxcBlob {
	fn clone(&self) -> Self { DxcBlob(self.0.clone()) }
}

pub trait IDxcBlob: IUnknown {
	fn as_blob(&self) -> &DxcBlob;
	fn into_blob(self) -> DxcBlob;

	fn GetBufferPointer(&self, ) -> *const () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> *const c_void
				= transmute(vt[3]);
			let _ret_ = f(vt, );
			_ret_ as _
		}
	}

	fn GetBufferSize(&self, ) -> usize {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> usize
				= transmute(vt[4]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl IDxcBlob for DxcBlob {
	fn as_blob(&self) -> &DxcBlob { self }
	fn into_blob(self) -> DxcBlob { self }
}

impl From<Unknown> for DxcBlob {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcBlob {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

