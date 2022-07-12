#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{MaybeUninit, size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::dxgi::*;

#[repr(C)]
pub struct DxgiDecodeSwapChain(pub(crate) Unknown);

impl Guid for DxgiDecodeSwapChain {
	const IID: &'static GUID = &GUID::from_u128(0x2633066b45144c7a8fd812ea98059d18u128);
}

impl Clone for DxgiDecodeSwapChain {
	fn clone(&self) -> Self { DxgiDecodeSwapChain(self.0.clone()) }
}

pub trait IDxgiDecodeSwapChain: IUnknown {
	fn as_decode_swap_chain(&self) -> &DxgiDecodeSwapChain;
	fn into_decode_swap_chain(self) -> DxgiDecodeSwapChain;

	fn PresentBuffer(&self, buffer_to_present: u32, sync_interval: u32, flags: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, buffer_to_present: u32, sync_interval: u32, flags: u32, ) -> HResult
				= transmute(vt[3]);
			let _ret_ = f(vt, buffer_to_present, sync_interval, flags, );
			_ret_.ok()
		}
	}

	fn SetSourceRect(&self, rect: &Rect, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, rect: &Rect, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, rect, );
			_ret_.ok()
		}
	}

	fn SetTargetRect(&self, rect: &Rect, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, rect: &Rect, ) -> HResult
				= transmute(vt[5]);
			let _ret_ = f(vt, rect, );
			_ret_.ok()
		}
	}

	fn SetDestSize(&self, width: u32, height: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, width: u32, height: u32, ) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, width, height, );
			_ret_.ok()
		}
	}

	fn GetSourceRect(&self, ) -> Result<Rect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_rect: MaybeUninit<Rect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_rect: *mut Rect, ) -> HResult
				= transmute(vt[7]);
			let _ret_ = f(vt, _out_rect.as_mut_ptr(), );
			Ok(_out_rect.assume_init())
		}
	}

	fn GetTargetRect(&self, ) -> Result<Rect, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_rect: MaybeUninit<Rect> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_rect: *mut Rect, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, _out_rect.as_mut_ptr(), );
			Ok(_out_rect.assume_init())
		}
	}

	fn GetDestSize(&self, ) -> Result<(u32, u32, ), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_width: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut _out_height: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_width: *mut u32, _out_height: *mut u32, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, _out_width.as_mut_ptr(), _out_height.as_mut_ptr(), );
			if _ret_.is_ok() {
				return Ok((_out_width.assume_init(), _out_height.assume_init(), ));
			}
			Err(_ret_)
		}
	}

	fn SetColorSpace(&self, color_space: DxgiMultiplaneOverlayYcbcrFlags, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, color_space: DxgiMultiplaneOverlayYcbcrFlags, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, color_space, );
			_ret_.ok()
		}
	}

	fn GetColorSpace(&self, ) -> DxgiMultiplaneOverlayYcbcrFlags {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> DxgiMultiplaneOverlayYcbcrFlags
				= transmute(vt[11]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl IDxgiDecodeSwapChain for DxgiDecodeSwapChain {
	fn as_decode_swap_chain(&self) -> &DxgiDecodeSwapChain { self }
	fn into_decode_swap_chain(self) -> DxgiDecodeSwapChain { self }
}

impl From<Unknown> for DxgiDecodeSwapChain {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for DxgiDecodeSwapChain {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

