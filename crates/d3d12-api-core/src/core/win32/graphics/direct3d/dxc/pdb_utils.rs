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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcPdbUtils(pub(crate) Unknown);

impl Deref for DxcPdbUtils {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcPdbUtils {
	const IID: &'static GUID = &GUID::from_u128(0xe6c9647e9d6a4c3bb94c524b5a6c343du128);
}

impl Com for DxcPdbUtils {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcPdbUtils {
	pub fn Load(&self, pdb_or_dxil: &DxcBlob) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, pdb_or_dxil.vtable());
			_ret_.ok()
		}
	}

	pub fn GetSourceCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetSource(&self, u_index: u32) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, u_index, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetSourceName(&self, u_index: u32) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, u_index, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetFlagCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetFlag(&self, u_index: u32) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, u_index, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetArgCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetArg(&self, u_index: u32) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, u_index, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetArgPairCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetArgPair(&self) { todo!() }

	pub fn GetDefineCount(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut count_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, count_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(count_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetDefine(&self, u_index: u32) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, u_index, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetTargetProfile(&self) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetEntryPoint(&self) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetMainFileName(&self) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetHash(&self) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetName(&self) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn IsFullPDB(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[20]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}

	pub fn GetFullPDB(&self) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut full_pdb_out_: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, transmute(&mut full_pdb_out_));
			if _ret_.is_ok() { if let Some(full_pdb_out_) = full_pdb_out_ { return Ok(full_pdb_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetVersionInfo(&self) -> Result<DxcVersionInfo, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut version_info_out_: Option<DxcVersionInfo> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, transmute(&mut version_info_out_));
			if _ret_.is_ok() { if let Some(version_info_out_) = version_info_out_ { return Ok(version_info_out_); } }
			Err(_ret_)
		}
	}

	pub fn SetCompiler(&self, compiler: &DxcCompiler3) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[23]);
			let _ret_ = f(vt, compiler.vtable());
			_ret_.ok()
		}
	}

	pub fn CompileForFullPDB(&self) -> Result<DxcResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcResult> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}

	pub fn OverrideArgs(&self, arg_pairs: &DxcArgPair, u_num_arg_pairs: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &DxcArgPair, u32) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, arg_pairs, u_num_arg_pairs);
			_ret_.ok()
		}
	}

	pub fn OverrideRootSignature(&self, root_signature: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, root_signature.to_utf16().as_ptr_or_null());
			_ret_.ok()
		}
	}
}

pub trait IDxcPdbUtils: IUnknown {
	fn as_pdb_utils(&self) -> &DxcPdbUtils;
	fn into_pdb_utils(self) -> DxcPdbUtils;
}

impl IDxcPdbUtils for DxcPdbUtils {
	fn as_pdb_utils(&self) -> &DxcPdbUtils { self }
	fn into_pdb_utils(self) -> DxcPdbUtils { self }
}
impl IUnknown for DxcPdbUtils {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcPdbUtils {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

