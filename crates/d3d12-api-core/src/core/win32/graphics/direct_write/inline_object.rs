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
use crate::core::win32::graphics::direct_write::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DWriteInlineObject(pub(crate) Unknown);

impl Deref for DWriteInlineObject {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DWriteInlineObject {
	const IID: &'static GUID = &GUID::from_u128(0x8339fde3106f47ab83731c6295eb10b3u128);
}

impl Com for DWriteInlineObject {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DWriteInlineObject {
	pub fn Draw(&self, client_drawing_context: *const (), renderer: &DWriteTextRenderer, origin_x: f32, origin_y: f32, is_sideways: bool, is_right_to_left: bool, client_drawing_effect: Option<&Unknown>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, VTable, f32, f32, Bool, Bool, *const c_void) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, client_drawing_context as _, renderer.vtable(), origin_x, origin_y, is_sideways.to_bool(), is_right_to_left.to_bool(), option_to_vtable(client_drawing_effect));
			_ret_.ok()
		}
	}

	pub fn GetMetrics(&self) -> Result<DWriteInlineObjectMetrics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut metrics_out_: MaybeUninit<DWriteInlineObjectMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DWriteInlineObjectMetrics) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, metrics_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(metrics_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetOverhangMetrics(&self) -> Result<DWriteOverhangMetrics, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut overhangs_out_: MaybeUninit<DWriteOverhangMetrics> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DWriteOverhangMetrics) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, overhangs_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(overhangs_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetBreakConditions(&self) -> Result<(DWriteBreakCondition, DWriteBreakCondition), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut break_condition_before_out_: MaybeUninit<DWriteBreakCondition> = MaybeUninit::zeroed();
			let mut break_condition_after_out_: MaybeUninit<DWriteBreakCondition> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DWriteBreakCondition, *mut DWriteBreakCondition) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, break_condition_before_out_.as_mut_ptr(), break_condition_after_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((break_condition_before_out_.assume_init(), break_condition_after_out_.assume_init())) } else { Err(_ret_) }
		}
	}
}

pub trait IDWriteInlineObject: IUnknown {
	fn as_inline_object(&self) -> &DWriteInlineObject;
	fn into_inline_object(self) -> DWriteInlineObject;
}

impl IDWriteInlineObject for DWriteInlineObject {
	fn as_inline_object(&self) -> &DWriteInlineObject { self }
	fn into_inline_object(self) -> DWriteInlineObject { self }
}
impl IUnknown for DWriteInlineObject {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DWriteInlineObject {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

