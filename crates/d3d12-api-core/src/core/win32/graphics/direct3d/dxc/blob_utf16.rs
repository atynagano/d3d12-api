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
pub struct DxcBlobUtf16(pub(crate) DxcBlobEncoding);

impl Deref for DxcBlobUtf16 {
	type Target = DxcBlobEncoding;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcBlobUtf16 {
	const IID: &'static GUID = &GUID::from_u128(0xa3f84eab0faa497ea39cee6ed60b2d84u128);
}

impl Com for DxcBlobUtf16 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcBlobUtf16 {
	pub fn GetStringPointer(&self) -> *const u16 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> *const u16
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetStringLength(&self) -> usize {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> usize
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait IDxcBlobUtf16: IDxcBlobEncoding {
	fn as_blob_utf16(&self) -> &DxcBlobUtf16;
	fn into_blob_utf16(self) -> DxcBlobUtf16;
}

impl IDxcBlobUtf16 for DxcBlobUtf16 {
	fn as_blob_utf16(&self) -> &DxcBlobUtf16 { self }
	fn into_blob_utf16(self) -> DxcBlobUtf16 { self }
}
impl IDxcBlobEncoding for DxcBlobUtf16 {
	fn as_blob_encoding(&self) -> &DxcBlobEncoding { &self.0.as_blob_encoding() }
	fn into_blob_encoding(self) -> DxcBlobEncoding { self.0.into_blob_encoding() }
}

impl IDxcBlob for DxcBlobUtf16 {
	fn as_blob(&self) -> &DxcBlob { &self.0.as_blob() }
	fn into_blob(self) -> DxcBlob { self.0.into_blob() }
}

impl IUnknown for DxcBlobUtf16 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcBlobUtf16 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxcBlobEncoding::from(v))
    }
}

