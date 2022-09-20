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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiDeviceSubObject(pub(crate) DxgiObject);

impl Deref for DxgiDeviceSubObject {
	type Target = DxgiObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiDeviceSubObject {
	const IID: &'static GUID = &GUID::from_u128(0x3d3e0379f9de4d58bb6c18d62992f1a6u128);
}

impl Com for DxgiDeviceSubObject {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiDeviceSubObject {
	pub fn GetDevice<T: IUnknown + From<UnknownWrapper>>(&self) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut device_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, T::IID, transmute(&mut device_out_));
			if _ret_.is_ok() { if let Some(device_out_) = device_out_ { return Ok(T::from(UnknownWrapper(device_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiDeviceSubObject: IDxgiObject {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject;
	fn into_device_sub_object(self) -> DxgiDeviceSubObject;
}

impl IDxgiDeviceSubObject for DxgiDeviceSubObject {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { self }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self }
}
impl IDxgiObject for DxgiDeviceSubObject {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiDeviceSubObject {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiDeviceSubObject {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiObject::from(v))
    }
}

