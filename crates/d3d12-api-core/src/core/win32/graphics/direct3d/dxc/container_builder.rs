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
pub struct DxcContainerBuilder(pub(crate) Unknown);

impl Deref for DxcContainerBuilder {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcContainerBuilder {
	const IID: &'static GUID = &GUID::from_u128(0x334b1f5022924b3599a125588d8c17feu128);
}

impl Com for DxcContainerBuilder {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcContainerBuilder {
	pub fn Load(&self, dxil_container_header: &DxcBlob) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, dxil_container_header.vtable());
			_ret_.ok()
		}
	}

	pub fn AddPart(&self, four_cc: u32, source: &DxcBlob) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, VTable) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, four_cc, source.vtable());
			_ret_.ok()
		}
	}

	pub fn RemovePart(&self, four_cc: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, four_cc);
			_ret_.ok()
		}
	}

	pub fn SerializeContainer(&self) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(result_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcContainerBuilder: IUnknown {
	fn as_container_builder(&self) -> &DxcContainerBuilder;
	fn into_container_builder(self) -> DxcContainerBuilder;
}

impl IDxcContainerBuilder for DxcContainerBuilder {
	fn as_container_builder(&self) -> &DxcContainerBuilder { self }
	fn into_container_builder(self) -> DxcContainerBuilder { self }
}
impl IUnknown for DxcContainerBuilder {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcContainerBuilder {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

