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
pub struct DxcBlobUtf8(pub(crate) DxcBlobEncoding);

impl Deref for DxcBlobUtf8 {
	type Target = DxcBlobEncoding;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcBlobUtf8 {
	const IID: &'static GUID = &GUID::from_u128(0x3da636c9ba714024a30130cbf125305bu128);
}

impl Com for DxcBlobUtf8 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcBlobUtf8 {
	pub fn GetStringPointer(&self) -> *const u8 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> *const u8
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

pub trait IDxcBlobUtf8: IDxcBlobEncoding {
	fn as_blob_utf8(&self) -> &DxcBlobUtf8;
	fn into_blob_utf8(self) -> DxcBlobUtf8;
}

impl IDxcBlobUtf8 for DxcBlobUtf8 {
	fn as_blob_utf8(&self) -> &DxcBlobUtf8 { self }
	fn into_blob_utf8(self) -> DxcBlobUtf8 { self }
}
impl IDxcBlobEncoding for DxcBlobUtf8 {
	fn as_blob_encoding(&self) -> &DxcBlobEncoding { &self.0.as_blob_encoding() }
	fn into_blob_encoding(self) -> DxcBlobEncoding { self.0.into_blob_encoding() }
}

impl IDxcBlob for DxcBlobUtf8 {
	fn as_blob(&self) -> &DxcBlob { &self.0.as_blob() }
	fn into_blob(self) -> DxcBlob { self.0.into_blob() }
}

impl IUnknown for DxcBlobUtf8 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcBlobUtf8 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxcBlobEncoding::from(v))
    }
}

