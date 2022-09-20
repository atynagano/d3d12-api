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

use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1Effect(pub(crate) D2D1Properties);

impl Deref for D2D1Effect {
	type Target = D2D1Properties;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1Effect {
	const IID: &'static GUID = &GUID::from_u128(0x28211a437d89476f81812d6159b220adu128);
}

impl Com for D2D1Effect {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1Effect {
	pub fn SetInput(&self, index: u32, input: Option<&D2D1Image>, invalidate: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, *const c_void, Bool) -> ()
				= transmute(vt[14]);
			let _ret_ = f(vt, index, option_to_vtable(input), invalidate.to_bool());
		}
	}

	pub fn SetInputCount(&self, input_count: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[15]);
			let _ret_ = f(vt, input_count);
			_ret_.ok()
		}
	}

	pub fn GetInput(&self, index: u32, mut input: Option<&mut Option<D2D1Image>>) -> () {
		unsafe {
			let vt = self.as_param();
			if let Some(ref mut input) = input { **input = None; }
			let f: extern "system" fn(Param<Self>, u32, *mut c_void) -> ()
				= transmute(vt[16]);
			let _ret_ = f(vt, index, transmute(input));
		}
	}

	pub fn GetInputCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[17]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn GetOutput(&self) -> Option<D2D1Image> {
		unsafe {
			let vt = self.as_param();
			let mut output_image_out_: Option<D2D1Image> = None;
			let f: extern "system" fn(Param<Self>, *mut c_void) -> ()
				= transmute(vt[18]);
			let _ret_ = f(vt, transmute(&mut output_image_out_));
			output_image_out_
		}
	}
}

pub trait ID2D1Effect: ID2D1Properties {
	fn as_effect(&self) -> &D2D1Effect;
	fn into_effect(self) -> D2D1Effect;
}

impl ID2D1Effect for D2D1Effect {
	fn as_effect(&self) -> &D2D1Effect { self }
	fn into_effect(self) -> D2D1Effect { self }
}
impl ID2D1Properties for D2D1Effect {
	fn as_properties(&self) -> &D2D1Properties { &self.0.as_properties() }
	fn into_properties(self) -> D2D1Properties { self.0.into_properties() }
}

impl IUnknown for D2D1Effect {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1Effect {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1Properties::from(v))
    }
}

