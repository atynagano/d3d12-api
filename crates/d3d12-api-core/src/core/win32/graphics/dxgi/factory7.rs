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
pub struct DxgiFactory7(pub(crate) DxgiFactory6);

impl Deref for DxgiFactory7 {
	type Target = DxgiFactory6;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactory7 {
	const IID: &'static GUID = &GUID::from_u128(0xa4966eed76db44da84c1ee9a7afb20a8u128);
}

impl Com for DxgiFactory7 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactory7 {
	pub fn RegisterAdaptersChangedEvent(&self, event: Handle) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pdw_cookie_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, Handle, *mut u32) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, event, pdw_cookie_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pdw_cookie_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn UnregisterAdaptersChangedEvent(&self, cookie: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, cookie);
			_ret_.ok()
		}
	}
}

pub trait IDxgiFactory7: IDxgiFactory6 {
	fn as_factory7(&self) -> &DxgiFactory7;
	fn into_factory7(self) -> DxgiFactory7;
}

impl IDxgiFactory7 for DxgiFactory7 {
	fn as_factory7(&self) -> &DxgiFactory7 { self }
	fn into_factory7(self) -> DxgiFactory7 { self }
}
impl IDxgiFactory6 for DxgiFactory7 {
	fn as_factory6(&self) -> &DxgiFactory6 { &self.0.as_factory6() }
	fn into_factory6(self) -> DxgiFactory6 { self.0.into_factory6() }
}

impl IDxgiFactory5 for DxgiFactory7 {
	fn as_factory5(&self) -> &DxgiFactory5 { &self.0.as_factory5() }
	fn into_factory5(self) -> DxgiFactory5 { self.0.into_factory5() }
}

impl IDxgiFactory4 for DxgiFactory7 {
	fn as_factory4(&self) -> &DxgiFactory4 { &self.0.as_factory4() }
	fn into_factory4(self) -> DxgiFactory4 { self.0.into_factory4() }
}

impl IDxgiFactory3 for DxgiFactory7 {
	fn as_factory3(&self) -> &DxgiFactory3 { &self.0.as_factory3() }
	fn into_factory3(self) -> DxgiFactory3 { self.0.into_factory3() }
}

impl IDxgiFactory2 for DxgiFactory7 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0.as_factory2() }
	fn into_factory2(self) -> DxgiFactory2 { self.0.into_factory2() }
}

impl IDxgiFactory1 for DxgiFactory7 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.as_factory1() }
	fn into_factory1(self) -> DxgiFactory1 { self.0.into_factory1() }
}

impl IDxgiFactory for DxgiFactory7 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.as_factory() }
	fn into_factory(self) -> DxgiFactory { self.0.into_factory() }
}

impl IDxgiObject for DxgiFactory7 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiFactory7 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactory7 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiFactory6::from(v))
    }
}

