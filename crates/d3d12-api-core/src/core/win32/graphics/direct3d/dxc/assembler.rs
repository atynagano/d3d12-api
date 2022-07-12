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
pub struct DxcAssembler(pub(crate) Unknown);

impl Guid for DxcAssembler {
	const IID: &'static GUID = &GUID::from_u128(0x091f7a261c1f4948904be6e3a8a771d5u128);
}

impl Clone for DxcAssembler {
	fn clone(&self) -> Self { DxcAssembler(self.0.clone()) }
}

pub trait IDxcAssembler: IUnknown {
	fn as_assembler(&self) -> &DxcAssembler;
	fn into_assembler(self) -> DxcAssembler;

	fn AssembleToContainer(&self, shader: &(impl IDxcBlob + ?Sized), ) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, shader: VTable, result: *mut c_void, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, shader.vtable(), transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcAssembler for DxcAssembler {
	fn as_assembler(&self) -> &DxcAssembler { self }
	fn into_assembler(self) -> DxcAssembler { self }
}

impl From<Unknown> for DxcAssembler {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcAssembler {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

