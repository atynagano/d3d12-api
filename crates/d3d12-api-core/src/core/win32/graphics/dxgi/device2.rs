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
pub struct DxgiDevice2(pub(crate) DxgiDevice1);

impl Deref for DxgiDevice2 {
	type Target = DxgiDevice1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiDevice2 {
	const IID: &'static GUID = &GUID::from_u128(0x05008617fbfd4051a790144884b4f6a9u128);
}

impl Com for DxgiDevice2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiDevice2 {
	pub fn OfferResources(&self, resources: &[Param<DxgiResource>], priority: DxgiOfferResourcePriority) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (resources_ptr_, resources_len_) = resources.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Param<DxgiResource>, DxgiOfferResourcePriority) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, resources_len_ as u32, resources_ptr_, priority);
			_ret_.ok()
		}
	}

	pub unsafe fn ReclaimResources() { todo!() }

	pub fn EnqueueSetEvent(&self, event: Handle) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Handle) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, event);
			_ret_.ok()
		}
	}
}

pub trait IDxgiDevice2: IDxgiDevice1 {
	fn as_device2(&self) -> &DxgiDevice2;
	fn into_device2(self) -> DxgiDevice2;
}

impl IDxgiDevice2 for DxgiDevice2 {
	fn as_device2(&self) -> &DxgiDevice2 { self }
	fn into_device2(self) -> DxgiDevice2 { self }
}
impl IDxgiDevice1 for DxgiDevice2 {
	fn as_device1(&self) -> &DxgiDevice1 { &self.0.as_device1() }
	fn into_device1(self) -> DxgiDevice1 { self.0.into_device1() }
}

impl IDxgiDevice for DxgiDevice2 {
	fn as_device(&self) -> &DxgiDevice { &self.0.as_device() }
	fn into_device(self) -> DxgiDevice { self.0.into_device() }
}

impl IDxgiObject for DxgiDevice2 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiDevice2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiDevice2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiDevice1::from(v))
    }
}

