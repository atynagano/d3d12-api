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
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SourceTransform(pub(crate) D2D1Transform);

impl Deref for D2D1SourceTransform {
	type Target = D2D1Transform;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SourceTransform {
	const IID: &'static GUID = &GUID::from_u128(0xdb1800dd0c344cf9be9031cc0a5653e1u128);
}

impl Com for D2D1SourceTransform {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SourceTransform {
	pub fn SetRenderInfo(&self, render_info: &D2D1RenderInfo) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, render_info.vtable());
			_ret_.ok()
		}
	}

	pub fn Draw(&self, target: &D2D1Bitmap1, draw_rect: &Rect, target_origin: D2DPoint2U) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, &Rect, D2DPoint2U) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, target.vtable(), draw_rect, target_origin);
			_ret_.ok()
		}
	}
}

pub trait ID2D1SourceTransform: ID2D1Transform {
	fn as_source_transform(&self) -> &D2D1SourceTransform;
	fn into_source_transform(self) -> D2D1SourceTransform;
}

impl ID2D1SourceTransform for D2D1SourceTransform {
	fn as_source_transform(&self) -> &D2D1SourceTransform { self }
	fn into_source_transform(self) -> D2D1SourceTransform { self }
}
impl ID2D1Transform for D2D1SourceTransform {
	fn as_transform(&self) -> &D2D1Transform { &self.0.as_transform() }
	fn into_transform(self) -> D2D1Transform { self.0.into_transform() }
}

impl ID2D1TransformNode for D2D1SourceTransform {
	fn as_transform_node(&self) -> &D2D1TransformNode { &self.0.as_transform_node() }
	fn into_transform_node(self) -> D2D1TransformNode { self.0.into_transform_node() }
}

impl IUnknown for D2D1SourceTransform {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SourceTransform {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Transform::from(v))
    }
}

