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
pub struct DxgiDevice4(pub(crate) DxgiDevice3);

impl Guid for DxgiDevice4 {
	const IID: &'static GUID = &GUID::from_u128(0x95b4f95fd8da4ca49ee63b76d5968a10u128);
}

impl Clone for DxgiDevice4 {
	fn clone(&self) -> Self { DxgiDevice4(self.0.clone()) }
}

pub trait IDxgiDevice4: IDxgiDevice3 {
	fn as_device4(&self) -> &DxgiDevice4;
	fn into_device4(self) -> DxgiDevice4;

	fn OfferResources1(&self, resources: &[Param<DxgiResource>], priority: DxgiOfferResourcePriority, flags: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_resources, _len_resources) = resources.deconstruct();
			let f: extern "system" fn(Param<Self>, num_resources: u32, resources: *const Param<DxgiResource>, priority: DxgiOfferResourcePriority, flags: u32, ) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, _len_resources as u32, _ptr_resources, priority, flags, );
			_ret_.ok()
		}
	}

	fn ReclaimResources1(&self, resources: &[Param<DxgiResource>], ) -> Result<DxgiReclaimResourceResults, HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_resources, _len_resources) = resources.deconstruct();
			let mut _out_results: MaybeUninit<DxgiReclaimResourceResults> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, num_resources: u32, resources: *const Param<DxgiResource>, _out_results: *mut DxgiReclaimResourceResults, ) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, _len_resources as u32, _ptr_resources, _out_results.as_mut_ptr(), );
			Ok(_out_results.assume_init())
		}
	}
}

impl IDxgiDevice4 for DxgiDevice4 {
	fn as_device4(&self) -> &DxgiDevice4 { self }
	fn into_device4(self) -> DxgiDevice4 { self }
}

impl IDxgiDevice3 for DxgiDevice4 {
	fn as_device3(&self) -> &DxgiDevice3 { &self.0 }
	fn into_device3(self) -> DxgiDevice3 { self.0 }
}

impl IDxgiDevice2 for DxgiDevice4 {
	fn as_device2(&self) -> &DxgiDevice2 { &self.0.0 }
	fn into_device2(self) -> DxgiDevice2 { self.0.0 }
}

impl IDxgiDevice1 for DxgiDevice4 {
	fn as_device1(&self) -> &DxgiDevice1 { &self.0.0.0 }
	fn into_device1(self) -> DxgiDevice1 { self.0.0.0 }
}

impl IDxgiDevice for DxgiDevice4 {
	fn as_device(&self) -> &DxgiDevice { &self.0.0.0.0 }
	fn into_device(self) -> DxgiDevice { self.0.0.0.0 }
}

impl IDxgiObject for DxgiDevice4 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0.0 }
}

impl From<Unknown> for DxgiDevice4 {
    fn from(v: Unknown) -> Self {
        Self(DxgiDevice3::from(v))
    }
}

impl IUnknown for DxgiDevice4 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0 }
}

