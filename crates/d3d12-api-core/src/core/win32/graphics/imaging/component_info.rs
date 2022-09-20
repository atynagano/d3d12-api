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
use crate::core::win32::graphics::imaging::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICComponentInfo(pub(crate) Unknown);

impl Deref for WICComponentInfo {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICComponentInfo {
	const IID: &'static GUID = &GUID::from_u128(0x23bc3f0a698b4357886bf24d50671334u128);
}

impl Com for WICComponentInfo {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICComponentInfo {
	pub fn GetComponentType(&self) -> Result<WicComponentType, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut r#type_out_: MaybeUninit<WicComponentType> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut WicComponentType) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, r#type_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(r#type_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetCLSID(&self) { todo!() }

	pub fn GetSigningStatus(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut status_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, status_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(status_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetAuthor(&self) { todo!() }

	pub unsafe fn GetVendorGUID(&self) { todo!() }

	pub unsafe fn GetVersion(&self) { todo!() }

	pub unsafe fn GetSpecVersion(&self) { todo!() }

	pub unsafe fn GetFriendlyName(&self) { todo!() }
}

pub trait IWICComponentInfo: IUnknown {
	fn as_component_info(&self) -> &WICComponentInfo;
	fn into_component_info(self) -> WICComponentInfo;
}

impl IWICComponentInfo for WICComponentInfo {
	fn as_component_info(&self) -> &WICComponentInfo { self }
	fn into_component_info(self) -> WICComponentInfo { self }
}
impl IUnknown for WICComponentInfo {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICComponentInfo {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

