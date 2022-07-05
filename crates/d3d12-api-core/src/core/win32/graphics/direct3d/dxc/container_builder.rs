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
pub struct DxcContainerBuilder(pub(crate) Unknown);

impl Guid for DxcContainerBuilder {
	const IID: &'static GUID = &GUID::from_u128(0x334b1f5022924b3599a125588d8c17feu128);
}

impl Clone for DxcContainerBuilder {
	fn clone(&self) -> Self { DxcContainerBuilder(self.0.clone()) }
}

pub trait IDxcContainerBuilder: IUnknown {
	fn as_container_builder(&self) -> &DxcContainerBuilder;
	fn into_container_builder(self) -> DxcContainerBuilder;

	fn Load(&self, dxil_container_header: &(impl IDxcBlob + ?Sized), ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, dxil_container_header: VTable, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, dxil_container_header.vtable(), );
			_ret_.ok()
		}
	}

	fn AddPart(&self, four_cc: u32, source: &(impl IDxcBlob + ?Sized), ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, four_cc: u32, source: VTable, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, four_cc, source.vtable(), );
			_ret_.ok()
		}
	}

	fn RemovePart(&self, four_cc: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, four_cc: u32, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, four_cc, );
			_ret_.ok()
		}
	}

	fn SerializeContainer(&self, ) -> Result<DxcOperationResult, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_result: Option<DxcOperationResult> = None;
			let f: extern "system" fn(Param<Self>, result: *mut c_void, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, transmute(&mut _out_result), );
			if _ret_.is_ok() {
				if let Some(_out_result) = _out_result {
					return Ok(_out_result);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxcContainerBuilder for DxcContainerBuilder {
	fn as_container_builder(&self) -> &DxcContainerBuilder { self }
	fn into_container_builder(self) -> DxcContainerBuilder { self }
}

impl From<Unknown> for DxcContainerBuilder {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcContainerBuilder {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

