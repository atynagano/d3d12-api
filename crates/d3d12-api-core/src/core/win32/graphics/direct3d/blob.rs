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
pub struct D3DBlob(pub(crate) Unknown);

impl Deref for D3DBlob {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3DBlob {
	const IID: &'static GUID = &GUID::from_u128(0x8ba5fb08519540e2ac580d989c3a0102u128);
}

impl Com for D3DBlob {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3DBlob {
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

pub trait ID3DBlob: IUnknown {
	fn as_blob(&self) -> &D3DBlob;
	fn into_blob(self) -> D3DBlob;
}

impl ID3DBlob for D3DBlob {
	fn as_blob(&self) -> &D3DBlob { self }
	fn into_blob(self) -> D3DBlob { self }
}
impl IUnknown for D3DBlob {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3DBlob {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

