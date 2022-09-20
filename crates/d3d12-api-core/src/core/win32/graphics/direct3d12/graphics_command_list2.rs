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

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12GraphicsCommandList2(pub(crate) D3D12GraphicsCommandList1);

impl Deref for D3D12GraphicsCommandList2 {
	type Target = D3D12GraphicsCommandList1;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12GraphicsCommandList2 {
	const IID: &'static GUID = &GUID::from_u128(0x38c3e585ff17412c91504fc6f9d72a28u128);
}

impl Com for D3D12GraphicsCommandList2 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12GraphicsCommandList2 {
	pub unsafe fn WriteBufferImmediate() { todo!() }
}

pub trait ID3D12GraphicsCommandList2: ID3D12GraphicsCommandList1 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2;
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2;
}

impl ID3D12GraphicsCommandList2 for D3D12GraphicsCommandList2 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2 { self }
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2 { self }
}
impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList2 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { &self.0.as_graphics_command_list1() }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self.0.into_graphics_command_list1() }
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList2 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0.as_graphics_command_list() }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0.into_graphics_command_list() }
}

impl ID3D12CommandList for D3D12GraphicsCommandList2 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.as_command_list() }
	fn into_command_list(self) -> D3D12CommandList { self.0.into_command_list() }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList2 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12GraphicsCommandList2 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12GraphicsCommandList2 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12GraphicsCommandList2 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12GraphicsCommandList1::from(v))
    }
}

