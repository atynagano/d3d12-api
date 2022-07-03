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
pub struct DxcValidator(pub(crate) Unknown);

impl Guid for DxcValidator {
	const IID: &'static GUID = &GUID::from_u128(0xa6e82bd21fd7482698112857e797f49au128);
}

impl Clone for DxcValidator {
	fn clone(&self) -> Self { DxcValidator(self.0.clone()) }
}

pub trait IDxcValidator: IUnknown {
	fn as_validator(&self) -> &DxcValidator;
	fn into_validator(self) -> DxcValidator;

	fn Validate(&self, shader: &(impl IDxcBlob + ?Sized), flags: u32, ) -> Result<(DxcOperationResult), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcOperationResult> = None;
		let f: extern "system" fn(Param<Self>, shader: VTable, flags: u32, _result: &mut Option<DxcOperationResult>, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, shader.vtable(), flags, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
	}
}

impl IDxcValidator for DxcValidator {
	fn as_validator(&self) -> &DxcValidator { self }
	fn into_validator(self) -> DxcValidator { self }
}

impl From<Unknown> for DxcValidator {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcValidator {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

