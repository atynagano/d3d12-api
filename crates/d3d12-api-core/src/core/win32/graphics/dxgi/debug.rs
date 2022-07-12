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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
pub struct DxgiDebug(pub(crate) Unknown);

impl Guid for DxgiDebug {
	const IID: &'static GUID = &GUID::from_u128(0x119e7452de9e40fe880688f90c12b441u128);
}

impl Clone for DxgiDebug {
	fn clone(&self) -> Self { DxgiDebug(self.0.clone()) }
}

pub trait IDxgiDebug: IUnknown {
	fn as_debug(&self) -> &DxgiDebug;
	fn into_debug(self) -> DxgiDebug;

	fn ReportLiveObjects(&self, apiid: GUID, flags: DxgiDebugRLoFlags, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, apiid: GUID, flags: DxgiDebugRLoFlags, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, apiid, flags, );
			_ret_.ok()
		}
	}
}

impl IDxgiDebug for DxgiDebug {
	fn as_debug(&self) -> &DxgiDebug { self }
	fn into_debug(self) -> DxgiDebug { self }
}

impl From<Unknown> for DxgiDebug {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxgiDebug {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

