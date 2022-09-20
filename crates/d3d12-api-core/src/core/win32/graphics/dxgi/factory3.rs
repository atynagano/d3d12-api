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


#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiFactory3(pub(crate) DxgiFactory2);

impl Deref for DxgiFactory3 {
	type Target = DxgiFactory2;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactory3 {
	const IID: &'static GUID = &GUID::from_u128(0x25483823cd464c7d86ca47aa95b837bdu128);
}

impl Com for DxgiFactory3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactory3 {
	pub fn GetCreationFlags(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[25]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait IDxgiFactory3: IDxgiFactory2 {
	fn as_factory3(&self) -> &DxgiFactory3;
	fn into_factory3(self) -> DxgiFactory3;
}

impl IDxgiFactory3 for DxgiFactory3 {
	fn as_factory3(&self) -> &DxgiFactory3 { self }
	fn into_factory3(self) -> DxgiFactory3 { self }
}
impl IDxgiFactory2 for DxgiFactory3 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0.as_factory2() }
	fn into_factory2(self) -> DxgiFactory2 { self.0.into_factory2() }
}

impl IDxgiFactory1 for DxgiFactory3 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.as_factory1() }
	fn into_factory1(self) -> DxgiFactory1 { self.0.into_factory1() }
}

impl IDxgiFactory for DxgiFactory3 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.as_factory() }
	fn into_factory(self) -> DxgiFactory { self.0.into_factory() }
}

impl IDxgiObject for DxgiFactory3 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiFactory3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactory3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiFactory2::from(v))
    }
}

