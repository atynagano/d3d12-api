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
pub struct DxcVersionInfo3(pub(crate) Unknown);

impl Guid for DxcVersionInfo3 {
	const IID: &'static GUID = &GUID::from_u128(0x5e13e8439d25473c9ad203b2d0b44b1eu128);
}

impl Clone for DxcVersionInfo3 {
	fn clone(&self) -> Self { DxcVersionInfo3(self.0.clone()) }
}

pub trait IDxcVersionInfo3: IUnknown {
	fn as_version_info3(&self) -> &DxcVersionInfo3;
	fn into_version_info3(self) -> DxcVersionInfo3;

	fn GetCustomVersionString(&self, ) -> Result<(&i8), HResult> {
		let vt = self.as_param();
		let mut _version_string: Option<&i8> = None;
		let f: extern "system" fn(Param<Self>, _version_string: &mut Option<&i8>, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, &mut _version_string, );
		if ret.is_ok() {
			if let (Some(_version_string)) = (_version_string) {
				return Ok((_version_string));
			}
		}
		Err(ret)
	}
}

impl IDxcVersionInfo3 for DxcVersionInfo3 {
	fn as_version_info3(&self) -> &DxcVersionInfo3 { self }
	fn into_version_info3(self) -> DxcVersionInfo3 { self }
}

impl From<Unknown> for DxcVersionInfo3 {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcVersionInfo3 {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

