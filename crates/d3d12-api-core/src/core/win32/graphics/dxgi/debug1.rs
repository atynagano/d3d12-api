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
pub struct DxgiDebug1(pub(crate) DxgiDebug);

impl Guid for DxgiDebug1 {
	const IID: &'static GUID = &GUID::from_u128(0xc5a05f0c16f24adf9f4da8c4d58ac550u128);
}

impl Clone for DxgiDebug1 {
	fn clone(&self) -> Self { DxgiDebug1(self.0.clone()) }
}

pub trait IDxgiDebug1: IDxgiDebug {
	fn as_debug1(&self) -> &DxgiDebug1;
	fn into_debug1(self) -> DxgiDebug1;

	fn EnableLeakTrackingForThread(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, );
		}
	}

	fn DisableLeakTrackingForThread(&self, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, );
		}
	}

	fn IsLeakTrackingEnabledForThread(&self, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Bool
				= transmute(vt[6]);
			let _ret_ = f(vt, );
			_ret_.to_bool()
		}
	}
}

impl IDxgiDebug1 for DxgiDebug1 {
	fn as_debug1(&self) -> &DxgiDebug1 { self }
	fn into_debug1(self) -> DxgiDebug1 { self }
}

impl IDxgiDebug for DxgiDebug1 {
	fn as_debug(&self) -> &DxgiDebug { &self.0 }
	fn into_debug(self) -> DxgiDebug { self.0 }
}

impl From<Unknown> for DxgiDebug1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiDebug::from(v))
    }
}

impl IUnknown for DxgiDebug1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

