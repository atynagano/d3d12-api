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
pub struct D2D1Device4(pub(crate) D2D1Device3);

impl Deref for D2D1Device4 {
	type Target = D2D1Device3;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Device4 {
	const IID: &'static GUID = &GUID::from_u128(0xd7bdb15956834a46bc9c72dc720b858bu128);
}

impl Com for D2D1Device4 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Device4 {
	pub fn CreateDeviceContext(&self, options: D2D1DeviceContextOptions) -> Result<D2D1DeviceContext4, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut device_context4_out_: Option<D2D1DeviceContext4> = None;
			let f: extern "system" fn(Param<Self>, D2D1DeviceContextOptions, *mut c_void) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, options, transmute(&mut device_context4_out_));
			if _ret_.is_ok() { if let Some(device_context4_out_) = device_context4_out_ { return Ok(device_context4_out_); } }
			Err(_ret_)
		}
	}

	pub fn SetMaximumColorGlyphCacheMemory(&self, maximum_in_bytes: u64) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u64) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, maximum_in_bytes);
		}
	}

	pub fn GetMaximumColorGlyphCacheMemory(&self) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u64
				= transmute(vt[18]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID2D1Device4: ID2D1Device3 {
	fn as_device4(&self) -> &D2D1Device4;
	fn into_device4(self) -> D2D1Device4;
}

impl ID2D1Device4 for D2D1Device4 {
	fn as_device4(&self) -> &D2D1Device4 { self }
	fn into_device4(self) -> D2D1Device4 { self }
}
impl ID2D1Device3 for D2D1Device4 {
	fn as_device3(&self) -> &D2D1Device3 { &self.0.as_device3() }
	fn into_device3(self) -> D2D1Device3 { self.0.into_device3() }
}

impl ID2D1Device2 for D2D1Device4 {
	fn as_device2(&self) -> &D2D1Device2 { &self.0.as_device2() }
	fn into_device2(self) -> D2D1Device2 { self.0.into_device2() }
}

impl ID2D1Device1 for D2D1Device4 {
	fn as_device1(&self) -> &D2D1Device1 { &self.0.as_device1() }
	fn into_device1(self) -> D2D1Device1 { self.0.into_device1() }
}

impl ID2D1Device for D2D1Device4 {
	fn as_device(&self) -> &D2D1Device { &self.0.as_device() }
	fn into_device(self) -> D2D1Device { self.0.into_device() }
}

impl ID2D1Resource for D2D1Device4 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Device4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Device4 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Device3::from(v))
    }
}

