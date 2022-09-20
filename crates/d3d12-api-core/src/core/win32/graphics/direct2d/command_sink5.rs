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
pub struct D2D1CommandSink5(pub(crate) D2D1CommandSink4);

impl Deref for D2D1CommandSink5 {
	type Target = D2D1CommandSink4;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1CommandSink5 {
	const IID: &'static GUID = &GUID::from_u128(0x7047dd26b1e744a7959a8349e2144fa8u128);
}

impl Com for D2D1CommandSink5 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1CommandSink5 {
	pub fn BlendImage(&self, image: &D2D1Image, blend_mode: D2D1BlendMode, target_offset: Option<&D2DPoint2F>, image_rectangle: Option<&D2DRectF>, interpolation_mode: D2D1InterpolationMode) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, D2D1BlendMode, *const c_void, *const c_void, D2D1InterpolationMode) -> HResult
				= transmute(vt[34]);
			let _ret_ = f(vt, image.vtable(), blend_mode, transmute(target_offset), transmute(image_rectangle), interpolation_mode);
			_ret_.ok()
		}
	}
}

pub trait ID2D1CommandSink5: ID2D1CommandSink4 {
	fn as_command_sink5(&self) -> &D2D1CommandSink5;
	fn into_command_sink5(self) -> D2D1CommandSink5;
}

impl ID2D1CommandSink5 for D2D1CommandSink5 {
	fn as_command_sink5(&self) -> &D2D1CommandSink5 { self }
	fn into_command_sink5(self) -> D2D1CommandSink5 { self }
}
impl ID2D1CommandSink4 for D2D1CommandSink5 {
	fn as_command_sink4(&self) -> &D2D1CommandSink4 { &self.0.as_command_sink4() }
	fn into_command_sink4(self) -> D2D1CommandSink4 { self.0.into_command_sink4() }
}

impl ID2D1CommandSink3 for D2D1CommandSink5 {
	fn as_command_sink3(&self) -> &D2D1CommandSink3 { &self.0.as_command_sink3() }
	fn into_command_sink3(self) -> D2D1CommandSink3 { self.0.into_command_sink3() }
}

impl ID2D1CommandSink2 for D2D1CommandSink5 {
	fn as_command_sink2(&self) -> &D2D1CommandSink2 { &self.0.as_command_sink2() }
	fn into_command_sink2(self) -> D2D1CommandSink2 { self.0.into_command_sink2() }
}

impl ID2D1CommandSink1 for D2D1CommandSink5 {
	fn as_command_sink1(&self) -> &D2D1CommandSink1 { &self.0.as_command_sink1() }
	fn into_command_sink1(self) -> D2D1CommandSink1 { self.0.into_command_sink1() }
}

impl ID2D1CommandSink for D2D1CommandSink5 {
	fn as_command_sink(&self) -> &D2D1CommandSink { &self.0.as_command_sink() }
	fn into_command_sink(self) -> D2D1CommandSink { self.0.into_command_sink() }
}

impl IUnknown for D2D1CommandSink5 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1CommandSink5 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1CommandSink4::from(v))
    }
}

