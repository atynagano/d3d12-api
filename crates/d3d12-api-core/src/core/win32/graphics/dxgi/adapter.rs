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
pub struct DxgiAdapter(pub(crate) DxgiObject);

impl Deref for DxgiAdapter {
	type Target = DxgiObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiAdapter {
	const IID: &'static GUID = &GUID::from_u128(0x2411e7e112ac4ccfbd149798e8534dc0u128);
}

impl Com for DxgiAdapter {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiAdapter {
	pub fn EnumOutputs(&self, output: u32) -> Result<DxgiOutput, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut output_out_: Option<DxgiOutput> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, output, transmute(&mut output_out_));
			if _ret_.is_ok() { if let Some(output_out_) = output_out_ { return Ok(output_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetDesc(&self) -> Result<DxgiAdapterDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiAdapterDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiAdapterDesc) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn CheckInterfaceSupport(&self, interface_name: &GUID) -> Result<i64, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut umd_version_out_: MaybeUninit<i64> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, &GUID, *mut i64) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, interface_name, umd_version_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(umd_version_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiAdapter: IDxgiObject {
	fn as_adapter(&self) -> &DxgiAdapter;
	fn into_adapter(self) -> DxgiAdapter;
}

impl IDxgiAdapter for DxgiAdapter {
	fn as_adapter(&self) -> &DxgiAdapter { self }
	fn into_adapter(self) -> DxgiAdapter { self }
}
impl IDxgiObject for DxgiAdapter {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiAdapter {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiAdapter {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiObject::from(v))
    }
}

