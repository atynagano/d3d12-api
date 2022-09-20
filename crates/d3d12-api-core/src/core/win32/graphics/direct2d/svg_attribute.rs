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
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SvgAttribute(pub(crate) D2D1Resource);

impl Deref for D2D1SvgAttribute {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SvgAttribute {
	const IID: &'static GUID = &GUID::from_u128(0xc9cdb0ddf8c94e70b7c2301c80292c5eu128);
}

impl Com for D2D1SvgAttribute {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SvgAttribute {
	pub fn GetElement(&self, mut element: Option<&mut Option<D2D1SvgElement>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut element) = element { **element = None; }
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, transmute(element));
		}
	}

	pub fn Clone(&self) -> Result<D2D1SvgAttribute, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut attribute_out_: Option<D2D1SvgAttribute> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, transmute(&mut attribute_out_));
			if _ret_.is_ok() { if let Some(attribute_out_) = attribute_out_ { return Ok(attribute_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1SvgAttribute: ID2D1Resource {
	fn as_svg_attribute(&self) -> &D2D1SvgAttribute;
	fn into_svg_attribute(self) -> D2D1SvgAttribute;
}

impl ID2D1SvgAttribute for D2D1SvgAttribute {
	fn as_svg_attribute(&self) -> &D2D1SvgAttribute { self }
	fn into_svg_attribute(self) -> D2D1SvgAttribute { self }
}
impl ID2D1Resource for D2D1SvgAttribute {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SvgAttribute {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SvgAttribute {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

