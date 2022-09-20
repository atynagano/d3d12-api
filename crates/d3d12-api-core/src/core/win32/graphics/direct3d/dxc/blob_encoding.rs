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
use crate::core::win32::graphics::direct3d::dxc::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcBlobEncoding(pub(crate) DxcBlob);

impl Deref for DxcBlobEncoding {
	type Target = DxcBlob;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcBlobEncoding {
	const IID: &'static GUID = &GUID::from_u128(0x7241d4242646419197c098e96e42fc68u128);
}

impl Com for DxcBlobEncoding {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcBlobEncoding {
	pub fn GetEncoding(&self) -> Result<(bool, DxcCp), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut known_out_ = Bool::FALSE;
			let mut code_page_out_: MaybeUninit<DxcCp> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, &mut Bool, *mut DxcCp) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, &mut known_out_, code_page_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((known_out_.to_bool(), code_page_out_.assume_init())) } else { Err(_ret_) }
		}
	}
}

pub trait IDxcBlobEncoding: IDxcBlob {
	fn as_blob_encoding(&self) -> &DxcBlobEncoding;
	fn into_blob_encoding(self) -> DxcBlobEncoding;
}

impl IDxcBlobEncoding for DxcBlobEncoding {
	fn as_blob_encoding(&self) -> &DxcBlobEncoding { self }
	fn into_blob_encoding(self) -> DxcBlobEncoding { self }
}
impl IDxcBlob for DxcBlobEncoding {
	fn as_blob(&self) -> &DxcBlob { &self.0.as_blob() }
	fn into_blob(self) -> DxcBlob { self.0.into_blob() }
}

impl IUnknown for DxcBlobEncoding {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcBlobEncoding {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxcBlob::from(v))
    }
}

