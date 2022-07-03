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

use crate::core::win32::graphics::direct3d12::*;
#[repr(C)]
pub struct D3D12GraphicsCommandList2(pub(crate) D3D12GraphicsCommandList1);

impl Guid for D3D12GraphicsCommandList2 {
	const IID: &'static GUID = &GUID::from_u128(0x38c3e585ff17412c91504fc6f9d72a28u128);
}

impl Clone for D3D12GraphicsCommandList2 {
	fn clone(&self) -> Self { D3D12GraphicsCommandList2(self.0.clone()) }
}

pub trait ID3D12GraphicsCommandList2: ID3D12GraphicsCommandList1 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2;
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2;

	fn WriteBufferImmediate(&self, params: &[D3D12WriteBufferImmediateParameter], modes: Option<&[D3D12WriteBufferImmediateMode]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, count: u32, params: *const D3D12WriteBufferImmediateParameter, modes: *const D3D12WriteBufferImmediateMode, ) -> ()
			= unsafe { transmute(vt[66]) };
		let ret = f(vt, params.len() as u32, params.as_ptr_or_null(), modes.as_ptr_or_null(), );
	}
}

impl ID3D12GraphicsCommandList2 for D3D12GraphicsCommandList2 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2 { self }
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2 { self }
}

impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList2 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { &self.0 }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self.0 }
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList2 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0.0 }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0.0 }
}

impl ID3D12CommandList for D3D12GraphicsCommandList2 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.0.0 }
	fn into_command_list(self) -> D3D12CommandList { self.0.0.0 }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList2 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0.0.0 }
}

impl ID3D12Object for D3D12GraphicsCommandList2 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0 }
}

impl From<Unknown> for D3D12GraphicsCommandList2 {
    fn from(v: Unknown) -> Self {
        Self(D3D12GraphicsCommandList1::from(v))
    }
}

impl IUnknown for D3D12GraphicsCommandList2 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0 }
}

