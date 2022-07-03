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
pub struct DxcValidator2(pub(crate) DxcValidator);

impl Guid for DxcValidator2 {
	const IID: &'static GUID = &GUID::from_u128(0x458e1fd1b1b24750a6e19c10f03bed92u128);
}

impl Clone for DxcValidator2 {
	fn clone(&self) -> Self { DxcValidator2(self.0.clone()) }
}

pub trait IDxcValidator2: IDxcValidator {
	fn as_validator2(&self) -> &DxcValidator2;
	fn into_validator2(self) -> DxcValidator2;

	fn ValidateWithDebug(&self, shader: &(impl IDxcBlob + ?Sized), flags: u32, opt_debug_bitcode: Option<&DxcBuffer>, ) -> Result<(DxcOperationResult), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcOperationResult> = None;
		let f: extern "system" fn(Param<Self>, shader: VTable, flags: u32, opt_debug_bitcode: Option<&DxcBuffer>, _result: &mut Option<DxcOperationResult>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, shader.vtable(), flags, opt_debug_bitcode, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}
}

impl IDxcValidator2 for DxcValidator2 {
	fn as_validator2(&self) -> &DxcValidator2 { self }
	fn into_validator2(self) -> DxcValidator2 { self }
}

impl IDxcValidator for DxcValidator2 {
	fn as_validator(&self) -> &DxcValidator { &self.0 }
	fn into_validator(self) -> DxcValidator { self.0 }
}

impl From<Unknown> for DxcValidator2 {
    fn from(v: Unknown) -> Self {
        Self(DxcValidator::from(v))
    }
}

impl IUnknown for DxcValidator2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

