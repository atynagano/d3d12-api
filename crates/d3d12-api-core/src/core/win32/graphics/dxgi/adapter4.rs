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
pub struct DxgiAdapter4(pub(crate) DxgiAdapter3);

impl Deref for DxgiAdapter4 {
	type Target = DxgiAdapter3;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiAdapter4 {
	const IID: &'static GUID = &GUID::from_u128(0x3c8d99d14fbf4181a82caf66bf7bd24eu128);
}

impl Com for DxgiAdapter4 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiAdapter4 {
	pub fn GetDesc3(&self) -> Result<DxgiAdapterDesc3, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiAdapterDesc3> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiAdapterDesc3) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiAdapter4: IDxgiAdapter3 {
	fn as_adapter4(&self) -> &DxgiAdapter4;
	fn into_adapter4(self) -> DxgiAdapter4;
}

impl IDxgiAdapter4 for DxgiAdapter4 {
	fn as_adapter4(&self) -> &DxgiAdapter4 { self }
	fn into_adapter4(self) -> DxgiAdapter4 { self }
}
impl IDxgiAdapter3 for DxgiAdapter4 {
	fn as_adapter3(&self) -> &DxgiAdapter3 { &self.0.as_adapter3() }
	fn into_adapter3(self) -> DxgiAdapter3 { self.0.into_adapter3() }
}

impl IDxgiAdapter2 for DxgiAdapter4 {
	fn as_adapter2(&self) -> &DxgiAdapter2 { &self.0.as_adapter2() }
	fn into_adapter2(self) -> DxgiAdapter2 { self.0.into_adapter2() }
}

impl IDxgiAdapter1 for DxgiAdapter4 {
	fn as_adapter1(&self) -> &DxgiAdapter1 { &self.0.as_adapter1() }
	fn into_adapter1(self) -> DxgiAdapter1 { self.0.into_adapter1() }
}

impl IDxgiAdapter for DxgiAdapter4 {
	fn as_adapter(&self) -> &DxgiAdapter { &self.0.as_adapter() }
	fn into_adapter(self) -> DxgiAdapter { self.0.into_adapter() }
}

impl IDxgiObject for DxgiAdapter4 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiAdapter4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiAdapter4 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiAdapter3::from(v))
    }
}

