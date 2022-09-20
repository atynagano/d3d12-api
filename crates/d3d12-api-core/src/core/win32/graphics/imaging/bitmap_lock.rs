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

#[repr(C)]
#[derive(Clone, Hash)]
pub struct WICBitmapLock(pub(crate) Unknown);

impl Deref for WICBitmapLock {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for WICBitmapLock {
	const IID: &'static GUID = &GUID::from_u128(0x00000123a8f24877ba0afd2b6645fb94u128);
}

impl Com for WICBitmapLock {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl WICBitmapLock {
	pub fn GetSize(&self) -> Result<(u32, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pui_width_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut pui_height_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32, *mut u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, pui_width_out_.as_mut_ptr(), pui_height_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((pui_width_out_.assume_init(), pui_height_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn GetStride(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pcb_stride_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, pcb_stride_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(pcb_stride_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub unsafe fn GetDataPointer(&self) { todo!() }

	pub unsafe fn GetPixelFormat(&self) { todo!() }
}

pub trait IWICBitmapLock: IUnknown {
	fn as_bitmap_lock(&self) -> &WICBitmapLock;
	fn into_bitmap_lock(self) -> WICBitmapLock;
}

impl IWICBitmapLock for WICBitmapLock {
	fn as_bitmap_lock(&self) -> &WICBitmapLock { self }
	fn into_bitmap_lock(self) -> WICBitmapLock { self }
}
impl IUnknown for WICBitmapLock {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for WICBitmapLock {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

