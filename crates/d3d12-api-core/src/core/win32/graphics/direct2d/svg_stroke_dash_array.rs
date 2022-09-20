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
pub struct D2D1SvgStrokeDashArray(pub(crate) D2D1SvgAttribute);

impl Deref for D2D1SvgStrokeDashArray {
	type Target = D2D1SvgAttribute;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SvgStrokeDashArray {
	const IID: &'static GUID = &GUID::from_u128(0xf1c0ca5292a34f00b4cef35691efd9d9u128);
}

impl Com for D2D1SvgStrokeDashArray {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SvgStrokeDashArray {
	pub fn RemoveDashesAtEnd(&self, dashes_count: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, dashes_count);
			_ret_.ok()
		}
	}

	pub fn UpdateDashes(&self, dashes: &[D2D1SvgLength], start_index: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (dashes_ptr_, dashes_len_) = dashes.deconstruct();
			let f: extern "system" fn(Param<Self>, *const D2D1SvgLength, u32, u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, dashes_ptr_, dashes_len_ as u32, start_index);
			_ret_.ok()
		}
	}

	pub unsafe fn GetDashes(&self) { todo!() }

	pub fn GetDashesCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[11]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1SvgStrokeDashArray: ID2D1SvgAttribute {
	fn as_svg_stroke_dash_array(&self) -> &D2D1SvgStrokeDashArray;
	fn into_svg_stroke_dash_array(self) -> D2D1SvgStrokeDashArray;
}

impl ID2D1SvgStrokeDashArray for D2D1SvgStrokeDashArray {
	fn as_svg_stroke_dash_array(&self) -> &D2D1SvgStrokeDashArray { self }
	fn into_svg_stroke_dash_array(self) -> D2D1SvgStrokeDashArray { self }
}
impl ID2D1SvgAttribute for D2D1SvgStrokeDashArray {
	fn as_svg_attribute(&self) -> &D2D1SvgAttribute { &self.0.as_svg_attribute() }
	fn into_svg_attribute(self) -> D2D1SvgAttribute { self.0.into_svg_attribute() }
}

impl ID2D1Resource for D2D1SvgStrokeDashArray {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SvgStrokeDashArray {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SvgStrokeDashArray {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1SvgAttribute::from(v))
    }
}

