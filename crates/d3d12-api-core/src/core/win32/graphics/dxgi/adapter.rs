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
use crate::core::win32::graphics::dxgi::*;
#[repr(C)]
pub struct DxgiAdapter(pub(crate) DxgiObject);

impl Guid for DxgiAdapter {
	const IID: &'static GUID = &GUID::from_u128(0x2411e7e112ac4ccfbd149798e8534dc0u128);
}

impl Clone for DxgiAdapter {
	fn clone(&self) -> Self { DxgiAdapter(self.0.clone()) }
}

pub trait IDxgiAdapter: IDxgiObject {
	fn as_adapter(&self) -> &DxgiAdapter;
	fn into_adapter(self) -> DxgiAdapter;

	fn EnumOutputs(&self, output: u32, ) -> Result<(DxgiOutput), HResult> {
		let vt = self.as_param();
		let mut _output: Option<DxgiOutput> = None;
		let f: extern "system" fn(Param<Self>, output: u32, _output: &mut Option<DxgiOutput>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, output, &mut _output, );
		if ret.is_ok() {
			if let (Some(_output)) = (_output) {
				return Ok((_output));
			}
		}
		Err(ret)
	}

	fn GetDesc(&self, ) -> Result<(DxgiAdapterDesc), HResult> {
		let vt = self.as_param();
		let mut _desc: DxgiAdapterDesc = DxgiAdapterDesc::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiAdapterDesc, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, &mut _desc, );
		if ret.is_ok() {
			return Ok((_desc));
		}
		Err(ret)
	}

	fn CheckInterfaceSupport(&self, interface_name: &GUID, ) -> Result<(i64), HResult> {
		let vt = self.as_param();
		let mut _umd_version: i64 = i64::zeroed();
		let f: extern "system" fn(Param<Self>, interface_name: &GUID, _umd_version: &mut i64, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, interface_name, &mut _umd_version, );
		if ret.is_ok() {
			return Ok((_umd_version));
		}
		Err(ret)
	}
}

impl IDxgiAdapter for DxgiAdapter {
	fn as_adapter(&self) -> &DxgiAdapter { self }
	fn into_adapter(self) -> DxgiAdapter { self }
}

impl IDxgiObject for DxgiAdapter {
	fn as_object(&self) -> &DxgiObject { &self.0 }
	fn into_object(self) -> DxgiObject { self.0 }
}

impl From<Unknown> for DxgiAdapter {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiAdapter {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

