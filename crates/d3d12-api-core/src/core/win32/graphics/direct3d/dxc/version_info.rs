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
pub struct DxcVersionInfo(pub(crate) Unknown);

impl Deref for DxcVersionInfo {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcVersionInfo {
	const IID: &'static GUID = &GUID::from_u128(0xb04f5b5020594f12a8ffa1e0cde1cc7eu128);
}

impl Com for DxcVersionInfo {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcVersionInfo {
	pub fn GetVersion(&self) -> Result<(u32, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut major_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut minor_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32, *mut u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, major_out_.as_mut_ptr(), minor_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((major_out_.assume_init(), minor_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetFlags(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut flags_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, flags_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(flags_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxcVersionInfo: IUnknown {
	fn as_version_info(&self) -> &DxcVersionInfo;
	fn into_version_info(self) -> DxcVersionInfo;
}

impl IDxcVersionInfo for DxcVersionInfo {
	fn as_version_info(&self) -> &DxcVersionInfo { self }
	fn into_version_info(self) -> DxcVersionInfo { self }
}
impl IUnknown for DxcVersionInfo {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcVersionInfo {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

