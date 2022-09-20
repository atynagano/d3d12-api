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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiDevice(pub(crate) DxgiObject);

impl Deref for DxgiDevice {
	type Target = DxgiObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiDevice {
	const IID: &'static GUID = &GUID::from_u128(0x54ec77fa137744e68c3288fd5f44c84cu128);
}

impl Com for DxgiDevice {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiDevice {
	pub fn GetAdapter(&self) -> Result<DxgiAdapter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut adapter_out_: Option<DxgiAdapter> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, transmute(&mut adapter_out_));
			if _ret_.is_ok() { if let Some(adapter_out_) = adapter_out_ { return Ok(adapter_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn CreateSurface(&self) { todo!() }

	pub unsafe fn QueryResourceResidency() { todo!() }

	pub fn SetGPUThreadPriority(&self, priority: i32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, i32) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, priority);
			_ret_.ok()
		}
	}

	pub fn GetGPUThreadPriority(&self) -> Result<i32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut priority_out_: MaybeUninit<i32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut i32) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, priority_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(priority_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiDevice: IDxgiObject {
	fn as_device(&self) -> &DxgiDevice;
	fn into_device(self) -> DxgiDevice;
}

impl IDxgiDevice for DxgiDevice {
	fn as_device(&self) -> &DxgiDevice { self }
	fn into_device(self) -> DxgiDevice { self }
}
impl IDxgiObject for DxgiDevice {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiDevice {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiDevice {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiObject::from(v))
    }
}

