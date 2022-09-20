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
use crate::core::win32::graphics::direct3d::dxc::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcUtils(pub(crate) Unknown);

impl Deref for DxcUtils {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcUtils {
	const IID: &'static GUID = &GUID::from_u128(0x4605c4cb2019492aada465f20bb7d67fu128);
}

impl Com for DxcUtils {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcUtils {
	pub fn CreateBlobFromBlob(&self, blob: &DxcBlob, offset: u32, length: u32) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, VTable, u32, u32, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, blob.vtable(), offset, length, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBlobFromPinned(&self, data: &[u8], code_page: DxcCp) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *const u8, u32, DxcCp, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, data_ptr_, data_len_ as u32, code_page, transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn MoveToBlob(&self, data: &[u8], i_malloc: &Malloc, code_page: DxcCp) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *const u8, VTable, u32, DxcCp, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, data_ptr_, i_malloc.vtable(), data_len_ as u32, code_page, transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBlob(&self, data: &[u8], code_page: DxcCp) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *const u8, u32, DxcCp, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, data_ptr_, data_len_ as u32, code_page, transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn LoadFile(&self, file_name: &str, code_page: Option<&DxcCp>) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut blob_encoding_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *const c_void, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, file_name.to_utf16().as_ptr_or_null(), transmute(code_page), transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateReadOnlyStreamFromBlob(&self, blob: &DxcBlob) -> Result<Stream, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut stream_out_: Option<Stream> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut stream_out_));
			if _ret_.is_ok() { if let Some(stream_out_) = stream_out_ { return Ok(stream_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateDefaultIncludeHandler(&self) -> Result<DxcIncludeHandler, HResult> {
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

	pub fn GetBlobAsUtf8(&self, blob: &DxcBlob) -> Result<DxcBlobUtf8, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut blob_encoding_out_: Option<DxcBlobUtf8> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetBlobAsUtf16(&self, blob: &DxcBlob) -> Result<DxcBlobUtf16, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut blob_encoding_out_: Option<DxcBlobUtf16> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut blob_encoding_out_));
			if _ret_.is_ok() { if let Some(blob_encoding_out_) = blob_encoding_out_ { return Ok(blob_encoding_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn GetDxilContainerPart(&self) { todo!() }

	pub unsafe fn CreateReflection(&self) { todo!() }

	pub fn BuildArguments(&self, source_name: Option<&str>, entry_point: Option<&str>, target_profile: &str, arguments: Option<&[&str]>, defines: &[DxcDefine]) -> Result<DxcCompilerArgs, HResult> {
		unsafe {
			let vt = self.as_param();
			let (arguments_ptr_, arguments_len_) = arguments.deconstruct();
			let (defines_ptr_, defines_len_) = defines.deconstruct();
			let mut args_out_: Option<DxcCompilerArgs> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *const u16, *const u16, *const PWStr, u32, *const DxcDefine, u32, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, source_name.map(str::to_utf16).as_ptr_or_null(), entry_point.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), arguments.to_utf16_vec().ptr(), arguments_len_ as u32, defines_ptr_, defines_len_ as u32, transmute(&mut args_out_));
			if _ret_.is_ok() { if let Some(args_out_) = args_out_ { return Ok(args_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn GetPDBContents(&self) { todo!() }
}

pub trait IDxcUtils: IUnknown {
	fn as_utils(&self) -> &DxcUtils;
	fn into_utils(self) -> DxcUtils;
}

impl IDxcUtils for DxcUtils {
	fn as_utils(&self) -> &DxcUtils { self }
	fn into_utils(self) -> DxcUtils { self }
}
impl IUnknown for DxcUtils {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcUtils {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

