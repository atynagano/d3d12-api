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
pub struct DxgiAdapter1(pub(crate) DxgiAdapter);

impl Deref for DxgiAdapter1 {
	type Target = DxgiAdapter;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiAdapter1 {
	const IID: &'static GUID = &GUID::from_u128(0x29038f613839462691fd086879011a05u128);
}

impl Com for DxgiAdapter1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiAdapter1 {
	pub fn GetDesc1(&self) -> Result<DxgiAdapterDesc1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiAdapterDesc1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiAdapterDesc1) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiAdapter1: IDxgiAdapter {
	fn as_adapter1(&self) -> &DxgiAdapter1;
	fn into_adapter1(self) -> DxgiAdapter1;
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

impl IUnknown for DxgiAdapter1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiAdapter1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiAdapter::from(v))
    }
}

