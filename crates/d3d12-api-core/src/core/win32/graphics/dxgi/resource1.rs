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
use crate::core::win32::security::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiResource1(pub(crate) DxgiResource);

impl Deref for DxgiResource1 {
	type Target = DxgiResource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiResource1 {
	const IID: &'static GUID = &GUID::from_u128(0x3096137946094a41998e54fe567ee0c1u128);
}

impl Com for DxgiResource1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiResource1 {
	pub fn CreateSubresourceSurface(&self, index: u32) -> Result<DxgiSurface2, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut surface_out_: Option<DxgiSurface2> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, index, transmute(&mut surface_out_));
			if _ret_.is_ok() { if let Some(surface_out_) = surface_out_ { return Ok(surface_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateSharedHandle(&self, attributes: Option<&SecurityAttributes>, access: u32, name: Option<&str>) -> Result<Handle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut handle_out_: Option<Handle> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, u32, *const u16, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(attributes), access, name.map(str::to_utf16).as_ptr_or_null(), transmute(&mut handle_out_));
			if _ret_.is_ok() { if let Some(handle_out_) = handle_out_ { return Ok(handle_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiResource1: IDxgiResource {
	fn as_resource1(&self) -> &DxgiResource1;
	fn into_resource1(self) -> DxgiResource1;
}

impl IDxgiResource1 for DxgiResource1 {
	fn as_resource1(&self) -> &DxgiResource1 { self }
	fn into_resource1(self) -> DxgiResource1 { self }
}
impl IDxgiResource for DxgiResource1 {
	fn as_resource(&self) -> &DxgiResource { &self.0.as_resource() }
	fn into_resource(self) -> DxgiResource { self.0.into_resource() }
}

impl IDxgiDeviceSubObject for DxgiResource1 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiResource1 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiResource1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiResource1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiResource::from(v))
    }
}

