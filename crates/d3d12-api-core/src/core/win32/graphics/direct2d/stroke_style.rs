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
pub struct D2D1StrokeStyle(pub(crate) D2D1Resource);

impl Deref for D2D1StrokeStyle {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1StrokeStyle {
	const IID: &'static GUID = &GUID::from_u128(0x2cd9069d12e211dc9fed001143a055f9u128);
}

impl Com for D2D1StrokeStyle {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1StrokeStyle {
	pub fn GetStartCap(&self) -> D2D1CapStyle {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1CapStyle
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetEndCap(&self) -> D2D1CapStyle {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1CapStyle
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetDashCap(&self) -> D2D1CapStyle {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1CapStyle
				= transmute(vt[6]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetMiterLimit(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetLineJoin(&self) -> D2D1LineJoin {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1LineJoin
				= transmute(vt[8]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetDashOffset(&self) -> f32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> f32
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetDashStyle(&self) -> D2D1DashStyle {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1DashStyle
				= transmute(vt[10]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetDashesCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[11]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetDashes(&self) { todo!() }
}

pub trait ID2D1StrokeStyle: ID2D1Resource {
	fn as_stroke_style(&self) -> &D2D1StrokeStyle;
	fn into_stroke_style(self) -> D2D1StrokeStyle;
}

impl ID2D1StrokeStyle for D2D1StrokeStyle {
	fn as_stroke_style(&self) -> &D2D1StrokeStyle { self }
	fn into_stroke_style(self) -> D2D1StrokeStyle { self }
}
impl ID2D1Resource for D2D1StrokeStyle {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1StrokeStyle {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1StrokeStyle {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

