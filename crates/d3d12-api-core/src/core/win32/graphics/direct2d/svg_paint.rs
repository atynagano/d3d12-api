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
pub struct D2D1SvgPaint(pub(crate) D2D1SvgAttribute);

impl Deref for D2D1SvgPaint {
	type Target = D2D1SvgAttribute;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SvgPaint {
	const IID: &'static GUID = &GUID::from_u128(0xd59bab0a68a2455ba5dc9eb2854e2490u128);
}

impl Com for D2D1SvgPaint {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SvgPaint {
	pub fn SetPaintType(&self, paint_type: D2D1SvgPaintType) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1SvgPaintType) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, paint_type);
			_ret_.ok()
		}
	}

	pub fn GetPaintType(&self) -> D2D1SvgPaintType {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1SvgPaintType
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetColor(&self, color: &D2D1ColorF) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1ColorF) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, color);
			_ret_.ok()
		}
	}

	pub fn GetColor(&self) -> D2D1ColorF {
		unsafe {
			let vt = self.as_param();
			let mut color_out_: MaybeUninit<D2D1ColorF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2D1ColorF) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, color_out_.as_mut_ptr());
			color_out_.assume_init()
		}
	}

	pub fn SetId(&self, id: &str) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, id.to_utf16().as_ptr_or_null());
			_ret_.ok()
		}
	}

	pub unsafe fn GetId(&self) { todo!() }

	pub fn GetIdLength(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[12]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1SvgPaint: ID2D1SvgAttribute {
	fn as_svg_paint(&self) -> &D2D1SvgPaint;
	fn into_svg_paint(self) -> D2D1SvgPaint;
}

impl ID2D1SvgPaint for D2D1SvgPaint {
	fn as_svg_paint(&self) -> &D2D1SvgPaint { self }
	fn into_svg_paint(self) -> D2D1SvgPaint { self }
}
impl ID2D1SvgAttribute for D2D1SvgPaint {
	fn as_svg_attribute(&self) -> &D2D1SvgAttribute { &self.0.as_svg_attribute() }
	fn into_svg_attribute(self) -> D2D1SvgAttribute { self.0.into_svg_attribute() }
}

impl ID2D1Resource for D2D1SvgPaint {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SvgPaint {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SvgPaint {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1SvgAttribute::from(v))
    }
}

