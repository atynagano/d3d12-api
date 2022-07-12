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
pub struct D3D12DebugCommandList2(pub(crate) D3D12DebugCommandList);

impl Guid for D3D12DebugCommandList2 {
	const IID: &'static GUID = &GUID::from_u128(0xaeb575cf4e0648beba3bc450fc96652eu128);
}

impl Clone for D3D12DebugCommandList2 {
	fn clone(&self) -> Self { D3D12DebugCommandList2(self.0.clone()) }
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

impl From<Unknown> for D3D12DebugCommandList2 {
    fn from(v: Unknown) -> Self {
        Self(D3D12DebugCommandList::from(v))
    }
}

impl IUnknown for D3D12DebugCommandList2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

