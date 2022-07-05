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

#[repr(C)]
pub struct DxcPdbUtils(pub(crate) Unknown);

impl Guid for DxcPdbUtils {
	const IID: &'static GUID = &GUID::from_u128(0xe6c9647e9d6a4c3bb94c524b5a6c343du128);
}

impl Clone for DxcPdbUtils {
	fn clone(&self) -> Self { DxcPdbUtils(self.0.clone()) }
}

pub trait IDxcPdbUtils: IUnknown {
	fn as_pdb_utils(&self) -> &DxcPdbUtils;
	fn into_pdb_utils(self) -> DxcPdbUtils;

	fn Load(&self, pdb_or_dxil: &(impl IDxcBlob + ?Sized), ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, pdb_or_dxil: VTable, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, pdb_or_dxil.vtable(), );
			_ret_.ok()
		}
	}

	fn GetSourceCount(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_count: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_count: *mut u32, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, _out_count.as_mut_ptr(), );
			Ok(_out_count.assume_init())
		}
	}

	fn GetSource(&self, index: u32, ) -> Result<DxcBlobEncoding, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcBlobEncoding> = None;
			let f: extern "system" fn(Param<Self>, index: u32, result: *mut c_void, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, index, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetSourceName(&self, index: u32, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, index: u32, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, index, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetFlagCount(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_count: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_count: *mut u32, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, _out_count.as_mut_ptr(), );
			Ok(_out_count.assume_init())
		}
	}

	fn GetFlag(&self, index: u32, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, index: u32, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, index, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetArgCount(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_count: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_count: *mut u32, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, _out_count.as_mut_ptr(), );
			Ok(_out_count.assume_init())
		}
	}

	fn GetArg(&self, index: u32, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, index: u32, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, index, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetArgPairCount(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_count: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_count: *mut u32, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, _out_count.as_mut_ptr(), );
			Ok(_out_count.assume_init())
		}
	}

	fn GetDefineCount(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_count: MaybeUninit<u32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_count: *mut u32, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, _out_count.as_mut_ptr(), );
			Ok(_out_count.assume_init())
		}
	}

	fn GetDefine(&self, index: u32, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, index: u32, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, index, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetTargetProfile(&self, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetEntryPoint(&self, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetMainFileName(&self, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetHash(&self, ) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, result: *mut c_void, ) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn GetName(&self, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, _out_result: *mut c_void, ) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn IsFullPDB(&self, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Bool
				= transmute(vt[20]);
			let _ret_ = f(vt, );
			_ret_.to_bool()
		}
	}

	fn GetFullPDB(&self, ) -> Result<DxcBlob, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_full_pdb: Option<DxcBlob> = None;
			let f: extern "system" fn(Param<Self>, full_pdb: *mut c_void, ) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, transmute(&mut _out_full_pdb), );
			if _ret_.is_ok() {
				if let Some(_out_full_pdb) = _out_full_pdb {
					return Ok(_out_full_pdb);
				}
			}
			Err(_ret_)
		}
	}

	fn GetVersionInfo(&self, ) -> Result<DxcVersionInfo, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_version_info: Option<DxcVersionInfo> = None;
			let f: extern "system" fn(Param<Self>, version_info: *mut c_void, ) -> HResult
				= transmute(vt[22]);
			let _ret_ = f(vt, transmute(&mut _out_version_info), );
			if _ret_.is_ok() {
				if let Some(_out_version_info) = _out_version_info {
					return Ok(_out_version_info);
				}
			}
			Err(_ret_)
		}
	}

	fn SetCompiler(&self, compiler: &(impl IDxcCompiler3 + ?Sized), ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, compiler: VTable, ) -> HResult
				= transmute(vt[23]);
			let _ret_ = f(vt, compiler.vtable(), );
			_ret_.ok()
		}
	}

	fn CompileForFullPDB(&self, ) -> Result<DxcResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcResult> = None;
			let f: extern "system" fn(Param<Self>, result: *mut c_void, ) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}

	fn OverrideArgs(&self, arg_pairs: &DxcArgPair, num_arg_pairs: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, arg_pairs: &DxcArgPair, num_arg_pairs: u32, ) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, arg_pairs, num_arg_pairs, );
			_ret_.ok()
		}
	}

	fn OverrideRootSignature(&self, root_signature: &str, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, root_signature: *const u16, ) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, root_signature.to_utf16().as_ptr_or_null(), );
			_ret_.ok()
		}
	}
}

impl IDxcPdbUtils for DxcPdbUtils {
	fn as_pdb_utils(&self) -> &DxcPdbUtils { self }
	fn into_pdb_utils(self) -> DxcPdbUtils { self }
}

impl From<Unknown> for DxcPdbUtils {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcPdbUtils {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

