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
pub struct DxcValidator2(pub(crate) DxcValidator);

impl Deref for DxcValidator2 {
	type Target = DxcValidator;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcValidator2 {
	const IID: &'static GUID = &GUID::from_u128(0x458e1fd1b1b24750a6e19c10f03bed92u128);
}

impl Com for DxcValidator2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcValidator2 {
	pub fn ValidateWithDebug(&self, shader: &DxcBlob, flags: u32, opt_debug_bitcode: Option<&DxcBuffer>) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, VTable, u32, *const c_void, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, shader.vtable(), flags, transmute(opt_debug_bitcode), transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcValidator2: IDxcValidator {
	fn as_validator2(&self) -> &DxcValidator2;
	fn into_validator2(self) -> DxcValidator2;
}

impl IDxcValidator2 for DxcValidator2 {
	fn as_validator2(&self) -> &DxcValidator2 { self }
	fn into_validator2(self) -> DxcValidator2 { self }
}
impl IDxcValidator for DxcValidator2 {
	fn as_validator(&self) -> &DxcValidator { &self.0.as_validator() }
	fn into_validator(self) -> DxcValidator { self.0.into_validator() }
}

impl IUnknown for DxcValidator2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcValidator2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxcValidator::from(v))
    }
}

