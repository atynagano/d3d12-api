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
#[repr(C)]
pub struct DxgiDeviceSubObject(pub(crate) DxgiObject);

impl Guid for DxgiDeviceSubObject {
	const IID: &'static GUID = &GUID::from_u128(0x3d3e0379f9de4d58bb6c18d62992f1a6u128);
}

impl Clone for DxgiDeviceSubObject {
	fn clone(&self) -> Self { DxgiDeviceSubObject(self.0.clone()) }
}

pub trait IDxgiDeviceSubObject: IDxgiObject {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject;
	fn into_device_sub_object(self) -> DxgiDeviceSubObject;

	fn GetDevice<T: IUnknown>(&self, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _device: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, riid: &GUID, _device: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, T::IID, &mut _device, );
		if ret.is_ok() {
			if let (Some(_device)) = (_device) {
				return Ok((T::from(_device)));
			}
		}
		Err(ret)
	}
}

impl IDxgiDeviceSubObject for DxgiDeviceSubObject {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { self }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self }
}

impl IDxgiObject for DxgiDeviceSubObject {
	fn as_object(&self) -> &DxgiObject { &self.0 }
	fn into_object(self) -> DxgiObject { self.0 }
}

impl From<Unknown> for DxgiDeviceSubObject {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiDeviceSubObject {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

