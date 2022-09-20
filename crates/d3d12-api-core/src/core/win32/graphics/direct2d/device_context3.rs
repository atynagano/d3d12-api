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
pub struct D2D1DeviceContext3(pub(crate) D2D1DeviceContext2);

impl Deref for D2D1DeviceContext3 {
	type Target = D2D1DeviceContext2;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DeviceContext3 {
	const IID: &'static GUID = &GUID::from_u128(0x235a74968351414cbcd46672ab2d8e00u128);
}

impl Com for D2D1DeviceContext3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DeviceContext3 {
	pub fn CreateSpriteBatch(&self) -> Result<D2D1SpriteBatch, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut sprite_batch_out_: Option<D2D1SpriteBatch> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[106]);
			let _ret_ = f(vt, transmute(&mut sprite_batch_out_));
			if _ret_.is_ok() { if let Some(sprite_batch_out_) = sprite_batch_out_ { return Ok(sprite_batch_out_); } }
			Err(_ret_)
		}
	}

	pub fn DrawSpriteBatch(&self, sprite_batch: &D2D1SpriteBatch, start_index: u32, sprite_count: u32, bitmap: &D2D1Bitmap, interpolation_mode: D2D1BitmapInterpolationMode, sprite_options: D2D1SpriteOptions) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, u32, VTable, D2D1BitmapInterpolationMode, D2D1SpriteOptions) -> ()
				= transmute(vt[107]);
			let _ret_ = f(vt, sprite_batch.vtable(), start_index, sprite_count, bitmap.vtable(), interpolation_mode, sprite_options);
		}
	}
}

pub trait ID2D1DeviceContext3: ID2D1DeviceContext2 {
	fn as_device_context3(&self) -> &D2D1DeviceContext3;
	fn into_device_context3(self) -> D2D1DeviceContext3;
}

impl ID2D1DeviceContext3 for D2D1DeviceContext3 {
	fn as_device_context3(&self) -> &D2D1DeviceContext3 { self }
	fn into_device_context3(self) -> D2D1DeviceContext3 { self }
}
impl ID2D1DeviceContext2 for D2D1DeviceContext3 {
	fn as_device_context2(&self) -> &D2D1DeviceContext2 { &self.0.as_device_context2() }
	fn into_device_context2(self) -> D2D1DeviceContext2 { self.0.into_device_context2() }
}

impl ID2D1DeviceContext1 for D2D1DeviceContext3 {
	fn as_device_context1(&self) -> &D2D1DeviceContext1 { &self.0.as_device_context1() }
	fn into_device_context1(self) -> D2D1DeviceContext1 { self.0.into_device_context1() }
}

impl ID2D1DeviceContext for D2D1DeviceContext3 {
	fn as_device_context(&self) -> &D2D1DeviceContext { &self.0.as_device_context() }
	fn into_device_context(self) -> D2D1DeviceContext { self.0.into_device_context() }
}

impl ID2D1RenderTarget for D2D1DeviceContext3 {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1DeviceContext3 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DeviceContext3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DeviceContext3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1DeviceContext2::from(v))
    }
}

