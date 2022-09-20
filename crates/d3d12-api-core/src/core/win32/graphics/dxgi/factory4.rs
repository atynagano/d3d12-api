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
pub struct DxgiFactory4(pub(crate) DxgiFactory3);

impl Deref for DxgiFactory4 {
	type Target = DxgiFactory3;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactory4 {
	const IID: &'static GUID = &GUID::from_u128(0x1bc6ea02ef36464fbf0c21ca39e5168au128);
}

impl Com for DxgiFactory4 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactory4 {
	pub fn EnumAdapterByLuid<T: IUnknown + From<UnknownWrapper>>(&self, adapter_luid: Luid) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut adapter_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, Luid, &GUID, *mut c_void) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, adapter_luid, T::IID, transmute(&mut adapter_out_));
			if _ret_.is_ok() { if let Some(adapter_out_) = adapter_out_ { return Ok(T::from(UnknownWrapper(adapter_out_))); } }
			Err(_ret_)
		}
	}

	pub fn EnumWarpAdapter<T: IUnknown + From<UnknownWrapper>>(&self) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut adapter_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, T::IID, transmute(&mut adapter_out_));
			if _ret_.is_ok() { if let Some(adapter_out_) = adapter_out_ { return Ok(T::from(UnknownWrapper(adapter_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiFactory4: IDxgiFactory3 {
	fn as_factory4(&self) -> &DxgiFactory4;
	fn into_factory4(self) -> DxgiFactory4;
}

impl IDxgiFactory4 for DxgiFactory4 {
	fn as_factory4(&self) -> &DxgiFactory4 { self }
	fn into_factory4(self) -> DxgiFactory4 { self }
}
impl IDxgiFactory3 for DxgiFactory4 {
	fn as_factory3(&self) -> &DxgiFactory3 { &self.0.as_factory3() }
	fn into_factory3(self) -> DxgiFactory3 { self.0.into_factory3() }
}

impl IDxgiFactory2 for DxgiFactory4 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0.as_factory2() }
	fn into_factory2(self) -> DxgiFactory2 { self.0.into_factory2() }
}

impl IDxgiFactory1 for DxgiFactory4 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.as_factory1() }
	fn into_factory1(self) -> DxgiFactory1 { self.0.into_factory1() }
}

impl IDxgiFactory for DxgiFactory4 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.as_factory() }
	fn into_factory(self) -> DxgiFactory { self.0.into_factory() }
}

impl IDxgiObject for DxgiFactory4 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiFactory4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactory4 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiFactory3::from(v))
    }
}

