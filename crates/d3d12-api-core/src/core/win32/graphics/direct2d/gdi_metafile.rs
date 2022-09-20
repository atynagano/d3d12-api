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
use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1GdiMetafile(pub(crate) D2D1Resource);

impl Deref for D2D1GdiMetafile {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GdiMetafile {
	const IID: &'static GUID = &GUID::from_u128(0x2f543dc3cfc14211864fcfd91c6f3395u128);
}

impl Com for D2D1GdiMetafile {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GdiMetafile {
	pub fn Stream(&self, sink: &D2D1GdiMetafileSink) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, sink.vtable());
			_ret_.ok()
		}
	}

	pub fn GetBounds(&self) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2DRectF) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1GdiMetafile: ID2D1Resource {
	fn as_gdi_metafile(&self) -> &D2D1GdiMetafile;
	fn into_gdi_metafile(self) -> D2D1GdiMetafile;
}

impl ID2D1GdiMetafile for D2D1GdiMetafile {
	fn as_gdi_metafile(&self) -> &D2D1GdiMetafile { self }
	fn into_gdi_metafile(self) -> D2D1GdiMetafile { self }
}
impl ID2D1Resource for D2D1GdiMetafile {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1GdiMetafile {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GdiMetafile {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

