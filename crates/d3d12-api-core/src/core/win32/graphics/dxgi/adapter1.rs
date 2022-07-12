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
pub struct DxgiAdapter1(pub(crate) DxgiAdapter);

impl Guid for DxgiAdapter1 {
	const IID: &'static GUID = &GUID::from_u128(0x29038f613839462691fd086879011a05u128);
}

impl Clone for DxgiAdapter1 {
	fn clone(&self) -> Self { DxgiAdapter1(self.0.clone()) }
}

pub trait IDxgiAdapter1: IDxgiAdapter {
	fn as_adapter1(&self) -> &DxgiAdapter1;
	fn into_adapter1(self) -> DxgiAdapter1;

	fn GetDesc1(&self, ) -> Result<DxgiAdapterDesc1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiAdapterDesc1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiAdapterDesc1, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			Ok(_out_desc.assume_init())
		}
	}
}

impl IDxgiAdapter1 for DxgiAdapter1 {
	fn as_adapter1(&self) -> &DxgiAdapter1 { self }
	fn into_adapter1(self) -> DxgiAdapter1 { self }
}

impl IDxgiAdapter for DxgiAdapter1 {
	fn as_adapter(&self) -> &DxgiAdapter { &self.0.as_adapter() }
	fn into_adapter(self) -> DxgiAdapter { self.0.into_adapter() }
}

impl IDxgiObject for DxgiAdapter1 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiAdapter1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiAdapter::from(v))
    }
}

impl IUnknown for DxgiAdapter1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

