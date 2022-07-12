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

	fn GetVersion(&self, ) -> Result<(u32, u32, ), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_major: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut _out_minor: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_major: *mut u32, _out_minor: *mut u32, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, _out_major.as_mut_ptr(), _out_minor.as_mut_ptr(), );
			if _ret_.is_ok() {
				return Ok((_out_major.assume_init(), _out_minor.assume_init(), ));
			}
			Err(_ret_)
		}
	}

	fn GetFlags(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_flags: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_flags: *mut u32, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, _out_flags.as_mut_ptr(), );
			Ok(_out_flags.assume_init())
		}
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
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

