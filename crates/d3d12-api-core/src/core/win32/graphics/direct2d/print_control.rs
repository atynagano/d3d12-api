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
use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D2D1PrintControl(pub(crate) Unknown);

impl Deref for D2D1PrintControl {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D2D1PrintControl {
	const IID: &'static GUID = &GUID::from_u128(0x2c1d867dc29041c8ae7e34a98702e9a5u128);
}

impl Com for D2D1PrintControl {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D2D1PrintControl {
	pub fn AddPage(&self, command_list: &D2D1CommandList, page_size: D2DSizeF, page_print_ticket_stream: Option<&Stream>, tag1: Option<&mut MaybeUninit<u64>>, tag2: Option<&mut MaybeUninit<u64>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, D2DSizeF, *const c_void, Option<&mut MaybeUninit<u64>>, Option<&mut MaybeUninit<u64>>) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, command_list.vtable(), page_size, option_to_vtable(page_print_ticket_stream), tag1, tag2);
			_ret_.ok()
		}
	}

	pub fn Close(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait ID2D1PrintControl: IUnknown {
	fn as_print_control(&self) -> &D2D1PrintControl;
	fn into_print_control(self) -> D2D1PrintControl;
}

impl ID2D1PrintControl for D2D1PrintControl {
	fn as_print_control(&self) -> &D2D1PrintControl { self }
	fn into_print_control(self) -> D2D1PrintControl { self }
}
impl IUnknown for D2D1PrintControl {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D2D1PrintControl {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

