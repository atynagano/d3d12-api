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
pub struct DxgiFactory6(pub(crate) DxgiFactory5);

impl Deref for DxgiFactory6 {
	type Target = DxgiFactory5;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiFactory6 {
	const IID: &'static GUID = &GUID::from_u128(0xc1b6694fff0944a9b03c77900a0a1d17u128);
}

impl Com for DxgiFactory6 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiFactory6 {
	pub fn EnumAdapterByGpuPreference<T: IUnknown + From<UnknownWrapper>>(&self, adapter: u32, gpu_preference: DxgiGpuPreference) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut adapter_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, u32, DxgiGpuPreference, &GUID, *mut c_void) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, adapter, gpu_preference, T::IID, transmute(&mut adapter_out_));
			if _ret_.is_ok() { if let Some(adapter_out_) = adapter_out_ { return Ok(T::from(UnknownWrapper(adapter_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiFactory6: IDxgiFactory5 {
	fn as_factory6(&self) -> &DxgiFactory6;
	fn into_factory6(self) -> DxgiFactory6;
}

impl IDxgiFactory6 for DxgiFactory6 {
	fn as_factory6(&self) -> &DxgiFactory6 { self }
	fn into_factory6(self) -> DxgiFactory6 { self }
}
impl IDxgiFactory5 for DxgiFactory6 {
	fn as_factory5(&self) -> &DxgiFactory5 { &self.0.as_factory5() }
	fn into_factory5(self) -> DxgiFactory5 { self.0.into_factory5() }
}

impl IDxgiFactory4 for DxgiFactory6 {
	fn as_factory4(&self) -> &DxgiFactory4 { &self.0.as_factory4() }
	fn into_factory4(self) -> DxgiFactory4 { self.0.into_factory4() }
}

impl IDxgiFactory3 for DxgiFactory6 {
	fn as_factory3(&self) -> &DxgiFactory3 { &self.0.as_factory3() }
	fn into_factory3(self) -> DxgiFactory3 { self.0.into_factory3() }
}

impl IDxgiFactory2 for DxgiFactory6 {
	fn as_factory2(&self) -> &DxgiFactory2 { &self.0.as_factory2() }
	fn into_factory2(self) -> DxgiFactory2 { self.0.into_factory2() }
}

impl IDxgiFactory1 for DxgiFactory6 {
	fn as_factory1(&self) -> &DxgiFactory1 { &self.0.as_factory1() }
	fn into_factory1(self) -> DxgiFactory1 { self.0.into_factory1() }
}

impl IDxgiFactory for DxgiFactory6 {
	fn as_factory(&self) -> &DxgiFactory { &self.0.as_factory() }
	fn into_factory(self) -> DxgiFactory { self.0.into_factory() }
}

impl IDxgiObject for DxgiFactory6 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiFactory6 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiFactory6 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiFactory5::from(v))
    }
}

