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
pub struct DWriteFontFileStream(pub(crate) Unknown);

impl Deref for DWriteFontFileStream {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteFontFileStream {
	const IID: &'static GUID = &GUID::from_u128(0x6d4865fe0ab84d918f625dd6be34a3e0u128);
}

impl Com for DWriteFontFileStream {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteFontFileStream {
	pub unsafe fn ReadFileFragment(&self) { todo!() }

	pub fn ReleaseFileFragment(&self, fragment_context: &mut ()) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &mut ()) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, fragment_context);
		}
	}

	pub fn GetFileSize(&self) -> Result<u64, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut file_size_out_: MaybeUninit<u64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u64) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, file_size_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(file_size_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetLastWriteTime(&self) -> Result<u64, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut last_write_time_out_: MaybeUninit<u64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u64) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, last_write_time_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(last_write_time_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDWriteFontFileStream: IUnknown {
	fn as_font_file_stream(&self) -> &DWriteFontFileStream;
	fn into_font_file_stream(self) -> DWriteFontFileStream;
}

impl IDWriteFontFileStream for DWriteFontFileStream {
	fn as_font_file_stream(&self) -> &DWriteFontFileStream { self }
	fn into_font_file_stream(self) -> DWriteFontFileStream { self }
}
impl IUnknown for DWriteFontFileStream {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteFontFileStream {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

