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
use crate::core::win32::graphics::direct_write::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWriteFontFileLoader(pub(crate) Unknown);

impl Deref for DWriteFontFileLoader {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteFontFileLoader {
	const IID: &'static GUID = &GUID::from_u128(0x727cad4ed6af4c9e8a08d695b11caa49u128);
}

impl Com for DWriteFontFileLoader {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteFontFileLoader {
	pub fn CreateStreamFromKey(&self, font_file_reference_key: &[u8]) -> Result<DWriteFontFileStream, HResult> {
		unsafe {
			let vt = self.as_param();
			let (font_file_reference_key_ptr_, font_file_reference_key_len_) = font_file_reference_key.deconstruct();
			let mut font_file_stream_out_: Option<DWriteFontFileStream> = None;
			let f: extern "system" fn(Param<Self>, *const u8, u32, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, font_file_reference_key_ptr_, font_file_reference_key_len_ as u32, transmute(&mut font_file_stream_out_));
			if _ret_.is_ok() { if let Some(font_file_stream_out_) = font_file_stream_out_ { return Ok(font_file_stream_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IDWriteFontFileLoader: IUnknown {
	fn as_font_file_loader(&self) -> &DWriteFontFileLoader;
	fn into_font_file_loader(self) -> DWriteFontFileLoader;
}

impl IDWriteFontFileLoader for DWriteFontFileLoader {
	fn as_font_file_loader(&self) -> &DWriteFontFileLoader { self }
	fn into_font_file_loader(self) -> DWriteFontFileLoader { self }
}
impl IUnknown for DWriteFontFileLoader {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteFontFileLoader {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

