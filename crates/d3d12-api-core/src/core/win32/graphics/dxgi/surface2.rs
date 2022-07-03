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
pub struct DxgiSurface2(pub(crate) DxgiSurface1);

impl Guid for DxgiSurface2 {
	const IID: &'static GUID = &GUID::from_u128(0xaba496ddb6174cb8a866bc44d7eb1fa2u128);
}

impl Clone for DxgiSurface2 {
	fn clone(&self) -> Self { DxgiSurface2(self.0.clone()) }
}

pub trait IDxgiSurface2: IDxgiSurface1 {
	fn as_surface2(&self) -> &DxgiSurface2;
	fn into_surface2(self) -> DxgiSurface2;

	fn GetResource<T: IUnknown>(&self, ) -> Result<(T, u32), HResult> {
		let vt = self.as_param();
		let mut _parent_resource: Option<Unknown> = None;
		let mut _subresource_index: u32 = u32::zeroed();
		let f: extern "system" fn(Param<Self>, riid: &GUID, _parent_resource: &mut Option<Unknown>, _subresource_index: &mut u32, ) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, T::IID, &mut _parent_resource, &mut _subresource_index, );
		if ret.is_ok() {
			if let (Some(_parent_resource), ) = (_parent_resource, ) {
				return Ok((T::from(_parent_resource), _subresource_index));
			}
		}
		Err(ret)
	}
}

impl IDxgiSurface2 for DxgiSurface2 {
	fn as_surface2(&self) -> &DxgiSurface2 { self }
	fn into_surface2(self) -> DxgiSurface2 { self }
}

impl IDxgiSurface1 for DxgiSurface2 {
	fn as_surface1(&self) -> &DxgiSurface1 { &self.0 }
	fn into_surface1(self) -> DxgiSurface1 { self.0 }
}

impl IDxgiSurface for DxgiSurface2 {
	fn as_surface(&self) -> &DxgiSurface { &self.0.0 }
	fn into_surface(self) -> DxgiSurface { self.0.0 }
}

impl IDxgiDeviceSubObject for DxgiSurface2 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.0.0 }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.0.0 }
}

impl IDxgiObject for DxgiSurface2 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0 }
}

impl From<Unknown> for DxgiSurface2 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSurface1::from(v))
    }
}

impl IUnknown for DxgiSurface2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0 }
}

