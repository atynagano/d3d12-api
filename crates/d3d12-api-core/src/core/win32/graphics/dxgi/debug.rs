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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiDebug(pub(crate) Unknown);

impl Deref for DxgiDebug {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiDebug {
	const IID: &'static GUID = &GUID::from_u128(0x119e7452de9e40fe880688f90c12b441u128);
}

impl Com for DxgiDebug {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiDebug {
	pub fn ReportLiveObjects(&self, apiid: GUID, flags: DxgiDebugRloFlags) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, GUID, DxgiDebugRloFlags) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, apiid, flags);
			_ret_.ok()
		}
	}
}

pub trait IDxgiDebug: IUnknown {
	fn as_debug(&self) -> &DxgiDebug;
	fn into_debug(self) -> DxgiDebug;
}

impl IDxgiDebug for DxgiDebug {
	fn as_debug(&self) -> &DxgiDebug { self }
	fn into_debug(self) -> DxgiDebug { self }
}
impl IUnknown for DxgiDebug {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiDebug {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

