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
pub struct D2D1DCRenderTarget(pub(crate) D2D1RenderTarget);

impl Deref for D2D1DCRenderTarget {
	type Target = D2D1RenderTarget;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DCRenderTarget {
	const IID: &'static GUID = &GUID::from_u128(0x1c51bc64de6146fd989963a5d8f03950u128);
}

impl Com for D2D1DCRenderTarget {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DCRenderTarget {
	pub fn BindDC(&self, dc: HDc, sub_rect: &Rect) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, HDc, &Rect) -> HResult
				= transmute(vt[57]);
			let _ret_ = f(vt, dc, sub_rect);
			_ret_.ok()
		}
	}
}

pub trait ID2D1DCRenderTarget: ID2D1RenderTarget {
	fn as_dc_render_target(&self) -> &D2D1DCRenderTarget;
	fn into_dc_render_target(self) -> D2D1DCRenderTarget;
}

impl ID2D1DCRenderTarget for D2D1DCRenderTarget {
	fn as_dc_render_target(&self) -> &D2D1DCRenderTarget { self }
	fn into_dc_render_target(self) -> D2D1DCRenderTarget { self }
}
impl ID2D1RenderTarget for D2D1DCRenderTarget {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1DCRenderTarget {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DCRenderTarget {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DCRenderTarget {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1RenderTarget::from(v))
    }
}

