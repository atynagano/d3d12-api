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
pub struct D2D1Device1(pub(crate) D2D1Device);

impl Deref for D2D1Device1 {
	type Target = D2D1Device;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Device1 {
	const IID: &'static GUID = &GUID::from_u128(0xd21768e123a44823a14b7c3eba85d658u128);
}

impl Com for D2D1Device1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Device1 {
	pub fn GetRenderingPriority(&self) -> D2D1RenderingPriority {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> D2D1RenderingPriority
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn SetRenderingPriority(&self, rendering_priority: D2D1RenderingPriority) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1RenderingPriority) -> ()
				= transmute(vt[10]);
			let _ret_ = f(vt, rendering_priority);
		}
	}

	pub fn CreateDeviceContext(&self, options: D2D1DeviceContextOptions) -> Result<D2D1DeviceContext1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut device_context1_out_: Option<D2D1DeviceContext1> = None;
			let f: extern "system" fn(Param<Self>, D2D1DeviceContextOptions, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, options, transmute(&mut device_context1_out_));
			if _ret_.is_ok() { if let Some(device_context1_out_) = device_context1_out_ { return Ok(device_context1_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1Device1: ID2D1Device {
	fn as_device1(&self) -> &D2D1Device1;
	fn into_device1(self) -> D2D1Device1;
}

impl ID2D1Device1 for D2D1Device1 {
	fn as_device1(&self) -> &D2D1Device1 { self }
	fn into_device1(self) -> D2D1Device1 { self }
}
impl ID2D1Device for D2D1Device1 {
	fn as_device(&self) -> &D2D1Device { &self.0.as_device() }
	fn into_device(self) -> D2D1Device { self.0.into_device() }
}

impl ID2D1Resource for D2D1Device1 {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Device1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Device1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Device::from(v))
    }
}

