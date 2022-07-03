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

	fn CreateBlobFromBlob(&self, blob: &(impl IDxcBlob + ?Sized), offset: u32, length: u32, ) -> Result<(DxcBlob), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcBlob> = None;
		let f: extern "system" fn(Param<Self>, blob: VTable, offset: u32, length: u32, _result: &mut Option<DxcBlob>, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, blob.vtable(), offset, length, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn CreateBlobFromPinned(&self, data: &[u8], code_page: DxcCp, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, data: *const u8, size: u32, code_page: DxcCp, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, data.as_ptr_or_null(), data.len() as u32, code_page, &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn MoveToBlob(&self, data: &[u8], i_malloc: &(impl IMalloc + ?Sized), code_page: DxcCp, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, data: *const u8, i_malloc: VTable, size: u32, code_page: DxcCp, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, data.as_ptr_or_null(), i_malloc.vtable(), data.len() as u32, code_page, &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn CreateBlob(&self, data: &[u8], code_page: DxcCp, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, data: *const u8, size: u32, code_page: DxcCp, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, data.as_ptr_or_null(), data.len() as u32, code_page, &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn LoadFile(&self, file_name: &str, code_page: Option<&DxcCp>, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, file_name: *const u16, code_page: Option<&DxcCp>, _blob_encoding: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, file_name.to_utf16().as_ptr_or_null(), code_page, &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn CreateReadOnlyStreamFromBlob(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<(Stream), HResult> {
		let vt = self.as_param();
		let mut _stream: Option<Stream> = None;
		let f: extern "system" fn(Param<Self>, blob: VTable, _stream: &mut Option<Stream>, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, blob.vtable(), &mut _stream, );
		if ret.is_ok() {
			if let (Some(_stream)) = (_stream) {
				return Ok((_stream));
			}
		}
		Err(ret)
	}

	fn CreateDefaultIncludeHandler(&self, ) -> Result<(DxcIncludeHandler), HResult> {
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

	fn GetBlobAsUtf8(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<(DxcBlobUtf8), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobUtf8> = None;
		let f: extern "system" fn(Param<Self>, blob: VTable, _blob_encoding: &mut Option<DxcBlobUtf8>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, blob.vtable(), &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn GetBlobAsUtf16(&self, blob: &(impl IDxcBlob + ?Sized), ) -> Result<(DxcBlobUtf16), HResult> {
		let vt = self.as_param();
		let mut _blob_encoding: Option<DxcBlobUtf16> = None;
		let f: extern "system" fn(Param<Self>, blob: VTable, _blob_encoding: &mut Option<DxcBlobUtf16>, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, blob.vtable(), &mut _blob_encoding, );
		if ret.is_ok() {
			if let (Some(_blob_encoding)) = (_blob_encoding) {
				return Ok((_blob_encoding));
			}
		}
		Err(ret)
	}

	fn GetDxilContainerPart(&self, shader: &DxcBuffer, dxc_part: u32, ) -> Result<(*const c_void, u32), HResult> {
		let vt = self.as_param();
		let mut _part_data: Option<*const c_void> = None;
		let mut _part_size_in_bytes: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, shader: &DxcBuffer, dxc_part: u32, _part_data: &mut Option<*const c_void>, _part_size_in_bytes: &mut u32, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, shader, dxc_part, &mut _part_data, &mut _part_size_in_bytes, );
		if ret.is_ok() {
			if let (Some(_part_data), ) = (_part_data, ) {
				return Ok((_part_data, _part_size_in_bytes));
			}
		}
		Err(ret)
	}

	fn CreateReflection<T: IUnknown>(&self, data: &DxcBuffer, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _reflection: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, data: &DxcBuffer, iid: &GUID, _reflection: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, data, T::IID, &mut _reflection, );
		if ret.is_ok() {
			if let (Some(_reflection)) = (_reflection) {
				return Ok((T::from(_reflection)));
			}
		}
		Err(ret)
	}

	fn BuildArguments(&self, source_name: Option<&str>, entry_point: Option<&str>, target_profile: &str, arguments: Option<&[&str]>, defines: &[DxcDefine], ) -> Result<(DxcCompilerArgs), HResult> {
		let vt = self.as_param();
		let mut _args: Option<DxcCompilerArgs> = None;
		let f: extern "system" fn(Param<Self>, source_name: *const u16, entry_point: *const u16, target_profile: *const u16, arguments: *const PWStr, arg_count: u32, defines: *const DxcDefine, define_count: u32, _args: &mut Option<DxcCompilerArgs>, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, source_name.map(str::to_utf16).as_ptr_or_null(), entry_point.map(str::to_utf16).as_ptr_or_null(), target_profile.to_utf16().as_ptr_or_null(), arguments.to_utf16_vec().ptr(), arguments.len() as u32, defines.as_ptr_or_null(), defines.len() as u32, &mut _args, );
		if ret.is_ok() {
			if let (Some(_args)) = (_args) {
				return Ok((_args));
			}
		}
		Err(ret)
	}

	fn GetPDBContents(&self, pdb_blob: &(impl IDxcBlob + ?Sized), ) -> Result<(DxcBlob, DxcBlob), HResult> {
		let vt = self.as_param();
		let mut _hash: Option<DxcBlob> = None;
		let mut _container: Option<DxcBlob> = None;
		let f: extern "system" fn(Param<Self>, pdb_blob: VTable, _hash: &mut Option<DxcBlob>, _container: &mut Option<DxcBlob>, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, pdb_blob.vtable(), &mut _hash, &mut _container, );
		if ret.is_ok() {
			if let (Some(_hash), Some(_container)) = (_hash, _container) {
				return Ok((_hash, _container));
			}
		}
		Err(ret)
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
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

