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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiOutput2(pub(crate) DxgiOutput1);

impl Deref for DxgiOutput2 {
	type Target = DxgiOutput1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiOutput2 {
	const IID: &'static GUID = &GUID::from_u128(0x595e39d12724466399b1da969de28364u128);
}

impl Com for DxgiOutput2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiOutput2 {
	pub fn SupportsOverlays(&self) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Bool
				= transmute(vt[23]);
			let _ret_ = f(vt);
			_ret_.to_bool()
		}
	}
}

pub trait IDxgiOutput2: IDxgiOutput1 {
	fn as_output2(&self) -> &DxgiOutput2;
	fn into_output2(self) -> DxgiOutput2;
}

impl IDxgiOutput2 for DxgiOutput2 {
	fn as_output2(&self) -> &DxgiOutput2 { self }
	fn into_output2(self) -> DxgiOutput2 { self }
}
impl IDxgiOutput1 for DxgiOutput2 {
	fn as_output1(&self) -> &DxgiOutput1 { &self.0.as_output1() }
	fn into_output1(self) -> DxgiOutput1 { self.0.into_output1() }
}

impl IDxgiOutput for DxgiOutput2 {
	fn as_output(&self) -> &DxgiOutput { &self.0.as_output() }
	fn into_output(self) -> DxgiOutput { self.0.into_output() }
}

impl IDxgiObject for DxgiOutput2 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiOutput2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiOutput2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiOutput1::from(v))
    }
}

