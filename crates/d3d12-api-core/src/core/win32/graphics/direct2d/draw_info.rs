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
pub struct D2D1DrawInfo(pub(crate) D2D1RenderInfo);

impl Deref for D2D1DrawInfo {
	type Target = D2D1RenderInfo;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DrawInfo {
	const IID: &'static GUID = &GUID::from_u128(0x693ce6327f2f45de93fe18d88b37aa21u128);
}

impl Com for D2D1DrawInfo {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DrawInfo {
	pub fn SetPixelShaderConstantBuffer(&self, buffer: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (buffer_ptr_, buffer_len_) = buffer.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u8, u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, buffer_ptr_, buffer_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn SetResourceTexture(&self, texture_index: u32, resource_texture: &D2D1ResourceTexture) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, VTable) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, texture_index, resource_texture.vtable());
			_ret_.ok()
		}
	}

	pub fn SetVertexShaderConstantBuffer(&self, buffer: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (buffer_ptr_, buffer_len_) = buffer.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u8, u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, buffer_ptr_, buffer_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn SetPixelShader(&self, shader_id: &GUID, pixel_options: D2D1PixelOptions) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &GUID, D2D1PixelOptions) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, shader_id, pixel_options);
			_ret_.ok()
		}
	}

	pub unsafe fn SetVertexProcessing(&self) { todo!() }
}

pub trait ID2D1DrawInfo: ID2D1RenderInfo {
	fn as_draw_info(&self) -> &D2D1DrawInfo;
	fn into_draw_info(self) -> D2D1DrawInfo;
}

impl ID2D1DrawInfo for D2D1DrawInfo {
	fn as_draw_info(&self) -> &D2D1DrawInfo { self }
	fn into_draw_info(self) -> D2D1DrawInfo { self }
}
impl ID2D1RenderInfo for D2D1DrawInfo {
	fn as_render_info(&self) -> &D2D1RenderInfo { &self.0.as_render_info() }
	fn into_render_info(self) -> D2D1RenderInfo { self.0.into_render_info() }
}

impl IUnknown for D2D1DrawInfo {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DrawInfo {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1RenderInfo::from(v))
    }
}

