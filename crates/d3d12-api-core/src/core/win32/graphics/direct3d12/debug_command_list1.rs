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
pub struct D3D12DebugCommandList1(pub(crate) Unknown);

impl Deref for D3D12DebugCommandList1 {
	type Target = Unknown;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DebugCommandList1 {
	const IID: &'static GUID = &GUID::from_u128(0x102ca951311b4b01b11fecb83e061b37u128);
}

impl Com for D3D12DebugCommandList1 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DebugCommandList1 {
	pub fn AssertResourceState(&self, resource: &D3D12Resource, subresource: u32, state: u32) -> bool {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, u32) -> Bool
				= transmute(vt[3]);
			let _ret_ = f(vt, resource.vtable(), subresource, state);
			_ret_.to_bool()
		}
	}

	pub fn SetDebugParameter(&self, r#type: D3D12DebugCommandListParameterType, data: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, D3D12DebugCommandListParameterType, *const u8, u32) -> HResult
				= transmute(vt[4]);
			let _ret_ = f(vt, r#type, data_ptr_, data_len_ as u32);
			_ret_.ok()
		}
	}

	pub unsafe fn GetDebugParameter(&self) { todo!() }
}

pub trait ID3D12DebugCommandList1: IUnknown {
	fn as_debug_command_list1(&self) -> &D3D12DebugCommandList1;
	fn into_debug_command_list1(self) -> D3D12DebugCommandList1;
}

impl ID3D12DebugCommandList1 for D3D12DebugCommandList1 {
	fn as_debug_command_list1(&self) -> &D3D12DebugCommandList1 { self }
	fn into_debug_command_list1(self) -> D3D12DebugCommandList1 { self }
}
impl IUnknown for D3D12DebugCommandList1 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DebugCommandList1 {
    fn from(v: UnknownWrapper) -> Self {
        Self(Unknown::from(v))
    }
}

