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
pub struct DxgiFactory5(pub(crate) DxgiFactory4);

impl Deref for DxgiFactory5 {
	type Target = DxgiFactory4;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactory5 {
	const IID: &'static GUID = &GUID::from_u128(0x7632e1f5ee654dca87fd84cd75f8838du128);
}

impl Com for DxgiFactory5 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactory5 {
	pub fn CheckFeatureSupport(&self, feature: DxgiFeature, feature_support_data: &mut [u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (feature_support_data_ptr_, feature_support_data_len_) = feature_support_data.deconstruct();
			let f: extern "system" fn(Param<Self>, DxgiFeature, *mut u8, u32) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, feature, feature_support_data_ptr_, feature_support_data_len_ as u32);
			_ret_.ok()
		}
	}
}

pub trait IDxgiFactory5: IDxgiFactory4 {
	fn as_factory5(&self) -> &DxgiFactory5;
	fn into_factory5(self) -> DxgiFactory5;
}

impl IDxgiFactory5 for DxgiFactory5 {
	fn as_factory5(&self) -> &DxgiFactory5 { self }
	fn into_factory5(self) -> DxgiFactory5 { self }
}
impl IDxgiFactory4 for DxgiFactory5 {
	fn as_factory4(&self) -> &DxgiFactory4 { &self.0.as_factory4() }
	fn into_factory4(self) -> DxgiFactory4 { self.0.into_factory4() }
}

impl IDxgiFactory3 for DxgiFactory5 {
	fn as_factory3(&self) -> &DxgiFactory3 { &self.0.as_factory3() }
	fn into_factory3(self) -> DxgiFactory3 { self.0.into_factory3() }
}

impl IDxgiFactory2 for DxgiFactory5 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0.as_factory2() }
	fn into_factory2(self) -> DxgiFactory2 { self.0.into_factory2() }
}

impl IDxgiFactory1 for DxgiFactory5 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.as_factory1() }
	fn into_factory1(self) -> DxgiFactory1 { self.0.into_factory1() }
}

impl IDxgiFactory for DxgiFactory5 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.as_factory() }
	fn into_factory(self) -> DxgiFactory { self.0.into_factory() }
}

impl IDxgiObject for DxgiFactory5 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiFactory5 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactory5 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiFactory4::from(v))
    }
}

