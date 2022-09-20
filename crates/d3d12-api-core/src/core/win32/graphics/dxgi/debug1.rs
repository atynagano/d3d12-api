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
pub struct DxgiDebug1(pub(crate) DxgiDebug);

impl Deref for DxgiDebug1 {
	type Target = DxgiDebug;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiDebug1 {
	const IID: &'static GUID = &GUID::from_u128(0xc5a05f0c16f24adf9f4da8c4d58ac550u128);
}

impl Com for DxgiDebug1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiDebug1 {
	pub fn EnableLeakTrackingForThread(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt);
		}
	}

	pub fn DisableLeakTrackingForThread(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt);
		}
	}

	pub fn IsLeakTrackingEnabledForThread(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}
}

pub trait IDxgiDebug1: IDxgiDebug {
	fn as_debug1(&self) -> &DxgiDebug1;
	fn into_debug1(self) -> DxgiDebug1;
}

impl IDxgiDebug1 for DxgiDebug1 {
	fn as_debug1(&self) -> &DxgiDebug1 { self }
	fn into_debug1(self) -> DxgiDebug1 { self }
}
impl IDxgiDebug for DxgiDebug1 {
	fn as_debug(&self) -> &DxgiDebug { &self.0.as_debug() }
	fn into_debug(self) -> DxgiDebug { self.0.into_debug() }
}

impl IUnknown for DxgiDebug1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiDebug1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiDebug::from(v))
    }
}

