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
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::direct3d::dxc::*;

#[repr(C)]
pub struct DxcLibrary(pub(crate) Unknown);

impl Guid for DxcLibrary {
	const IID: &'static GUID = &GUID::from_u128(0xe5204dc7d18c4c3cbdfb851673980fe7u128);
}

impl Clone for DxcLibrary {
	fn clone(&self) -> Self { DxcLibrary(self.0.clone()) }
}

pub trait IDxcLibrary: IUnknown {
	fn as_library(&self) -> &DxcLibrary;
	fn into_library(self) -> DxcLibrary;

	fn SetMalloc(&self, malloc: Option<&Malloc>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, malloc: *const c_void, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, option_to_vtable(malloc), );
			_ret_.ok()
		}
	}

	fn CreateBlobFromBlob(&self, blob: &(impl IDxcBlob + ?Sized), offset: u32, length: u32, ) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, blob: VTable, offset: u32, length: u32, result: *mut c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, blob.vtable(), offset, length, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateBlobFromFile(&self, file_name: &str, code_page: Option<&DxcCp>, ) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_blob_encoding: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, file_name: *const u16, code_page: *const c_void, blob_encoding: *mut c_void, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, file_name.to_utf16().as_ptr_or_null(), transmute(code_page), transmute(&mut _out_blob_encoding), );
			if _ret_.is_ok() {
				if let Some(_out_blob_encoding) = _out_blob_encoding {
					return Ok(_out_blob_encoding);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateIncludeHandler(&self, ) -> Result<DxcIncludeHandler, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcIncludeHandler> = None;
			let f: extern "system" fn(Param<Self>, result: *mut c_void, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateStreamFromBlobReadOnly(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<Stream, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_stream: Option<Stream> = None;
			let f: extern "system" fn(Param<Self>, blob: VTable, stream: *mut c_void, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut _out_stream), );
			if _ret_.is_ok() {
				if let Some(_out_stream) = _out_stream {
					return Ok(_out_stream);
				}
			}
			Err(_ret_)
		}
	}

	fn GetBlobAsUtf8(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_blob_encoding: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, blob: VTable, blob_encoding: *mut c_void, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut _out_blob_encoding), );
			if _ret_.is_ok() {
				if let Some(_out_blob_encoding) = _out_blob_encoding {
					return Ok(_out_blob_encoding);
				}
			}
			Err(_ret_)
		}
	}

	fn GetBlobAsUtf16(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_blob_encoding: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, blob: VTable, blob_encoding: *mut c_void, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut _out_blob_encoding), );
			if _ret_.is_ok() {
				if let Some(_out_blob_encoding) = _out_blob_encoding {
					return Ok(_out_blob_encoding);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcLibrary for DxcLibrary {
	fn as_library(&self) -> &DxcLibrary { self }
	fn into_library(self) -> DxcLibrary { self }
}

impl From<Unknown> for DxcLibrary {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcLibrary {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

