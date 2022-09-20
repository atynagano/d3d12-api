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
pub struct ErrorLog(pub(crate) Unknown);

impl Deref for ErrorLog {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for ErrorLog {
	const IID: &'static GUID = &GUID::from_u128(0x3127ca40446e11ce813500aa004bb851u128);
}

impl Com for ErrorLog {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl ErrorLog {
	pub fn AddError(&self, psz_prop_name: &str, excep_info: &ExCepInfo) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, &ExCepInfo) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, psz_prop_name.to_utf16().as_ptr_or_null(), excep_info);
			_ret_.ok()
		}
	}
}

pub trait IErrorLog: IUnknown {
	fn as_error_log(&self) -> &ErrorLog;
	fn into_error_log(self) -> ErrorLog;
}

impl IErrorLog for ErrorLog {
	fn as_error_log(&self) -> &ErrorLog { self }
	fn into_error_log(self) -> ErrorLog { self }
}
impl IUnknown for ErrorLog {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for ErrorLog {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

