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
use crate::core::win32::graphics::gdi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1GdiInteropRenderTarget(pub(crate) Unknown);

impl Deref for D2D1GdiInteropRenderTarget {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1GdiInteropRenderTarget {
	const IID: &'static GUID = &GUID::from_u128(0xe0db51c36f774baeb3d5e47509b35838u128);
}

impl Com for D2D1GdiInteropRenderTarget {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1GdiInteropRenderTarget {
	pub fn GetDC(&self, mode: D2D1DcInitializeMode) -> Result<HDc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut hdc_out_: Option<HDc> = None;
			let f: extern "system" fn(Param<Self>, D2D1DcInitializeMode, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, mode, transmute(&mut hdc_out_));
			if _ret_.is_ok() { if let Some(hdc_out_) = hdc_out_ { return Ok(hdc_out_); } }
			Err(_ret_)
		}
	}

	pub fn ReleaseDC(&self, update: Option<&Rect>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(update));
			_ret_.ok()
		}
	}
}

pub trait ID2D1GdiInteropRenderTarget: IUnknown {
	fn as_gdi_interop_render_target(&self) -> &D2D1GdiInteropRenderTarget;
	fn into_gdi_interop_render_target(self) -> D2D1GdiInteropRenderTarget;
}

impl ID2D1GdiInteropRenderTarget for D2D1GdiInteropRenderTarget {
	fn as_gdi_interop_render_target(&self) -> &D2D1GdiInteropRenderTarget { self }
	fn into_gdi_interop_render_target(self) -> D2D1GdiInteropRenderTarget { self }
}
impl IUnknown for D2D1GdiInteropRenderTarget {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1GdiInteropRenderTarget {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

