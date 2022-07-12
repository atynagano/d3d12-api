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
use crate::core::win32::system::com::*;

#[repr(C)]
pub struct DxgiObject(pub(crate) Unknown);

impl Guid for DxgiObject {
	const IID: &'static GUID = &GUID::from_u128(0xaec22fb876f346399be028eb43a67a2eu128);
}

impl Clone for DxgiObject {
	fn clone(&self) -> Self { DxgiObject(self.0.clone()) }
}

pub trait IDxgiObject: IUnknown {
	fn as_object(&self) -> &DxgiObject;
	fn into_object(self) -> DxgiObject;

	fn SetPrivateDataInterface(&self, name: &GUID, unknown: Option<&Unknown>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, name: &GUID, unknown: *const c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, name, option_to_vtable(unknown), );
			_ret_.ok()
		}
	}

	fn GetParent<T: IUnknown>(&self, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_parent: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, riid: &GUID, _out_parent: *mut c_void, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, T::IID, transmute(&mut _out_parent), );
			if _ret_.is_ok() {
				if let Some(_out_parent) = _out_parent {
					return Ok(T::from(_out_parent));
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxgiObject for DxgiObject {
	fn as_object(&self) -> &DxgiObject { self }
	fn into_object(self) -> DxgiObject { self }
}

impl From<Unknown> for DxgiObject {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxgiObject {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

