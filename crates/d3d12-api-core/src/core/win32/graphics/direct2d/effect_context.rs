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
use crate::core::win32::graphics::direct3d::*;
use crate::core::win32::graphics::imaging::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1EffectContext(pub(crate) Unknown);

impl Deref for D2D1EffectContext {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1EffectContext {
	const IID: &'static GUID = &GUID::from_u128(0x3d9f916b27dc4ad7b4f164945340f563u128);
}

impl Com for D2D1EffectContext {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1EffectContext {
	pub fn GetDpi(&self) -> (f32, f32) {
		unsafe {
			let vt = self.as_param();
			let mut dpi_x_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let mut dpi_y_out_: MaybeUninit<f32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut f32, *mut f32) -> ()
				= transmute(vt[3]);
			let _ret_ = f(vt, dpi_x_out_.as_mut_ptr(), dpi_y_out_.as_mut_ptr());
			(dpi_x_out_.assume_init(), dpi_y_out_.assume_init())
		}
	}

	pub fn CreateEffect(&self, effect_id: &GUID) -> Result<D2D1Effect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut effect_out_: Option<D2D1Effect> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, effect_id, transmute(&mut effect_out_));
			if _ret_.is_ok() { if let Some(effect_out_) = effect_out_ { return Ok(effect_out_); } }
			Err(_ret_)
		}
	}

	pub fn GetMaximumSupportedFeatureLevel(&self, feature_levels: &[D3DFeatureLevel]) -> Result<D3DFeatureLevel, HResult> {
		unsafe {
			let vt = self.as_param();
			let (feature_levels_ptr_, feature_levels_len_) = feature_levels.deconstruct();
			let mut maximum_supported_feature_level_out_: MaybeUninit<D3DFeatureLevel> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *const D3DFeatureLevel, u32, *mut D3DFeatureLevel) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, feature_levels_ptr_, feature_levels_len_ as u32, maximum_supported_feature_level_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(maximum_supported_feature_level_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn CreateTransformNodeFromEffect(&self, effect: &D2D1Effect) -> Result<D2D1TransformNode, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut transform_node_out_: Option<D2D1TransformNode> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, effect.vtable(), transmute(&mut transform_node_out_));
			if _ret_.is_ok() { if let Some(transform_node_out_) = transform_node_out_ { return Ok(transform_node_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBlendTransform(&self, num_inputs: u32, blend_description: &D2D1BlendDescription) -> Result<D2D1BlendTransform, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut transform_out_: Option<D2D1BlendTransform> = None;
			let f: extern "system" fn(Param<Self>, u32, &D2D1BlendDescription, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, num_inputs, blend_description, transmute(&mut transform_out_));
			if _ret_.is_ok() { if let Some(transform_out_) = transform_out_ { return Ok(transform_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBorderTransform(&self, extend_mode_x: D2D1ExtendMode, extend_mode_y: D2D1ExtendMode) -> Result<D2D1BorderTransform, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut transform_out_: Option<D2D1BorderTransform> = None;
			let f: extern "system" fn(Param<Self>, D2D1ExtendMode, D2D1ExtendMode, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, extend_mode_x, extend_mode_y, transmute(&mut transform_out_));
			if _ret_.is_ok() { if let Some(transform_out_) = transform_out_ { return Ok(transform_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateOffsetTransform(&self, offset: Point) -> Result<D2D1OffsetTransform, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut transform_out_: Option<D2D1OffsetTransform> = None;
			let f: extern "system" fn(Param<Self>, Point, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, offset, transmute(&mut transform_out_));
			if _ret_.is_ok() { if let Some(transform_out_) = transform_out_ { return Ok(transform_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateBoundsAdjustmentTransform(&self, output_rectangle: &Rect) -> Result<D2D1BoundsAdjustmentTransform, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut transform_out_: Option<D2D1BoundsAdjustmentTransform> = None;
			let f: extern "system" fn(Param<Self>, &Rect, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, output_rectangle, transmute(&mut transform_out_));
			if _ret_.is_ok() { if let Some(transform_out_) = transform_out_ { return Ok(transform_out_); } }
			Err(_ret_)
		}
	}

	pub fn LoadPixelShader(&self, shader_id: &GUID, shader_buffer: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (shader_buffer_ptr_, shader_buffer_len_) = shader_buffer.deconstruct();
			let f: extern "system" fn(Param<Self>, &GUID, *const u8, u32) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, shader_id, shader_buffer_ptr_, shader_buffer_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn LoadVertexShader(&self, resource_id: &GUID, shader_buffer: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (shader_buffer_ptr_, shader_buffer_len_) = shader_buffer.deconstruct();
			let f: extern "system" fn(Param<Self>, &GUID, *const u8, u32) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, resource_id, shader_buffer_ptr_, shader_buffer_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn LoadComputeShader(&self, resource_id: &GUID, shader_buffer: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (shader_buffer_ptr_, shader_buffer_len_) = shader_buffer.deconstruct();
			let f: extern "system" fn(Param<Self>, &GUID, *const u8, u32) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, resource_id, shader_buffer_ptr_, shader_buffer_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn IsShaderLoaded(&self, shader_id: &GUID) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &GUID) -> Bool
				= transmute(vt[14]);
			let _ret_ = f(vt, shader_id);
			_ret_.to_bool()
		}
	}

	pub unsafe fn CreateResourceTexture(&self) { todo!() }

	pub fn FindResourceTexture(&self, resource_id: &GUID) -> Result<D2D1ResourceTexture, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_texture_out_: Option<D2D1ResourceTexture> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, resource_id, transmute(&mut resource_texture_out_));
			if _ret_.is_ok() { if let Some(resource_texture_out_) = resource_texture_out_ { return Ok(resource_texture_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn CreateVertexBuffer(&self) { todo!() }

	pub fn FindVertexBuffer(&self, resource_id: &GUID) -> Result<D2D1VertexBuffer, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut buffer_out_: Option<D2D1VertexBuffer> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[18]);
			let _ret_ = f(vt, resource_id, transmute(&mut buffer_out_));
			if _ret_.is_ok() { if let Some(buffer_out_) = buffer_out_ { return Ok(buffer_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContext(&self, space: D2D1ColorSpace, profile: Option<&[u8]>) -> Result<D2D1ColorContext, HResult> {
		unsafe {
			let vt = self.as_param();
			let (profile_ptr_, profile_len_) = profile.deconstruct();
			let mut color_context_out_: Option<D2D1ColorContext> = None;
			let f: extern "system" fn(Param<Self>, D2D1ColorSpace, *const u8, u32, *mut c_void) -> HResult
				= transmute(vt[19]);
			let _ret_ = f(vt, space, profile_ptr_, profile_len_ as u32, transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContextFromFilename(&self, filename: &str) -> Result<D2D1ColorContext, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_context_out_: Option<D2D1ColorContext> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *mut c_void) -> HResult
				= transmute(vt[20]);
			let _ret_ = f(vt, filename.to_utf16().as_ptr_or_null(), transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContextFromWicColorContext(&self, wic_color_context: &WICColorContext) -> Result<D2D1ColorContext, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_context_out_: Option<D2D1ColorContext> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[21]);
			let _ret_ = f(vt, wic_color_context.vtable(), transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}

	pub unsafe fn CheckFeatureSupport(&self) { todo!() }

	pub fn IsBufferPrecisionSupported(&self, buffer_precision: D2D1BufferPrecision) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1BufferPrecision) -> Bool
				= transmute(vt[23]);
			let _ret_ = f(vt, buffer_precision);
			_ret_.to_bool()
		}
	}
}

pub trait ID2D1EffectContext: IUnknown {
	fn as_effect_context(&self) -> &D2D1EffectContext;
	fn into_effect_context(self) -> D2D1EffectContext;
}

impl ID2D1EffectContext for D2D1EffectContext {
	fn as_effect_context(&self) -> &D2D1EffectContext { self }
	fn into_effect_context(self) -> D2D1EffectContext { self }
}
impl IUnknown for D2D1EffectContext {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1EffectContext {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

