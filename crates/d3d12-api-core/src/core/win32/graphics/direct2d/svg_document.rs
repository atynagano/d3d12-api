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
use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SvgDocument(pub(crate) D2D1Resource);

impl Deref for D2D1SvgDocument {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SvgDocument {
	const IID: &'static GUID = &GUID::from_u128(0x86b88e4dafa44d7b88e468a51c4a0aecu128);
}

impl Com for D2D1SvgDocument {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SvgDocument {
	pub fn SetViewportSize(&self, viewport_size: D2DSizeF) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2DSizeF) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, viewport_size);
			_ret_.ok()
		}
	}

	pub fn GetViewportSize(&self) -> D2DSizeF {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2DSizeF
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetRoot(&self, root: Option<&D2D1SvgElement>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, option_to_vtable(root));
			_ret_.ok()
		}
	}

	pub fn GetRoot(&self, mut root: Option<&mut Option<D2D1SvgElement>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut root) = root { **root = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, transmute(root));
		}
	}

	pub fn FindElementById(&self, id: &str, mut svg_element: Option<&mut Option<D2D1SvgElement>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut svg_element) = svg_element { **svg_element = None; }
			let f: extern "system" fn(Param<Self>, *const u16, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, id.to_utf16().as_ptr_or_null(), transmute(svg_element));
			_ret_.ok()
		}
	}

	pub fn find_element_by_id(&self, id: &str) -> Result<D2D1SvgElement, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut svg_element_out_: Option<D2D1SvgElement> = None;
			let f: extern "system" fn(Param<Self>, *const u16, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, id.to_utf16().as_ptr_or_null(), transmute(&mut svg_element_out_));
			if _ret_.is_ok() { if let Some(svg_element_out_) = svg_element_out_ { return Ok(svg_element_out_); } }
			Err(_ret_)
		}
	}

	pub fn Serialize(&self, output_xml_stream: &Stream, subtree: Option<&D2D1SvgElement>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, output_xml_stream.vtable(), option_to_vtable(subtree));
			_ret_.ok()
		}
	}

	pub fn Deserialize(&self, input_xml_stream: &Stream) -> Result<D2D1SvgElement, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut subtree_out_: Option<D2D1SvgElement> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, input_xml_stream.vtable(), transmute(&mut subtree_out_));
			if _ret_.is_ok() { if let Some(subtree_out_) = subtree_out_ { return Ok(subtree_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreatePaint(&self, paint_type: D2D1SvgPaintType, color: Option<&D2D1ColorF>, id: Option<&str>) -> Result<D2D1SvgPaint, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut paint_out_: Option<D2D1SvgPaint> = None;
			let f: extern "system" fn(Param<Self>, D2D1SvgPaintType, *const c_void, *const u16, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, paint_type, transmute(color), id.map(str::to_utf16).as_ptr_or_null(), transmute(&mut paint_out_));
			if _ret_.is_ok() { if let Some(paint_out_) = paint_out_ { return Ok(paint_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateStrokeDashArray(&self, dashes: Option<&[D2D1SvgLength]>) -> Result<D2D1SvgStrokeDashArray, HResult> {
		unsafe {
			let vt = self.as_param();
			let (dashes_ptr_, dashes_len_) = dashes.deconstruct();
			let mut stroke_dash_array_out_: Option<D2D1SvgStrokeDashArray> = None;
			let f: extern "system" fn(Param<Self>, *const D2D1SvgLength, u32, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, dashes_ptr_, dashes_len_ as u32, transmute(&mut stroke_dash_array_out_));
			if _ret_.is_ok() { if let Some(stroke_dash_array_out_) = stroke_dash_array_out_ { return Ok(stroke_dash_array_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreatePointCollection(&self, points: Option<&[D2DPoint2F]>) -> Result<D2D1SvgPointCollection, HResult> {
		unsafe {
			let vt = self.as_param();
			let (points_ptr_, points_len_) = points.deconstruct();
			let mut point_collection_out_: Option<D2D1SvgPointCollection> = None;
			let f: extern "system" fn(Param<Self>, *const D2DPoint2F, u32, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, points_ptr_, points_len_ as u32, transmute(&mut point_collection_out_));
			if _ret_.is_ok() { if let Some(point_collection_out_) = point_collection_out_ { return Ok(point_collection_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreatePathData(&self, segment_data: Option<&[f32]>, commands: Option<&[D2D1SvgPathCommand]>) -> Result<D2D1SvgPathData, HResult> {
		unsafe {
			let vt = self.as_param();
			let (segment_data_ptr_, segment_data_len_) = segment_data.deconstruct();
			let (commands_ptr_, commands_len_) = commands.deconstruct();
			let mut path_data_out_: Option<D2D1SvgPathData> = None;
			let f: extern "system" fn(Param<Self>, *const f32, u32, *const D2D1SvgPathCommand, u32, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, segment_data_ptr_, segment_data_len_ as u32, commands_ptr_, commands_len_ as u32, transmute(&mut path_data_out_));
			if _ret_.is_ok() { if let Some(path_data_out_) = path_data_out_ { return Ok(path_data_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1SvgDocument: ID2D1Resource {
	fn as_svg_document(&self) -> &D2D1SvgDocument;
	fn into_svg_document(self) -> D2D1SvgDocument;
}

impl ID2D1SvgDocument for D2D1SvgDocument {
	fn as_svg_document(&self) -> &D2D1SvgDocument { self }
	fn into_svg_document(self) -> D2D1SvgDocument { self }
}
impl ID2D1Resource for D2D1SvgDocument {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SvgDocument {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SvgDocument {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

