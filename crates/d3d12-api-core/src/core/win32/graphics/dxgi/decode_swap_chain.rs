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
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct DxgiDecodeSwapChain(pub(crate) Unknown);

impl Deref for DxgiDecodeSwapChain {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiDecodeSwapChain {
	const IID: &'static GUID = &GUID::from_u128(0x2633066b45144c7a8fd812ea98059d18u128);
}

impl Com for DxgiDecodeSwapChain {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiDecodeSwapChain {
	pub fn PresentBuffer(&self, buffer_to_present: u32, sync_interval: u32, flags: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, u32) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, buffer_to_present, sync_interval, flags);
			_ret_.ok()
		}
	}

	pub fn SetSourceRect(&self, rect: &Rect) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &Rect) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, rect);
			_ret_.ok()
		}
	}

	pub fn SetTargetRect(&self, rect: &Rect) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &Rect) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, rect);
			_ret_.ok()
		}
	}

	pub fn SetDestSize(&self, width: u32, height: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, width, height);
			_ret_.ok()
		}
	}

	pub fn GetSourceRect(&self) -> Result<Rect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut rect_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut Rect) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, rect_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(rect_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetTargetRect(&self) -> Result<Rect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut rect_out_: MaybeUninit<Rect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut Rect) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, rect_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(rect_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetDestSize(&self) -> Result<(u32, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut width_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut height_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32, *mut u32) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, width_out_.as_mut_ptr(), height_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((width_out_.assume_init(), height_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn SetColorSpace(&self, color_space: DxgiMultiplaneOverlayYCbCrFlags) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, DxgiMultiplaneOverlayYCbCrFlags) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, color_space);
			_ret_.ok()
		}
	}

	pub fn GetColorSpace(&self) -> DxgiMultiplaneOverlayYCbCrFlags {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> DxgiMultiplaneOverlayYCbCrFlags
				= transmute(vt[11]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait IDxgiDecodeSwapChain: IUnknown {
	fn as_decode_swap_chain(&self) -> &DxgiDecodeSwapChain;
	fn into_decode_swap_chain(self) -> DxgiDecodeSwapChain;
}

impl IDxgiDecodeSwapChain for DxgiDecodeSwapChain {
	fn as_decode_swap_chain(&self) -> &DxgiDecodeSwapChain { self }
	fn into_decode_swap_chain(self) -> DxgiDecodeSwapChain { self }
}
impl IUnknown for DxgiDecodeSwapChain {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiDecodeSwapChain {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

