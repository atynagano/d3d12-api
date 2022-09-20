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
pub struct D3D12DebugCommandList2(pub(crate) D3D12DebugCommandList);

impl Deref for D3D12DebugCommandList2 {
	type Target = D3D12DebugCommandList;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12DebugCommandList2 {
	const IID: &'static GUID = &GUID::from_u128(0xaeb575cf4e0648beba3bc450fc96652eu128);
}

impl Com for D3D12DebugCommandList2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12DebugCommandList2 {
	pub fn SetDebugParameter(&self, r#type: D3D12DebugCommandListParameterType, data: &[u8]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, D3D12DebugCommandListParameterType, *const u8, u32) -> HResult
				= transmute(vt[6]);
			let _ret_ = f(vt, r#type, data_ptr_, data_len_ as u32);
			_ret_.ok()
		}
	}

	pub unsafe fn GetDebugParameter(&self) { todo!() }
}

pub trait ID3D12DebugCommandList2: ID3D12DebugCommandList {
	fn as_debug_command_list2(&self) -> &D3D12DebugCommandList2;
	fn into_debug_command_list2(self) -> D3D12DebugCommandList2;
}

impl ID3D12DebugCommandList2 for D3D12DebugCommandList2 {
	fn as_debug_command_list2(&self) -> &D3D12DebugCommandList2 { self }
	fn into_debug_command_list2(self) -> D3D12DebugCommandList2 { self }
}
impl ID3D12DebugCommandList for D3D12DebugCommandList2 {
	fn as_debug_command_list(&self) -> &D3D12DebugCommandList { &self.0.as_debug_command_list() }
	fn into_debug_command_list(self) -> D3D12DebugCommandList { self.0.into_debug_command_list() }
}

impl IUnknown for D3D12DebugCommandList2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12DebugCommandList2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12DebugCommandList::from(v))
    }
}

