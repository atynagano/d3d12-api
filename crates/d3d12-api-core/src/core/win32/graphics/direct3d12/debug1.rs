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
pub struct D3D12Debug1(pub(crate) Unknown);

impl Deref for D3D12Debug1 {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Debug1 {
	const IID: &'static GUID = &GUID::from_u128(0xaffaa4ca63fe4d8eb8ad159000af4304u128);
}

impl Com for D3D12Debug1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Debug1 {
	pub fn EnableDebugLayer(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt);
		}
	}

	pub fn SetEnableGPUBasedValidation(&self, enable: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, enable.to_bool());
		}
	}

	pub fn SetEnableSynchronizedCommandQueueValidation(&self, enable: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, enable.to_bool());
		}
	}
}

pub trait ID3D12Debug1: IUnknown {
	fn as_debug1(&self) -> &D3D12Debug1;
	fn into_debug1(self) -> D3D12Debug1;
}

impl ID3D12Debug1 for D3D12Debug1 {
	fn as_debug1(&self) -> &D3D12Debug1 { self }
	fn into_debug1(self) -> D3D12Debug1 { self }
}
impl IUnknown for D3D12Debug1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Debug1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

