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
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SpriteBatch(pub(crate) D2D1Resource);

impl Deref for D2D1SpriteBatch {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SpriteBatch {
	const IID: &'static GUID = &GUID::from_u128(0x4dc583bf3a10438a8722e9765224f1f1u128);
}

impl Com for D2D1SpriteBatch {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SpriteBatch {
	pub fn AddSprites(&self, sprite_count: u32, destination_rectangles: &D2DRectF, source_rectangles: Option<&D2DRectU>, colors: Option<&D2D1ColorF>, transforms: Option<&D2DMatrix3x2F>, destination_rectangles_stride: u32, source_rectangles_stride: u32, colors_stride: u32, transforms_stride: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, &D2DRectF, *const c_void, *const c_void, *const c_void, u32, u32, u32, u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, sprite_count, destination_rectangles, transmute(source_rectangles), transmute(colors), transmute(transforms), destination_rectangles_stride, source_rectangles_stride, colors_stride, transforms_stride);
			_ret_.ok()
		}
	}

	pub fn SetSprites(&self, start_index: u32, sprite_count: u32, destination_rectangles: Option<&D2DRectF>, source_rectangles: Option<&D2DRectU>, colors: Option<&D2D1ColorF>, transforms: Option<&D2DMatrix3x2F>, destination_rectangles_stride: u32, source_rectangles_stride: u32, colors_stride: u32, transforms_stride: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, *const c_void, *const c_void, *const c_void, *const c_void, u32, u32, u32, u32) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, start_index, sprite_count, transmute(destination_rectangles), transmute(source_rectangles), transmute(colors), transmute(transforms), destination_rectangles_stride, source_rectangles_stride, colors_stride, transforms_stride);
			_ret_.ok()
		}
	}

	pub unsafe fn GetSprites() { todo!() }

	pub fn GetSpriteCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn Clear(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt);
		}
	}
}

pub trait ID2D1SpriteBatch: ID2D1Resource {
	fn as_sprite_batch(&self) -> &D2D1SpriteBatch;
	fn into_sprite_batch(self) -> D2D1SpriteBatch;
}

impl ID2D1SpriteBatch for D2D1SpriteBatch {
	fn as_sprite_batch(&self) -> &D2D1SpriteBatch { self }
	fn into_sprite_batch(self) -> D2D1SpriteBatch { self }
}
impl ID2D1Resource for D2D1SpriteBatch {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SpriteBatch {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SpriteBatch {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

