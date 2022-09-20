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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct EnumString(pub(crate) Unknown);

impl Deref for EnumString {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for EnumString {
	const IID: &'static GUID = &GUID::from_u128(0x0000010100000000c000000000000046u128);
}

impl Com for EnumString {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl EnumString {
	pub unsafe fn Next(&self) { todo!() }

	pub fn Skip(&self, celt: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, celt);
			_ret_.ok()
		}
	}

	pub fn Reset(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn Clone(&self) -> Result<EnumString, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut ppenum_out_: Option<EnumString> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, transmute(&mut ppenum_out_));
			if _ret_.is_ok() { if let Some(ppenum_out_) = ppenum_out_ { return Ok(ppenum_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IEnumString: IUnknown {
	fn as_enum_string(&self) -> &EnumString;
	fn into_enum_string(self) -> EnumString;
}

impl IEnumString for EnumString {
	fn as_enum_string(&self) -> &EnumString { self }
	fn into_enum_string(self) -> EnumString { self }
}
impl IUnknown for EnumString {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for EnumString {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

