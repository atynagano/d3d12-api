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
pub struct D2D1SvgPathData(pub(crate) D2D1SvgAttribute);

impl Deref for D2D1SvgPathData {
	type Target = D2D1SvgAttribute;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SvgPathData {
	const IID: &'static GUID = &GUID::from_u128(0xc095e4f4bb9843d697454d1b84ec9888u128);
}

impl Com for D2D1SvgPathData {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SvgPathData {
	pub fn RemoveSegmentDataAtEnd(&self, data_count: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, data_count);
			_ret_.ok()
		}
	}

	pub fn UpdateSegmentData(&self, data: &[f32], start_index: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, *const f32, u32, u32) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, data_ptr_, data_len_ as u32, start_index);
			_ret_.ok()
		}
	}

	pub unsafe fn GetSegmentData(&self) { todo!() }

	pub fn GetSegmentDataCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn RemoveCommandsAtEnd(&self, commands_count: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, commands_count);
			_ret_.ok()
		}
	}

	pub fn UpdateCommands(&self, commands: &[D2D1SvgPathCommand], start_index: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (commands_ptr_, commands_len_) = commands.deconstruct();
			let f: extern "system" fn(Param<Self>, *const D2D1SvgPathCommand, u32, u32) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, commands_ptr_, commands_len_ as u32, start_index);
			_ret_.ok()
		}
	}

	pub unsafe fn GetCommands(&self) { todo!() }

	pub fn GetCommandsCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[13]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn CreatePathGeometry(&self, fill_mode: D2D1FillMode) -> Result<D2D1PathGeometry1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut path_geometry_out_: Option<D2D1PathGeometry1> = None;
			let f: extern "system" fn(Param<Self>, D2D1FillMode, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, fill_mode, transmute(&mut path_geometry_out_));
			if _ret_.is_ok() { if let Some(path_geometry_out_) = path_geometry_out_ { return Ok(path_geometry_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1SvgPathData: ID2D1SvgAttribute {
	fn as_svg_path_data(&self) -> &D2D1SvgPathData;
	fn into_svg_path_data(self) -> D2D1SvgPathData;
}

impl ID2D1SvgPathData for D2D1SvgPathData {
	fn as_svg_path_data(&self) -> &D2D1SvgPathData { self }
	fn into_svg_path_data(self) -> D2D1SvgPathData { self }
}
impl ID2D1SvgAttribute for D2D1SvgPathData {
	fn as_svg_attribute(&self) -> &D2D1SvgAttribute { &self.0.as_svg_attribute() }
	fn into_svg_attribute(self) -> D2D1SvgAttribute { self.0.into_svg_attribute() }
}

impl ID2D1Resource for D2D1SvgPathData {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SvgPathData {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SvgPathData {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1SvgAttribute::from(v))
    }
}

