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
pub struct SequentialStream(pub(crate) Unknown);

impl Deref for SequentialStream {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for SequentialStream {
	const IID: &'static GUID = &GUID::from_u128(0x0c733a302a1c11ceade500aa0044773du128);
}

impl Com for SequentialStream {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl SequentialStream {
	pub unsafe fn Read(&self) { todo!() }

	pub fn Write(&self, pv: &[u8], pcb_written: Option<&mut MaybeUninit<u32>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (pv_ptr_, pv_len_) = pv.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u8, u32, Option<&mut MaybeUninit<u32>>) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, pv_ptr_, pv_len_ as u32, pcb_written);
			_ret_.ok()
		}
	}

	pub fn write(&self, pv: &[u8]) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let (pv_ptr_, pv_len_) = pv.deconstruct();
			let mut pcb_written_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const u8, u32, *mut u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, pv_ptr_, pv_len_ as u32, pcb_written_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pcb_written_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ISequentialStream: IUnknown {
	fn as_sequential_stream(&self) -> &SequentialStream;
	fn into_sequential_stream(self) -> SequentialStream;
}

impl ISequentialStream for SequentialStream {
	fn as_sequential_stream(&self) -> &SequentialStream { self }
	fn into_sequential_stream(self) -> SequentialStream { self }
}
impl IUnknown for SequentialStream {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for SequentialStream {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

