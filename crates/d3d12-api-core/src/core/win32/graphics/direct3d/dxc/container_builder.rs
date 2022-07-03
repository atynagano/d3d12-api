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
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dxil_container_header: VTable, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, dxil_container_header.vtable(), );
		ret.ok()
	}

	fn AddPart(&self, four_cc: u32, source: &(impl IDxcBlob + ?Sized), ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, four_cc: u32, source: VTable, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, four_cc, source.vtable(), );
		ret.ok()
	}

	fn RemovePart(&self, four_cc: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, four_cc: u32, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, four_cc, );
		ret.ok()
	}

	fn SerializeContainer(&self, ) -> Result<(DxcOperationResult), HResult> {
		let vt = self.as_param();
		let mut _result: Option<DxcOperationResult> = None;
		let f: extern "system" fn(Param<Self>, _result: &mut Option<DxcOperationResult>, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, &mut _result, );
		if ret.is_ok() {
			if let (Some(_result)) = (_result) {
				return Ok((_result));
			}
		}
		Err(ret)
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

