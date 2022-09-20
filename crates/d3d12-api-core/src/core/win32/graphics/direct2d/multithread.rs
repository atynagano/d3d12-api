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
pub struct D2D1Multithread(pub(crate) Unknown);

impl Deref for D2D1Multithread {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Multithread {
	const IID: &'static GUID = &GUID::from_u128(0x31e6e7bce0ff4d468c64a0a8c41c15d3u128);
}

impl Com for D2D1Multithread {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Multithread {
	pub fn GetMultithreadProtected(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}

	pub fn Enter(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt);
		}
	}

	pub fn Leave(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt);
		}
	}
}

pub trait ID2D1Multithread: IUnknown {
	fn as_multithread(&self) -> &D2D1Multithread;
	fn into_multithread(self) -> D2D1Multithread;
}

impl ID2D1Multithread for D2D1Multithread {
	fn as_multithread(&self) -> &D2D1Multithread { self }
	fn into_multithread(self) -> D2D1Multithread { self }
}
impl IUnknown for D2D1Multithread {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Multithread {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

