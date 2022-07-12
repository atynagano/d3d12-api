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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
pub struct DxgiFactory1(pub(crate) DxgiFactory);

impl Guid for DxgiFactory1 {
	const IID: &'static GUID = &GUID::from_u128(0x770aae78f26f4dbaa829253c83d1b387u128);
}

impl Clone for DxgiFactory1 {
	fn clone(&self) -> Self { DxgiFactory1(self.0.clone()) }
}

pub trait IDxgiFactory1: IDxgiFactory {
	fn as_factory1(&self) -> &DxgiFactory1;
	fn into_factory1(self) -> DxgiFactory1;

	fn EnumAdapters1(&self, adapter: u32, ) -> Result<DxgiAdapter1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_adapter: Option<DxgiAdapter1> = None;
			let f: extern "system" fn(Param<Self>, adapter: u32, adapter: *mut c_void, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, adapter, transmute(&mut _out_adapter), );
			if _ret_.is_ok() {
				if let Some(_out_adapter) = _out_adapter {
					return Ok(_out_adapter);
				}
			}
			Err(_ret_)
		}
	}

	fn IsCurrent(&self, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Bool
				= transmute(vt[13]);
			let _ret_ = f(vt, );
			_ret_.to_bool()
		}
	}
}

impl IDxgiFactory1 for DxgiFactory1 {
	fn as_factory1(&self) -> &DxgiFactory1 { self }
	fn into_factory1(self) -> DxgiFactory1 { self }
}

impl IDxgiFactory for DxgiFactory1 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.as_factory() }
	fn into_factory(self) -> DxgiFactory { self.0.into_factory() }
}

impl IDxgiObject for DxgiFactory1 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiFactory1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiFactory::from(v))
    }
}

impl IUnknown for DxgiFactory1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

