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

use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1TessellationSink(pub(crate) Unknown);

impl Deref for D2D1TessellationSink {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1TessellationSink {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906c112e211dc9fed001143a055f9u128);
}

impl Com for D2D1TessellationSink {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1TessellationSink {
	pub fn AddTriangles(&self, triangles: &[D2D1Triangle]) -> () {
		unsafe {
			let vt = self.as_param();
			let (triangles_ptr_, triangles_len_) = triangles.deconstruct();
			let f: extern "system" fn(Param<Self>, *const D2D1Triangle, u32) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, triangles_ptr_, triangles_len_ as u32);
		}
	}

	pub fn Close(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait ID2D1TessellationSink: IUnknown {
	fn as_tessellation_sink(&self) -> &D2D1TessellationSink;
	fn into_tessellation_sink(self) -> D2D1TessellationSink;
}

impl ID2D1TessellationSink for D2D1TessellationSink {
	fn as_tessellation_sink(&self) -> &D2D1TessellationSink { self }
	fn into_tessellation_sink(self) -> D2D1TessellationSink { self }
}
impl IUnknown for D2D1TessellationSink {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1TessellationSink {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

