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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12DeviceChild(pub(crate) D3D12Object);

impl Deref for D3D12DeviceChild {
	type Target = D3D12Object;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DeviceChild {
	const IID: &'static GUID = &GUID::from_u128(0x905db94ba00c41409df52b64ca9ea357u128);
}

impl Com for D3D12DeviceChild {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DeviceChild {
	pub fn GetDevice<T: IUnknown + From<UnknownWrapper>>(&self, device: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut device_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, T::IID, transmute(if device_out_.is_some() { Some(&mut device_out_) } else { None }));
			if let Some(device_out_) = device_out_ { *device.unwrap_unchecked() = Some(T::from(UnknownWrapper(device_out_))); }
			_ret_.ok()
		}
	}

	pub fn get_device<T: IUnknown + From<UnknownWrapper>>(&self) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut device_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &GUID, *mut c_void) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, T::IID, transmute(&mut device_out_));
			if _ret_.is_ok() { if let Some(device_out_) = device_out_ { return Ok(T::from(UnknownWrapper(device_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait ID3D12DeviceChild: ID3D12Object {
	fn as_device_child(&self) -> &D3D12DeviceChild;
	fn into_device_child(self) -> D3D12DeviceChild;
}

impl ID3D12DeviceChild for D3D12DeviceChild {
	fn as_device_child(&self) -> &D3D12DeviceChild { self }
	fn into_device_child(self) -> D3D12DeviceChild { self }
}
impl ID3D12Object for D3D12DeviceChild {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12DeviceChild {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DeviceChild {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Object::from(v))
    }
}

