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
pub struct D3D12GraphicsCommandList4(pub(crate) D3D12GraphicsCommandList3);

impl Deref for D3D12GraphicsCommandList4 {
	type Target = D3D12GraphicsCommandList3;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12GraphicsCommandList4 {
	const IID: &'static GUID = &GUID::from_u128(0x8754318ed3a9454198cf645b50dc4874u128);
}

impl Com for D3D12GraphicsCommandList4 {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12GraphicsCommandList4 {
	pub fn BeginRenderPass(&self, render_targets: Option<&[D3D12RenderPassRenderTargetDesc]>, depth_stencil: Option<&D3D12RenderPassDepthStencilDesc>, flags: D3D12RenderPassFlags) -> () {
		unsafe {
			let vt = self.as_param();
			let (render_targets_ptr_, render_targets_len_) = render_targets.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const D3D12RenderPassRenderTargetDesc, *const c_void, D3D12RenderPassFlags) -> ()
				= transmute(vt[68]);
			let _ret_ = f(vt, render_targets_len_ as u32, render_targets_ptr_, transmute(depth_stencil), flags);
		}
	}

	pub fn EndRenderPass(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[69]);
			let _ret_ = f(vt);
		}
	}

	pub fn InitializeMetaCommand(&self, meta_command: &D3D12MetaCommand, initialization_parameters_data: Option<&[u8]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (initialization_parameters_data_ptr_, initialization_parameters_data_len_) = initialization_parameters_data.deconstruct();
			let f: extern "system" fn(Param<Self>, VTable, *const u8, usize) -> ()
				= transmute(vt[70]);
			let _ret_ = f(vt, meta_command.vtable(), initialization_parameters_data_ptr_, initialization_parameters_data_len_ as usize);
		}
	}

	pub fn ExecuteMetaCommand(&self, meta_command: &D3D12MetaCommand, execution_parameters_data: Option<&[u8]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (execution_parameters_data_ptr_, execution_parameters_data_len_) = execution_parameters_data.deconstruct();
			let f: extern "system" fn(Param<Self>, VTable, *const u8, usize) -> ()
				= transmute(vt[71]);
			let _ret_ = f(vt, meta_command.vtable(), execution_parameters_data_ptr_, execution_parameters_data_len_ as usize);
		}
	}

	pub fn BuildRaytracingAccelerationStructure(&self, desc: &D3D12BuildRaytracingAccelerationStructureDesc, postbuild_info_descs: Option<&[D3D12RaytracingAccelerationStructurePostBuildInfoDesc]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (postbuild_info_descs_ptr_, postbuild_info_descs_len_) = postbuild_info_descs.deconstruct();
			let f: extern "system" fn(Param<Self>, &D3D12BuildRaytracingAccelerationStructureDesc, u32, *const D3D12RaytracingAccelerationStructurePostBuildInfoDesc) -> ()
				= transmute(vt[72]);
			let _ret_ = f(vt, desc, postbuild_info_descs_len_ as u32, postbuild_info_descs_ptr_);
		}
	}

	pub fn EmitRaytracingAccelerationStructurePostbuildInfo(&self, desc: &D3D12RaytracingAccelerationStructurePostBuildInfoDesc, source_acceleration_structure_data: &[u64]) -> () {
		unsafe {
			let vt = self.as_param();
			let (source_acceleration_structure_data_ptr_, source_acceleration_structure_data_len_) = source_acceleration_structure_data.deconstruct();
			let f: extern "system" fn(Param<Self>, &D3D12RaytracingAccelerationStructurePostBuildInfoDesc, u32, *const u64) -> ()
				= transmute(vt[73]);
			let _ret_ = f(vt, desc, source_acceleration_structure_data_len_ as u32, source_acceleration_structure_data_ptr_);
		}
	}

	pub fn CopyRaytracingAccelerationStructure(&self, dest_acceleration_structure_data: D3D12GpuVirtualAddress, source_acceleration_structure_data: D3D12GpuVirtualAddress, mode: D3D12RaytracingAccelerationStructureCopyMode) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12GpuVirtualAddress, D3D12GpuVirtualAddress, D3D12RaytracingAccelerationStructureCopyMode) -> ()
				= transmute(vt[74]);
			let _ret_ = f(vt, dest_acceleration_structure_data, source_acceleration_structure_data, mode);
		}
	}

	pub fn SetPipelineState1(&self, state_object: &D3D12StateObject) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> ()
				= transmute(vt[75]);
			let _ret_ = f(vt, state_object.vtable());
		}
	}

	pub fn DispatchRays(&self, desc: &D3D12DispatchRaysDesc) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[76]);
			let _ret_ = f(vt, transmute(desc));
		}
	}
}

pub trait ID3D12GraphicsCommandList4: ID3D12GraphicsCommandList3 {
	fn as_graphics_command_list4(&self) -> &D3D12GraphicsCommandList4;
	fn into_graphics_command_list4(self) -> D3D12GraphicsCommandList4;
}

impl ID3D12GraphicsCommandList4 for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list4(&self) -> &D3D12GraphicsCommandList4 { self }
	fn into_graphics_command_list4(self) -> D3D12GraphicsCommandList4 { self }
}
impl ID3D12GraphicsCommandList3 for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list3(&self) -> &D3D12GraphicsCommandList3 { &self.0.as_graphics_command_list3() }
	fn into_graphics_command_list3(self) -> D3D12GraphicsCommandList3 { self.0.into_graphics_command_list3() }
}

impl ID3D12GraphicsCommandList2 for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list2(&self) -> &D3D12GraphicsCommandList2 { &self.0.as_graphics_command_list2() }
	fn into_graphics_command_list2(self) -> D3D12GraphicsCommandList2 { self.0.into_graphics_command_list2() }
}

impl ID3D12GraphicsCommandList1 for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list1(&self) -> &D3D12GraphicsCommandList1 { &self.0.as_graphics_command_list1() }
	fn into_graphics_command_list1(self) -> D3D12GraphicsCommandList1 { self.0.into_graphics_command_list1() }
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList4 {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { &self.0.as_graphics_command_list() }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self.0.into_graphics_command_list() }
}

impl ID3D12CommandList for D3D12GraphicsCommandList4 {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.as_command_list() }
	fn into_command_list(self) -> D3D12CommandList { self.0.into_command_list() }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList4 {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12GraphicsCommandList4 {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12GraphicsCommandList4 {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12GraphicsCommandList4 {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12GraphicsCommandList3::from(v))
    }
}

