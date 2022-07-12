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
use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
pub struct D3D12DebugCommandQueue(pub(crate) Unknown);

impl Guid for D3D12DebugCommandQueue {
	const IID: &'static GUID = &GUID::from_u128(0x09e0bf3654ac484f88474baeeab6053au128);
}

impl Clone for D3D12DebugCommandQueue {
	fn clone(&self) -> Self { D3D12DebugCommandQueue(self.0.clone()) }
}

pub trait ID3D12DebugCommandQueue: IUnknown {
	fn as_debug_command_queue(&self) -> &D3D12DebugCommandQueue;
	fn into_debug_command_queue(self) -> D3D12DebugCommandQueue;

	fn AssertResourceState(&self, resource: &(impl ID3D12Resource + ?Sized), subresource: u32, state: u32, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, resource: VTable, subresource: u32, state: u32, ) -> Bool
				= transmute(vt[3]);
			let _ret_ = f(vt, resource.vtable(), subresource, state, );
			_ret_.to_bool()
		}
	}
}

impl ID3D12DebugCommandQueue for D3D12DebugCommandQueue {
	fn as_debug_command_queue(&self) -> &D3D12DebugCommandQueue { self }
	fn into_debug_command_queue(self) -> D3D12DebugCommandQueue { self }
}

impl From<Unknown> for D3D12DebugCommandQueue {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12DebugCommandQueue {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

