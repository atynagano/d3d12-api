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
use crate::core::win32::graphics::direct_write::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWritePixelSnapping(pub(crate) Unknown);

impl Deref for DWritePixelSnapping {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWritePixelSnapping {
	const IID: &'static GUID = &GUID::from_u128(0xeaf3a2daecf44d24b644b34f6842024bu128);
}

impl Com for DWritePixelSnapping {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWritePixelSnapping {
	pub fn IsPixelSnappingDisabled(&self, client_drawing_context: *const ()) -> Result<bool, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut is_disabled_out_ = Bool::FALSE;
			let f: extern "system" fn(Param<Self>, *const c_void, &mut Bool) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, client_drawing_context as _, &mut is_disabled_out_);
			if _ret_.is_ok() { Ok(is_disabled_out_.to_bool()) } else { Err(_ret_) }
		}
	}

	pub fn GetCurrentTransform(&self, client_drawing_context: *const ()) -> Result<DWriteMatrix, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut transform_out_: MaybeUninit<DWriteMatrix> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const c_void, *mut DWriteMatrix) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, client_drawing_context as _, transform_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(transform_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetPixelsPerDip(&self, client_drawing_context: *const ()) -> Result<f32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pixels_per_dip_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const c_void, *mut f32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, client_drawing_context as _, pixels_per_dip_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pixels_per_dip_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDWritePixelSnapping: IUnknown {
	fn as_pixel_snapping(&self) -> &DWritePixelSnapping;
	fn into_pixel_snapping(self) -> DWritePixelSnapping;
}

impl IDWritePixelSnapping for DWritePixelSnapping {
	fn as_pixel_snapping(&self) -> &DWritePixelSnapping { self }
	fn into_pixel_snapping(self) -> DWritePixelSnapping { self }
}
impl IUnknown for DWritePixelSnapping {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWritePixelSnapping {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

