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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Properties(pub(crate) Unknown);

impl Deref for D2D1Properties {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Properties {
	const IID: &'static GUID = &GUID::from_u128(0x483473d7cd464f9d9d3a3112aa80159du128);
}

impl Com for D2D1Properties {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Properties {
	pub fn GetPropertyCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[3]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub unsafe fn GetPropertyName(&self) { todo!() }

	pub fn GetPropertyNameLength(&self, index: u32) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> u32
				= transmute(vt[5]);
			let _ret_ = f(vt, index);
			_ret_
		}
	}

	pub fn GetType(&self, index: u32) -> D2D1PropertyType {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> D2D1PropertyType
				= transmute(vt[6]);
			let _ret_ = f(vt, index);
			_ret_
		}
	}

	pub fn GetPropertyIndex(&self, name: &str) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const u16) -> u32
				= transmute(vt[7]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null());
			_ret_
		}
	}

	pub fn SetValueByName(&self, name: &str, r#type: D2D1PropertyType, data: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, *const u16, D2D1PropertyType, *const u8, u32) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), r#type, data_ptr_, data_len_ as u32);
			_ret_.ok()
		}
	}

	pub fn SetValue(&self, index: u32, r#type: D2D1PropertyType, data: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, D2D1PropertyType, *const u8, u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, index, r#type, data_ptr_, data_len_ as u32);
			_ret_.ok()
		}
	}

	pub unsafe fn GetValueByName(&self) { todo!() }

	pub unsafe fn GetValue(&self) { todo!() }

	pub fn GetValueSize(&self, index: u32) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> u32
				= transmute(vt[12]);
			let _ret_ = f(vt, index);
			_ret_
		}
	}

	pub fn GetSubProperties(&self, index: u32, mut sub_properties: Option<&mut Option<D2D1Properties>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut sub_properties) = sub_properties { **sub_properties = None; }
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, index, transmute(sub_properties));
			_ret_.ok()
		}
	}

	pub fn get_sub_properties(&self, index: u32) -> Result<D2D1Properties, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut sub_properties_out_: Option<D2D1Properties> = None;
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt, index, transmute(&mut sub_properties_out_));
			if _ret_.is_ok() { if let Some(sub_properties_out_) = sub_properties_out_ { return Ok(sub_properties_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1Properties: IUnknown {
	fn as_properties(&self) -> &D2D1Properties;
	fn into_properties(self) -> D2D1Properties;
}

impl ID2D1Properties for D2D1Properties {
	fn as_properties(&self) -> &D2D1Properties { self }
	fn into_properties(self) -> D2D1Properties { self }
}
impl IUnknown for D2D1Properties {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Properties {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

