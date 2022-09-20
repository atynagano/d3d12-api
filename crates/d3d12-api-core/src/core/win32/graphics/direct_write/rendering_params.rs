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

use crate::core::win32::graphics::direct_write::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWriteRenderingParams(pub(crate) Unknown);

impl Deref for DWriteRenderingParams {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteRenderingParams {
	const IID: &'static GUID = &GUID::from_u128(0x2f0da53a2add47cd82eed9ec34688e75u128);
}

impl Com for DWriteRenderingParams {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteRenderingParams {
	pub fn GetGamma(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetEnhancedContrast(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetClearTypeLevel(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetPixelGeometry(&self) -> DWritePixelGeometry {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWritePixelGeometry
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetRenderingMode(&self) -> DWriteRenderingMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DWriteRenderingMode
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait IDWriteRenderingParams: IUnknown {
	fn as_rendering_params(&self) -> &DWriteRenderingParams;
	fn into_rendering_params(self) -> DWriteRenderingParams;
}

impl IDWriteRenderingParams for DWriteRenderingParams {
	fn as_rendering_params(&self) -> &DWriteRenderingParams { self }
	fn into_rendering_params(self) -> DWriteRenderingParams { self }
}
impl IUnknown for DWriteRenderingParams {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteRenderingParams {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

