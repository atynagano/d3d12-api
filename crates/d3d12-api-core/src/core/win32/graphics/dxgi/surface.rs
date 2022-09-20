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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiSurface(pub(crate) DxgiDeviceSubObject);

impl Deref for DxgiSurface {
	type Target = DxgiDeviceSubObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiSurface {
	const IID: &'static GUID = &GUID::from_u128(0xcafcb56c6ac34889bf479e23bbd260ecu128);
}

impl Com for DxgiSurface {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiSurface {
	pub fn GetDesc(&self) -> Result<DxgiSurfaceDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiSurfaceDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiSurfaceDesc) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(desc_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn Map(&self, map_flags: u32) -> Result<DxgiMappedRect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut locked_rect_out_: MaybeUninit<DxgiMappedRect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiMappedRect, u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, locked_rect_out_.as_mut_ptr(), map_flags);
			if _ret_.is_ok() { Ok(locked_rect_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn Unmap(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait IDxgiSurface: IDxgiDeviceSubObject {
	fn as_surface(&self) -> &DxgiSurface;
	fn into_surface(self) -> DxgiSurface;
}

impl IDxgiSurface for DxgiSurface {
	fn as_surface(&self) -> &DxgiSurface { self }
	fn into_surface(self) -> DxgiSurface { self }
}
impl IDxgiDeviceSubObject for DxgiSurface {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiSurface {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiSurface {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiSurface {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiDeviceSubObject::from(v))
    }
}

