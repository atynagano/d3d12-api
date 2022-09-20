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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiObject(pub(crate) Unknown);

impl Deref for DxgiObject {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiObject {
	const IID: &'static GUID = &GUID::from_u128(0xaec22fb876f346399be028eb43a67a2eu128);
}

impl Com for DxgiObject {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiObject {
	pub fn SetPrivateData(&self, name: &GUID, data: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, &GUID, u32, *const u8) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, name, data_len_ as u32, data_ptr_);
			_ret_.ok()
		}
	}

	pub fn SetPrivateDataInterface(&self, name: &GUID, unknown: Option<&Unknown>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &GUID, *const c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, name, option_to_vtable(unknown));
			_ret_.ok()
		}
	}

	pub unsafe fn GetPrivateData(&self) { todo!() }

	pub fn GetParent<T: IUnknown + From<UnknownWrapper>>(&self) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut parent_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, T::IID, transmute(&mut parent_out_));
			if _ret_.is_ok() { if let Some(parent_out_) = parent_out_ { return Ok(T::from(UnknownWrapper(parent_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiObject: IUnknown {
	fn as_object(&self) -> &DxgiObject;
	fn into_object(self) -> DxgiObject;
}

impl IDxgiObject for DxgiObject {
	fn as_object(&self) -> &DxgiObject { self }
	fn into_object(self) -> DxgiObject { self }
}
impl IUnknown for DxgiObject {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiObject {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

