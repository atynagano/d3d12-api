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

use crate::core::win32::graphics::dxgi::*;
use crate::core::win32::foundation::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiOutputDuplication(pub(crate) DxgiObject);

impl Deref for DxgiOutputDuplication {
	type Target = DxgiObject;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiOutputDuplication {
	const IID: &'static GUID = &GUID::from_u128(0x191cfac3a341470db26ea864f428319cu128);
}

impl Com for DxgiOutputDuplication {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiOutputDuplication {
	pub fn GetDesc(&self) -> DxgiOutDuplDesc {
		unsafe {
			let vt = self.as_param();
			let mut desc_out_: MaybeUninit<DxgiOutDuplDesc> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiOutDuplDesc) -> ()
				= transmute(vt[7]);
			let _ret_ = f(vt, desc_out_.as_mut_ptr());
			desc_out_.assume_init()
		}
	}

	pub unsafe fn AcquireNextFrame(&self) { todo!() }

	pub unsafe fn GetFrameDirtyRects(&self) { todo!() }

	pub unsafe fn GetFrameMoveRects(&self) { todo!() }

	pub unsafe fn GetFramePointerShape(&self) { todo!() }

	pub fn MapDesktopSurface(&self) -> Result<DxgiMappedRect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut locked_rect_out_: MaybeUninit<DxgiMappedRect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiMappedRect) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, locked_rect_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(locked_rect_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn UnMapDesktopSurface(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[13]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn ReleaseFrame(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}
}

pub trait IDxgiOutputDuplication: IDxgiObject {
	fn as_output_duplication(&self) -> &DxgiOutputDuplication;
	fn into_output_duplication(self) -> DxgiOutputDuplication;
}

impl IDxgiOutputDuplication for DxgiOutputDuplication {
	fn as_output_duplication(&self) -> &DxgiOutputDuplication { self }
	fn into_output_duplication(self) -> DxgiOutputDuplication { self }
}
impl IDxgiObject for DxgiOutputDuplication {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl IUnknown for DxgiOutputDuplication {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiOutputDuplication {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiObject::from(v))
    }
}

