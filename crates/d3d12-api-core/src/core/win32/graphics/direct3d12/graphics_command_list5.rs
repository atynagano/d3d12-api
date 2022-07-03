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
pub struct D3D12GraphicsCommandList5(pub(crate) D3D12GraphicsCommandList4);

impl Guid for D3D12GraphicsCommandList5 {
	const IID: &'static GUID = &GUID::from_u128(0x550508594024474c87f56472eaee44eau128);
}

impl Clone for D3D12GraphicsCommandList5 {
	fn clone(&self) -> Self { D3D12GraphicsCommandList5(self.0.clone()) }
}

pub trait ID3D12GraphicsCommandList5: ID3D12GraphicsCommandList4 {
	fn as_graphics_command_list5(&self) -> &D3D12GraphicsCommandList5;
	fn into_graphics_command_list5(self) -> D3D12GraphicsCommandList5;

	fn RSSetShadingRate(&self, base_shading_rate: D3D12ShadingRate, combiners: Option<&D3D12ShadingRateCombiner>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, base_shading_rate: D3D12ShadingRate, combiners: Option<&D3D12ShadingRateCombiner>, ) -> ()
			= unsafe { transmute(vt[77]) };
		let ret = f(vt, base_shading_rate, combiners, );
	}

	fn RSSetShadingRateImage(&self, shading_rate_image: Option<&D3D12Resource>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, shading_rate_image: Option<VTable>, ) -> ()
			= unsafe { transmute(vt[78]) };
		let ret = f(vt, option_to_vtable(shading_rate_image), );
	}
}

impl ID3D12GraphicsCommandList5 for D3D12GraphicsCommandList5 {
	fn as_graphics_command_list5(&self) -> &D3D12GraphicsCommandList5 { self }
	fn into_graphics_command_list5(self) -> D3D12GraphicsCommandList5 { self }
}

impl ID3D12GraphicsCommandList4 for D3D12GraphicsCommandList5 {
	fn as_graphics_command_list4(&self) -> &D3D12GraphicsCommandList4 { &self.0 }
	fn into_graphics_command_list4(self) -> D3D12GraphicsCommandList4 { self.0 }
}

impl ID3D12GraphicsCommandList3 for D3D12GraphicsCommandList5 {
	fn as_graphics_command_list3(&self) -> &D3D12GraphicsCommandList3 { &self.0.0 }
	fn into_graphics_command_list3(self) -> D3D12GraphicsCommandList3 { self.0.0 }
}

impl ID3D12GraphicsCommandList2 for D3D12GraphicsCommandList5 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2 { &self.0.0.0 }
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2 { self.0.0.0 }
}

impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList5 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { &self.0.0.0.0 }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self.0.0.0.0 }
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList5 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0.0.0.0.0 }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0.0.0.0.0 }
}

impl ID3D12CommandList for D3D12GraphicsCommandList5 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.0.0.0.0.0 }
	fn into_command_list(self) -> D3D12CommandList { self.0.0.0.0.0.0 }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList5 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0.0.0.0.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0.0.0.0.0.0 }
}

impl ID3D12Object for D3D12GraphicsCommandList5 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0.0.0.0 }
}

impl From<Unknown> for D3D12GraphicsCommandList5 {
    fn from(v: Unknown) -> Self {
        Self(D3D12GraphicsCommandList4::from(v))
    }
}

impl IUnknown for D3D12GraphicsCommandList5 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0.0 }
}

