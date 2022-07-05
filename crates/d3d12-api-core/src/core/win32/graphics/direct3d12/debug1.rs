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
pub struct D3D12Debug1(pub(crate) Unknown);

impl Guid for D3D12Debug1 {
	const IID: &'static GUID = &GUID::from_u128(0xaffaa4ca63fe4d8eb8ad159000af4304u128);
}

impl Clone for D3D12Debug1 {
	fn clone(&self) -> Self { D3D12Debug1(self.0.clone()) }
}

pub trait ID3D12Debug1: IUnknown {
	fn as_debug1(&self) -> &D3D12Debug1;
	fn into_debug1(self) -> D3D12Debug1;

	fn EnableDebugLayer(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, );
		}
	}

	fn SetEnableGPUBasedValidation(&self, enable: bool, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, enable: Bool, ) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, enable.to_bool(), );
		}
	}

	fn SetEnableSynchronizedCommandQueueValidation(&self, enable: bool, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, enable: Bool, ) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, enable.to_bool(), );
		}
	}
}

impl ID3D12Debug1 for D3D12Debug1 {
	fn as_debug1(&self) -> &D3D12Debug1 { self }
	fn into_debug1(self) -> D3D12Debug1 { self }
}

impl From<Unknown> for D3D12Debug1 {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12Debug1 {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

