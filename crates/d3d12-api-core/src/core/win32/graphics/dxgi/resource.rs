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

	fn GetSharedHandle(&self, ) -> Result<Handle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_shared_handle: Option<Handle> = None;
			let f: extern "system" fn(Param<Self>, _out_shared_handle: *mut c_void, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, transmute(&mut _out_shared_handle), );
			if _ret_.is_ok() {
				if let Some(_out_shared_handle) = _out_shared_handle {
					return Ok(_out_shared_handle);
				}
			}
			Err(_ret_)
		}
	}

	fn GetUsage(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_usage: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_usage: *mut u32, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, _out_usage.as_mut_ptr(), );
			Ok(_out_usage.assume_init())
		}
	}

	fn SetEvictionPriority(&self, eviction_priority: DxgiResourcePriority, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, eviction_priority: DxgiResourcePriority, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, eviction_priority, );
			_ret_.ok()
		}
	}

	fn GetEvictionPriority(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_eviction_priority: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_eviction_priority: *mut u32, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, _out_eviction_priority.as_mut_ptr(), );
			Ok(_out_eviction_priority.assume_init())
		}
	}
}

impl IDxgiResource for DxgiResource {
	fn as_resource(&self) -> &DxgiResource { self }
	fn into_resource(self) -> DxgiResource { self }
}

impl IDxgiDeviceSubObject for DxgiResource {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiResource {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiResource {
    fn from(v: Unknown) -> Self {
        Self(DxgiDeviceSubObject::from(v))
    }
}

impl IUnknown for DxgiResource {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

