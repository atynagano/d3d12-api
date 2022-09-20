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

use crate::core::win32::graphics::direct2d::common::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1SolidColorBrush(pub(crate) D2D1Brush);

impl Deref for D2D1SolidColorBrush {
	type Target = D2D1Brush;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1SolidColorBrush {
	const IID: &'static GUID = &GUID::from_u128(0x2cd906a912e211dc9fed001143a055f9u128);
}

impl Com for D2D1SolidColorBrush {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1SolidColorBrush {
	pub fn SetColor(&self, color: &D2D1ColorF) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D2D1ColorF) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, color);
		}
	}

	pub fn GetColor(&self) -> D2D1ColorF {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ColorF
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1SolidColorBrush: ID2D1Brush {
	fn as_solid_color_brush(&self) -> &D2D1SolidColorBrush;
	fn into_solid_color_brush(self) -> D2D1SolidColorBrush;
}

impl ID2D1SolidColorBrush for D2D1SolidColorBrush {
	fn as_solid_color_brush(&self) -> &D2D1SolidColorBrush { self }
	fn into_solid_color_brush(self) -> D2D1SolidColorBrush { self }
}
impl ID2D1Brush for D2D1SolidColorBrush {
	fn as_brush(&self) -> &D2D1Brush { &self.0.as_brush() }
	fn into_brush(self) -> D2D1Brush { self.0.into_brush() }
}

impl ID2D1Resource for D2D1SolidColorBrush {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1SolidColorBrush {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1SolidColorBrush {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Brush::from(v))
    }
}

