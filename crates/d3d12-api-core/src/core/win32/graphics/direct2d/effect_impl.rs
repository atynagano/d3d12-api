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
pub struct D2D1EffectImpl(pub(crate) Unknown);

impl Deref for D2D1EffectImpl {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1EffectImpl {
	const IID: &'static GUID = &GUID::from_u128(0xa248fd3f3e6c4e639f037f68ecc91db9u128);
}

impl Com for D2D1EffectImpl {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1EffectImpl {
	pub fn Initialize(&self, effect_context: &D2D1EffectContext, transform_graph: &D2D1TransformGraph) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, effect_context.vtable(), transform_graph.vtable());
			_ret_.ok()
		}
	}

	pub fn PrepareForRender(&self, change_type: D2D1ChangeType) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1ChangeType) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, change_type);
			_ret_.ok()
		}
	}

	pub fn SetGraph(&self, transform_graph: &D2D1TransformGraph) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, transform_graph.vtable());
			_ret_.ok()
		}
	}
}

pub trait ID2D1EffectImpl: IUnknown {
	fn as_effect_impl(&self) -> &D2D1EffectImpl;
	fn into_effect_impl(self) -> D2D1EffectImpl;
}

impl ID2D1EffectImpl for D2D1EffectImpl {
	fn as_effect_impl(&self) -> &D2D1EffectImpl { self }
	fn into_effect_impl(self) -> D2D1EffectImpl { self }
}
impl IUnknown for D2D1EffectImpl {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1EffectImpl {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

