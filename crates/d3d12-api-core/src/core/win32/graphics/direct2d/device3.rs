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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Device3(pub(crate) D2D1Device2);

impl Deref for D2D1Device3 {
	type Target = D2D1Device2;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Device3 {
	const IID: &'static GUID = &GUID::from_u128(0x852f2087802c4037ab60ff2e7ee6fc01u128);
}

impl Com for D2D1Device3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Device3 {
	pub fn CreateDeviceContext(&self, options: D2D1DeviceContextOptions) -> Result<D2D1DeviceContext3, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut device_context3_out_: Option<D2D1DeviceContext3> = None;
			let f: extern "system" fn(Param<Self>, D2D1DeviceContextOptions, *mut c_void) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, options, transmute(&mut device_context3_out_));
			if _ret_.is_ok() { if let Some(device_context3_out_) = device_context3_out_ { return Ok(device_context3_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1Device3: ID2D1Device2 {
	fn as_device3(&self) -> &D2D1Device3;
	fn into_device3(self) -> D2D1Device3;
}

impl ID2D1Device3 for D2D1Device3 {
	fn as_device3(&self) -> &D2D1Device3 { self }
	fn into_device3(self) -> D2D1Device3 { self }
}
impl ID2D1Device2 for D2D1Device3 {
	fn as_device2(&self) -> &D2D1Device2 { &self.0.as_device2() }
	fn into_device2(self) -> D2D1Device2 { self.0.into_device2() }
}

impl ID2D1Device1 for D2D1Device3 {
	fn as_device1(&self) -> &D2D1Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D2D1Device1 { self.0.into_device1() }
}

impl ID2D1Device for D2D1Device3 {
	fn as_device(&self) -> &D2D1Device { &self.0.as_device() }
	fn into_device(self) -> D2D1Device { self.0.into_device() }
}

impl ID2D1Resource for D2D1Device3 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Device3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Device3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Device2::from(v))
    }
}

