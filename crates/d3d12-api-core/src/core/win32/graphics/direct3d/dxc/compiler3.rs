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
use crate::core::win32::graphics::direct3d::dxc::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxcCompiler3(pub(crate) Unknown);

impl Deref for DxcCompiler3 {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxcCompiler3 {
	const IID: &'static GUID = &GUID::from_u128(0x228b46875a6a4730900c9702b2203f54u128);
}

impl Com for DxcCompiler3 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxcCompiler3 {
	pub fn Compile<T: IUnknown + From<UnknownWrapper>>(&self, source: &DxcBuffer, arguments: Option<&[&str]>, include_handler: Option<&DxcIncludeHandler>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let (arguments_ptr_, arguments_len_) = arguments.deconstruct();
			let mut result_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &DxcBuffer, *const PWStr, u32, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, source, arguments.to_utf16_vec().ptr(), arguments_len_ as u32, option_to_vtable(include_handler), T::IID, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(T::from(UnknownWrapper(result_out_))); } }
			Err(_ret_)
		}
	}

	pub fn Disassemble<T: IUnknown + From<UnknownWrapper>>(&self, object: &DxcBuffer) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut result_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &DxcBuffer, &GUID, *mut c_void) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, object, T::IID, transmute(&mut result_out_));
			if _ret_.is_ok() { if let Some(result_out_) = result_out_ { return Ok(T::from(UnknownWrapper(result_out_))); } }
			Err(_ret_)
		}
	}
}

pub trait IDxcCompiler3: IUnknown {
	fn as_compiler3(&self) -> &DxcCompiler3;
	fn into_compiler3(self) -> DxcCompiler3;
}

impl IDxcCompiler3 for DxcCompiler3 {
	fn as_compiler3(&self) -> &DxcCompiler3 { self }
	fn into_compiler3(self) -> DxcCompiler3 { self }
}
impl IUnknown for DxcCompiler3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxcCompiler3 {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

