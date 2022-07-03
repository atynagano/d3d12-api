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
use crate::core::win32::system::com::*;
#[repr(C)]
pub struct DxgiDevice(pub(crate) DxgiObject);

impl Guid for DxgiDevice {
	const IID: &'static GUID = &GUID::from_u128(0x54ec77fa137744e68c3288fd5f44c84cu128);
}

impl Clone for DxgiDevice {
	fn clone(&self) -> Self { DxgiDevice(self.0.clone()) }
}

pub trait IDxgiDevice: IDxgiObject {
	fn as_device(&self) -> &DxgiDevice;
	fn into_device(self) -> DxgiDevice;

	fn GetAdapter(&self, ) -> Result<(DxgiAdapter), HResult> {
		let vt = self.as_param();
		let mut _adapter: Option<DxgiAdapter> = None;
		let f: extern "system" fn(Param<Self>, _adapter: &mut Option<DxgiAdapter>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, &mut _adapter, );
		if ret.is_ok() {
			if let (Some(_adapter)) = (_adapter) {
				return Ok((_adapter));
			}
		}
		Err(ret)
	}

	fn CreateSurface(&self, desc: &DxgiSurfaceDesc, usage: u32, shared_resource: Option<&DxgiSharedResource>, mut surface: &mut [Param<DxgiSurface>], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, desc: &DxgiSurfaceDesc, num_surfaces: u32, usage: u32, shared_resource: Option<&DxgiSharedResource>, surface: *mut Param<DxgiSurface>, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, desc, surface.len() as u32, usage, shared_resource, surface.as_mut_ptr_or_null(), );
		ret.ok()
	}

	fn QueryResourceResidency(&self, resources: &[Param<Unknown>], mut residency_status: &mut [DxgiResidency], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resources: *const Param<Unknown>, residency_status: *mut DxgiResidency, num_resources: u32, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, resources.as_ptr_or_null(), residency_status.as_mut_ptr_or_null(), resources.len() as u32, );
		ret.ok()
	}

	fn SetGPUThreadPriority(&self, priority: i32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, priority: i32, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, priority, );
		ret.ok()
	}

	fn GetGPUThreadPriority(&self, ) -> Result<(i32), HResult> {
		let vt = self.as_param();
		let mut _priority: i32 = i32::zeroed();
		let f: extern "system" fn(Param<Self>, _priority: &mut i32, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, &mut _priority, );
		if ret.is_ok() {
			return Ok((_priority));
		}
		Err(ret)
	}
}

impl IDxgiDevice for DxgiDevice {
	fn as_device(&self) -> &DxgiDevice { self }
	fn into_device(self) -> DxgiDevice { self }
}

impl IDxgiObject for DxgiDevice {
	fn as_object(&self) -> &DxgiObject { &self.0 }
	fn into_object(self) -> DxgiObject { self.0 }
}

impl From<Unknown> for DxgiDevice {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiDevice {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

