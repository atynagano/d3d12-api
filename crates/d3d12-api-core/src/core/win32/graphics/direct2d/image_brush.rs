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
use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1ImageBrush(pub(crate) D2D1Brush);

impl Deref for D2D1ImageBrush {
	type Target = D2D1Brush;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1ImageBrush {
	const IID: &'static GUID = &GUID::from_u128(0xfe9e984d3f95407cb5dbcb94d4e8f87cu128);
}

impl Com for D2D1ImageBrush {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1ImageBrush {
	pub fn SetImage(&self, image: Option<&D2D1Image>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, option_to_vtable(image));
		}
	}

	pub fn SetExtendModeX(&self, extend_mode_x: D2D1ExtendMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1ExtendMode) -> ()
				= transmute(vt[9]);
			let _ret_ = f(vt, extend_mode_x);
		}
	}

	pub fn SetExtendModeY(&self, extend_mode_y: D2D1ExtendMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1ExtendMode) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, extend_mode_y);
		}
	}

	pub fn SetInterpolationMode(&self, interpolation_mode: D2D1InterpolationMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1InterpolationMode) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, interpolation_mode);
		}
	}

	pub fn SetSourceRectangle(&self, source_rectangle: &D2DRectF) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2DRectF) -> ()
				= transmute(vt[12]);
			let _ret_ = f(vt, source_rectangle);
		}
	}

	pub fn GetImage(&self, mut image: Option<&mut Option<D2D1Image>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut image) = image { **image = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt, transmute(image));
		}
	}

	pub fn GetExtendModeX(&self) -> D2D1ExtendMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ExtendMode
				= transmute(vt[14]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetExtendModeY(&self) -> D2D1ExtendMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ExtendMode
				= transmute(vt[15]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetInterpolationMode(&self) -> D2D1InterpolationMode {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1InterpolationMode
				= transmute(vt[16]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetSourceRectangle(&self) -> D2DRectF {
		unsafe {
			let vt = self.as_param();
			let mut source_rectangle_out_: MaybeUninit<D2DRectF> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2DRectF) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, source_rectangle_out_.as_mut_ptr());
			source_rectangle_out_.assume_init()
		}
	}
}

pub trait ID2D1ImageBrush: ID2D1Brush {
	fn as_image_brush(&self) -> &D2D1ImageBrush;
	fn into_image_brush(self) -> D2D1ImageBrush;
}

impl ID2D1ImageBrush for D2D1ImageBrush {
	fn as_image_brush(&self) -> &D2D1ImageBrush { self }
	fn into_image_brush(self) -> D2D1ImageBrush { self }
}
impl ID2D1Brush for D2D1ImageBrush {
	fn as_brush(&self) -> &D2D1Brush { &self.0.as_brush() }
	fn into_brush(self) -> D2D1Brush { self.0.into_brush() }
}

impl ID2D1Resource for D2D1ImageBrush {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1ImageBrush {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1ImageBrush {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Brush::from(v))
    }
}

