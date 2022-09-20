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

use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1HwndRenderTarget(pub(crate) D2D1RenderTarget);

impl Deref for D2D1HwndRenderTarget {
	type Target = D2D1RenderTarget;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1HwndRenderTarget {
	const IID: &'static GUID = &GUID::from_u128(0x2cd9069812e211dc9fed001143a055f9u128);
}

impl Com for D2D1HwndRenderTarget {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1HwndRenderTarget {
	pub fn CheckWindowState(&self) -> D2D1WindowState {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1WindowState
				= transmute(vt[57]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn Resize(&self, pixel_size: &D2DSizeU) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DSizeU) -> HResult
				= transmute(vt[58]);
			let _ret_ = f(vt, pixel_size);
			_ret_.ok()
		}
	}

	pub fn GetHwnd(&self) -> Result<HWnd, Win32Error> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> *const c_void
				= transmute(vt[59]);
			let _ret_ = f(vt);
			if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
		}
	}
}

pub trait ID2D1HwndRenderTarget: ID2D1RenderTarget {
	fn as_hwnd_render_target(&self) -> &D2D1HwndRenderTarget;
	fn into_hwnd_render_target(self) -> D2D1HwndRenderTarget;
}

impl ID2D1HwndRenderTarget for D2D1HwndRenderTarget {
	fn as_hwnd_render_target(&self) -> &D2D1HwndRenderTarget { self }
	fn into_hwnd_render_target(self) -> D2D1HwndRenderTarget { self }
}
impl ID2D1RenderTarget for D2D1HwndRenderTarget {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1HwndRenderTarget {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1HwndRenderTarget {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1HwndRenderTarget {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1RenderTarget::from(v))
    }
}

