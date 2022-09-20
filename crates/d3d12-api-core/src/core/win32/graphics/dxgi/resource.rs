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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiResource(pub(crate) DxgiDeviceSubObject);

impl Deref for DxgiResource {
	type Target = DxgiDeviceSubObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiResource {
	const IID: &'static GUID = &GUID::from_u128(0x035f3ab4482e4e50b41f8a7f8bd8960bu128);
}

impl Com for DxgiResource {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiResource {
	pub fn GetSharedHandle(&self) -> Result<Handle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut shared_handle_out_: Option<Handle> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, transmute(&mut shared_handle_out_));
			if _ret_.is_ok() { if let Some(shared_handle_out_) = shared_handle_out_ { return Ok(shared_handle_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetUsage(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut usage_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, usage_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(usage_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn SetEvictionPriority(&self, eviction_priority: DxgiResourcePriority) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DxgiResourcePriority) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, eviction_priority);
			_ret_.ok()
		}
	}

	pub fn GetEvictionPriority(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut eviction_priority_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, eviction_priority_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(eviction_priority_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiResource: IDxgiDeviceSubObject {
	fn as_resource(&self) -> &DxgiResource;
	fn into_resource(self) -> DxgiResource;
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

impl IUnknown for DxgiResource {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiResource {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiDeviceSubObject::from(v))
    }
}

