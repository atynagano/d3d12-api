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
use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12InfoQueue1(pub(crate) D3D12InfoQueue);

impl Deref for D3D12InfoQueue1 {
	type Target = D3D12InfoQueue;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12InfoQueue1 {
	const IID: &'static GUID = &GUID::from_u128(0x2852dd88b4844c0cb6b167168500e600u128);
}

impl Com for D3D12InfoQueue1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12InfoQueue1 {
	pub fn RegisterMessageCallback(&self, callback_func: D3D12MessageFunc, callback_filter_flags: D3D12MessageCallbackFlags, context: *const impl Sized, callback_cookie: &mut u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12MessageFunc, D3D12MessageCallbackFlags, *const c_void, &mut u32) -> HResult
				= transmute(vt[38]);
			let _ret_ = f(vt, callback_func, callback_filter_flags, context as _, callback_cookie);
			_ret_.ok()
		}
	}

	pub fn UnregisterMessageCallback(&self, callback_cookie: u32) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> HResult
				= transmute(vt[39]);
			let _ret_ = f(vt, callback_cookie);
			_ret_.ok()
		}
	}
}

pub trait ID3D12InfoQueue1: ID3D12InfoQueue {
	fn as_info_queue1(&self) -> &D3D12InfoQueue1;
	fn into_info_queue1(self) -> D3D12InfoQueue1;
}

impl ID3D12InfoQueue1 for D3D12InfoQueue1 {
	fn as_info_queue1(&self) -> &D3D12InfoQueue1 { self }
	fn into_info_queue1(self) -> D3D12InfoQueue1 { self }
}
impl ID3D12InfoQueue for D3D12InfoQueue1 {
	fn as_info_queue(&self) -> &D3D12InfoQueue { &self.0.as_info_queue() }
	fn into_info_queue(self) -> D3D12InfoQueue { self.0.into_info_queue() }
}

impl IUnknown for D3D12InfoQueue1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12InfoQueue1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12InfoQueue::from(v))
    }
}

