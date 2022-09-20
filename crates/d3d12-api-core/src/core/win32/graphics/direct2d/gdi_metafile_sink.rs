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
pub struct D2D1GdiMetafileSink(pub(crate) Unknown);

impl Deref for D2D1GdiMetafileSink {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GdiMetafileSink {
	const IID: &'static GUID = &GUID::from_u128(0x8223732681114f7cbcf4b5c1175564feu128);
}

impl Com for D2D1GdiMetafileSink {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GdiMetafileSink {
	pub fn ProcessRecord(&self, record_type: u32, record_data: *const (), record_data_size: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, *const c_void, u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, record_type, record_data as _, record_data_size);
			_ret_.ok()
		}
	}
}

pub trait ID2D1GdiMetafileSink: IUnknown {
	fn as_gdi_metafile_sink(&self) -> &D2D1GdiMetafileSink;
	fn into_gdi_metafile_sink(self) -> D2D1GdiMetafileSink;
}

impl ID2D1GdiMetafileSink for D2D1GdiMetafileSink {
	fn as_gdi_metafile_sink(&self) -> &D2D1GdiMetafileSink { self }
	fn into_gdi_metafile_sink(self) -> D2D1GdiMetafileSink { self }
}
impl IUnknown for D2D1GdiMetafileSink {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GdiMetafileSink {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

