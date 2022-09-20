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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Device2(pub(crate) D2D1Device1);

impl Deref for D2D1Device2 {
	type Target = D2D1Device1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Device2 {
	const IID: &'static GUID = &GUID::from_u128(0xa44472e18dfb4e6084926e2861c9ca8bu128);
}

impl Com for D2D1Device2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Device2 {
	pub fn CreateDeviceContext(&self, options: D2D1DeviceContextOptions) -> Result<D2D1DeviceContext2, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut device_context2_out_: Option<D2D1DeviceContext2> = None;
			let f: extern "system" fn(Param<Self>, D2D1DeviceContextOptions, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, options, transmute(&mut device_context2_out_));
			if _ret_.is_ok() { if let Some(device_context2_out_) = device_context2_out_ { return Ok(device_context2_out_); } }
			Err(_ret_)
		}
	}

	pub fn FlushDeviceContexts(&self, bitmap: &D2D1Bitmap) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt, bitmap.vtable());
		}
	}

	pub fn GetDxgiDevice(&self) -> Result<DxgiDevice, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut dxgi_device_out_: Option<DxgiDevice> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, transmute(&mut dxgi_device_out_));
			if _ret_.is_ok() { if let Some(dxgi_device_out_) = dxgi_device_out_ { return Ok(dxgi_device_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1Device2: ID2D1Device1 {
	fn as_device2(&self) -> &D2D1Device2;
	fn into_device2(self) -> D2D1Device2;
}

impl ID2D1Device2 for D2D1Device2 {
	fn as_device2(&self) -> &D2D1Device2 { self }
	fn into_device2(self) -> D2D1Device2 { self }
}
impl ID2D1Device1 for D2D1Device2 {
	fn as_device1(&self) -> &D2D1Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D2D1Device1 { self.0.into_device1() }
}

impl ID2D1Device for D2D1Device2 {
	fn as_device(&self) -> &D2D1Device { &self.0.as_device() }
	fn into_device(self) -> D2D1Device { self.0.into_device() }
}

impl ID2D1Resource for D2D1Device2 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Device2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Device2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Device1::from(v))
    }
}

