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
use crate::core::win32::graphics::imaging::*;
use crate::core::win32::storage::xps::printing::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Device(pub(crate) D2D1Resource);

impl Deref for D2D1Device {
	type Target = D2D1Resource;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Device {
	const IID: &'static GUID = &GUID::from_u128(0x47dd575dac054cdd80499b02cd16f44cu128);
}

impl Com for D2D1Device {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Device {
	pub fn CreateDeviceContext(&self, options: D2D1DeviceContextOptions) -> Result<D2D1DeviceContext, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut device_context_out_: Option<D2D1DeviceContext> = None;
			let f: extern "system" fn(Param<Self>, D2D1DeviceContextOptions, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, options, transmute(&mut device_context_out_));
			if _ret_.is_ok() { if let Some(device_context_out_) = device_context_out_ { return Ok(device_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreatePrintControl(&self, wic_factory: &WICImagingFactory, document_target: &PrintDocumentPackageTarget, print_control_properties: Option<&D2D1PrintControlProperties>) -> Result<D2D1PrintControl, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut print_control_out_: Option<D2D1PrintControl> = None;
			let f: extern "system" fn(Param<Self>, VTable, VTable, *const c_void, *mut c_void) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, wic_factory.vtable(), document_target.vtable(), transmute(print_control_properties), transmute(&mut print_control_out_));
			if _ret_.is_ok() { if let Some(print_control_out_) = print_control_out_ { return Ok(print_control_out_); } }
			Err(_ret_)
		}
	}

	pub fn SetMaximumTextureMemory(&self, maximum_in_bytes: u64) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u64) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, maximum_in_bytes);
		}
	}

	pub fn GetMaximumTextureMemory(&self) -> u64 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u64
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn ClearResources(&self, milliseconds_since_use: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> ()
				= transmute(vt[8]);
			let _ret_ = f(vt, milliseconds_since_use);
		}
	}
}

pub trait ID2D1Device: ID2D1Resource {
	fn as_device(&self) -> &D2D1Device;
	fn into_device(self) -> D2D1Device;
}

impl ID2D1Device for D2D1Device {
	fn as_device(&self) -> &D2D1Device { self }
	fn into_device(self) -> D2D1Device { self }
}
impl ID2D1Resource for D2D1Device {
	fn as_resource(&self) -> &D2D1Resource { &self.0.as_resource() }
	fn into_resource(self) -> D2D1Resource { self.0.into_resource() }
}

impl IUnknown for D2D1Device {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Device {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Resource::from(v))
    }
}

