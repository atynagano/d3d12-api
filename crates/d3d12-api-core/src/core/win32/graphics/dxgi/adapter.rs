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

#[repr(C)]
pub struct DxgiAdapter(pub(crate) DxgiObject);

impl Guid for DxgiAdapter {
	const IID: &'static GUID = &GUID::from_u128(0x2411e7e112ac4ccfbd149798e8534dc0u128);
}

impl Clone for DxgiAdapter {
	fn clone(&self) -> Self { DxgiAdapter(self.0.clone()) }
}

pub trait IDxgiAdapter: IDxgiObject {
	fn as_adapter(&self) -> &DxgiAdapter;
	fn into_adapter(self) -> DxgiAdapter;

	fn EnumOutputs(&self, output: u32, ) -> Result<DxgiOutput, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_output: Option<DxgiOutput> = None;
			let f: extern "system" fn(Param<Self>, output: u32, output: *mut c_void, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, output, transmute(&mut _out_output), );
			if _ret_.is_ok() {
				if let Some(_out_output) = _out_output {
					return Ok(_out_output);
				}
			}
			Err(_ret_)
		}
	}

	fn GetDesc(&self, ) -> Result<DxgiAdapterDesc, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_desc: MaybeUninit<DxgiAdapterDesc> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, _out_desc: *mut DxgiAdapterDesc, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, _out_desc.as_mut_ptr(), );
			Ok(_out_desc.assume_init())
		}
	}

	fn CheckInterfaceSupport(&self, interface_name: &GUID, ) -> Result<i64, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_umd_version: MaybeUninit<i64> = MaybeUninit::uninit();
			let f: extern "system" fn(Param<Self>, interface_name: &GUID, _out_umd_version: *mut i64, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, interface_name, _out_umd_version.as_mut_ptr(), );
			Ok(_out_umd_version.assume_init())
		}
	}
}

impl IDxgiAdapter for DxgiAdapter {
	fn as_adapter(&self) -> &DxgiAdapter { self }
	fn into_adapter(self) -> DxgiAdapter { self }
}

impl IDxgiObject for DxgiAdapter {
	fn as_object(&self) -> &DxgiObject { &self.0 }
	fn into_object(self) -> DxgiObject { self.0 }
}

impl From<Unknown> for DxgiAdapter {
    fn from(v: Unknown) -> Self {
        Self(DxgiObject::from(v))
    }
}

impl IUnknown for DxgiAdapter {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

