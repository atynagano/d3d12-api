#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{MaybeUninit, size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct3d::dxc::*;

#[repr(C)]
pub struct DxcBlobEncoding(pub(crate) DxcBlob);

impl Guid for DxcBlobEncoding {
	const IID: &'static GUID = &GUID::from_u128(0x7241d4242646419197c098e96e42fc68u128);
}

impl Clone for DxcBlobEncoding {
	fn clone(&self) -> Self { DxcBlobEncoding(self.0.clone()) }
}

pub trait IDxcBlobEncoding: IDxcBlob {
	fn as_blob_encoding(&self) -> &DxcBlobEncoding;
	fn into_blob_encoding(self) -> DxcBlobEncoding;

	fn GetEncoding(&self, ) -> Result<(bool, DxcCp, ), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_known = Bool { value: 0 };
			let mut _out_code_page: MaybeUninit<DxcCp> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_known: &mut Bool, _out_code_page: *mut DxcCp, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, &mut _out_known, _out_code_page.as_mut_ptr(), );
			if _ret_.is_ok() {
				return Ok((_out_known.to_bool(), _out_code_page.assume_init(), ));
			}
			Err(_ret_)
		}
	}
}

impl IDxcBlobEncoding for DxcBlobEncoding {
	fn as_blob_encoding(&self) -> &DxcBlobEncoding { self }
	fn into_blob_encoding(self) -> DxcBlobEncoding { self }
}

impl IDxcBlob for DxcBlobEncoding {
	fn as_blob(&self) -> &DxcBlob { &self.0.as_blob() }
	fn into_blob(self) -> DxcBlob { self.0.into_blob() }
}

impl From<Unknown> for DxcBlobEncoding {
    fn from(v: Unknown) -> Self {
        Self(DxcBlob::from(v))
    }
}

impl IUnknown for DxcBlobEncoding {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

