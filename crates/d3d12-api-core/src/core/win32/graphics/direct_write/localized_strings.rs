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
pub struct DWriteLocalizedStrings(pub(crate) Unknown);

impl Deref for DWriteLocalizedStrings {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteLocalizedStrings {
	const IID: &'static GUID = &GUID::from_u128(0x08256209099a4b34b86dc22b110e7771u128);
}

impl Com for DWriteLocalizedStrings {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteLocalizedStrings {
	pub fn GetCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn FindLocaleName(&self, locale_name: &str) -> Result<(u32, bool), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut index_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut exists_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, *const u16, *mut u32, &mut Bool) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, locale_name.to_utf16().as_ptr_or_null(), index_out_.as_mut_ptr(), &mut exists_out_);
			if _ret_.is_ok() { Ok((index_out_.assume_init(), exists_out_.to_bool())) } else { Err(_ret_) }
		}
	}

	pub fn GetLocaleNameLength(&self, index: u32) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, index, length_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(length_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetLocaleName(&self) { todo!() }

	pub fn GetStringLength(&self, index: u32) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut length_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, index, length_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(length_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetString(&self) { todo!() }
}

pub trait IDWriteLocalizedStrings: IUnknown {
	fn as_localized_strings(&self) -> &DWriteLocalizedStrings;
	fn into_localized_strings(self) -> DWriteLocalizedStrings;
}

impl IDWriteLocalizedStrings for DWriteLocalizedStrings {
	fn as_localized_strings(&self) -> &DWriteLocalizedStrings { self }
	fn into_localized_strings(self) -> DWriteLocalizedStrings { self }
}
impl IUnknown for DWriteLocalizedStrings {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteLocalizedStrings {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

