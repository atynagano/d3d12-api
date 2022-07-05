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

#[repr(C)]
pub struct DxgiFactory4(pub(crate) DxgiFactory3);

impl Guid for DxgiFactory4 {
	const IID: &'static GUID = &GUID::from_u128(0x1bc6ea02ef36464fbf0c21ca39e5168au128);
}

impl Clone for DxgiFactory4 {
	fn clone(&self) -> Self { DxgiFactory4(self.0.clone()) }
}

pub trait IDxgiFactory4: IDxgiFactory3 {
	fn as_factory4(&self) -> &DxgiFactory4;
	fn into_factory4(self) -> DxgiFactory4;

	fn EnumAdapterByLuid<T: IUnknown>(&self, adapter_luid: Luid, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_adapter: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, adapter_luid: Luid, riid: &GUID, _out_adapter: *mut c_void, ) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, adapter_luid, T::IID, transmute(&mut _out_adapter), );
			if _ret_.is_ok() {
				if let Some(_out_adapter) = _out_adapter {
					return Ok(T::from(_out_adapter));
				}
			}
			Err(_ret_)
		}
	}

	fn EnumWarpAdapter<T: IUnknown>(&self, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_adapter: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, riid: &GUID, _out_adapter: *mut c_void, ) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, T::IID, transmute(&mut _out_adapter), );
			if _ret_.is_ok() {
				if let Some(_out_adapter) = _out_adapter {
					return Ok(T::from(_out_adapter));
				}
			}
			Err(_ret_)
		}
	}
}

impl IDxgiFactory4 for DxgiFactory4 {
	fn as_factory4(&self) -> &DxgiFactory4 { self }
	fn into_factory4(self) -> DxgiFactory4 { self }
}

impl IDxgiFactory3 for DxgiFactory4 {
	fn as_factory3(&self) -> &DxgiFactory3 { &self.0 }
	fn into_factory3(self) -> DxgiFactory3 { self.0 }
}

impl IDxgiFactory2 for DxgiFactory4 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0.0 }
	fn into_factory2(self) -> DxgiFactory2 { self.0.0 }
}

impl IDxgiFactory1 for DxgiFactory4 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.0.0 }
	fn into_factory1(self) -> DxgiFactory1 { self.0.0.0 }
}

impl IDxgiFactory for DxgiFactory4 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.0.0.0 }
	fn into_factory(self) -> DxgiFactory { self.0.0.0.0 }
}

impl IDxgiObject for DxgiFactory4 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0.0 }
}

impl From<Unknown> for DxgiFactory4 {
    fn from(v: Unknown) -> Self {
        Self(DxgiFactory3::from(v))
    }
}

impl IUnknown for DxgiFactory4 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0 }
}

