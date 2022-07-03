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
pub struct DxcVersionInfo2(pub(crate) DxcVersionInfo);

impl Guid for DxcVersionInfo2 {
	const IID: &'static GUID = &GUID::from_u128(0xfb6904c442f04b629c46983af7da7c83u128);
}

impl Clone for DxcVersionInfo2 {
	fn clone(&self) -> Self { DxcVersionInfo2(self.0.clone()) }
}

pub trait IDxcVersionInfo2: IDxcVersionInfo {
	fn as_version_info2(&self) -> &DxcVersionInfo2;
	fn into_version_info2(self) -> DxcVersionInfo2;

	fn GetCommitInfo(&self, ) -> Result<(u32, &i8), HResult> {
		let vt = self.as_param();
		let mut _commit_count: u32 = u32::zeroed();
		let mut _commit_hash: Option<&i8> = None;
		let f: extern "system" fn(Param<Self>, _commit_count: &mut u32, _commit_hash: &mut Option<&i8>, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, &mut _commit_count, &mut _commit_hash, );
		if ret.is_ok() {
			if let (Some(_commit_hash)) = (_commit_hash) {
				return Ok((_commit_count, _commit_hash));
			}
		}
		Err(ret)
	}
}

impl IDxcVersionInfo2 for DxcVersionInfo2 {
	fn as_version_info2(&self) -> &DxcVersionInfo2 { self }
	fn into_version_info2(self) -> DxcVersionInfo2 { self }
}

impl IDxcVersionInfo for DxcVersionInfo2 {
	fn as_version_info(&self) -> &DxcVersionInfo { &self.0 }
	fn into_version_info(self) -> DxcVersionInfo { self.0 }
}

impl From<Unknown> for DxcVersionInfo2 {
    fn from(v: Unknown) -> Self {
        Self(DxcVersionInfo::from(v))
    }
}

impl IUnknown for DxcVersionInfo2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

