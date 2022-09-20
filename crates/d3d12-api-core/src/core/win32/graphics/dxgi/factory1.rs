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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiFactory1(pub(crate) DxgiFactory);

impl Deref for DxgiFactory1 {
	type Target = DxgiFactory;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactory1 {
	const IID: &'static GUID = &GUID::from_u128(0x770aae78f26f4dbaa829253c83d1b387u128);
}

impl Com for DxgiFactory1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactory1 {
	pub fn EnumAdapters1(&self, adapter: u32) -> Result<DxgiAdapter1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut adapter_out_: Option<DxgiAdapter1> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, adapter, transmute(&mut adapter_out_));
			if _ret_.is_ok() { if let Some(adapter_out_) = adapter_out_ { return Ok(adapter_out_); } }
			Err(_ret_)
		}
	}

	pub fn IsCurrent(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[13]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}
}

pub trait IDxgiFactory1: IDxgiFactory {
	fn as_factory1(&self) -> &DxgiFactory1;
	fn into_factory1(self) -> DxgiFactory1;
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

impl IUnknown for DxgiFactory1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactory1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiFactory::from(v))
    }
}

