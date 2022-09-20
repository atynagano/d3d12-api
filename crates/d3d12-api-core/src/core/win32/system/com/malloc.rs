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
pub struct Malloc(pub(crate) Unknown);

impl Deref for Malloc {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for Malloc {
	const IID: &'static GUID = &GUID::from_u128(0x0000000200000000c000000000000046u128);
}

impl Com for Malloc {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl Malloc {
	pub fn Alloc(&self, cb: usize) -> Option<NonNull<()>> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, usize) -> Option<NonNull<()>>
				= transmute(vt[3]);
			let _ret_ = f(vt, cb);
			_ret_
		}
	}

	pub fn Free(&self, pv: *const ()) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, pv as _);
		}
	}

	pub fn GetSize(&self, pv: *const ()) -> usize {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> usize
				= transmute(vt[6]);
			let _ret_ = f(vt, pv as _);
			_ret_
		}
	}

	pub fn DidAlloc(&self, pv: *const ()) -> i32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> i32
				= transmute(vt[7]);
			let _ret_ = f(vt, pv as _);
			_ret_
		}
	}

	pub fn HeapMinimize(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt);
		}
	}
}

pub trait IMalloc: IUnknown {
	fn as_malloc(&self) -> &Malloc;
	fn into_malloc(self) -> Malloc;
}

impl IMalloc for Malloc {
	fn as_malloc(&self) -> &Malloc { self }
	fn into_malloc(self) -> Malloc { self }
}
impl IUnknown for Malloc {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for Malloc {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

