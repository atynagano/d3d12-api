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
pub struct D3D12Debug(pub(crate) Unknown);

impl Guid for D3D12Debug {
	const IID: &'static GUID = &GUID::from_u128(0x344488b76846474bb989f027448245e0u128);
}

impl Clone for D3D12Debug {
	fn clone(&self) -> Self { D3D12Debug(self.0.clone()) }
}

pub trait ID3D12Debug: IUnknown {
	fn as_debug(&self) -> &D3D12Debug;
	fn into_debug(self) -> D3D12Debug;

	fn EnableDebugLayer(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, );
		}
	}
}

impl ID3D12Debug for D3D12Debug {
	fn as_debug(&self) -> &D3D12Debug { self }
	fn into_debug(self) -> D3D12Debug { self }
}

impl From<Unknown> for D3D12Debug {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12Debug {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

