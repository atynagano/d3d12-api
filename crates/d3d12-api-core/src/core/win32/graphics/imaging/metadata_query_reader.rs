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
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICMetadataQueryReader(pub(crate) Unknown);

impl Deref for WICMetadataQueryReader {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICMetadataQueryReader {
	const IID: &'static GUID = &GUID::from_u128(0x30989668e1c94597b395458eedb808dfu128);
}

impl Com for WICMetadataQueryReader {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICMetadataQueryReader {
	pub unsafe fn GetContainerFormat(&self) { todo!() }

	pub unsafe fn GetLocation(&self) { todo!() }

	pub fn GetMetadataByName(&self, wz_name: &str, pvar_value: &mut PropVariant) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16, &mut PropVariant) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, wz_name.to_utf16().as_ptr_or_null(), pvar_value);
			_ret_.ok()
		}
	}

	pub fn GetEnumerator(&self) -> Result<EnumString, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut i_enum_string_out_: Option<EnumString> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, transmute(&mut i_enum_string_out_));
			if _ret_.is_ok() { if let Some(i_enum_string_out_) = i_enum_string_out_ { return Ok(i_enum_string_out_); } }
			Err(_ret_)
		}
	}
}

pub trait IWICMetadataQueryReader: IUnknown {
	fn as_metadata_query_reader(&self) -> &WICMetadataQueryReader;
	fn into_metadata_query_reader(self) -> WICMetadataQueryReader;
}

impl IWICMetadataQueryReader for WICMetadataQueryReader {
	fn as_metadata_query_reader(&self) -> &WICMetadataQueryReader { self }
	fn into_metadata_query_reader(self) -> WICMetadataQueryReader { self }
}
impl IUnknown for WICMetadataQueryReader {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICMetadataQueryReader {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

