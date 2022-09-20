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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::direct2d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1EffectContext2(pub(crate) D2D1EffectContext1);

impl Deref for D2D1EffectContext2 {
	type Target = D2D1EffectContext1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1EffectContext2 {
	const IID: &'static GUID = &GUID::from_u128(0x577ad2a09fc74dda8b18dab810140052u128);
}

impl Com for D2D1EffectContext2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1EffectContext2 {
	pub fn CreateColorContextFromDxgiColorSpace(&self, color_space: DxgiColorSpaceType) -> Result<D2D1ColorContext1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_context_out_: Option<D2D1ColorContext1> = None;
			let f: extern "system" fn(Param<Self>, DxgiColorSpaceType, *mut c_void) -> HResult
				= transmute(vt[25]);
			let _ret_ = f(vt, color_space, transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}

	pub fn CreateColorContextFromSimpleColorProfile(&self, simple_profile: &D2D1SimpleColorProfile) -> Result<D2D1ColorContext1, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut color_context_out_: Option<D2D1ColorContext1> = None;
			let f: extern "system" fn(Param<Self>, &D2D1SimpleColorProfile, *mut c_void) -> HResult
				= transmute(vt[26]);
			let _ret_ = f(vt, simple_profile, transmute(&mut color_context_out_));
			if _ret_.is_ok() { if let Some(color_context_out_) = color_context_out_ { return Ok(color_context_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1EffectContext2: ID2D1EffectContext1 {
	fn as_effect_context2(&self) -> &D2D1EffectContext2;
	fn into_effect_context2(self) -> D2D1EffectContext2;
}

impl ID2D1EffectContext2 for D2D1EffectContext2 {
	fn as_effect_context2(&self) -> &D2D1EffectContext2 { self }
	fn into_effect_context2(self) -> D2D1EffectContext2 { self }
}
impl ID2D1EffectContext1 for D2D1EffectContext2 {
	fn as_effect_context1(&self) -> &D2D1EffectContext1 { &self.0.as_effect_context1() }
	fn into_effect_context1(self) -> D2D1EffectContext1 { self.0.into_effect_context1() }
}

impl ID2D1EffectContext for D2D1EffectContext2 {
	fn as_effect_context(&self) -> &D2D1EffectContext { &self.0.as_effect_context() }
	fn into_effect_context(self) -> D2D1EffectContext { self.0.into_effect_context() }
}

impl IUnknown for D2D1EffectContext2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1EffectContext2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1EffectContext1::from(v))
    }
}

