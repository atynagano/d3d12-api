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
pub struct D3D12DebugCommandList(pub(crate) Unknown);

impl Guid for D3D12DebugCommandList {
	const IID: &'static GUID = &GUID::from_u128(0x09e0bf3654ac484f88474baeeab6053fu128);
}

impl Clone for D3D12DebugCommandList {
	fn clone(&self) -> Self { D3D12DebugCommandList(self.0.clone()) }
}

pub trait ID3D12DebugCommandList: IUnknown {
	fn as_debug_command_list(&self) -> &D3D12DebugCommandList;
	fn into_debug_command_list(self) -> D3D12DebugCommandList;

	fn AssertResourceState(&self, resource: &(impl ID3D12Resource + ?Sized), subresource: u32, state: u32, ) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, resource: VTable, subresource: u32, state: u32, ) -> Bool
				= transmute(vt[3]);
			let _ret_ = f(vt, resource.vtable(), subresource, state, );
			_ret_.to_bool()
		}
	}

	fn SetFeatureMask(&self, mask: D3D12DebugFeature, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, mask: D3D12DebugFeature, ) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, mask, );
			_ret_.ok()
		}
	}

	fn GetFeatureMask(&self, ) -> D3D12DebugFeature {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> D3D12DebugFeature
				= transmute(vt[5]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl ID3D12DebugCommandList for D3D12DebugCommandList {
	fn as_debug_command_list(&self) -> &D3D12DebugCommandList { self }
	fn into_debug_command_list(self) -> D3D12DebugCommandList { self }
}

impl From<Unknown> for D3D12DebugCommandList {
    fn from(v: Unknown) -> Self {
        Self(Unknown::from(v))
    }
}

impl IUnknown for D3D12DebugCommandList {
	fn as_unknown(&self) -> &Unknown { &self.0 }
	fn into_unknown(self) -> Unknown { self.0 }
}

