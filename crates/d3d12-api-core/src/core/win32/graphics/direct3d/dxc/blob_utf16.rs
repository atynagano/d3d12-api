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
pub struct DxcBlobUtf16(pub(crate) DxcBlobEncoding);

impl Guid for DxcBlobUtf16 {
	const IID: &'static GUID = &GUID::from_u128(0xa3f84eab0faa497ea39cee6ed60b2d84u128);
}

impl Clone for DxcBlobUtf16 {
	fn clone(&self) -> Self { DxcBlobUtf16(self.0.clone()) }
}

pub trait IDxcBlobUtf16: IDxcBlobEncoding {
	fn as_blob_utf16(&self) -> &DxcBlobUtf16;
	fn into_blob_utf16(self) -> DxcBlobUtf16;

	fn GetStringPointer(&self, ) -> (PWStr) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> PWStr
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, );
		return (ret);
	}

	fn GetStringLength(&self, ) -> (usize) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> usize
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl IDxcBlobUtf16 for DxcBlobUtf16 {
	fn as_blob_utf16(&self) -> &DxcBlobUtf16 { self }
	fn into_blob_utf16(self) -> DxcBlobUtf16 { self }
}

impl IDxcBlobEncoding for DxcBlobUtf16 {
	fn as_blob_encoding(&self) -> &DxcBlobEncoding { &self.0 }
	fn into_blob_encoding(self) -> DxcBlobEncoding { self.0 }
}

impl IDxcBlob for DxcBlobUtf16 {
	fn as_blob(&self) -> &DxcBlob { &self.0.0 }
	fn into_blob(self) -> DxcBlob { self.0.0 }
}

impl From<Unknown> for DxcBlobUtf16 {
    fn from(v: Unknown) -> Self {
        Self(DxcBlobEncoding::from(v))
    }
}

impl IUnknown for DxcBlobUtf16 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

