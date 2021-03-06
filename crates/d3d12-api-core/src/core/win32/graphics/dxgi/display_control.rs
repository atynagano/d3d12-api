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

#[repr(C)]
pub struct DxgiDisplayControl(pub(crate) Unknown);

impl Guid for DxgiDisplayControl {
	const IID: &'static GUID = &GUID::from_u128(0xea9dbf1ac88e4486854a98aa0138f30cu128);
}

impl Clone for DxgiDisplayControl {
	fn clone(&self) -> Self { DxgiDisplayControl(self.0.clone()) }
}

pub trait IDxgiDisplayControl: IUnknown {
	fn as_display_control(&self) -> &DxgiDisplayControl;
	fn into_display_control(self) -> DxgiDisplayControl;

	fn IsStereoEnabled(&self, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Bool
				= transmute(vt[3]);
			let _ret_ = f(vt, );
			_ret_.to_bool()
		}
	}

	fn SetStereoEnabled(&self, enabled: bool, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, enabled: Bool, ) -> ()
				= transmute(vt[4]);
			let _ret_ = f(vt, enabled.to_bool(), );
		}
	}
}

impl IDxgiDisplayControl for DxgiDisplayControl {
	fn as_display_control(&self) -> &DxgiDisplayControl { self }
	fn into_display_control(self) -> DxgiDisplayControl { self }
}

impl From<Unknown> for DxgiDisplayControl {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxgiDisplayControl {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

