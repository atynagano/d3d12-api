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
pub struct DxgiSurface(pub(crate) DxgiDeviceSubObject);

impl Guid for DxgiSurface {
	const IID: &'static GUID = &GUID::from_u128(0xcafcb56c6ac34889bf479e23bbd260ecu128);
}

impl Clone for DxgiSurface {
	fn clone(&self) -> Self { DxgiSurface(self.0.clone()) }
}

pub trait IDxgiSurface: IDxgiDeviceSubObject {
	fn as_surface(&self) -> &DxgiSurface;
	fn into_surface(self) -> DxgiSurface;

	fn GetDesc(&self, ) -> Result<(DxgiSurfaceDesc), HResult> {
		let vt = self.as_param();
		let mut _desc: DxgiSurfaceDesc = DxgiSurfaceDesc::zeroed();
		let f: extern "system" fn(Param<Self>, _desc: &mut DxgiSurfaceDesc, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, &mut _desc, );
		if ret.is_ok() {
			return Ok((_desc));
		}
		Err(ret)
	}

	fn Map(&self, map_flags: u32, ) -> Result<(DxgiMappedRect), HResult> {
		let vt = self.as_param();
		let mut _locked_rect: DxgiMappedRect = DxgiMappedRect::zeroed();
		let f: extern "system" fn(Param<Self>, _locked_rect: &mut DxgiMappedRect, map_flags: u32, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, &mut _locked_rect, map_flags, );
		if ret.is_ok() {
			return Ok((_locked_rect));
		}
		Err(ret)
	}

	fn Unmap(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, );
		ret.ok()
	}
}

impl IDxgiSurface for DxgiSurface {
	fn as_surface(&self) -> &DxgiSurface { self }
	fn into_surface(self) -> DxgiSurface { self }
}

impl IDxgiDeviceSubObject for DxgiSurface {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0 }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0 }
}

impl IDxgiObject for DxgiSurface {
	fn as_object(&self) -> &DxgiObject { &self.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0 }
}

impl From<Unknown> for DxgiSurface {
    fn from(v: Unknown) -> Self {
        Self(DxgiDeviceSubObject::from(v))
    }
}

impl IUnknown for DxgiSurface {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

