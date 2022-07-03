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

#[repr(C)]
pub struct D3DBlob(pub(crate) Unknown);

impl Guid for D3DBlob {
	const IID: &'static GUID = &GUID::from_u128(0x8ba5fb08519540e2ac580d989c3a0102u128);
}

impl Clone for D3DBlob {
	fn clone(&self) -> Self { D3DBlob(self.0.clone()) }
}

pub trait ID3DBlob: IUnknown {
	fn as_blob(&self) -> &D3DBlob;
	fn into_blob(self) -> D3DBlob;

	fn GetBufferPointer(&self, ) -> (*const c_void) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> *const c_void
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetBufferSize(&self, ) -> (usize) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> usize
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl ID3DBlob for D3DBlob {
	fn as_blob(&self) -> &D3DBlob { self }
	fn into_blob(self) -> D3DBlob { self }
}

impl From<Unknown> for D3DBlob {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3DBlob {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

