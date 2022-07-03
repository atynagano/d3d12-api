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
pub struct DxgiResource(pub(crate) DxgiDeviceSubObject);

impl Guid for DxgiResource {
	const IID: &'static GUID = &GUID::from_u128(0x035f3ab4482e4e50b41f8a7f8bd8960bu128);
}

impl Clone for DxgiResource {
	fn clone(&self) -> Self { DxgiResource(self.0.clone()) }
}

pub trait IDxgiResource: IDxgiDeviceSubObject {
	fn as_resource(&self) -> &DxgiResource;
	fn into_resource(self) -> DxgiResource;

	fn GetSharedHandle(&self, ) -> Result<(Handle), HResult> {
		let vt = self.as_param();
		let mut _shared_handle: Handle = Handle::zeroed();
		let f: extern "system" fn(Param<Self>, _shared_handle: &mut Handle, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, &mut _shared_handle, );
		if ret.is_ok() {
			return Ok((_shared_handle));
		}
		Err(ret)
	}

	fn GetUsage(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _usage: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _usage: &mut u32, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, &mut _usage, );
		if ret.is_ok() {
			return Ok((_usage));
		}
		Err(ret)
	}

	fn SetEvictionPriority(&self, eviction_priority: DxgiResourcePriority, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, eviction_priority: DxgiResourcePriority, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, eviction_priority, );
		ret.ok()
	}

	fn GetEvictionPriority(&self, ) -> Result<(u32), HResult> {
		let vt = self.as_param();
		let mut _eviction_priority: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, _eviction_priority: &mut u32, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, &mut _eviction_priority, );
		if ret.is_ok() {
			return Ok((_eviction_priority));
		}
		Err(ret)
	}
}

impl IDxgiResource for DxgiResource {
	fn as_resource(&self) -> &DxgiResource { self }
	fn into_resource(self) -> DxgiResource { self }
}

impl IDxgiDeviceSubObject for DxgiResource {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0 }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0 }
}

impl IDxgiObject for DxgiResource {
	fn as_object(&self) -> &DxgiObject { &self.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0 }
}

impl From<Unknown> for DxgiResource {
    fn from(v: Unknown) -> Self {
        Self(DxgiDeviceSubObject::from(v))
    }
}

impl IUnknown for DxgiResource {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0 }
}

