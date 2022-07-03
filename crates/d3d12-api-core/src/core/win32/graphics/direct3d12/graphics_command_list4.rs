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
pub struct D3D12GraphicsCommandList4(pub(crate) D3D12GraphicsCommandList3);

impl Guid for D3D12GraphicsCommandList4 {
	const IID: &'static GUID = &GUID::from_u128(0x8754318ed3a9454198cf645b50dc4874u128);
}

impl Clone for D3D12GraphicsCommandList4 {
	fn clone(&self) -> Self { D3D12GraphicsCommandList4(self.0.clone()) }
}

pub trait ID3D12GraphicsCommandList4: ID3D12GraphicsCommandList3 {
	fn as_graphics_command_list4(&self) -> &D3D12GraphicsCommandList4;
	fn into_graphics_command_list4(self) -> D3D12GraphicsCommandList4;

	fn BeginRenderPass(&self, render_targets: Option<&[D3D12RenderPassRenderTargetDesc]>, depth_stencil: Option<&D3D12RenderPassDepthStencilDesc>, flags: D3D12RenderPassFlags, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_render_targets: u32, render_targets: *const D3D12RenderPassRenderTargetDesc, depth_stencil: Option<&D3D12RenderPassDepthStencilDesc>, flags: D3D12RenderPassFlags, ) -> ()
			= unsafe { transmute(vt[68]) };
		let ret = f(vt, render_targets.len() as u32, render_targets.as_ptr_or_null(), depth_stencil, flags, );
	}

	fn EndRenderPass(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[69]) };
		let ret = f(vt, );
	}

	fn InitializeMetaCommand(&self, meta_command: &(impl ID3D12MetaCommand + ?Sized), initialization_parameters_data: Option<&[u8]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, meta_command: VTable, initialization_parameters_data: *const u8, initialization_parameters_data_size_in_bytes: usize, ) -> ()
			= unsafe { transmute(vt[70]) };
		let ret = f(vt, meta_command.vtable(), initialization_parameters_data.as_ptr_or_null(), initialization_parameters_data.len() as usize, );
	}

	fn ExecuteMetaCommand(&self, meta_command: &(impl ID3D12MetaCommand + ?Sized), execution_parameters_data: Option<&[u8]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, meta_command: VTable, execution_parameters_data: *const u8, execution_parameters_data_size_in_bytes: usize, ) -> ()
			= unsafe { transmute(vt[71]) };
		let ret = f(vt, meta_command.vtable(), execution_parameters_data.as_ptr_or_null(), execution_parameters_data.len() as usize, );
	}

	fn BuildRaytracingAccelerationStructure(&self, desc: &D3D12BuildRaytracingAccelerationStructureDesc, postbuild_info_descs: Option<&[D3D12RaytracingAccelerationStructurePostBuildInfoDesc]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, desc: &D3D12BuildRaytracingAccelerationStructureDesc, num_postbuild_info_descs: u32, postbuild_info_descs: *const D3D12RaytracingAccelerationStructurePostBuildInfoDesc, ) -> ()
			= unsafe { transmute(vt[72]) };
		let ret = f(vt, desc, postbuild_info_descs.len() as u32, postbuild_info_descs.as_ptr_or_null(), );
	}

	fn EmitRaytracingAccelerationStructurePostbuildInfo(&self, desc: &D3D12RaytracingAccelerationStructurePostBuildInfoDesc, source_acceleration_structure_data: &[u64], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, desc: &D3D12RaytracingAccelerationStructurePostBuildInfoDesc, num_source_acceleration_structures: u32, source_acceleration_structure_data: *const u64, ) -> ()
			= unsafe { transmute(vt[73]) };
		let ret = f(vt, desc, source_acceleration_structure_data.len() as u32, source_acceleration_structure_data.as_ptr_or_null(), );
	}

	fn CopyRaytracingAccelerationStructure(&self, dest_acceleration_structure_data: u64, source_acceleration_structure_data: u64, mode: D3D12RaytracingAccelerationStructureCopyMode, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dest_acceleration_structure_data: u64, source_acceleration_structure_data: u64, mode: D3D12RaytracingAccelerationStructureCopyMode, ) -> ()
			= unsafe { transmute(vt[74]) };
		let ret = f(vt, dest_acceleration_structure_data, source_acceleration_structure_data, mode, );
	}

	fn SetPipelineState1(&self, state_object: &(impl ID3D12StateObject + ?Sized), ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, state_object: VTable, ) -> ()
			= unsafe { transmute(vt[75]) };
		let ret = f(vt, state_object.vtable(), );
	}

	fn DispatchRays(&self, desc: &D3D12DispatchRaysDesc, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, desc: &D3D12DispatchRaysDesc, ) -> ()
			= unsafe { transmute(vt[76]) };
		let ret = f(vt, desc, );
	}
}

impl ID3D12GraphicsCommandList4 for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list4(&self) -> &D3D12GraphicsCommandList4 { self }
	fn into_graphics_command_list4(self) -> D3D12GraphicsCommandList4 { self }
}

impl ID3D12GraphicsCommandList3 for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list3(&self) -> &D3D12GraphicsCommandList3 { &self.0 }
	fn into_graphics_command_list3(self) -> D3D12GraphicsCommandList3 { self.0 }
}

impl ID3D12GraphicsCommandList2 for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2 { &self.0.0 }
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2 { self.0.0 }
}

impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { &self.0.0.0 }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self.0.0.0 }
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0.0.0.0 }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0.0.0.0 }
}

impl ID3D12CommandList for D3D12GraphicsCommandList4 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.0.0.0.0 }
	fn into_command_list(self) -> D3D12CommandList { self.0.0.0.0.0 }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList4 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0.0.0.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0.0.0.0.0 }
}

impl ID3D12Object for D3D12GraphicsCommandList4 {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0.0.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0.0.0.0.0 }
}

impl From<Unknown> for D3D12GraphicsCommandList4 {
    fn from(v: Unknown) -> Self {
        Self(D3D12GraphicsCommandList3::from(v))
    }
}

impl IUnknown for D3D12GraphicsCommandList4 {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0.0.0.0.0 }
}

