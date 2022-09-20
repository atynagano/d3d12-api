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
pub struct DXGraphicsAnalysis(pub(crate) Unknown);

impl Deref for DXGraphicsAnalysis {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DXGraphicsAnalysis {
	const IID: &'static GUID = &GUID::from_u128(0x9f2515149d4d49029d6018988ab7d4b5u128);
}

impl Com for DXGraphicsAnalysis {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DXGraphicsAnalysis {
	pub fn BeginCapture(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt);
		}
	}

	pub fn EndCapture(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt);
		}
	}
}

pub trait IDXGraphicsAnalysis: IUnknown {
	fn as_graphics_analysis(&self) -> &DXGraphicsAnalysis;
	fn into_graphics_analysis(self) -> DXGraphicsAnalysis;
}

impl IDXGraphicsAnalysis for DXGraphicsAnalysis {
	fn as_graphics_analysis(&self) -> &DXGraphicsAnalysis { self }
	fn into_graphics_analysis(self) -> DXGraphicsAnalysis { self }
}
impl IUnknown for DXGraphicsAnalysis {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DXGraphicsAnalysis {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

