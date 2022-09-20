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
pub struct DxgiDevice4(pub(crate) DxgiDevice3);

impl Deref for DxgiDevice4 {
	type Target = DxgiDevice3;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiDevice4 {
	const IID: &'static GUID = &GUID::from_u128(0x95b4f95fd8da4ca49ee63b76d5968a10u128);
}

impl Com for DxgiDevice4 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiDevice4 {
	pub fn OfferResources1(&self, resources: &[Param<DxgiResource>], priority: DxgiOfferResourcePriority, flags: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (resources_ptr_, resources_len_) = resources.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Param<DxgiResource>, DxgiOfferResourcePriority, u32) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, resources_len_ as u32, resources_ptr_, priority, flags);
			_ret_.ok()
		}
	}

	pub unsafe fn ReclaimResources1() { todo!() }
}

pub trait IDxgiDevice4: IDxgiDevice3 {
	fn as_device4(&self) -> &DxgiDevice4;
	fn into_device4(self) -> DxgiDevice4;
}

impl IDxgiDevice4 for DxgiDevice4 {
	fn as_device4(&self) -> &DxgiDevice4 { self }
	fn into_device4(self) -> DxgiDevice4 { self }
}
impl IDxgiDevice3 for DxgiDevice4 {
	fn as_device3(&self) -> &DxgiDevice3 { &self.0.as_device3() }
	fn into_device3(self) -> DxgiDevice3 { self.0.into_device3() }
}

impl IDxgiDevice2 for DxgiDevice4 {
	fn as_device2(&self) -> &DxgiDevice2 { &self.0.as_device2() }
	fn into_device2(self) -> DxgiDevice2 { self.0.into_device2() }
}

impl IDxgiDevice1 for DxgiDevice4 {
	fn as_device1(&self) -> &DxgiDevice1 { &self.0.as_device1() }
	fn into_device1(self) -> DxgiDevice1 { self.0.into_device1() }
}

impl IDxgiDevice for DxgiDevice4 {
	fn as_device(&self) -> &DxgiDevice { &self.0.as_device() }
	fn into_device(self) -> DxgiDevice { self.0.into_device() }
}

impl IDxgiObject for DxgiDevice4 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiDevice4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiDevice4 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiDevice3::from(v))
    }
}

