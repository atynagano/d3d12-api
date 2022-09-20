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
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1DeviceContext6(pub(crate) D2D1DeviceContext5);

impl Deref for D2D1DeviceContext6 {
	type Target = D2D1DeviceContext5;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DeviceContext6 {
	const IID: &'static GUID = &GUID::from_u128(0x985f7e374ed04a1998a315b0edfde306u128);
}

impl Com for D2D1DeviceContext6 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DeviceContext6 {
	pub fn BlendImage(&self, image: &D2D1Image, blend_mode: D2D1BlendMode, target_offset: Option<&D2DPoint2F>, image_rectangle: Option<&D2DRectF>, interpolation_mode: D2D1InterpolationMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, D2D1BlendMode, *const c_void, *const c_void, D2D1InterpolationMode) -> ()
				= transmute(vt[119]);
			let _ret_ = f(vt, image.vtable(), blend_mode, transmute(target_offset), transmute(image_rectangle), interpolation_mode);
		}
	}
}

pub trait ID2D1DeviceContext6: ID2D1DeviceContext5 {
	fn as_device_context6(&self) -> &D2D1DeviceContext6;
	fn into_device_context6(self) -> D2D1DeviceContext6;
}

impl ID2D1DeviceContext6 for D2D1DeviceContext6 {
	fn as_device_context6(&self) -> &D2D1DeviceContext6 { self }
	fn into_device_context6(self) -> D2D1DeviceContext6 { self }
}
impl ID2D1DeviceContext5 for D2D1DeviceContext6 {
	fn as_device_context5(&self) -> &D2D1DeviceContext5 { &self.0.as_device_context5() }
	fn into_device_context5(self) -> D2D1DeviceContext5 { self.0.into_device_context5() }
}

impl ID2D1DeviceContext4 for D2D1DeviceContext6 {
	fn as_device_context4(&self) -> &D2D1DeviceContext4 { &self.0.as_device_context4() }
	fn into_device_context4(self) -> D2D1DeviceContext4 { self.0.into_device_context4() }
}

impl ID2D1DeviceContext3 for D2D1DeviceContext6 {
	fn as_device_context3(&self) -> &D2D1DeviceContext3 { &self.0.as_device_context3() }
	fn into_device_context3(self) -> D2D1DeviceContext3 { self.0.into_device_context3() }
}

impl ID2D1DeviceContext2 for D2D1DeviceContext6 {
	fn as_device_context2(&self) -> &D2D1DeviceContext2 { &self.0.as_device_context2() }
	fn into_device_context2(self) -> D2D1DeviceContext2 { self.0.into_device_context2() }
}

impl ID2D1DeviceContext1 for D2D1DeviceContext6 {
	fn as_device_context1(&self) -> &D2D1DeviceContext1 { &self.0.as_device_context1() }
	fn into_device_context1(self) -> D2D1DeviceContext1 { self.0.into_device_context1() }
}

impl ID2D1DeviceContext for D2D1DeviceContext6 {
	fn as_device_context(&self) -> &D2D1DeviceContext { &self.0.as_device_context() }
	fn into_device_context(self) -> D2D1DeviceContext { self.0.into_device_context() }
}

impl ID2D1RenderTarget for D2D1DeviceContext6 {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1DeviceContext6 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DeviceContext6 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DeviceContext6 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1DeviceContext5::from(v))
    }
}

