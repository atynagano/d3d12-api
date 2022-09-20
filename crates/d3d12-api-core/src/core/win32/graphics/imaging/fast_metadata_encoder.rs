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
pub struct WICFastMetadataEncoder(pub(crate) Unknown);

impl Deref for WICFastMetadataEncoder {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICFastMetadataEncoder {
	const IID: &'static GUID = &GUID::from_u128(0xb84e2c0978c94ac48bd3524ae1663a2fu128);
}

impl Com for WICFastMetadataEncoder {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICFastMetadataEncoder {
	pub fn Commit(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn GetMetadataQueryWriter(&self) -> Result<WICMetadataQueryWriter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_metadata_query_writer_out_: Option<WICMetadataQueryWriter> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(&mut i_metadata_query_writer_out_));
			if _ret_.is_ok() { if let Some(i_metadata_query_writer_out_) = i_metadata_query_writer_out_ { return Ok(i_metadata_query_writer_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICFastMetadataEncoder: IUnknown {
	fn as_fast_metadata_encoder(&self) -> &WICFastMetadataEncoder;
	fn into_fast_metadata_encoder(self) -> WICFastMetadataEncoder;
}

impl IWICFastMetadataEncoder for WICFastMetadataEncoder {
	fn as_fast_metadata_encoder(&self) -> &WICFastMetadataEncoder { self }
	fn into_fast_metadata_encoder(self) -> WICFastMetadataEncoder { self }
}
impl IUnknown for WICFastMetadataEncoder {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICFastMetadataEncoder {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

