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
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1CommandSink1(pub(crate) D2D1CommandSink);

impl Deref for D2D1CommandSink1 {
	type Target = D2D1CommandSink;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1CommandSink1 {
	const IID: &'static GUID = &GUID::from_u128(0x9eb767fd42694467b8c2eb30cb305743u128);
}

impl Com for D2D1CommandSink1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1CommandSink1 {
	pub fn SetPrimitiveBlend1(&self, primitive_blend: D2D1PrimitiveBlend) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1PrimitiveBlend) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, primitive_blend);
			_ret_.ok()
		}
	}
}

pub trait ID2D1CommandSink1: ID2D1CommandSink {
	fn as_command_sink1(&self) -> &D2D1CommandSink1;
	fn into_command_sink1(self) -> D2D1CommandSink1;
}

impl ID2D1CommandSink1 for D2D1CommandSink1 {
	fn as_command_sink1(&self) -> &D2D1CommandSink1 { self }
	fn into_command_sink1(self) -> D2D1CommandSink1 { self }
}
impl ID2D1CommandSink for D2D1CommandSink1 {
	fn as_command_sink(&self) -> &D2D1CommandSink { &self.0.as_command_sink() }
	fn into_command_sink(self) -> D2D1CommandSink { self.0.into_command_sink() }
}

impl IUnknown for D2D1CommandSink1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1CommandSink1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1CommandSink::from(v))
    }
}

