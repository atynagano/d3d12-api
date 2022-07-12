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
use crate::core::win32::graphics::direct3d::dxc::*;
use crate::core::win32::system::com::*;

#[repr(C)]
pub struct DxcUtils(pub(crate) Unknown);

impl Guid for DxcUtils {
	const IID: &'static GUID = &GUID::from_u128(0x4605c4cb2019492aada465f20bb7d67fu128);
}

impl Clone for DxcUtils {
	fn clone(&self) -> Self { DxcUtils(self.0.clone()) }
}

pub trait IDxcUtils: IUnknown {
	fn as_utils(&self) -> &DxcUtils;
	fn into_utils(self) -> DxcUtils;

	fn CreateBlobFromBlob(&self, blob: &(impl IDxcBlob + ?Sized), offset: u32, length: u32, ) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, blob: VTable, offset: u32, length: u32, result: *mut c_void, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, blob.vtable(), offset, length, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn LoadFile(&self, file_name: &str, code_page: Option<&DxcCp>, ) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_blob_encoding: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, file_name: *const u16, code_page: *const c_void, blob_encoding: *mut c_void, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, file_name.to_utf16().as_ptr_or_null(), transmute(code_page), transmute(&mut _out_blob_encoding), );
			if _ret_.is_ok() {
				if let Some(_out_blob_encoding) = _out_blob_encoding {
					return Ok(_out_blob_encoding);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateReadOnlyStreamFromBlob(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<Stream, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_stream: Option<Stream> = None;
			let f: extern "system" fn(Param<Self>, blob: VTable, stream: *mut c_void, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut _out_stream), );
			if _ret_.is_ok() {
				if let Some(_out_stream) = _out_stream {
					return Ok(_out_stream);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateDefaultIncludeHandler(&self, ) -> Result<DxcIncludeHandler, HResult> {
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

	fn GetBlobAsUtf8(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<DxcBlobUtf8, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_blob_encoding: Option<DxcBlobUtf8> = None;
			let f: extern "system" fn(Param<Self>, blob: VTable, blob_encoding: *mut c_void, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, blob.vtable(), transmute(&mut _out_blob_encoding), );
			if _ret_.is_ok() {
				if let Some(_out_blob_encoding) = _out_blob_encoding {
					return Ok(_out_blob_encoding);
				}
			}
			Err(_ret_)
		}
	}

	fn GetBlobAsUtf16(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<DxcBlobUtf16, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_blob_encoding: Option<DxcBlobUtf16> = None;
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

	fn BuildArguments(&self, source_name: Option<&str>, entry_point: Option<&str>, target_profile: &str, arguments: Option<&[&str]>, defines: &[DxcDefine], ) -> Result<DxcCompilerArgs, HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_arguments, _len_arguments) = arguments.deconstruct();
			let (_ptr_defines, _len_defines) = defines.deconstruct();
			let mut _out_args: Option<DxcCompilerArgs> = None;
			let f: extern "system" fn(Param<Self>, source_name: *const u16, entry_point: *const u16, target_profile: *const u16, arguments: *const PWStr, arg_count: u32, defines: *const DxcDefine, define_count: u32, args: *mut c_void, ) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, source_name.map(str::to_utf16).as_ptr_or_null(), entry_point.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), arguments.to_utf16_vec().ptr(), _len_arguments as u32, _ptr_defines, _len_defines as u32, transmute(&mut _out_args), );
			if _ret_.is_ok() {
				if let Some(_out_args) = _out_args {
					return Ok(_out_args);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcUtils for DxcUtils {
	fn as_utils(&self) -> &DxcUtils { self }
	fn into_utils(self) -> DxcUtils { self }
}

impl From<Unknown> for DxcUtils {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcUtils {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

