#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{MaybeUninit, size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::system::com::*;

#[repr(C)]
pub struct DxgiDevice(pub(crate) DxgiObject);

impl Guid for DxgiDevice {
	const IID: &'static GUID = &GUID::from_u128(0x54ec77fa137744e68c3288fd5f44c84cu128);
}

impl Clone for DxgiDevice {
	fn clone(&self) -> Self { DxgiDevice(self.0.clone()) }
}

pub trait IDxgiDevice: IDxgiObject {
	fn as_device(&self) -> &DxgiDevice;
	fn into_device(self) -> DxgiDevice;

	fn GetAdapter(&self, ) -> Result<DxgiAdapter, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_adapter: Option<DxgiAdapter> = None;
			let f: extern "system" fn(Param<Self>, adapter: *mut c_void, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, transmute(&mut _out_adapter), );
			if _ret_.is_ok() {
				if let Some(_out_adapter) = _out_adapter {
					return Ok(_out_adapter);
				}
			}
			Err(_ret_)
		}
	}

	fn SetGPUThreadPriority(&self, priority: i32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, priority: i32, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, priority, );
			_ret_.ok()
		}
	}

	fn GetGPUThreadPriority(&self, ) -> Result<i32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_priority: MaybeUninit<i32> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_priority: *mut i32, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, _out_priority.as_mut_ptr(), );
			Ok(_out_priority.assume_init())
		}
	}
}

impl IDxgiDevice for DxgiDevice {
	fn as_device(&self) -> &DxgiDevice { self }
	fn into_device(self) -> DxgiDevice { self }
}

impl IDxgiObject for DxgiDevice {
	fn as_object(&self) -> &DxgiObject { &self.0 }
	fn into_object(self) -> DxgiObject { self.0 }
}

impl From<Unknown> for DxgiDevice {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiDevice {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

