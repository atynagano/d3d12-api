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
pub struct DxgiSwapChain2(pub(crate) DxgiSwapChain1);

impl Deref for DxgiSwapChain2 {
	type Target = DxgiSwapChain1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for DxgiSwapChain2 {
	const IID: &'static GUID = &GUID::from_u128(0xa8be2ac4199f4946b33179599fb98de7u128);
}

impl Com for DxgiSwapChain2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl DxgiSwapChain2 {
	pub fn SetSourceSize(&self, width: u32, height: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, width, height);
			_ret_.ok()
		}
	}

	pub fn GetSourceSize(&self) -> Result<(u32, u32), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut width_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let mut height_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32, *mut u32) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, width_out_.as_mut_ptr(), height_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok((width_out_.assume_init(), height_out_.assume_init())) } else { Err(_ret_) }
		}
	}

	pub fn SetMaximumFrameLatency(&self, max_latency: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, max_latency);
			_ret_.ok()
		}
	}

	pub fn GetMaximumFrameLatency(&self) -> Result<u32, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut max_latency_out_: MaybeUninit<u32> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut u32) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, max_latency_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(max_latency_out_.assume_init()) } else { Err(_ret_) }
		}
	}

	pub fn GetFrameLatencyWaitableObject(&self) -> Result<Handle, Win32Error> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> *const c_void
				= transmute(vt[33]);
			let _ret_ = f(vt);
			if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
		}
	}

	pub fn SetMatrixTransform(&self, matrix: &DxgiMatrix3x2F) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &DxgiMatrix3x2F) -> HResult
				= transmute(vt[34]);
			let _ret_ = f(vt, matrix);
			_ret_.ok()
		}
	}

	pub fn GetMatrixTransform(&self) -> Result<DxgiMatrix3x2F, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut matrix_out_: MaybeUninit<DxgiMatrix3x2F> = MaybeUninit::zeroed();
			let f: extern "system" fn(Param<Self>, *mut DxgiMatrix3x2F) -> HResult
				= transmute(vt[35]);
			let _ret_ = f(vt, matrix_out_.as_mut_ptr());
			if _ret_.is_ok() { Ok(matrix_out_.assume_init()) } else { Err(_ret_) }
		}
	}
}

pub trait IDxgiSwapChain2: IDxgiSwapChain1 {
	fn as_swap_chain2(&self) -> &DxgiSwapChain2;
	fn into_swap_chain2(self) -> DxgiSwapChain2;
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

impl IUnknown for DxgiSwapChain2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for DxgiSwapChain2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(DxgiSwapChain1::from(v))
    }
}

