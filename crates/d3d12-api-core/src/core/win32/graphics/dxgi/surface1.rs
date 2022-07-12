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
use crate::core::win32::graphics::gdi::*;

#[repr(C)]
pub struct DxgiSurface1(pub(crate) DxgiSurface);

impl Guid for DxgiSurface1 {
	const IID: &'static GUID = &GUID::from_u128(0x4ae6309263274c1b80aebfe12ea32b86u128);
}

impl Clone for DxgiSurface1 {
	fn clone(&self) -> Self { DxgiSurface1(self.0.clone()) }
}

pub trait IDxgiSurface1: IDxgiSurface {
	fn as_surface1(&self) -> &DxgiSurface1;
	fn into_surface1(self) -> DxgiSurface1;

	fn GetDC(&self, discard: bool, ) -> Result<HDc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_phdc: Option<HDc> = None;
			let f: extern "system" fn(Param<Self>, discard: Bool, _out_phdc: *mut c_void, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, discard.to_bool(), transmute(&mut _out_phdc), );
			if _ret_.is_ok() {
				if let Some(_out_phdc) = _out_phdc {
					return Ok(_out_phdc);
				}
			}
			Err(_ret_)
		}
	}

	fn ReleaseDC(&self, dirty_rect: Option<&Rect>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, dirty_rect: *const c_void, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, transmute(dirty_rect), );
			_ret_.ok()
		}
	}
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

impl From<Unknown> for DxgiSurface1 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSurface::from(v))
    }
}

impl IUnknown for DxgiSurface1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

