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

	fn SetPrivateData(&self, name: &GUID, data: &[u8], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, name: &GUID, data_size: u32, data: *const u8, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, name, data.len() as u32, data.as_ptr_or_null(), );
		ret.ok()
	}

	fn SetPrivateDataInterface(&self, name: &GUID, unknown: Option<&Unknown>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, name: &GUID, unknown: Option<VTable>, ) -> HResult
			= unsafe { transmute(vt[4]) };
		let ret = f(vt, name, option_to_vtable(unknown), );
		ret.ok()
	}

	fn GetPrivateData<'a>(&self, name: &GUID, mut data: &'a mut [u8], ) -> Result<(&'a mut [u8]), HResult> {
		let vt = self.as_param();
		let mut data_size = data.len() as u32;
		let f: extern "system" fn(Param<Self>, name: &GUID, data_size: &mut u32, data: *mut u8, ) -> HResult
			= unsafe { transmute(vt[5]) };
		let ret = f(vt, name, &mut data_size, data.as_mut_ptr_or_null(), );
		if ret.is_ok() {
			return Ok((&mut data[..(data_size as usize)]));
		}
		Err(ret)
	}

	fn GetParent<T: IUnknown>(&self, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _parent: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, riid: &GUID, _parent: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[6]) };
		let ret = f(vt, T::IID, &mut _parent, );
		if ret.is_ok() {
			if let (Some(_parent)) = (_parent) {
				return Ok((T::from(_parent)));
			}
		}
		Err(ret)
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
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

