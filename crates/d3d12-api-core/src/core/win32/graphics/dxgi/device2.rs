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
pub struct DxgiDevice2(pub(crate) DxgiDevice1);

impl Guid for DxgiDevice2 {
	const IID: &'static GUID = &GUID::from_u128(0x05008617fbfd4051a790144884b4f6a9u128);
}

impl Clone for DxgiDevice2 {
	fn clone(&self) -> Self { DxgiDevice2(self.0.clone()) }
}

pub trait IDxgiDevice2: IDxgiDevice1 {
	fn as_device2(&self) -> &DxgiDevice2;
	fn into_device2(self) -> DxgiDevice2;

	fn OfferResources(&self, resources: &[Param<DxgiResource>], priority: DxgiOfferResourcePriority, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_resources: u32, resources: *const Param<DxgiResource>, priority: DxgiOfferResourcePriority, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, resources.len() as u32, resources.as_ptr_or_null(), priority, );
		ret.ok()
	}

	fn ReclaimResources(&self, resources: &[Param<DxgiResource>], discarded: Option<&mut Bool>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_resources: u32, resources: *const Param<DxgiResource>, discarded: Option<&mut Bool>, ) -> HResult
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, resources.len() as u32, resources.as_ptr_or_null(), discarded, );
		ret.ok()
	}

	fn EnqueueSetEvent(&self, event: Handle, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, event: Handle, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, event, );
		ret.ok()
	}
}

impl IDxgiDevice2 for DxgiDevice2 {
	fn as_device2(&self) -> &DxgiDevice2 { self }
	fn into_device2(self) -> DxgiDevice2 { self }
}

impl IDxgiDevice1 for DxgiDevice2 {
	fn as_device1(&self) -> &DxgiDevice1 { &self.0 }
	fn into_device1(self) -> DxgiDevice1 { self.0 }
}

impl IDxgiDevice for DxgiDevice2 {
	fn as_device(&self) -> &DxgiDevice { &self.0.0 }
	fn into_device(self) -> DxgiDevice { self.0.0 }
}

impl IDxgiObject for DxgiDevice2 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0 }
}

impl From<Unknown> for DxgiDevice2 {
    fn from(v: Unknown) -> Self {
        Self(DxgiDevice1::from(v))
    }
}

impl IUnknown for DxgiDevice2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

