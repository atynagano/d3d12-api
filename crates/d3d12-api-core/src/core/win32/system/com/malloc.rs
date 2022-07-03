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
pub struct Malloc(pub(crate) Unknown);

impl Guid for Malloc {
	const IID: &'static GUID = &GUID::from_u128(0x0000000200000000c000000000000046u128);
}

impl Clone for Malloc {
	fn clone(&self) -> Self { Malloc(self.0.clone()) }
}

pub trait IMalloc: IUnknown {
	fn as_malloc(&self) -> &Malloc;
	fn into_malloc(self) -> Malloc;

	fn Alloc(&self, cb: usize, ) -> (*const c_void) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, cb: usize, ) -> *const c_void
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, cb, );
		return (ret);
	}

	fn Free(&self, pv: Option<*const c_void>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, pv: *const c_void, ) -> ()
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, pv.as_ptr_or_null(), );
	}

	fn GetSize(&self, pv: Option<*const c_void>, ) -> (usize) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, pv: *const c_void, ) -> usize
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, pv.as_ptr_or_null(), );
		return (ret);
	}

	fn DidAlloc(&self, pv: Option<*const c_void>, ) -> (i32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, pv: *const c_void, ) -> i32
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, pv.as_ptr_or_null(), );
		return (ret);
	}

	fn HeapMinimize(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, );
	}
}

impl IMalloc for Malloc {
	fn as_malloc(&self) -> &Malloc { self }
	fn into_malloc(self) -> Malloc { self }
}

impl From<Unknown> for Malloc {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for Malloc {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

