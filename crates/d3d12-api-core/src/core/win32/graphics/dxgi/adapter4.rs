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
pub struct DxgiAdapter4(pub(crate) DxgiAdapter3);

impl Guid for DxgiAdapter4 {
	const IID: &'static GUID = &GUID::from_u128(0x3c8d99d14fbf4181a82caf66bf7bd24eu128);
}

impl Clone for DxgiAdapter4 {
	fn clone(&self) -> Self { DxgiAdapter4(self.0.clone()) }
}

pub trait IDxgiAdapter4: IDxgiAdapter3 {
	fn as_adapter4(&self) -> &DxgiAdapter4;
	fn into_adapter4(self) -> DxgiAdapter4;

	fn GetDesc3(&self, ) -> Result<DxgiAdapterDesc3, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiAdapterDesc3> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiAdapterDesc3, ) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			Ok(_out_desc.assume_init())
		}
	}
}

impl IDxgiAdapter4 for DxgiAdapter4 {
	fn as_adapter4(&self) -> &DxgiAdapter4 { self }
	fn into_adapter4(self) -> DxgiAdapter4 { self }
}

impl IDxgiAdapter3 for DxgiAdapter4 {
	fn as_adapter3(&self) -> &DxgiAdapter3 { &self.0 }
	fn into_adapter3(self) -> DxgiAdapter3 { self.0 }
}

impl IDxgiAdapter2 for DxgiAdapter4 {
	fn as_adapter2(&self) -> &DxgiAdapter2 { &self.0.0 }
	fn into_adapter2(self) -> DxgiAdapter2 { self.0.0 }
}

impl IDxgiAdapter1 for DxgiAdapter4 {
	fn as_adapter1(&self) -> &DxgiAdapter1 { &self.0.0.0 }
	fn into_adapter1(self) -> DxgiAdapter1 { self.0.0.0 }
}

impl IDxgiAdapter for DxgiAdapter4 {
	fn as_adapter(&self) -> &DxgiAdapter { &self.0.0.0.0 }
	fn into_adapter(self) -> DxgiAdapter { self.0.0.0.0 }
}

impl IDxgiObject for DxgiAdapter4 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0.0.0 }
}

impl From<Unknown> for DxgiAdapter4 {
    fn from(v: Unknown) -> Self {
        Self(DxgiAdapter3::from(v))
    }
}

impl IUnknown for DxgiAdapter4 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0 }
}

