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
pub struct DxgiAdapter2(pub(crate) DxgiAdapter1);

impl Guid for DxgiAdapter2 {
	const IID: &'static GUID = &GUID::from_u128(0x0aa1ae0afa0e4b848644e05ff8e5acb5u128);
}

impl Clone for DxgiAdapter2 {
	fn clone(&self) -> Self { DxgiAdapter2(self.0.clone()) }
}

pub trait IDxgiAdapter2: IDxgiAdapter1 {
	fn as_adapter2(&self) -> &DxgiAdapter2;
	fn into_adapter2(self) -> DxgiAdapter2;

	fn GetDesc2(&self, ) -> Result<DxgiAdapterDesc2, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiAdapterDesc2> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiAdapterDesc2, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			Ok(_out_desc.assume_init())
		}
	}
}

impl IDxgiAdapter2 for DxgiAdapter2 {
	fn as_adapter2(&self) -> &DxgiAdapter2 { self }
	fn into_adapter2(self) -> DxgiAdapter2 { self }
}

impl IDxgiAdapter1 for DxgiAdapter2 {
	fn as_adapter1(&self) -> &DxgiAdapter1 { &self.0 }
	fn into_adapter1(self) -> DxgiAdapter1 { self.0 }
}

impl IDxgiAdapter for DxgiAdapter2 {
	fn as_adapter(&self) -> &DxgiAdapter { &self.0.0 }
	fn into_adapter(self) -> DxgiAdapter { self.0.0 }
}

impl IDxgiObject for DxgiAdapter2 {
	fn as_object(&self) -> &DxgiObject { &self.0.0.0 }
	fn into_object(self) -> DxgiObject { self.0.0.0 }
}

impl From<Unknown> for DxgiAdapter2 {
    fn from(v: Unknown) -> Self {
        Self(DxgiAdapter1::from(v))
    }
}

impl IUnknown for DxgiAdapter2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

