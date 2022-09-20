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
use crate::core::win32::graphics::gdi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiSurface1(pub(crate) DxgiSurface);

impl Deref for DxgiSurface1 {
	type Target = DxgiSurface;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiSurface1 {
	const IID: &'static GUID = &GUID::from_u128(0x4ae6309263274c1b80aebfe12ea32b86u128);
}

impl Com for DxgiSurface1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiSurface1 {
	pub fn GetDC(&self, discard: bool) -> Result<HDc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut phdc_out_: Option<HDc> = None;
			let f: extern "system" fn(Param<Self>, Bool, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, discard.to_bool(), transmute(&mut phdc_out_));
			if _ret_.is_ok() { if let Some(phdc_out_) = phdc_out_ { return Ok(phdc_out_); } }
			Err(_ret_)
		}
	}

	pub fn ReleaseDC(&self, dirty_rect: Option<&Rect>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(dirty_rect));
			_ret_.ok()
		}
	}
}

pub trait IDxgiSurface1: IDxgiSurface {
	fn as_surface1(&self) -> &DxgiSurface1;
	fn into_surface1(self) -> DxgiSurface1;
}

impl IDxgiSurface1 for DxgiSurface1 {
	fn as_surface1(&self) -> &DxgiSurface1 { self }
	fn into_surface1(self) -> DxgiSurface1 { self }
}
impl IDxgiSurface for DxgiSurface1 {
	fn as_surface(&self) -> &DxgiSurface { &self.0.as_surface() }
	fn into_surface(self) -> DxgiSurface { self.0.into_surface() }
}

impl IDxgiDeviceSubObject for DxgiSurface1 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiSurface1 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiSurface1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiSurface1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiSurface::from(v))
    }
}

