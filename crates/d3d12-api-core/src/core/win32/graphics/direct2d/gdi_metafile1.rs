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
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1GdiMetafile1(pub(crate) D2D1GdiMetafile);

impl Deref for D2D1GdiMetafile1 {
	type Target = D2D1GdiMetafile;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GdiMetafile1 {
	const IID: &'static GUID = &GUID::from_u128(0x2e69f9e8dd3f4bf995bac04f49d788dfu128);
}

impl Com for D2D1GdiMetafile1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GdiMetafile1 {
	pub fn GetDpi(&self) -> Result<(f32, f32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut dpi_x_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut dpi_y_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut f32, *mut f32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, dpi_x_out_.as_mut_ptr(), dpi_y_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((dpi_x_out_.assume_init(), dpi_y_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetSourceBounds(&self) -> Result<D2DRectF, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bounds_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2DRectF) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, bounds_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(bounds_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1GdiMetafile1: ID2D1GdiMetafile {
	fn as_gdi_metafile1(&self) -> &D2D1GdiMetafile1;
	fn into_gdi_metafile1(self) -> D2D1GdiMetafile1;
}

impl ID2D1GdiMetafile1 for D2D1GdiMetafile1 {
	fn as_gdi_metafile1(&self) -> &D2D1GdiMetafile1 { self }
	fn into_gdi_metafile1(self) -> D2D1GdiMetafile1 { self }
}
impl ID2D1GdiMetafile for D2D1GdiMetafile1 {
	fn as_gdi_metafile(&self) -> &D2D1GdiMetafile { &self.0.as_gdi_metafile() }
	fn into_gdi_metafile(self) -> D2D1GdiMetafile { self.0.into_gdi_metafile() }
}

impl ID2D1Resource for D2D1GdiMetafile1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1GdiMetafile1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GdiMetafile1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1GdiMetafile::from(v))
    }
}

