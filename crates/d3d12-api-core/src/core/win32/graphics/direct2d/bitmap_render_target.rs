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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1BitmapRenderTarget(pub(crate) D2D1RenderTarget);

impl Deref for D2D1BitmapRenderTarget {
	type Target = D2D1RenderTarget;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1BitmapRenderTarget {
	const IID: &'static GUID = &GUID::from_u128(0x2cd9069512e211dc9fed001143a055f9u128);
}

impl Com for D2D1BitmapRenderTarget {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1BitmapRenderTarget {
	pub fn GetBitmap(&self) -> Result<D2D1Bitmap, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut bitmap_out_: Option<D2D1Bitmap> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[57]);
			let _ret_ = f(vt, transmute(&mut bitmap_out_));
			if _ret_.is_ok() { if let Some(bitmap_out_) = bitmap_out_ { return Ok(bitmap_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1BitmapRenderTarget: ID2D1RenderTarget {
	fn as_bitmap_render_target(&self) -> &D2D1BitmapRenderTarget;
	fn into_bitmap_render_target(self) -> D2D1BitmapRenderTarget;
}

impl ID2D1BitmapRenderTarget for D2D1BitmapRenderTarget {
	fn as_bitmap_render_target(&self) -> &D2D1BitmapRenderTarget { self }
	fn into_bitmap_render_target(self) -> D2D1BitmapRenderTarget { self }
}
impl ID2D1RenderTarget for D2D1BitmapRenderTarget {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1BitmapRenderTarget {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1BitmapRenderTarget {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1BitmapRenderTarget {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1RenderTarget::from(v))
    }
}

