#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct3d12::*;
#[repr(C)]
pub struct D3D12InfoQueue1(pub(crate) D3D12InfoQueue);

impl Guid for D3D12InfoQueue1 {
	const IID: &'static GUID = &GUID::from_u128(0x2852dd88b4844c0cb6b167168500e600u128);
}

impl Clone for D3D12InfoQueue1 {
	fn clone(&self) -> Self { D3D12InfoQueue1(self.0.clone()) }
}

pub trait ID3D12InfoQueue1: ID3D12InfoQueue {
	fn as_info_queue1(&self) -> &D3D12InfoQueue1;
	fn into_info_queue1(self) -> D3D12InfoQueue1;

	fn RegisterMessageCallback(&self, callback_func: D3D12MessageFunc, callback_filter_flags: D3D12MessageCallbackFlags, context: *const c_void, callback_cookie: &mut u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, callback_func: D3D12MessageFunc, callback_filter_flags: D3D12MessageCallbackFlags, context: *const c_void, callback_cookie: &mut u32, ) -> HResult
			= unsafe { transmute(vt[38]) };
		let ret = f(vt, callback_func, callback_filter_flags, context, callback_cookie, );
		ret.ok()
	}

	fn UnregisterMessageCallback(&self, callback_cookie: u32, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, callback_cookie: u32, ) -> HResult
			= unsafe { transmute(vt[39]) };
		let ret = f(vt, callback_cookie, );
		ret.ok()
	}
}

impl ID3D12InfoQueue1 for D3D12InfoQueue1 {
	fn as_info_queue1(&self) -> &D3D12InfoQueue1 { self }
	fn into_info_queue1(self) -> D3D12InfoQueue1 { self }
}

impl ID3D12InfoQueue for D3D12InfoQueue1 {
	fn as_info_queue(&self) -> &D3D12InfoQueue { &self.0 }
	fn into_info_queue(self) -> D3D12InfoQueue { self.0 }
}

impl From<Unknown> for D3D12InfoQueue1 {
    fn from(v: Unknown) -> Self {
        Self(D3D12InfoQueue::from(v))
    }
}

impl IUnknown for D3D12InfoQueue1 {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

