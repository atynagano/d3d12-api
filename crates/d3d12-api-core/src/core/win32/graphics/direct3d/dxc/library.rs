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
use crate::core::win32::graphics::direct3d::dxc::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcLibrary(pub(crate) Unknown);

impl Deref for DxcLibrary {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcLibrary {
	const IID: &'static GUID = &GUID::from_u128(0xe5204dc7d18c4c3cbdfb851673980fe7u128);
}

impl Com for DxcLibrary {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcLibrary {
	pub fn SetMalloc(&self, malloc: Option<&Malloc>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, option_to_vtable(malloc));
			_ret_.ok()
		}
	}

	pub fn CreateBlobFromBlob(&self, blob: &DxcBlob, offset: u32, length: u32) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, VTable, u32, u32, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, blob.vtable(), offset, length, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBlobFromFile(&self, file_name: &str, code_page: Option<&DxcCp>) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *const c_void, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, file_name.to_utf16().as_ptr_or_null(), transmute(code_page), transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBlobWithEncodingFromPinned(&self, text: &[u8], code_page: DxcCp) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let (text_ptr_, text_len_) = text.deconstruct();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *const u8, u32, DxcCp, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, text_ptr_, text_len_ as u32, code_page, transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBlobWithEncodingOnHeapCopy(&self, text: &[u8], code_page: DxcCp) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let (text_ptr_, text_len_) = text.deconstruct();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *const u8, u32, DxcCp, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, text_ptr_, text_len_ as u32, code_page, transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBlobWithEncodingOnMalloc(&self, text: &[u8], i_malloc: &Malloc, code_page: DxcCp) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let (text_ptr_, text_len_) = text.deconstruct();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *const u8, VTable, u32, DxcCp, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, text_ptr_, i_malloc.vtable(), text_len_ as u32, code_page, transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateIncludeHandler(&self) -> Result<DxcIncludeHandler, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcIncludeHandler> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateStreamFromBlobReadOnly(&self, blob: &DxcBlob) -> Result<Stream, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut stream_out_: Option<Stream> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut stream_out_));
			if _ret_.is_ok() { if let Some(stream_out_) = stream_out_ { return Ok(stream_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetBlobAsUtf8(&self, blob: &DxcBlob) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetBlobAsUtf16(&self, blob: &DxcBlob) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcLibrary: IUnknown {
	fn as_library(&self) -> &DxcLibrary;
	fn into_library(self) -> DxcLibrary;
}

impl IDxcLibrary for DxcLibrary {
	fn as_library(&self) -> &DxcLibrary { self }
	fn into_library(self) -> DxcLibrary { self }
}
impl IUnknown for DxcLibrary {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcLibrary {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

