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
use crate::core::win32::security::*;

#[repr(C)]
pub struct DxgiResource1(pub(crate) DxgiResource);

impl Guid for DxgiResource1 {
	const IID: &'static GUID = &GUID::from_u128(0x3096137946094a41998e54fe567ee0c1u128);
}

impl Clone for DxgiResource1 {
	fn clone(&self) -> Self { DxgiResource1(self.0.clone()) }
}

pub trait IDxgiResource1: IDxgiResource {
	fn as_resource1(&self) -> &DxgiResource1;
	fn into_resource1(self) -> DxgiResource1;

	fn CreateSubresourceSurface(&self, index: u32, ) -> Result<DxgiSurface2, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_surface: Option<DxgiSurface2> = None;
			let f: extern "system" fn(Param<Self>, index: u32, surface: *mut c_void, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, index, transmute(&mut _out_surface), );
			if _ret_.is_ok() {
				if let Some(_out_surface) = _out_surface {
					return Ok(_out_surface);
				}
			}
			Err(_ret_)
		}
	}

	fn CreateSharedHandle(&self, attributes: Option<&SecurityAttributes>, access: u32, name: Option<&str>, ) -> Result<Handle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_handle: Option<Handle> = None;
			let f: extern "system" fn(Param<Self>, attributes: *const c_void, access: u32, name: *const u16, _out_handle: *mut c_void, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(attributes), access, name.map(str::to_utf16).as_ptr_or_null(), transmute(&mut _out_handle), );
			if _ret_.is_ok() {
				if let Some(_out_handle) = _out_handle {
					return Ok(_out_handle);
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxgiResource1 for DxgiResource1 {
	fn as_resource1(&self) -> &DxgiResource1 { self }
	fn into_resource1(self) -> DxgiResource1 { self }
}

impl IDxgiResource for DxgiResource1 {
	fn as_resource(&self) -> &DxgiResource { &self.0 }
	fn into_resource(self) -> DxgiResource { self.0 }
}

impl IDxgiDeviceSubObject for DxgiResource1 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.0 }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.0 }
}

impl IDxgiObject for DxgiResource1 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0 }
}

impl From<Unknown> for DxgiResource1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiResource::from(v))
    }
}

impl IUnknown for DxgiResource1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

