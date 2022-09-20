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
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Factory3(pub(crate) D2D1Factory2);

impl Deref for D2D1Factory3 {
	type Target = D2D1Factory2;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Factory3 {
	const IID: &'static GUID = &GUID::from_u128(0x0869759f4f00413fb03e2bda45404d0fu128);
}

impl Com for D2D1Factory3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Factory3 {
	pub fn CreateDevice(&self, dxgi_device: &DxgiDevice) -> Result<D2D1Device2, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut d2d_device2_out_: Option<D2D1Device2> = None;
			let f: extern "system" fn(Param<Self>, VTable, *mut c_void) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, dxgi_device.vtable(), transmute(&mut d2d_device2_out_));
			if _ret_.is_ok() { if let Some(d2d_device2_out_) = d2d_device2_out_ { return Ok(d2d_device2_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1Factory3: ID2D1Factory2 {
	fn as_factory3(&self) -> &D2D1Factory3;
	fn into_factory3(self) -> D2D1Factory3;
}

impl ID2D1Factory3 for D2D1Factory3 {
	fn as_factory3(&self) -> &D2D1Factory3 { self }
	fn into_factory3(self) -> D2D1Factory3 { self }
}
impl ID2D1Factory2 for D2D1Factory3 {
	fn as_factory2(&self) -> &D2D1Factory2 { &self.0.as_factory2() }
	fn into_factory2(self) -> D2D1Factory2 { self.0.into_factory2() }
}

impl ID2D1Factory1 for D2D1Factory3 {
	fn as_factory1(&self) -> &D2D1Factory1 { &self.0.as_factory1() }
	fn into_factory1(self) -> D2D1Factory1 { self.0.into_factory1() }
}

impl ID2D1Factory for D2D1Factory3 {
	fn as_factory(&self) -> &D2D1Factory { &self.0.as_factory() }
	fn into_factory(self) -> D2D1Factory { self.0.into_factory() }
}

impl IUnknown for D2D1Factory3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Factory3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Factory2::from(v))
    }
}

