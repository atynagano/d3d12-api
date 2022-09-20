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
use crate::core::win32::system::com::structured_storage::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICMetadataQueryWriter(pub(crate) WICMetadataQueryReader);

impl Deref for WICMetadataQueryWriter {
	type Target = WICMetadataQueryReader;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICMetadataQueryWriter {
	const IID: &'static GUID = &GUID::from_u128(0xa721791a0def4d06bd912118bf1db10bu128);
}

impl Com for WICMetadataQueryWriter {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICMetadataQueryWriter {
	pub fn SetMetadataByName(&self, wz_name: &str, pvar_value: &PropVariant) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, &PropVariant) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, wz_name.to_utf16().as_ptr_or_null(), pvar_value);
			_ret_.ok()
		}
	}

	pub fn RemoveMetadataByName(&self, wz_name: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, wz_name.to_utf16().as_ptr_or_null());
			_ret_.ok()
		}
	}
}

pub trait IWICMetadataQueryWriter: IWICMetadataQueryReader {
	fn as_metadata_query_writer(&self) -> &WICMetadataQueryWriter;
	fn into_metadata_query_writer(self) -> WICMetadataQueryWriter;
}

impl IWICMetadataQueryWriter for WICMetadataQueryWriter {
	fn as_metadata_query_writer(&self) -> &WICMetadataQueryWriter { self }
	fn into_metadata_query_writer(self) -> WICMetadataQueryWriter { self }
}
impl IWICMetadataQueryReader for WICMetadataQueryWriter {
	fn as_metadata_query_reader(&self) -> &WICMetadataQueryReader { &self.0.as_metadata_query_reader() }
	fn into_metadata_query_reader(self) -> WICMetadataQueryReader { self.0.into_metadata_query_reader() }
}

impl IUnknown for WICMetadataQueryWriter {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICMetadataQueryWriter {
    fn from(v: UnknownWrapper) -> Self {
        Self(WICMetadataQueryReader::from(v))
    }
}

