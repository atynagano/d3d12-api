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
pub struct D2D1RenderInfo(pub(crate) Unknown);

impl Deref for D2D1RenderInfo {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1RenderInfo {
	const IID: &'static GUID = &GUID::from_u128(0x519ae1bdd19a420db849364f594776b7u128);
}

impl Com for D2D1RenderInfo {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1RenderInfo {
	pub fn SetInputDescription(&self, input_index: u32, input_description: D2D1InputDescription) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D2D1InputDescription) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, input_index, input_description);
			_ret_.ok()
		}
	}

	pub fn SetOutputBuffer(&self, buffer_precision: D2D1BufferPrecision, channel_depth: D2D1ChannelDepth) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D2D1BufferPrecision, D2D1ChannelDepth) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, buffer_precision, channel_depth);
			_ret_.ok()
		}
	}

	pub fn SetCached(&self, is_cached: bool) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool) -> ()
				= transmute(vt[5]);
			let _ret_ = f(vt, is_cached.to_bool());
		}
	}

	pub fn SetInstructionCountHint(&self, instruction_count: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> ()
				= transmute(vt[6]);
			let _ret_ = f(vt, instruction_count);
		}
	}
}

pub trait ID2D1RenderInfo: IUnknown {
	fn as_render_info(&self) -> &D2D1RenderInfo;
	fn into_render_info(self) -> D2D1RenderInfo;
}

impl ID2D1RenderInfo for D2D1RenderInfo {
	fn as_render_info(&self) -> &D2D1RenderInfo { self }
	fn into_render_info(self) -> D2D1RenderInfo { self }
}
impl IUnknown for D2D1RenderInfo {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1RenderInfo {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

