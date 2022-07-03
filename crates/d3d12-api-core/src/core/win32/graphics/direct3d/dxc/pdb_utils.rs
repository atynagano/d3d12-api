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
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, pdb_or_dxil: VTable, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, pdb_or_dxil.vtable(), );
		ret.ok()
	}

	fn GetSourceCount(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _count: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _count: &mut u32, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, &mut _count, );
		if ret.is_ok() {
			return Ok((_count));
		}
		Err(ret)
	}

	fn GetSource(&self, index: u32, ) -> Result<(DxcBlobEncoding), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcBlobEncoding> = None;
		let f: extern "system" fn(Param<Self>, index: u32, _result: &mut Option<DxcBlobEncoding>, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, index, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn GetSourceName(&self, index: u32, ) -> Result<(BStr), HResult> {
		let vt = self.as_param();
		let mut _result: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, index: u32, _result: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, index, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetFlagCount(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _count: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _count: &mut u32, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, &mut _count, );
		if ret.is_ok() {
			return Ok((_count));
		}
		Err(ret)
	}

	fn GetFlag(&self, index: u32, ) -> Result<(BStr), HResult> {
		let vt = self.as_param();
		let mut _result: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, index: u32, _result: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, index, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetArgCount(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _count: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _count: &mut u32, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, &mut _count, );
		if ret.is_ok() {
			return Ok((_count));
		}
		Err(ret)
	}

	fn GetArg(&self, index: u32, ) -> Result<(BStr), HResult> {
		let vt = self.as_param();
		let mut _result: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, index: u32, _result: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, index, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetArgPairCount(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _count: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _count: &mut u32, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, &mut _count, );
		if ret.is_ok() {
			return Ok((_count));
		}
		Err(ret)
	}

	fn GetArgPair(&self, index: u32, ) -> Result<(BStr, BStr), HResult> {
		let vt = self.as_param();
		let mut _name: BStr = BStr::zeroed();
		let mut _value: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, index: u32, _name: &mut BStr, _value: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, index, &mut _name, &mut _value, );
		if ret.is_ok() {
			return Ok((_name, _value));
		}
		Err(ret)
	}

	fn GetDefineCount(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _count: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _count: &mut u32, ) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, &mut _count, );
		if ret.is_ok() {
			return Ok((_count));
		}
		Err(ret)
	}

	fn GetDefine(&self, index: u32, ) -> Result<(BStr), HResult> {
		let vt = self.as_param();
		let mut _result: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, index: u32, _result: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, index, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetTargetProfile(&self, ) -> Result<(BStr), HResult> {
		let vt = self.as_param();
		let mut _result: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, _result: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetEntryPoint(&self, ) -> Result<(BStr), HResult> {
		let vt = self.as_param();
		let mut _result: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, _result: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetMainFileName(&self, ) -> Result<(BStr), HResult> {
		let vt = self.as_param();
		let mut _result: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, _result: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn GetHash(&self, ) -> Result<(DxcBlob), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcBlob> = None;
		let f: extern "system" fn(Param<Self>, _result: &mut Option<DxcBlob>, ) -> HResult
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn GetName(&self, ) -> Result<(BStr), HResult> {
		let vt = self.as_param();
		let mut _result: BStr = BStr::zeroed();
		let f: extern "system" fn(Param<Self>, _result: &mut BStr, ) -> HResult
			= unsafe { transmute(vt[19]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			return Ok((_result));
		}
		Err(ret)
	}

	fn IsFullPDB(&self, ) -> (bool) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Bool
			= unsafe { transmute(vt[20]) };
		let ret = f(vt, );
		return (ret.to_bool());
	}

	fn GetFullPDB(&self, ) -> Result<(DxcBlob), HResult> {
		let vt = self.as_param();
		let mut _full_pdb: Option<DxcBlob> = None;
		let f: extern "system" fn(Param<Self>, _full_pdb: &mut Option<DxcBlob>, ) -> HResult
			= unsafe { transmute(vt[21]) };
		let ret = f(vt, &mut _full_pdb, );
		if ret.is_ok() {
			if let (Some(_full_pdb)) = (_full_pdb) {
				return Ok((_full_pdb));
			}
		}
		Err(ret)
	}

	fn GetVersionInfo(&self, ) -> Result<(DxcVersionInfo), HResult> {
		let vt = self.as_param();
		let mut _version_info: Option<DxcVersionInfo> = None;
		let f: extern "system" fn(Param<Self>, _version_info: &mut Option<DxcVersionInfo>, ) -> HResult
			= unsafe { transmute(vt[22]) };
		let ret = f(vt, &mut _version_info, );
		if ret.is_ok() {
			if let (Some(_version_info)) = (_version_info) {
				return Ok((_version_info));
			}
		}
		Err(ret)
	}

	fn SetCompiler(&self, compiler: &(impl IDxcCompiler3 + ?Sized), ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, compiler: VTable, ) -> HResult
			= unsafe { transmute(vt[23]) };
		let ret = f(vt, compiler.vtable(), );
		ret.ok()
	}

	fn CompileForFullPDB(&self, ) -> Result<(DxcResult), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcResult> = None;
		let f: extern "system" fn(Param<Self>, _result: &mut Option<DxcResult>, ) -> HResult
			= unsafe { transmute(vt[24]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}

	fn OverrideArgs(&self, arg_pairs: &DxcArgPair, num_arg_pairs: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, arg_pairs: &DxcArgPair, num_arg_pairs: u32, ) -> HResult
			= unsafe { transmute(vt[25]) };
		let ret = f(vt, arg_pairs, num_arg_pairs, );
		ret.ok()
	}

	fn OverrideRootSignature(&self, root_signature: &str, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_signature: *const u16, ) -> HResult
			= unsafe { transmute(vt[26]) };
		let ret = f(vt, root_signature.to_utf16().as_ptr_or_null(), );
		ret.ok()
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

