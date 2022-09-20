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
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::direct3d::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12GraphicsCommandList(pub(crate) D3D12CommandList);

impl Deref for D3D12GraphicsCommandList {
	type Target = D3D12CommandList;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12GraphicsCommandList {
	const IID: &'static GUID = &GUID::from_u128(0x5b160d0fac1b41858ba8b3ae42a5a455u128);
}

impl Com for D3D12GraphicsCommandList {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12GraphicsCommandList {
	pub fn Close(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn Reset(&self, allocator: &D3D12CommandAllocator, initial_state: Option<&D3D12PipelineState>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, allocator.vtable(), option_to_vtable(initial_state));
			_ret_.ok()
		}
	}

	pub fn ClearState(&self, pipeline_state: Option<&D3D12PipelineState>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[11]);
			let _ret_ = f(vt, option_to_vtable(pipeline_state));
		}
	}

	pub fn DrawInstanced(&self, vertex_count_per_instance: u32, instance_count: u32, start_vertex_location: u32, start_instance_location: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, u32, u32) -> ()
				= transmute(vt[12]);
			let _ret_ = f(vt, vertex_count_per_instance, instance_count, start_vertex_location, start_instance_location);
		}
	}

	pub fn DrawIndexedInstanced(&self, index_count_per_instance: u32, instance_count: u32, start_index_location: u32, base_vertex_location: i32, start_instance_location: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, u32, i32, u32) -> ()
				= transmute(vt[13]);
			let _ret_ = f(vt, index_count_per_instance, instance_count, start_index_location, base_vertex_location, start_instance_location);
		}
	}

	pub fn Dispatch(&self, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, u32) -> ()
				= transmute(vt[14]);
			let _ret_ = f(vt, thread_group_count_x, thread_group_count_y, thread_group_count_z);
		}
	}

	pub fn CopyBufferRegion(&self, dst_buffer: &D3D12Resource, dst_offset: u64, src_buffer: &D3D12Resource, src_offset: u64, num_bytes: u64) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u64, VTable, u64, u64) -> ()
				= transmute(vt[15]);
			let _ret_ = f(vt, dst_buffer.vtable(), dst_offset, src_buffer.vtable(), src_offset, num_bytes);
		}
	}

	pub fn CopyTextureRegion(&self, dst: &D3D12TextureCopyLocation, dst_x: u32, dst_y: u32, dst_z: u32, src: &D3D12TextureCopyLocation, src_box: Option<&D3D12Box>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D3D12TextureCopyLocation, u32, u32, u32, &D3D12TextureCopyLocation, *const c_void) -> ()
				= transmute(vt[16]);
			let _ret_ = f(vt, dst, dst_x, dst_y, dst_z, src, transmute(src_box));
		}
	}

	pub fn CopyResource(&self, dst_resource: &D3D12Resource, src_resource: &D3D12Resource) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, VTable) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, dst_resource.vtable(), src_resource.vtable());
		}
	}

	pub fn CopyTiles(&self, tiled_resource: &D3D12Resource, tile_region_start_coordinate: &D3D12TiledResourceCoordinate, tile_region_size: &D3D12TileRegionSize, buffer: &D3D12Resource, buffer_start_offset_in_bytes: u64, flags: D3D12TileCopyFlags) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, &D3D12TiledResourceCoordinate, &D3D12TileRegionSize, VTable, u64, D3D12TileCopyFlags) -> ()
				= transmute(vt[18]);
			let _ret_ = f(vt, tiled_resource.vtable(), tile_region_start_coordinate, tile_region_size, buffer.vtable(), buffer_start_offset_in_bytes, flags);
		}
	}

	pub fn ResolveSubresource(&self, dst_resource: &D3D12Resource, dst_subresource: u32, src_resource: &D3D12Resource, src_subresource: u32, format: DxgiFormat) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, VTable, u32, DxgiFormat) -> ()
				= transmute(vt[19]);
			let _ret_ = f(vt, dst_resource.vtable(), dst_subresource, src_resource.vtable(), src_subresource, format);
		}
	}

	pub fn IASetPrimitiveTopology(&self, primitive_topology: D3DPrimitiveTopology) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3DPrimitiveTopology) -> ()
				= transmute(vt[20]);
			let _ret_ = f(vt, primitive_topology);
		}
	}

	pub fn RSSetViewports(&self, viewports: &[D3D12Viewport]) -> () {
		unsafe {
			let vt = self.as_param();
			let (viewports_ptr_, viewports_len_) = viewports.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const D3D12Viewport) -> ()
				= transmute(vt[21]);
			let _ret_ = f(vt, viewports_len_ as u32, viewports_ptr_);
		}
	}

	pub fn RSSetScissorRects(&self, rects: &[Rect]) -> () {
		unsafe {
			let vt = self.as_param();
			let (rects_ptr_, rects_len_) = rects.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Rect) -> ()
				= transmute(vt[22]);
			let _ret_ = f(vt, rects_len_ as u32, rects_ptr_);
		}
	}

	pub fn OMSetBlendFactor(&self, blend_factor: Option<&[f32; 4]>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const f32) -> ()
				= transmute(vt[23]);
			let _ret_ = f(vt, blend_factor.as_ptr_or_null());
		}
	}

	pub fn OMSetStencilRef(&self, stencil_ref: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32) -> ()
				= transmute(vt[24]);
			let _ret_ = f(vt, stencil_ref);
		}
	}

	pub fn SetPipelineState(&self, pipeline_state: &D3D12PipelineState) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> ()
				= transmute(vt[25]);
			let _ret_ = f(vt, pipeline_state.vtable());
		}
	}

	pub fn ResourceBarrier(&self, barriers: &[D3D12ResourceBarrier]) -> () {
		unsafe {
			let vt = self.as_param();
			let (barriers_ptr_, barriers_len_) = barriers.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const D3D12ResourceBarrier) -> ()
				= transmute(vt[26]);
			let _ret_ = f(vt, barriers_len_ as u32, barriers_ptr_);
		}
	}

	pub fn ExecuteBundle(&self, command_list: &D3D12GraphicsCommandList) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable) -> ()
				= transmute(vt[27]);
			let _ret_ = f(vt, command_list.vtable());
		}
	}

	pub fn SetDescriptorHeaps(&self, descriptor_heaps: &[Param<D3D12DescriptorHeap>]) -> () {
		unsafe {
			let vt = self.as_param();
			let (descriptor_heaps_ptr_, descriptor_heaps_len_) = descriptor_heaps.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Param<D3D12DescriptorHeap>) -> ()
				= transmute(vt[28]);
			let _ret_ = f(vt, descriptor_heaps_len_ as u32, descriptor_heaps_ptr_);
		}
	}

	pub fn SetComputeRootSignature(&self, root_signature: Option<&D3D12RootSignature>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[29]);
			let _ret_ = f(vt, option_to_vtable(root_signature));
		}
	}

	pub fn SetGraphicsRootSignature(&self, root_signature: Option<&D3D12RootSignature>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[30]);
			let _ret_ = f(vt, option_to_vtable(root_signature));
		}
	}

	pub fn SetComputeRootDescriptorTable(&self, root_parameter_index: u32, base_descriptor: D3D12GpuDescriptorHandle) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12GpuDescriptorHandle) -> ()
				= transmute(vt[31]);
			let _ret_ = f(vt, root_parameter_index, base_descriptor);
		}
	}

	pub fn SetGraphicsRootDescriptorTable(&self, root_parameter_index: u32, base_descriptor: D3D12GpuDescriptorHandle) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12GpuDescriptorHandle) -> ()
				= transmute(vt[32]);
			let _ret_ = f(vt, root_parameter_index, base_descriptor);
		}
	}

	pub fn SetComputeRoot32BitConstant(&self, root_parameter_index: u32, src_data: u32, dest_offset_in32bit_values: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, u32) -> ()
				= transmute(vt[33]);
			let _ret_ = f(vt, root_parameter_index, src_data, dest_offset_in32bit_values);
		}
	}

	pub fn SetGraphicsRoot32BitConstant(&self, root_parameter_index: u32, src_data: u32, dest_offset_in32bit_values: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, u32) -> ()
				= transmute(vt[34]);
			let _ret_ = f(vt, root_parameter_index, src_data, dest_offset_in32bit_values);
		}
	}

	pub fn SetComputeRoot32BitConstants(&self, root_parameter_index: u32, num32bit_values_to_set: u32, src_data: *const impl Sized, dest_offset_in32bit_values: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, u32, *const c_void, u32) -> ()
				= transmute(vt[35]);
			let _ret_ = f(vt, root_parameter_index, num32bit_values_to_set, src_data as _, dest_offset_in32bit_values);
		}
	}

	pub fn SetComputeRootConstantBufferView(&self, root_parameter_index: u32, buffer_location: D3D12GpuVirtualAddress) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12GpuVirtualAddress) -> ()
				= transmute(vt[37]);
			let _ret_ = f(vt, root_parameter_index, buffer_location);
		}
	}

	pub fn SetGraphicsRootConstantBufferView(&self, root_parameter_index: u32, buffer_location: D3D12GpuVirtualAddress) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12GpuVirtualAddress) -> ()
				= transmute(vt[38]);
			let _ret_ = f(vt, root_parameter_index, buffer_location);
		}
	}

	pub fn SetComputeRootShaderResourceView(&self, root_parameter_index: u32, buffer_location: D3D12GpuVirtualAddress) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12GpuVirtualAddress) -> ()
				= transmute(vt[39]);
			let _ret_ = f(vt, root_parameter_index, buffer_location);
		}
	}

	pub fn SetGraphicsRootShaderResourceView(&self, root_parameter_index: u32, buffer_location: D3D12GpuVirtualAddress) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12GpuVirtualAddress) -> ()
				= transmute(vt[40]);
			let _ret_ = f(vt, root_parameter_index, buffer_location);
		}
	}

	pub fn SetComputeRootUnorderedAccessView(&self, root_parameter_index: u32, buffer_location: D3D12GpuVirtualAddress) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12GpuVirtualAddress) -> ()
				= transmute(vt[41]);
			let _ret_ = f(vt, root_parameter_index, buffer_location);
		}
	}

	pub fn SetGraphicsRootUnorderedAccessView(&self, root_parameter_index: u32, buffer_location: D3D12GpuVirtualAddress) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12GpuVirtualAddress) -> ()
				= transmute(vt[42]);
			let _ret_ = f(vt, root_parameter_index, buffer_location);
		}
	}

	pub fn IASetIndexBuffer(&self, view: Option<&D3D12IndexBufferView>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void) -> ()
				= transmute(vt[43]);
			let _ret_ = f(vt, transmute(view));
		}
	}

	pub fn IASetVertexBuffers(&self, start_slot: u32, views: Option<&[D3D12VertexBufferView]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (views_ptr_, views_len_) = views.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, u32, *const D3D12VertexBufferView) -> ()
				= transmute(vt[44]);
			let _ret_ = f(vt, start_slot, views_len_ as u32, views_ptr_);
		}
	}

	pub fn SOSetTargets(&self, start_slot: u32, views: Option<&[D3D12StreamOutputBufferView]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (views_ptr_, views_len_) = views.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, u32, *const D3D12StreamOutputBufferView) -> ()
				= transmute(vt[45]);
			let _ret_ = f(vt, start_slot, views_len_ as u32, views_ptr_);
		}
	}

	pub fn om_set_render_target(&self, render_target_descriptor: Option<&D3D12CpuDescriptorHandle>, depth_stencil_descriptor: Option<&D3D12CpuDescriptorHandle>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, *const D3D12CpuDescriptorHandle, Bool, *const D3D12CpuDescriptorHandle) -> ()
				= transmute(vt[46]);
			f(vt, if render_target_descriptor.is_some() { 1 } else { 0 }, transmute(render_target_descriptor), Bool::FALSE, transmute(depth_stencil_descriptor));
		}
	}

	pub fn ClearDepthStencilView(&self, depth_stencil_view: D3D12CpuDescriptorHandle, clear_flags: D3D12ClearFlags, depth: f32, stencil: u8, rects: &[Rect]) -> () {
		unsafe {
			let vt = self.as_param();
			let (rects_ptr_, rects_len_) = rects.deconstruct();
			let f: extern "system" fn(Param<Self>, D3D12CpuDescriptorHandle, D3D12ClearFlags, f32, u8, u32, *const Rect) -> ()
				= transmute(vt[47]);
			let _ret_ = f(vt, depth_stencil_view, clear_flags, depth, stencil, rects_len_ as u32, rects_ptr_);
		}
	}

	pub fn ClearRenderTargetView(&self, render_target_view: D3D12CpuDescriptorHandle, color_rgba: &[f32; 4], rects: Option<&[Rect]>) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, render_target_view: D3D12CpuDescriptorHandle, color_rgba: &[f32; 4], num_rects: u32, rects: *const Rect) -> ()
			= unsafe { transmute(vt[48]) };
		let ret = f(vt, render_target_view, &color_rgba, rects.len() as u32, rects.as_ptr_or_null());
	}

	pub fn ClearUnorderedAccessViewUint(&self, view_gpu_handle_in_current_heap: D3D12GpuDescriptorHandle, view_cpu_handle: D3D12CpuDescriptorHandle, resource: &D3D12Resource, values: &u32, rects: &[Rect]) -> () {
		unsafe {
			let vt = self.as_param();
			let (rects_ptr_, rects_len_) = rects.deconstruct();
			let f: extern "system" fn(Param<Self>, D3D12GpuDescriptorHandle, D3D12CpuDescriptorHandle, VTable, &u32, u32, *const Rect) -> ()
				= transmute(vt[49]);
			let _ret_ = f(vt, view_gpu_handle_in_current_heap, view_cpu_handle, resource.vtable(), values, rects_len_ as u32, rects_ptr_);
		}
	}

	pub fn ClearUnorderedAccessViewFloat(&self, view_gpu_handle_in_current_heap: D3D12GpuDescriptorHandle, view_cpu_handle: D3D12CpuDescriptorHandle, resource: &D3D12Resource, values: &f32, rects: &[Rect]) -> () {
		unsafe {
			let vt = self.as_param();
			let (rects_ptr_, rects_len_) = rects.deconstruct();
			let f: extern "system" fn(Param<Self>, D3D12GpuDescriptorHandle, D3D12CpuDescriptorHandle, VTable, &f32, u32, *const Rect) -> ()
				= transmute(vt[50]);
			let _ret_ = f(vt, view_gpu_handle_in_current_heap, view_cpu_handle, resource.vtable(), values, rects_len_ as u32, rects_ptr_);
		}
	}

	pub fn DiscardResource(&self, resource: &D3D12Resource, region: Option<&D3D12DiscardRegion>) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, *const c_void) -> ()
				= transmute(vt[51]);
			let _ret_ = f(vt, resource.vtable(), transmute(region));
		}
	}

	pub fn BeginQuery(&self, query_heap: &D3D12QueryHeap, r#type: D3D12QueryType, index: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, D3D12QueryType, u32) -> ()
				= transmute(vt[52]);
			let _ret_ = f(vt, query_heap.vtable(), r#type, index);
		}
	}

	pub fn EndQuery(&self, query_heap: &D3D12QueryHeap, r#type: D3D12QueryType, index: u32) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, D3D12QueryType, u32) -> ()
				= transmute(vt[53]);
			let _ret_ = f(vt, query_heap.vtable(), r#type, index);
		}
	}

	pub fn ResolveQueryData(&self, query_heap: &D3D12QueryHeap, r#type: D3D12QueryType, start_index: u32, num_queries: u32, destination_buffer: &D3D12Resource, aligned_destination_buffer_offset: u64) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, D3D12QueryType, u32, u32, VTable, u64) -> ()
				= transmute(vt[54]);
			let _ret_ = f(vt, query_heap.vtable(), r#type, start_index, num_queries, destination_buffer.vtable(), aligned_destination_buffer_offset);
		}
	}

	pub fn SetPredication(&self, buffer: Option<&D3D12Resource>, aligned_buffer_offset: u64, operation: D3D12PredicationOp) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, u64, D3D12PredicationOp) -> ()
				= transmute(vt[55]);
			let _ret_ = f(vt, option_to_vtable(buffer), aligned_buffer_offset, operation);
		}
	}

	pub fn SetMarker(&self, metadata: u32, data: Option<&[u8]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const u8, u32) -> ()
				= transmute(vt[56]);
			let _ret_ = f(vt, metadata, data_ptr_, data_len_ as u32);
		}
	}

	pub fn BeginEvent(&self, metadata: u32, data: Option<&[u8]>) -> () {
		unsafe {
			let vt = self.as_param();
			let (data_ptr_, data_len_) = data.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const u8, u32) -> ()
				= transmute(vt[57]);
			let _ret_ = f(vt, metadata, data_ptr_, data_len_ as u32);
		}
	}

	pub fn EndEvent(&self) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> ()
				= transmute(vt[58]);
			let _ret_ = f(vt);
		}
	}

	pub fn ExecuteIndirect(&self, command_signature: &D3D12CommandSignature, max_command_count: u32, argument_buffer: &D3D12Resource, argument_buffer_offset: u64, count_buffer: Option<&D3D12Resource>, count_buffer_offset: u64) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, VTable, u32, VTable, u64, *const c_void, u64) -> ()
				= transmute(vt[59]);
			let _ret_ = f(vt, command_signature.vtable(), max_command_count, argument_buffer.vtable(), argument_buffer_offset, option_to_vtable(count_buffer), count_buffer_offset);
		}
	}
}

pub trait ID3D12GraphicsCommandList: ID3D12CommandList {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList;
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList;
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { self }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self }
}
impl ID3D12CommandList for D3D12GraphicsCommandList {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0.as_command_list() }
	fn into_command_list(self) -> D3D12CommandList { self.0.into_command_list() }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.as_device_child() }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.into_device_child() }
}

impl ID3D12Object for D3D12GraphicsCommandList {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12GraphicsCommandList {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12GraphicsCommandList {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12CommandList::from(v))
    }
}

