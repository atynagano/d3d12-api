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
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, malloc: Option<VTable>, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, option_to_vtable(malloc), );
		ret.ok()
	}

	fn CreateBlobFromBlob(&self, blob: &(impl IDxcBlob + ?Sized), offset: u32, length: u32, ) -> Result<(DxcBlob), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcBlob> = None;
		let f: extern "system" fn(Param<Self>, blob: VTable, offset: u32, length: u32, _result: &mut Option<DxcBlob>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, blob.vtable(), offset, length, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn CreateBlobFromFile(&self, file_name: &str, code_page: Option<&DxcCp>, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, file_name: *const u16, code_page: Option<&DxcCp>, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, file_name.to_utf16().as_ptr_or_null(), code_page, &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn CreateBlobWithEncodingFromPinned(&self, text: &[u8], code_page: DxcCp, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, text: *const u8, size: u32, code_page: DxcCp, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, text.as_ptr_or_null(), text.len() as u32, code_page, &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn CreateBlobWithEncodingOnHeapCopy(&self, text: &[u8], code_page: DxcCp, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, text: *const u8, size: u32, code_page: DxcCp, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, text.as_ptr_or_null(), text.len() as u32, code_page, &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn CreateBlobWithEncodingOnMalloc(&self, text: &[u8], i_malloc: &(impl IMalloc + ?Sized), code_page: DxcCp, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, text: *const u8, i_malloc: VTable, size: u32, code_page: DxcCp, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, text.as_ptr_or_null(), i_malloc.vtable(), text.len() as u32, code_page, &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn CreateIncludeHandler(&self, ) -> Result<(DxcIncludeHandler), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcIncludeHandler> = None;
		let f: extern "system" fn(Param<Self>, _result: &mut Option<DxcIncludeHandler>, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn CreateStreamFromBlobReadOnly(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<(Stream), HResult> {
		let vt = self.as_param();
		let mut _stream: Option<Stream> = None;
		let f: extern "system" fn(Param<Self>, blob: VTable, _stream: &mut Option<Stream>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, blob.vtable(), &mut _stream, );
		if ret.is_ok() {
			if let (Some(_stream)) = (_stream) {
				return Ok((_stream));
			}
		}
		Err(ret)
	}

	fn GetBlobAsUtf8(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, blob: VTable, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, blob.vtable(), &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn GetBlobAsUtf16(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, blob: VTable, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, blob.vtable(), &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
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

