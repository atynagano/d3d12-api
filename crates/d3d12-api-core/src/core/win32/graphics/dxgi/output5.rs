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
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiOutput5(pub(crate) DxgiOutput4);

impl Deref for DxgiOutput5 {
	type Target = DxgiOutput4;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiOutput5 {
	const IID: &'static GUID = &GUID::from_u128(0x80a07424ab5242eb833c0c42fd282d98u128);
}

impl Com for DxgiOutput5 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiOutput5 {
	pub fn DuplicateOutput1(&self, device: &Unknown, flags: u32, supported_formats: &[DxgiFormat]) -> Result<DxgiOutputDuplication, HResult> {
		unsafe {
			let vt = self.as_param();
			let (supported_formats_ptr_, supported_formats_len_) = supported_formats.deconstruct();
			let mut output_duplication_out_: Option<DxgiOutputDuplication> = None;
			let f: extern "system" fn(Param<Self>, VTable, u32, u32, *const DxgiFormat, *mut c_void) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, device.vtable(), flags, supported_formats_len_ as u32, supported_formats_ptr_, transmute(&mut output_duplication_out_));
			if _ret_.is_ok() { if let Some(output_duplication_out_) = output_duplication_out_ { return Ok(output_duplication_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDxgiOutput5: IDxgiOutput4 {
	fn as_output5(&self) -> &DxgiOutput5;
	fn into_output5(self) -> DxgiOutput5;
}

impl IDxgiOutput5 for DxgiOutput5 {
	fn as_output5(&self) -> &DxgiOutput5 { self }
	fn into_output5(self) -> DxgiOutput5 { self }
}
impl IDxgiOutput4 for DxgiOutput5 {
	fn as_output4(&self) -> &DxgiOutput4 { &self.0.as_output4() }
	fn into_output4(self) -> DxgiOutput4 { self.0.into_output4() }
}

impl IDxgiOutput3 for DxgiOutput5 {
	fn as_output3(&self) -> &DxgiOutput3 { &self.0.as_output3() }
	fn into_output3(self) -> DxgiOutput3 { self.0.into_output3() }
}

impl IDxgiOutput2 for DxgiOutput5 {
	fn as_output2(&self) -> &DxgiOutput2 { &self.0.as_output2() }
	fn into_output2(self) -> DxgiOutput2 { self.0.into_output2() }
}

impl IDxgiOutput1 for DxgiOutput5 {
	fn as_output1(&self) -> &DxgiOutput1 { &self.0.as_output1() }
	fn into_output1(self) -> DxgiOutput1 { self.0.into_output1() }
}

impl IDxgiOutput for DxgiOutput5 {
	fn as_output(&self) -> &DxgiOutput { &self.0.as_output() }
	fn into_output(self) -> DxgiOutput { self.0.into_output() }
}

impl IDxgiObject for DxgiOutput5 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiOutput5 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiOutput5 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiOutput4::from(v))
    }
}

