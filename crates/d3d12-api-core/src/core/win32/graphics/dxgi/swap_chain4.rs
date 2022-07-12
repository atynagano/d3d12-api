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
pub struct DxgiSwapChain4(pub(crate) DxgiSwapChain3);

impl Guid for DxgiSwapChain4 {
	const IID: &'static GUID = &GUID::from_u128(0x3d585d5abd4a489eb1f43dbcb6452ffbu128);
}

impl Clone for DxgiSwapChain4 {
	fn clone(&self) -> Self { DxgiSwapChain4(self.0.clone()) }
}

pub trait IDxgiSwapChain4: IDxgiSwapChain3 {
	fn as_swap_chain4(&self) -> &DxgiSwapChain4;
	fn into_swap_chain4(self) -> DxgiSwapChain4;

	fn SetHDRMetaData(&self, ty: DxgiHdrMetaDataType, meta_data: Option<&[u8]>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_meta_data, _len_meta_data) = meta_data.deconstruct();
			let f: extern "system" fn(Param<Self>, ty: DxgiHdrMetaDataType, size: u32, meta_data: *const u8, ) -> HResult
				= transmute(vt[40]);
			let _ret_ = f(vt, ty, _len_meta_data as u32, _ptr_meta_data, );
			_ret_.ok()
		}
	}
}

impl IDxgiSwapChain4 for DxgiSwapChain4 {
	fn as_swap_chain4(&self) -> &DxgiSwapChain4 { self }
	fn into_swap_chain4(self) -> DxgiSwapChain4 { self }
}

impl IDxgiSwapChain3 for DxgiSwapChain4 {
	fn as_swap_chain3(&self) -> &DxgiSwapChain3 { &self.0.as_swap_chain3() }
	fn into_swap_chain3(self) -> DxgiSwapChain3 { self.0.into_swap_chain3() }
}

impl IDxgiSwapChain2 for DxgiSwapChain4 {
	fn as_swap_chain2(&self) -> &DxgiSwapChain2 { &self.0.as_swap_chain2() }
	fn into_swap_chain2(self) -> DxgiSwapChain2 { self.0.into_swap_chain2() }
}

impl IDxgiSwapChain1 for DxgiSwapChain4 {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1 { &self.0.as_swap_chain1() }
	fn into_swap_chain1(self) -> DxgiSwapChain1 { self.0.into_swap_chain1() }
}

impl IDxgiSwapChain for DxgiSwapChain4 {
	fn as_swap_chain(&self) -> &DxgiSwapChain { &self.0.as_swap_chain() }
	fn into_swap_chain(self) -> DxgiSwapChain { self.0.into_swap_chain() }
}

impl IDxgiDeviceSubObject for DxgiSwapChain4 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiSwapChain4 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiSwapChain4 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSwapChain3::from(v))
    }
}

impl IUnknown for DxgiSwapChain4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

