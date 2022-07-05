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


#[repr(C)]
pub struct D3D12GraphicsCommandList6(pub(crate) D3D12GraphicsCommandList5);

impl Guid for D3D12GraphicsCommandList6 {
	const IID: &'static GUID = &GUID::from_u128(0xc3827890e5484cfa96cf5689a9370f80u128);
}

impl Clone for D3D12GraphicsCommandList6 {
	fn clone(&self) -> Self { D3D12GraphicsCommandList6(self.0.clone()) }
}

pub trait ID3D12GraphicsCommandList6: ID3D12GraphicsCommandList5 {
	fn as_graphics_command_list6(&self) -> &D3D12GraphicsCommandList6;
	fn into_graphics_command_list6(self) -> D3D12GraphicsCommandList6;

	fn DispatchMesh(&self, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32, ) -> ()
				= transmute(vt[79]);
			let _ret_ = f(vt, thread_group_count_x, thread_group_count_y, thread_group_count_z, );
		}
	}
}

impl ID3D12GraphicsCommandList6 for D3D12GraphicsCommandList6 {
	fn as_graphics_command_list6(&self) -> &D3D12GraphicsCommandList6 { self }
	fn into_graphics_command_list6(self) -> D3D12GraphicsCommandList6 { self }
}

impl ID3D12GraphicsCommandList5 for D3D12GraphicsCommandList6 {
	fn as_graphics_command_list5(&self) -> &D3D12GraphicsCommandList5 { &self.0 }
	fn into_graphics_command_list5(self) -> D3D12GraphicsCommandList5 { self.0 }
}

impl ID3D12GraphicsCommandList4 for D3D12GraphicsCommandList6 {
	fn as_graphics_command_list4(&self) -> &D3D12GraphicsCommandList4 { &self.0.0 }
	fn into_graphics_command_list4(self) -> D3D12GraphicsCommandList4 { self.0.0 }
}

impl ID3D12GraphicsCommandList3 for D3D12GraphicsCommandList6 {
	fn as_graphics_command_list3(&self) -> &D3D12GraphicsCommandList3 { &self.0.0.0 }
	fn into_graphics_command_list3(self) -> D3D12GraphicsCommandList3 { self.0.0.0 }
}

impl ID3D12GraphicsCommandList2 for D3D12GraphicsCommandList6 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2 { &self.0.0.0.0 }
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2 { self.0.0.0.0 }
}

impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList6 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { &self.0.0.0.0.0 }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self.0.0.0.0.0 }
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList6 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0.0.0.0.0.0 }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0.0.0.0.0.0 }
}

impl ID3D12CommandList for D3D12GraphicsCommandList6 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.0.0.0.0.0.0 }
	fn into_command_list(self) -> D3D12CommandList { self.0.0.0.0.0.0.0 }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList6 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0.0.0.0.0.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0.0.0.0.0.0.0 }
}

impl ID3D12Object for D3D12GraphicsCommandList6 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0.0.0.0.0 }
}

impl From<Unknown> for D3D12GraphicsCommandList6 {
    fn from(v: Unknown) -> Self {
        Self(D3D12GraphicsCommandList5::from(v))
    }
}

impl IUnknown for D3D12GraphicsCommandList6 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0.0.0 }
}

