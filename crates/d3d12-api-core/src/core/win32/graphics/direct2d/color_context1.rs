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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1ColorContext1(pub(crate) D2D1ColorContext);

impl Deref for D2D1ColorContext1 {
	type Target = D2D1ColorContext;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1ColorContext1 {
	const IID: &'static GUID = &GUID::from_u128(0x1ab42875c57f4be9bd859cd78d6f55eeu128);
}

impl Com for D2D1ColorContext1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1ColorContext1 {
	pub fn GetColorContextType(&self) -> D2D1ColorContextType {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1ColorContextType
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetDXGIColorSpace(&self) -> DxgiColorSpaceType {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DxgiColorSpaceType
				= transmute(vt[8]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetSimpleColorProfile(&self) -> Result<D2D1SimpleColorProfile, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut simple_profile_out_: MaybeUninit<D2D1SimpleColorProfile> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut D2D1SimpleColorProfile) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, simple_profile_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(simple_profile_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait ID2D1ColorContext1: ID2D1ColorContext {
	fn as_color_context1(&self) -> &D2D1ColorContext1;
	fn into_color_context1(self) -> D2D1ColorContext1;
}

impl ID2D1ColorContext1 for D2D1ColorContext1 {
	fn as_color_context1(&self) -> &D2D1ColorContext1 { self }
	fn into_color_context1(self) -> D2D1ColorContext1 { self }
}
impl ID2D1ColorContext for D2D1ColorContext1 {
	fn as_color_context(&self) -> &D2D1ColorContext { &self.0.as_color_context() }
	fn into_color_context(self) -> D2D1ColorContext { self.0.into_color_context() }
}

impl ID2D1Resource for D2D1ColorContext1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1ColorContext1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1ColorContext1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1ColorContext::from(v))
    }
}

