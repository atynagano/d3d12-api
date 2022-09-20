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
pub struct D2D1ColorContext(pub(crate) D2D1Resource);

impl Deref for D2D1ColorContext {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1ColorContext {
	const IID: &'static GUID = &GUID::from_u128(0x1c4820bb57714518a5812fe4dd0ec657u128);
}

impl Com for D2D1ColorContext {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1ColorContext {
	pub fn GetColorSpace(&self) -> D2D1ColorSpace {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ColorSpace
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetProfileSize(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[5]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetProfile(&self) { todo!() }
}

pub trait ID2D1ColorContext: ID2D1Resource {
	fn as_color_context(&self) -> &D2D1ColorContext;
	fn into_color_context(self) -> D2D1ColorContext;
}

impl ID2D1ColorContext for D2D1ColorContext {
	fn as_color_context(&self) -> &D2D1ColorContext { self }
	fn into_color_context(self) -> D2D1ColorContext { self }
}
impl ID2D1Resource for D2D1ColorContext {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1ColorContext {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1ColorContext {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

