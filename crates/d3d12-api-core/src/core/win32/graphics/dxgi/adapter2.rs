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
pub struct DxgiAdapter2(pub(crate) DxgiAdapter1);

impl Deref for DxgiAdapter2 {
	type Target = DxgiAdapter1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiAdapter2 {
	const IID: &'static GUID = &GUID::from_u128(0x0aa1ae0afa0e4b848644e05ff8e5acb5u128);
}

impl Com for DxgiAdapter2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiAdapter2 {
	pub fn GetDesc2(&self) -> Result<DxgiAdapterDesc2, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiAdapterDesc2> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiAdapterDesc2) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiAdapter2: IDxgiAdapter1 {
	fn as_adapter2(&self) -> &DxgiAdapter2;
	fn into_adapter2(self) -> DxgiAdapter2;
}

impl IDxgiAdapter2 for DxgiAdapter2 {
	fn as_adapter2(&self) -> &DxgiAdapter2 { self }
	fn into_adapter2(self) -> DxgiAdapter2 { self }
}
impl IDxgiAdapter1 for DxgiAdapter2 {
	fn as_adapter1(&self) -> &DxgiAdapter1 { &self.0.as_adapter1() }
	fn into_adapter1(self) -> DxgiAdapter1 { self.0.into_adapter1() }
}

impl IDxgiAdapter for DxgiAdapter2 {
	fn as_adapter(&self) -> &DxgiAdapter { &self.0.as_adapter() }
	fn into_adapter(self) -> DxgiAdapter { self.0.into_adapter() }
}

impl IDxgiObject for DxgiAdapter2 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiAdapter2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiAdapter2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiAdapter1::from(v))
    }
}

