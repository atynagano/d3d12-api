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
use crate::core::win32::system::com::structured_storage::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct PropertyBag2(pub(crate) Unknown);

impl Deref for PropertyBag2 {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for PropertyBag2 {
	const IID: &'static GUID = &GUID::from_u128(0x22f55882280b11d0a8a900a0c90c2004u128);
}

impl Com for PropertyBag2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl PropertyBag2 {
	pub unsafe fn Read() { todo!() }

	pub unsafe fn Write() { todo!() }

	pub fn CountProperties(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pc_properties_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, pc_properties_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pc_properties_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetPropertyInfo(&self) { todo!() }

	pub fn LoadObject(&self, pstr_name: &str, hint: u32, unk_object: &Unknown, err_log: &ErrorLog) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, u32, VTable, VTable) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, pstr_name.to_utf16().as_ptr_or_null(), hint, unk_object.vtable(), err_log.vtable());
			_ret_.ok()
		}
	}
}

pub trait IPropertyBag2: IUnknown {
	fn as_property_bag2(&self) -> &PropertyBag2;
	fn into_property_bag2(self) -> PropertyBag2;
}

impl IPropertyBag2 for PropertyBag2 {
	fn as_property_bag2(&self) -> &PropertyBag2 { self }
	fn into_property_bag2(self) -> PropertyBag2 { self }
}
impl IUnknown for PropertyBag2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for PropertyBag2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

