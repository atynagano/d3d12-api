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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1StrokeStyle1(pub(crate) D2D1StrokeStyle);

impl Deref for D2D1StrokeStyle1 {
	type Target = D2D1StrokeStyle;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1StrokeStyle1 {
	const IID: &'static GUID = &GUID::from_u128(0x10a72a66e91c43f4993fddf4b82b0b4au128);
}

impl Com for D2D1StrokeStyle1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1StrokeStyle1 {
	pub fn GetStrokeTransformType(&self) -> D2D1StrokeTransformType {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1StrokeTransformType
				= transmute(vt[13]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1StrokeStyle1: ID2D1StrokeStyle {
	fn as_stroke_style1(&self) -> &D2D1StrokeStyle1;
	fn into_stroke_style1(self) -> D2D1StrokeStyle1;
}

impl ID2D1StrokeStyle1 for D2D1StrokeStyle1 {
	fn as_stroke_style1(&self) -> &D2D1StrokeStyle1 { self }
	fn into_stroke_style1(self) -> D2D1StrokeStyle1 { self }
}
impl ID2D1StrokeStyle for D2D1StrokeStyle1 {
	fn as_stroke_style(&self) -> &D2D1StrokeStyle { &self.0.as_stroke_style() }
	fn into_stroke_style(self) -> D2D1StrokeStyle { self.0.into_stroke_style() }
}

impl ID2D1Resource for D2D1StrokeStyle1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1StrokeStyle1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1StrokeStyle1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1StrokeStyle::from(v))
    }
}

