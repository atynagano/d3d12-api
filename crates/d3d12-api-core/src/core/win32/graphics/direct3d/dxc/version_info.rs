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

use crate::core::win32::foundation::*;
#[repr(C)]
pub struct DxcVersionInfo(pub(crate) Unknown);

impl Guid for DxcVersionInfo {
	const IID: &'static GUID = &GUID::from_u128(0xb04f5b5020594f12a8ffa1e0cde1cc7eu128);
}

impl Clone for DxcVersionInfo {
	fn clone(&self) -> Self { DxcVersionInfo(self.0.clone()) }
}

pub trait IDxcVersionInfo: IUnknown {
	fn as_version_info(&self) -> &DxcVersionInfo;
	fn into_version_info(self) -> DxcVersionInfo;

	fn GetVersion(&self, ) -> Result<(u32, u32), HResult> {
		let vt = self.as_param();
		let mut _major: u32 = u32::zeroed();
		let mut _minor: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _major: &mut u32, _minor: &mut u32, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, &mut _major, &mut _minor, );
		if ret.is_ok() {
			return Ok((_major, _minor));
		}
		Err(ret)
	}

	fn GetFlags(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _flags: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _flags: &mut u32, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, &mut _flags, );
		if ret.is_ok() {
			return Ok((_flags));
		}
		Err(ret)
	}
}

impl IDxcVersionInfo for DxcVersionInfo {
	fn as_version_info(&self) -> &DxcVersionInfo { self }
	fn into_version_info(self) -> DxcVersionInfo { self }
}

impl From<Unknown> for DxcVersionInfo {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcVersionInfo {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

