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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICStream(pub(crate) Stream);

impl Deref for WICStream {
	type Target = Stream;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICStream {
	const IID: &'static GUID = &GUID::from_u128(0x135ff86022b74ddfb0f6218f4f299a43u128);
}

impl Com for WICStream {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICStream {
	pub fn InitializeFromIStream(&self, i_stream: &Stream) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, i_stream.vtable());
			_ret_.ok()
		}
	}

	pub fn InitializeFromFilename(&self, wz_file_name: &str, desired_access: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, u32) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, wz_file_name.to_utf16().as_ptr_or_null(), desired_access);
			_ret_.ok()
		}
	}

	pub fn InitializeFromMemory(&self, pb_buffer: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (pb_buffer_ptr_, pb_buffer_len_) = pb_buffer.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u8, u32) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, pb_buffer_ptr_, pb_buffer_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn InitializeFromIStreamRegion(&self, i_stream: &Stream, ul_offset: u64, ul_max_size: u64) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u64, u64) -> HResult
				= transmute(vt[17]);
			let _ret_ = f(vt, i_stream.vtable(), ul_offset, ul_max_size);
			_ret_.ok()
		}
	}
}

pub trait IWICStream: IStream {
	fn as_stream(&self) -> &WICStream;
	fn into_stream(self) -> WICStream;
}

impl IWICStream for WICStream {
	fn as_stream(&self) -> &WICStream { self }
	fn into_stream(self) -> WICStream { self }
}
impl IStream for WICStream {
	fn as_stream(&self) -> &Stream { &self.0.as_stream() }
	fn into_stream(self) -> Stream { self.0.into_stream() }
}

impl ISequentialStream for WICStream {
	fn as_sequential_stream(&self) -> &SequentialStream { &self.0.as_sequential_stream() }
	fn into_sequential_stream(self) -> SequentialStream { self.0.into_sequential_stream() }
}

impl IUnknown for WICStream {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICStream {
    fn from(v: UnknownWrapper) -> Self {
        Self(Stream::from(v))
    }
}

