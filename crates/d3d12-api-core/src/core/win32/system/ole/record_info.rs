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
use crate::core::win32::system::ole::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct RecordInfo(pub(crate) Unknown);

impl Deref for RecordInfo {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for RecordInfo {
	const IID: &'static GUID = &GUID::from_u128(0x0000002f00000000c000000000000046u128);
}

impl Com for RecordInfo {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl RecordInfo {
	pub fn RecordClear(&self, pv_existing: *const impl Sized, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, pv_existing: *const c_void, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, pv_existing as _, );
			_ret_.ok()
		}
	}

	pub fn GetName(&self, ) -> Result<BStr, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pbstr_name_out_: Option<BStr> = None;
			let f: extern "system" fn(Param<Self>, pbstr_name_out_: *mut c_void, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, transmute(&mut pbstr_name_out_), );
			if _ret_.is_ok() {
				if let Some(pbstr_name_out_) = pbstr_name_out_ {
					return Ok(pbstr_name_out_);
				}
			}
			Err(_ret_)
		}
	}

	pub fn GetSize(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pcb_size_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, pcb_size_out_: *mut u32, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, pcb_size_out_.as_mut_ptr(), );
			Ok(pcb_size_out_.assume_init())
		}
	}

	pub fn GetTypeInfo(&self, ) -> Result<TypeInfo, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut type_info_out_: Option<TypeInfo> = None;
			let f: extern "system" fn(Param<Self>, type_info: *mut c_void, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, transmute(&mut type_info_out_), );
			if _ret_.is_ok() {
				if let Some(type_info_out_) = type_info_out_ {
					return Ok(type_info_out_);
				}
			}
			Err(_ret_)
		}
	}

	pub fn GetField(&self, pv_data: *const impl Sized, sz_field_name: &str, ) -> Result<Variant, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pvar_field_out_: MaybeUninit<Variant> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, pv_data: *const c_void, sz_field_name: *const u16, pvar_field_out_: *mut Variant, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, pv_data as _, sz_field_name.to_utf16().as_ptr_or_null(), pvar_field_out_.as_mut_ptr(), );
			Ok(pvar_field_out_.assume_init())
		}
	}

	pub fn PutField(&self, flags: u32, pv_data: &mut (), sz_field_name: &str, pvar_field: &Variant, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, flags: u32, pv_data: &mut (), sz_field_name: *const u16, pvar_field: &Variant, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, flags, pv_data, sz_field_name.to_utf16().as_ptr_or_null(), pvar_field, );
			_ret_.ok()
		}
	}

	pub fn PutFieldNoCopy(&self, flags: u32, pv_data: &mut (), sz_field_name: &str, pvar_field: &Variant, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, flags: u32, pv_data: &mut (), sz_field_name: *const u16, pvar_field: &Variant, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, flags, pv_data, sz_field_name.to_utf16().as_ptr_or_null(), pvar_field, );
			_ret_.ok()
		}
	}

	pub fn IsMatchingType(&self, record_info: &RecordInfo, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, record_info: VTable, ) -> Bool
				= transmute(vt[15]);
			let _ret_ = f(vt, record_info.vtable(), );
			_ret_.to_bool()
		}
	}

	pub fn RecordDestroy(&self, pv_record: *const impl Sized, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, pv_record: *const c_void, ) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, pv_record as _, );
			_ret_.ok()
		}
	}
}

pub trait IRecordInfo: IUnknown {
	fn as_record_info(&self) -> &RecordInfo;
	fn into_record_info(self) -> RecordInfo;
}

impl IRecordInfo for RecordInfo {
	fn as_record_info(&self) -> &RecordInfo { self }
	fn into_record_info(self) -> RecordInfo { self }
}
impl IUnknown for RecordInfo {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for RecordInfo {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

