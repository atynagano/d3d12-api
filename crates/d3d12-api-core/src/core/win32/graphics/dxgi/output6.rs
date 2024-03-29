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
pub struct DxgiOutput6(pub(crate) DxgiOutput5);

impl Deref for DxgiOutput6 {
	type Target = DxgiOutput5;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiOutput6 {
	const IID: &'static GUID = &GUID::from_u128(0x068346e8aaec4b84add7137f513f77a1u128);
}

impl Com for DxgiOutput6 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiOutput6 {
	pub fn GetDesc1(&self) -> Result<DxgiOutputDesc1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiOutputDesc1> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiOutputDesc1) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn CheckHardwareCompositionSupport(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut flags_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, flags_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(flags_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiOutput6: IDxgiOutput5 {
	fn as_output6(&self) -> &DxgiOutput6;
	fn into_output6(self) -> DxgiOutput6;
}

impl IDxgiOutput6 for DxgiOutput6 {
	fn as_output6(&self) -> &DxgiOutput6 { self }
	fn into_output6(self) -> DxgiOutput6 { self }
}
impl IDxgiOutput5 for DxgiOutput6 {
	fn as_output5(&self) -> &DxgiOutput5 { &self.0.as_output5() }
	fn into_output5(self) -> DxgiOutput5 { self.0.into_output5() }
}

impl IDxgiOutput4 for DxgiOutput6 {
	fn as_output4(&self) -> &DxgiOutput4 { &self.0.as_output4() }
	fn into_output4(self) -> DxgiOutput4 { self.0.into_output4() }
}

impl IDxgiOutput3 for DxgiOutput6 {
	fn as_output3(&self) -> &DxgiOutput3 { &self.0.as_output3() }
	fn into_output3(self) -> DxgiOutput3 { self.0.into_output3() }
}

impl IDxgiOutput2 for DxgiOutput6 {
	fn as_output2(&self) -> &DxgiOutput2 { &self.0.as_output2() }
	fn into_output2(self) -> DxgiOutput2 { self.0.into_output2() }
}

impl IDxgiOutput1 for DxgiOutput6 {
	fn as_output1(&self) -> &DxgiOutput1 { &self.0.as_output1() }
	fn into_output1(self) -> DxgiOutput1 { self.0.into_output1() }
}

impl IDxgiOutput for DxgiOutput6 {
	fn as_output(&self) -> &DxgiOutput { &self.0.as_output() }
	fn into_output(self) -> DxgiOutput { self.0.into_output() }
}

impl IDxgiObject for DxgiOutput6 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiOutput6 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiOutput6 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiOutput5::from(v))
    }
}

