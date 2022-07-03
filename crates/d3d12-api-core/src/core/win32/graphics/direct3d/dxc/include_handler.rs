#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct3d::dxc::*;
#[repr(C)]
pub struct DxcIncludeHandler(pub(crate) Unknown);

impl Guid for DxcIncludeHandler {
	const IID: &'static GUID = &GUID::from_u128(0x7f61fc7d950d467fb3e33c02fb49187cu128);
}

impl Clone for DxcIncludeHandler {
	fn clone(&self) -> Self { DxcIncludeHandler(self.0.clone()) }
}

pub trait IDxcIncludeHandler: IUnknown {
	fn as_include_handler(&self) -> &DxcIncludeHandler;
	fn into_include_handler(self) -> DxcIncludeHandler;

	fn LoadSource(&self, filename: &str, mut include_source: Option<&mut Option<DxcBlob>>,) -> Result<(), HResult> {
		let vt = self.as_param();
		if let Some(ref mut include_source) = include_source { **include_source = None; }
		let f: extern "system" fn(Param<Self>, filename: *const u16, include_source: Option<&mut Option<DxcBlob>>, ) -> HResult
			= unsafe { transmute(vt[3]) };
		let ret = f(vt, filename.to_utf16().as_ptr_or_null(), include_source, );
		ret.ok()
	}
}

impl IDxcIncludeHandler for DxcIncludeHandler {
	fn as_include_handler(&self) -> &DxcIncludeHandler { self }
	fn into_include_handler(self) -> DxcIncludeHandler { self }
}

impl From<Unknown> for DxcIncludeHandler {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxcIncludeHandler {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

