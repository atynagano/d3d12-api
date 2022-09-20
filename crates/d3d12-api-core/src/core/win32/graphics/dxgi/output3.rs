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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiOutput3(pub(crate) DxgiOutput2);

impl Deref for DxgiOutput3 {
	type Target = DxgiOutput2;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiOutput3 {
	const IID: &'static GUID = &GUID::from_u128(0x8a6bb3017e7e41f4a8e05b32f7f99b18u128);
}

impl Com for DxgiOutput3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiOutput3 {
	pub fn CheckOverlaySupport(&self, enum_format: DxgiFormat, concerned_device: &Unknown) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut flags_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, DxgiFormat, VTable, *mut u32) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, enum_format, concerned_device.vtable(), flags_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(flags_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiOutput3: IDxgiOutput2 {
	fn as_output3(&self) -> &DxgiOutput3;
	fn into_output3(self) -> DxgiOutput3;
}

impl IDxgiOutput3 for DxgiOutput3 {
	fn as_output3(&self) -> &DxgiOutput3 { self }
	fn into_output3(self) -> DxgiOutput3 { self }
}
impl IDxgiOutput2 for DxgiOutput3 {
	fn as_output2(&self) -> &DxgiOutput2 { &self.0.as_output2() }
	fn into_output2(self) -> DxgiOutput2 { self.0.into_output2() }
}

impl IDxgiOutput1 for DxgiOutput3 {
	fn as_output1(&self) -> &DxgiOutput1 { &self.0.as_output1() }
	fn into_output1(self) -> DxgiOutput1 { self.0.into_output1() }
}

impl IDxgiOutput for DxgiOutput3 {
	fn as_output(&self) -> &DxgiOutput { &self.0.as_output() }
	fn into_output(self) -> DxgiOutput { self.0.into_output() }
}

impl IDxgiObject for DxgiOutput3 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiOutput3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiOutput3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiOutput2::from(v))
    }
}

