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

use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::foundation::*;

#[repr(C)]
pub struct DxgiOutputDuplication(pub(crate) DxgiObject);

impl Guid for DxgiOutputDuplication {
	const IID: &'static GUID = &GUID::from_u128(0x191cfac3a341470db26ea864f428319cu128);
}

impl Clone for DxgiOutputDuplication {
	fn clone(&self) -> Self { DxgiOutputDuplication(self.0.clone()) }
}

pub trait IDxgiOutputDuplication: IDxgiObject {
	fn as_output_duplication(&self) -> &DxgiOutputDuplication;
	fn into_output_duplication(self) -> DxgiOutputDuplication;

	fn GetDesc(&self, ) -> DxgiOutDuplDesc {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiOutDuplDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiOutDuplDesc, ) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			_out_desc.assume_init()
		}
	}

	fn MapDesktopSurface(&self, ) -> Result<DxgiMappedRect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_locked_rect: MaybeUninit<DxgiMappedRect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_locked_rect: *mut DxgiMappedRect, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, _out_locked_rect.as_mut_ptr(), );
			Ok(_out_locked_rect.assume_init())
		}
	}

	fn UnMapDesktopSurface(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}

	fn ReleaseFrame(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}
}

impl IDxgiOutputDuplication for DxgiOutputDuplication {
	fn as_output_duplication(&self) -> &DxgiOutputDuplication { self }
	fn into_output_duplication(self) -> DxgiOutputDuplication { self }
}

impl IDxgiObject for DxgiOutputDuplication {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiOutputDuplication {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiOutputDuplication {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

