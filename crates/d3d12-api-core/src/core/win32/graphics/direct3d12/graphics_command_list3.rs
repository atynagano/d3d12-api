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

use crate::core::win32::graphics::direct3d12::*;

#[repr(C)]
pub struct D3D12GraphicsCommandList3(pub(crate) D3D12GraphicsCommandList2);

impl Guid for D3D12GraphicsCommandList3 {
	const IID: &'static GUID = &GUID::from_u128(0x6fda83a7b84c4e389ac8c7bd22016b3du128);
}

impl Clone for D3D12GraphicsCommandList3 {
	fn clone(&self) -> Self { D3D12GraphicsCommandList3(self.0.clone()) }
}

pub trait ID3D12GraphicsCommandList3: ID3D12GraphicsCommandList2 {
	fn as_graphics_command_list3(&self) -> &D3D12GraphicsCommandList3;
	fn into_graphics_command_list3(self) -> D3D12GraphicsCommandList3;

	fn SetProtectedResourceSession(&self, protected_resource_session: Option<&D3D12ProtectedResourceSession>, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, protected_resource_session: *const c_void, ) -> ()
				= transmute(vt[67]);
			let _ret_ = f(vt, option_to_vtable(protected_resource_session), );
		}
	}
}

impl ID3D12GraphicsCommandList3 for D3D12GraphicsCommandList3 {
	fn as_graphics_command_list3(&self) -> &D3D12GraphicsCommandList3 { self }
	fn into_graphics_command_list3(self) -> D3D12GraphicsCommandList3 { self }
}

impl ID3D12GraphicsCommandList2 for D3D12GraphicsCommandList3 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2 { &self.0.as_graphics_command_list2() }
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2 { self.0.into_graphics_command_list2() }
}

impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList3 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { &self.0.as_graphics_command_list1() }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self.0.into_graphics_command_list1() }
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList3 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0.as_graphics_command_list() }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0.into_graphics_command_list() }
}

impl ID3D12CommandList for D3D12GraphicsCommandList3 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.as_command_list() }
	fn into_command_list(self) -> D3D12CommandList { self.0.into_command_list() }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList3 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12GraphicsCommandList3 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl From<Unknown> for D3D12GraphicsCommandList3 {
    fn from(v: Unknown) -> Self {
        Self(D3D12GraphicsCommandList2::from(v))
    }
}

impl IUnknown for D3D12GraphicsCommandList3 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

