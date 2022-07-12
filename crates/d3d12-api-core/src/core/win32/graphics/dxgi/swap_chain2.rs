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
pub struct DxgiSwapChain2(pub(crate) DxgiSwapChain1);

impl Guid for DxgiSwapChain2 {
	const IID: &'static GUID = &GUID::from_u128(0xa8be2ac4199f4946b33179599fb98de7u128);
}

impl Clone for DxgiSwapChain2 {
	fn clone(&self) -> Self { DxgiSwapChain2(self.0.clone()) }
}

pub trait IDxgiSwapChain2: IDxgiSwapChain1 {
	fn as_swap_chain2(&self) -> &DxgiSwapChain2;
	fn into_swap_chain2(self) -> DxgiSwapChain2;

	fn SetSourceSize(&self, width: u32, height: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, width: u32, height: u32, ) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, width, height, );
			_ret_.ok()
		}
	}

	fn GetSourceSize(&self, ) -> Result<(u32, u32, ), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_width: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut _out_height: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_width: *mut u32, _out_height: *mut u32, ) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, _out_width.as_mut_ptr(), _out_height.as_mut_ptr(), );
			if _ret_.is_ok() {
				return Ok((_out_width.assume_init(), _out_height.assume_init(), ));
			}
			Err(_ret_)
		}
	}

	fn SetMaximumFrameLatency(&self, max_latency: u32, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, max_latency: u32, ) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, max_latency, );
			_ret_.ok()
		}
	}

	fn GetMaximumFrameLatency(&self, ) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_max_latency: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_max_latency: *mut u32, ) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, _out_max_latency.as_mut_ptr(), );
			Ok(_out_max_latency.assume_init())
		}
	}

	fn GetFrameLatencyWaitableObject(&self, ) -> Option<Handle> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> *const c_void
				= transmute(vt[33]);
			let _ret_ = f(vt, );
			transmute(_ret_)
		}
	}

	fn SetMatrixTransform(&self, matrix: &DxgiMatrix3X2F, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, matrix: &DxgiMatrix3X2F, ) -> HResult
				= transmute(vt[34]);
			let _ret_ = f(vt, matrix, );
			_ret_.ok()
		}
	}

	fn GetMatrixTransform(&self, ) -> Result<DxgiMatrix3X2F, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_matrix: MaybeUninit<DxgiMatrix3X2F> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, _out_matrix: *mut DxgiMatrix3X2F, ) -> HResult
				= transmute(vt[35]);
			let _ret_ = f(vt, _out_matrix.as_mut_ptr(), );
			Ok(_out_matrix.assume_init())
		}
	}
}

impl IDxgiSwapChain2 for DxgiSwapChain2 {
	fn as_swap_chain2(&self) -> &DxgiSwapChain2 { self }
	fn into_swap_chain2(self) -> DxgiSwapChain2 { self }
}

impl IDxgiSwapChain1 for DxgiSwapChain2 {
	fn as_swap_chain1(&self) -> &DxgiSwapChain1 { &self.0.as_swap_chain1() }
	fn into_swap_chain1(self) -> DxgiSwapChain1 { self.0.into_swap_chain1() }
}

impl IDxgiSwapChain for DxgiSwapChain2 {
	fn as_swap_chain(&self) -> &DxgiSwapChain { &self.0.as_swap_chain() }
	fn into_swap_chain(self) -> DxgiSwapChain { self.0.into_swap_chain() }
}

impl IDxgiDeviceSubObject for DxgiSwapChain2 {
	fn as_device_sub_object(&self) -> &DxgiDeviceSubObject { &self.0.as_device_sub_object() }
	fn into_device_sub_object(self) -> DxgiDeviceSubObject { self.0.into_device_sub_object() }
}

impl IDxgiObject for DxgiSwapChain2 {
	fn as_object(&self) -> &DxgiObject { &self.0.as_object() }
	fn into_object(self) -> DxgiObject { self.0.into_object() }
}

impl From<Unknown> for DxgiSwapChain2 {
    fn from(v: Unknown) -> Self {
        Self(DxgiSwapChain1::from(v))
    }
}

impl IUnknown for DxgiSwapChain2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

