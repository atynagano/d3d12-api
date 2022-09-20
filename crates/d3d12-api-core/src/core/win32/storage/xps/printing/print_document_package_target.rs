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
pub struct PrintDocumentPackageTarget(pub(crate) Unknown);

impl Deref for PrintDocumentPackageTarget {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for PrintDocumentPackageTarget {
	const IID: &'static GUID = &GUID::from_u128(0x1b8efec430194c27964e367202156906u128);
}

impl Com for PrintDocumentPackageTarget {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl PrintDocumentPackageTarget {
	pub unsafe fn GetPackageTargetTypes(&self) { todo!() }

	pub fn GetPackageTarget<T: IUnknown + From<UnknownWrapper>>(&self, guid_target_type: &GUID) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut target_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, &GUID, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, guid_target_type, T::IID, transmute(&mut target_out_));
			if _ret_.is_ok() { if let Some(target_out_) = target_out_ { return Ok(T::from(UnknownWrapper(target_out_))); } }
			Err(_ret_)
		}
	}

	pub fn Cancel(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait IPrintDocumentPackageTarget: IUnknown {
	fn as_print_document_package_target(&self) -> &PrintDocumentPackageTarget;
	fn into_print_document_package_target(self) -> PrintDocumentPackageTarget;
}

impl IPrintDocumentPackageTarget for PrintDocumentPackageTarget {
	fn as_print_document_package_target(&self) -> &PrintDocumentPackageTarget { self }
	fn into_print_document_package_target(self) -> PrintDocumentPackageTarget { self }
}
impl IUnknown for PrintDocumentPackageTarget {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for PrintDocumentPackageTarget {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

