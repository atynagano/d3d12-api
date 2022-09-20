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
pub struct D2D1EffectContext1(pub(crate) D2D1EffectContext);

impl Deref for D2D1EffectContext1 {
	type Target = D2D1EffectContext;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1EffectContext1 {
	const IID: &'static GUID = &GUID::from_u128(0x84ab595afc814546bacde8ef4d8abe7au128);
}

impl Com for D2D1EffectContext1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1EffectContext1 {
	pub fn CreateLookupTable3D(&self, precision: D2D1BufferPrecision, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> Result<D2D1LookupTable3D, HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let mut lookup_table_out_: Option<D2D1LookupTable3D> = None;
			let f: extern "system" fn(Param<Self>, D2D1BufferPrecision, *const u32, *const u8, u32, *const u32, *mut c_void) -> HResult
				= transmute(vt[24]);
			let _ret_ = f(vt, precision, extents.as_ptr_or_null(), data_ptr_, data_len_ as u32, strides.as_ptr_or_null(), transmute(&mut lookup_table_out_));
			if _ret_.is_ok() { if let Some(lookup_table_out_) = lookup_table_out_ { return Ok(lookup_table_out_); } }
			Err(_ret_)
		}
	}
}

pub trait ID2D1EffectContext1: ID2D1EffectContext {
	fn as_effect_context1(&self) -> &D2D1EffectContext1;
	fn into_effect_context1(self) -> D2D1EffectContext1;
}

impl ID2D1EffectContext1 for D2D1EffectContext1 {
	fn as_effect_context1(&self) -> &D2D1EffectContext1 { self }
	fn into_effect_context1(self) -> D2D1EffectContext1 { self }
}
impl ID2D1EffectContext for D2D1EffectContext1 {
	fn as_effect_context(&self) -> &D2D1EffectContext { &self.0.as_effect_context() }
	fn into_effect_context(self) -> D2D1EffectContext { self.0.into_effect_context() }
}

impl IUnknown for D2D1EffectContext1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1EffectContext1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D2D1EffectContext::from(v))
    }
}

