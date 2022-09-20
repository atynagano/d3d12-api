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


#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcBlob(pub(crate) Unknown);

impl Deref for DxcBlob {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcBlob {
	const IID: &'static GUID = &GUID::from_u128(0x8ba5fb08519540e2ac580d989c3a0102u128);
}

impl Com for DxcBlob {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcBlob {
	pub fn GetBufferPointer(&self) -> *const u8 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> *const u8
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetBufferSize(&self) -> usize {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> usize
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait IDxcBlob: IUnknown {
	fn as_blob(&self) -> &DxcBlob;
	fn into_blob(self) -> DxcBlob;
}

impl IDxcBlob for DxcBlob {
	fn as_blob(&self) -> &DxcBlob { self }
	fn into_blob(self) -> DxcBlob { self }
}
impl IUnknown for DxcBlob {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcBlob {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

