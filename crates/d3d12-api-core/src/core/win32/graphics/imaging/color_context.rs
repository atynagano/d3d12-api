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
use crate::core::win32::graphics::imaging::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICColorContext(pub(crate) Unknown);

impl Deref for WICColorContext {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICColorContext {
	const IID: &'static GUID = &GUID::from_u128(0x3c613a0234b244ea9a7c45aea9c6fd6du128);
}

impl Com for WICColorContext {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICColorContext {
	pub fn InitializeFromFilename(&self, wz_filename: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, wz_filename.to_utf16().as_ptr_or_null());
			_ret_.ok()
		}
	}

	pub fn InitializeFromMemory(&self, pb_buffer: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (pb_buffer_ptr_, pb_buffer_len_) = pb_buffer.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u8, u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, pb_buffer_ptr_, pb_buffer_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn InitializeFromExifColorSpace(&self, value: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, value);
			_ret_.ok()
		}
	}

	pub fn GetType(&self) -> Result<WicColorContextType, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut r#type_out_: MaybeUninit<WicColorContextType> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut WicColorContextType) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, r#type_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(r#type_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetProfileBytes(&self, pb_buffer: &mut [u8]) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let (pb_buffer_ptr_, pb_buffer_len_) = pb_buffer.deconstruct();
			let mut pcb_actual_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, u32, *mut u8, *mut u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, pb_buffer_len_ as u32, pb_buffer_ptr_, pcb_actual_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pcb_actual_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetExifColorSpace(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut value_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, value_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(value_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IWICColorContext: IUnknown {
	fn as_color_context(&self) -> &WICColorContext;
	fn into_color_context(self) -> WICColorContext;
}

impl IWICColorContext for WICColorContext {
	fn as_color_context(&self) -> &WICColorContext { self }
	fn into_color_context(self) -> WICColorContext { self }
}
impl IUnknown for WICColorContext {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICColorContext {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

