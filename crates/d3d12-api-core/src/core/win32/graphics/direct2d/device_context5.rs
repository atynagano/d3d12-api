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
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::graphics::dxgi::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1DeviceContext5(pub(crate) D2D1DeviceContext4);

impl Deref for D2D1DeviceContext5 {
	type Target = D2D1DeviceContext4;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1DeviceContext5 {
	const IID: &'static GUID = &GUID::from_u128(0x7836d24868cc4df6b9e8de991bf62eb7u128);
}

impl Com for D2D1DeviceContext5 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1DeviceContext5 {
	pub fn CreateSvgDocument(&self, input_xml_stream: Option<&Stream>, viewport_size: D2DSizeF) -> Result<D2D1SvgDocument, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut svg_document_out_: Option<D2D1SvgDocument> = None;
			let f: extern "system" fn(Param<Self>, *const c_void, D2DSizeF, *mut c_void) -> HResult
				= transmute(vt[115]);
			let _ret_ = f(vt, option_to_vtable(input_xml_stream), viewport_size, transmute(&mut svg_document_out_));
			if _ret_.is_ok() { if let Some(svg_document_out_) = svg_document_out_ { return Ok(svg_document_out_); } }
			Err(_ret_)
		}
	}

	pub fn DrawSvgDocument(&self, svg_document: &D2D1SvgDocument) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> ()
				= transmute(vt[116]);
			let _ret_ = f(vt, svg_document.vtable());
		}
	}

	pub fn CreateColorContextFromDxgiColorSpace(&self, color_space: DxgiColorSpaceType) -> Result<D2D1ColorContext1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_context_out_: Option<D2D1ColorContext1> = None;
			let f: extern "system" fn(Param<Self>, DxgiColorSpaceType, *mut c_void) -> HResult
				= transmute(vt[117]);
			let _ret_ = f(vt, color_space, transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContextFromSimpleColorProfile(&self, simple_profile: &D2D1SimpleColorProfile) -> Result<D2D1ColorContext1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_context_out_: Option<D2D1ColorContext1> = None;
			let f: extern "system" fn(Param<Self>, &D2D1SimpleColorProfile, *mut c_void) -> HResult
				= transmute(vt[118]);
			let _ret_ = f(vt, simple_profile, transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1DeviceContext5: ID2D1DeviceContext4 {
	fn as_device_context5(&self) -> &D2D1DeviceContext5;
	fn into_device_context5(self) -> D2D1DeviceContext5;
}

impl ID2D1DeviceContext5 for D2D1DeviceContext5 {
	fn as_device_context5(&self) -> &D2D1DeviceContext5 { self }
	fn into_device_context5(self) -> D2D1DeviceContext5 { self }
}
impl ID2D1DeviceContext4 for D2D1DeviceContext5 {
	fn as_device_context4(&self) -> &D2D1DeviceContext4 { &self.0.as_device_context4() }
	fn into_device_context4(self) -> D2D1DeviceContext4 { self.0.into_device_context4() }
}

impl ID2D1DeviceContext3 for D2D1DeviceContext5 {
	fn as_device_context3(&self) -> &D2D1DeviceContext3 { &self.0.as_device_context3() }
	fn into_device_context3(self) -> D2D1DeviceContext3 { self.0.into_device_context3() }
}

impl ID2D1DeviceContext2 for D2D1DeviceContext5 {
	fn as_device_context2(&self) -> &D2D1DeviceContext2 { &self.0.as_device_context2() }
	fn into_device_context2(self) -> D2D1DeviceContext2 { self.0.into_device_context2() }
}

impl ID2D1DeviceContext1 for D2D1DeviceContext5 {
	fn as_device_context1(&self) -> &D2D1DeviceContext1 { &self.0.as_device_context1() }
	fn into_device_context1(self) -> D2D1DeviceContext1 { self.0.into_device_context1() }
}

impl ID2D1DeviceContext for D2D1DeviceContext5 {
	fn as_device_context(&self) -> &D2D1DeviceContext { &self.0.as_device_context() }
	fn into_device_context(self) -> D2D1DeviceContext { self.0.into_device_context() }
}

impl ID2D1RenderTarget for D2D1DeviceContext5 {
	fn as_render_target(&self) -> &D2D1RenderTarget { &self.0.as_render_target() }
	fn into_render_target(self) -> D2D1RenderTarget { self.0.into_render_target() }
}

impl ID2D1Resource for D2D1DeviceContext5 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1DeviceContext5 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1DeviceContext5 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1DeviceContext4::from(v))
    }
}

