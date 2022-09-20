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
pub struct DxcAssembler(pub(crate) Unknown);

impl Deref for DxcAssembler {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcAssembler {
	const IID: &'static GUID = &GUID::from_u128(0x091f7a261c1f4948904be6e3a8a771d5u128);
}

impl Com for DxcAssembler {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcAssembler {
	pub fn AssembleToContainer(&self, shader: &DxcBlob) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, shader.vtable(), transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcAssembler: IUnknown {
	fn as_assembler(&self) -> &DxcAssembler;
	fn into_assembler(self) -> DxcAssembler;
}

impl IDxcAssembler for DxcAssembler {
	fn as_assembler(&self) -> &DxcAssembler { self }
	fn into_assembler(self) -> DxcAssembler { self }
}
impl IUnknown for DxcAssembler {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcAssembler {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

