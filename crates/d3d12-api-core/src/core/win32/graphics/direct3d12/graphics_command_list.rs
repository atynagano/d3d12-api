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

use crate::core::win32::foundation::*;
use crate::core::win32::graphics::direct3d12::*;
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::direct3d::*;
#[repr(C)]
pub struct D3D12GraphicsCommandList(pub(crate) D3D12CommandList);

impl Guid for D3D12GraphicsCommandList {
	const IID: &'static GUID = &GUID::from_u128(0x5b160d0fac1b41858ba8b3ae42a5a455u128);
}

impl Clone for D3D12GraphicsCommandList {
	fn clone(&self) -> Self { D3D12GraphicsCommandList(self.0.clone()) }
}

pub trait ID3D12GraphicsCommandList: ID3D12CommandList {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList;
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList;

	fn Close(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn Reset(&self, allocator: &(impl ID3D12CommandAllocator + ?Sized), initial_state: Option<&D3D12PipelineState>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, allocator: VTable, initial_state: Option<VTable>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, allocator.vtable(), option_to_vtable(initial_state), );
		ret.ok()
	}

	fn ClearState(&self, pipeline_state: Option<&D3D12PipelineState>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, pipeline_state: Option<VTable>, ) -> ()
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, option_to_vtable(pipeline_state), );
	}

	fn DrawInstanced(&self, vertex_count_per_instance: u32, instance_count: u32, start_vertex_location: u32, start_instance_location: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, vertex_count_per_instance: u32, instance_count: u32, start_vertex_location: u32, start_instance_location: u32, ) -> ()
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, vertex_count_per_instance, instance_count, start_vertex_location, start_instance_location, );
	}

	fn DrawIndexedInstanced(&self, index_count_per_instance: u32, instance_count: u32, start_index_location: u32, base_vertex_location: i32, start_instance_location: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, index_count_per_instance: u32, instance_count: u32, start_index_location: u32, base_vertex_location: i32, start_instance_location: u32, ) -> ()
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, index_count_per_instance, instance_count, start_index_location, base_vertex_location, start_instance_location, );
	}

	fn Dispatch(&self, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32, ) -> ()
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, thread_group_count_x, thread_group_count_y, thread_group_count_z, );
	}

	fn CopyBufferRegion(&self, dst_buffer: &(impl ID3D12Resource + ?Sized), dst_offset: u64, src_buffer: &(impl ID3D12Resource + ?Sized), src_offset: u64, num_bytes: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dst_buffer: VTable, dst_offset: u64, src_buffer: VTable, src_offset: u64, num_bytes: u64, ) -> ()
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, dst_buffer.vtable(), dst_offset, src_buffer.vtable(), src_offset, num_bytes, );
	}

	fn CopyTextureRegion(&self, dst: &D3D12TextureCopyLocation, dst_x: u32, dst_y: u32, dst_z: u32, src: &D3D12TextureCopyLocation, src_box: Option<&D3D12Box>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dst: &D3D12TextureCopyLocation, dst_x: u32, dst_y: u32, dst_z: u32, src: &D3D12TextureCopyLocation, src_box: Option<&D3D12Box>, ) -> ()
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, dst, dst_x, dst_y, dst_z, src, src_box, );
	}

	fn CopyResource(&self, dst_resource: &(impl ID3D12Resource + ?Sized), src_resource: &(impl ID3D12Resource + ?Sized), ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dst_resource: VTable, src_resource: VTable, ) -> ()
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, dst_resource.vtable(), src_resource.vtable(), );
	}

	fn CopyTiles(&self, tiled_resource: &(impl ID3D12Resource + ?Sized), tile_region_start_coordinate: &D3D12TiledResourceCoordinate, tile_region_size: &D3D12TileRegionSize, buffer: &(impl ID3D12Resource + ?Sized), buffer_start_offset_in_bytes: u64, flags: D3D12TileCopyFlags, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, tiled_resource: VTable, tile_region_start_coordinate: &D3D12TiledResourceCoordinate, tile_region_size: &D3D12TileRegionSize, buffer: VTable, buffer_start_offset_in_bytes: u64, flags: D3D12TileCopyFlags, ) -> ()
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, tiled_resource.vtable(), tile_region_start_coordinate, tile_region_size, buffer.vtable(), buffer_start_offset_in_bytes, flags, );
	}

	fn ResolveSubresource(&self, dst_resource: &(impl ID3D12Resource + ?Sized), dst_subresource: u32, src_resource: &(impl ID3D12Resource + ?Sized), src_subresource: u32, format: DxgiFormat, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, dst_resource: VTable, dst_subresource: u32, src_resource: VTable, src_subresource: u32, format: DxgiFormat, ) -> ()
			= unsafe { transmute(vt[19]) };
		let ret = f(vt, dst_resource.vtable(), dst_subresource, src_resource.vtable(), src_subresource, format, );
	}

	fn IASetPrimitiveTopology(&self, primitive_topology: D3DPrimitiveTopology, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, primitive_topology: D3DPrimitiveTopology, ) -> ()
			= unsafe { transmute(vt[20]) };
		let ret = f(vt, primitive_topology, );
	}

	fn RSSetViewports(&self, viewports: &[D3D12Viewport], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_viewports: u32, viewports: *const D3D12Viewport, ) -> ()
			= unsafe { transmute(vt[21]) };
		let ret = f(vt, viewports.len() as u32, viewports.as_ptr_or_null(), );
	}

	fn RSSetScissorRects(&self, rects: &[Rect], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_rects: u32, rects: *const Rect, ) -> ()
			= unsafe { transmute(vt[22]) };
		let ret = f(vt, rects.len() as u32, rects.as_ptr_or_null(), );
	}

	fn OMSetBlendFactor(&self, blend_factor: Option<&[f32; 4]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, blend_factor: *const f32, ) -> ()
			= unsafe { transmute(vt[23]) };
		let ret = f(vt, blend_factor.as_ptr_or_null(), );
	}

	fn OMSetStencilRef(&self, stencil_ref: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, stencil_ref: u32, ) -> ()
			= unsafe { transmute(vt[24]) };
		let ret = f(vt, stencil_ref, );
	}

	fn SetPipelineState(&self, pipeline_state: &(impl ID3D12PipelineState + ?Sized), ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, pipeline_state: VTable, ) -> ()
			= unsafe { transmute(vt[25]) };
		let ret = f(vt, pipeline_state.vtable(), );
	}

	fn ResourceBarrier(&self, barriers: &[D3D12ResourceBarrier], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_barriers: u32, barriers: *const D3D12ResourceBarrier, ) -> ()
			= unsafe { transmute(vt[26]) };
		let ret = f(vt, barriers.len() as u32, barriers.as_ptr_or_null(), );
	}

	fn ExecuteBundle(&self, command_list: &(impl ID3D12GraphicsCommandList + ?Sized), ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, command_list: VTable, ) -> ()
			= unsafe { transmute(vt[27]) };
		let ret = f(vt, command_list.vtable(), );
	}

	fn SetDescriptorHeaps(&self, descriptor_heaps: &[Param<D3D12DescriptorHeap>], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_descriptor_heaps: u32, descriptor_heaps: *const Param<D3D12DescriptorHeap>, ) -> ()
			= unsafe { transmute(vt[28]) };
		let ret = f(vt, descriptor_heaps.len() as u32, descriptor_heaps.as_ptr_or_null(), );
	}

	fn SetComputeRootSignature(&self, root_signature: Option<&D3D12RootSignature>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_signature: Option<VTable>, ) -> ()
			= unsafe { transmute(vt[29]) };
		let ret = f(vt, option_to_vtable(root_signature), );
	}

	fn SetGraphicsRootSignature(&self, root_signature: Option<&D3D12RootSignature>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_signature: Option<VTable>, ) -> ()
			= unsafe { transmute(vt[30]) };
		let ret = f(vt, option_to_vtable(root_signature), );
	}

	fn SetComputeRootDescriptorTable(&self, root_parameter_index: u32, base_descriptor: D3D12GpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, base_descriptor: D3D12GpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[31]) };
		let ret = f(vt, root_parameter_index, base_descriptor, );
	}

	fn SetGraphicsRootDescriptorTable(&self, root_parameter_index: u32, base_descriptor: D3D12GpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, base_descriptor: D3D12GpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[32]) };
		let ret = f(vt, root_parameter_index, base_descriptor, );
	}

	fn SetComputeRoot32BitConstant(&self, root_parameter_index: u32, src_data: u32, dest_offset_in32bit_values: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, src_data: u32, dest_offset_in32bit_values: u32, ) -> ()
			= unsafe { transmute(vt[33]) };
		let ret = f(vt, root_parameter_index, src_data, dest_offset_in32bit_values, );
	}

	fn SetGraphicsRoot32BitConstant(&self, root_parameter_index: u32, src_data: u32, dest_offset_in32bit_values: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, src_data: u32, dest_offset_in32bit_values: u32, ) -> ()
			= unsafe { transmute(vt[34]) };
		let ret = f(vt, root_parameter_index, src_data, dest_offset_in32bit_values, );
	}

	fn SetComputeRoot32BitConstants(&self, root_parameter_index: u32, num32bit_values_to_set: u32, src_data: *const c_void, dest_offset_in32bit_values: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, num32bit_values_to_set: u32, src_data: *const c_void, dest_offset_in32bit_values: u32, ) -> ()
			= unsafe { transmute(vt[35]) };
		let ret = f(vt, root_parameter_index, num32bit_values_to_set, src_data, dest_offset_in32bit_values, );
	}

	fn SetComputeRootConstantBufferView(&self, root_parameter_index: u32, buffer_location: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, buffer_location: u64, ) -> ()
			= unsafe { transmute(vt[37]) };
		let ret = f(vt, root_parameter_index, buffer_location, );
	}

	fn SetGraphicsRootConstantBufferView(&self, root_parameter_index: u32, buffer_location: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, buffer_location: u64, ) -> ()
			= unsafe { transmute(vt[38]) };
		let ret = f(vt, root_parameter_index, buffer_location, );
	}

	fn SetComputeRootShaderResourceView(&self, root_parameter_index: u32, buffer_location: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, buffer_location: u64, ) -> ()
			= unsafe { transmute(vt[39]) };
		let ret = f(vt, root_parameter_index, buffer_location, );
	}

	fn SetGraphicsRootShaderResourceView(&self, root_parameter_index: u32, buffer_location: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, buffer_location: u64, ) -> ()
			= unsafe { transmute(vt[40]) };
		let ret = f(vt, root_parameter_index, buffer_location, );
	}

	fn SetComputeRootUnorderedAccessView(&self, root_parameter_index: u32, buffer_location: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, buffer_location: u64, ) -> ()
			= unsafe { transmute(vt[41]) };
		let ret = f(vt, root_parameter_index, buffer_location, );
	}

	fn SetGraphicsRootUnorderedAccessView(&self, root_parameter_index: u32, buffer_location: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, root_parameter_index: u32, buffer_location: u64, ) -> ()
			= unsafe { transmute(vt[42]) };
		let ret = f(vt, root_parameter_index, buffer_location, );
	}

	fn IASetIndexBuffer(&self, view: Option<&D3D12IndexBufferView>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, view: Option<&D3D12IndexBufferView>, ) -> ()
			= unsafe { transmute(vt[43]) };
		let ret = f(vt, view, );
	}

	fn IASetVertexBuffers(&self, start_slot: u32, views: Option<&[D3D12VertexBufferView]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, start_slot: u32, num_views: u32, views: *const D3D12VertexBufferView, ) -> ()
			= unsafe { transmute(vt[44]) };
		let ret = f(vt, start_slot, views.len() as u32, views.as_ptr_or_null(), );
	}

	fn SOSetTargets(&self, start_slot: u32, views: Option<&[D3D12StreamOutputBufferView]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, start_slot: u32, num_views: u32, views: *const D3D12StreamOutputBufferView, ) -> ()
			= unsafe { transmute(vt[45]) };
		let ret = f(vt, start_slot, views.len() as u32, views.as_ptr_or_null(), );
	}

	fn OMSetRenderTargets(&self, render_target_descriptors: Option<&[D3D12CpuDescriptorHandle]>, rts_single_handle_to_descriptor_range: bool, depth_stencil_descriptor: Option<&[D3D12CpuDescriptorHandle]>) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, u32, *const D3D12CpuDescriptorHandle, Bool, *const D3D12CpuDescriptorHandle) -> ()
			= unsafe { transmute(vt[46]) };
		let (n, rt, ds) = match (render_target_descriptors, depth_stencil_descriptor) {
			(Some(rt), Some(ds)) => {
				assert_eq!(rt.len(), ds.len());
				(rt.len(), rt.as_ptr_or_null(), ds.as_ptr_or_null())
			}
			(Some(rt), None) => (rt.len(), rt.as_ptr_or_null(), null()),
			(None, Some(ds)) => (ds.len(), null(), ds.as_ptr_or_null()),
			(None, None) => (0, null(), null()),
		};
		f(vt, n as u32, rt, rts_single_handle_to_descriptor_range.to_bool(), ds);
	}

	fn ClearDepthStencilView(&self, depth_stencil_view: D3D12CpuDescriptorHandle, clear_flags: D3D12ClearFlags, depth: f32, stencil: u8, rects: &[Rect], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, depth_stencil_view: D3D12CpuDescriptorHandle, clear_flags: D3D12ClearFlags, depth: f32, stencil: u8, num_rects: u32, rects: *const Rect, ) -> ()
			= unsafe { transmute(vt[47]) };
		let ret = f(vt, depth_stencil_view, clear_flags, depth, stencil, rects.len() as u32, rects.as_ptr_or_null(), );
	}

	fn ClearRenderTargetView(&self, render_target_view: D3D12CpuDescriptorHandle, color_rgba: [f32; 4], rects: Option<&[Rect]>) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, render_target_view: D3D12CpuDescriptorHandle, color_rgba: &f32, num_rects: u32, rects: *const Rect) -> ()
			= unsafe { transmute(vt[48]) };
		let ret = f(vt, render_target_view, &color_rgba[0], rects.len() as u32, rects.as_ptr_or_null());
	}

	fn ClearUnorderedAccessViewUint(&self, view_gpu_handle_in_current_heap: D3D12GpuDescriptorHandle, view_cpu_handle: D3D12CpuDescriptorHandle, resource: &(impl ID3D12Resource + ?Sized), values: &u32, rects: &[Rect], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, view_gpu_handle_in_current_heap: D3D12GpuDescriptorHandle, view_cpu_handle: D3D12CpuDescriptorHandle, resource: VTable, values: &u32, num_rects: u32, rects: *const Rect, ) -> ()
			= unsafe { transmute(vt[49]) };
		let ret = f(vt, view_gpu_handle_in_current_heap, view_cpu_handle, resource.vtable(), values, rects.len() as u32, rects.as_ptr_or_null(), );
	}

	fn ClearUnorderedAccessViewFloat(&self, view_gpu_handle_in_current_heap: D3D12GpuDescriptorHandle, view_cpu_handle: D3D12CpuDescriptorHandle, resource: &(impl ID3D12Resource + ?Sized), values: &f32, rects: &[Rect], ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, view_gpu_handle_in_current_heap: D3D12GpuDescriptorHandle, view_cpu_handle: D3D12CpuDescriptorHandle, resource: VTable, values: &f32, num_rects: u32, rects: *const Rect, ) -> ()
			= unsafe { transmute(vt[50]) };
		let ret = f(vt, view_gpu_handle_in_current_heap, view_cpu_handle, resource.vtable(), values, rects.len() as u32, rects.as_ptr_or_null(), );
	}

	fn DiscardResource(&self, resource: &(impl ID3D12Resource + ?Sized), region: Option<&D3D12DiscardRegion>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resource: VTable, region: Option<&D3D12DiscardRegion>, ) -> ()
			= unsafe { transmute(vt[51]) };
		let ret = f(vt, resource.vtable(), region, );
	}

	fn BeginQuery(&self, query_heap: &(impl ID3D12QueryHeap + ?Sized), ty: D3D12QueryType, index: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, query_heap: VTable, ty: D3D12QueryType, index: u32, ) -> ()
			= unsafe { transmute(vt[52]) };
		let ret = f(vt, query_heap.vtable(), ty, index, );
	}

	fn EndQuery(&self, query_heap: &(impl ID3D12QueryHeap + ?Sized), ty: D3D12QueryType, index: u32, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, query_heap: VTable, ty: D3D12QueryType, index: u32, ) -> ()
			= unsafe { transmute(vt[53]) };
		let ret = f(vt, query_heap.vtable(), ty, index, );
	}

	fn ResolveQueryData(&self, query_heap: &(impl ID3D12QueryHeap + ?Sized), ty: D3D12QueryType, start_index: u32, num_queries: u32, destination_buffer: &(impl ID3D12Resource + ?Sized), aligned_destination_buffer_offset: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, query_heap: VTable, ty: D3D12QueryType, start_index: u32, num_queries: u32, destination_buffer: VTable, aligned_destination_buffer_offset: u64, ) -> ()
			= unsafe { transmute(vt[54]) };
		let ret = f(vt, query_heap.vtable(), ty, start_index, num_queries, destination_buffer.vtable(), aligned_destination_buffer_offset, );
	}

	fn SetPredication(&self, buffer: Option<&D3D12Resource>, aligned_buffer_offset: u64, operation: D3D12PredicationOp, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, buffer: Option<VTable>, aligned_buffer_offset: u64, operation: D3D12PredicationOp, ) -> ()
			= unsafe { transmute(vt[55]) };
		let ret = f(vt, option_to_vtable(buffer), aligned_buffer_offset, operation, );
	}

	fn SetMarker(&self, metadata: u32, data: Option<&[u8]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, metadata: u32, data: *const u8, size: u32, ) -> ()
			= unsafe { transmute(vt[56]) };
		let ret = f(vt, metadata, data.as_ptr_or_null(), data.len() as u32, );
	}

	fn BeginEvent(&self, metadata: u32, data: Option<&[u8]>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, metadata: u32, data: *const u8, size: u32, ) -> ()
			= unsafe { transmute(vt[57]) };
		let ret = f(vt, metadata, data.as_ptr_or_null(), data.len() as u32, );
	}

	fn EndEvent(&self, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> ()
			= unsafe { transmute(vt[58]) };
		let ret = f(vt, );
	}

	fn ExecuteIndirect(&self, command_signature: &(impl ID3D12CommandSignature + ?Sized), max_command_count: u32, argument_buffer: &(impl ID3D12Resource + ?Sized), argument_buffer_offset: u64, count_buffer: Option<&D3D12Resource>, count_buffer_offset: u64, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, command_signature: VTable, max_command_count: u32, argument_buffer: VTable, argument_buffer_offset: u64, count_buffer: Option<VTable>, count_buffer_offset: u64, ) -> ()
			= unsafe { transmute(vt[59]) };
		let ret = f(vt, command_signature.vtable(), max_command_count, argument_buffer.vtable(), argument_buffer_offset, option_to_vtable(count_buffer), count_buffer_offset, );
	}
}

impl ID3D12GraphicsCommandList for D3D12GraphicsCommandList {
	fn as_graphics_command_list(&self) -> &D3D12GraphicsCommandList { self }
	fn into_graphics_command_list(self) -> D3D12GraphicsCommandList { self }
}

impl ID3D12CommandList for D3D12GraphicsCommandList {
	fn as_command_list(&self) -> &D3D12CommandList { &self.0 }
	fn into_command_list(self) -> D3D12CommandList { self.0 }
}

impl ID3D12DeviceChild for D3D12GraphicsCommandList {
	fn as_device_child(&self) -> &D3D12DeviceChild { &self.0.0 }
	fn into_device_child(self) -> D3D12DeviceChild { self.0.0 }
}

impl ID3D12Object for D3D12GraphicsCommandList {
	fn as_object(&self) -> &D3D12Object { &self.0.0.0 }
	fn into_object(self) -> D3D12Object { self.0.0.0 }
}

impl From<Unknown> for D3D12GraphicsCommandList {
    fn from(v: Unknown) -> Self {
        Self(D3D12CommandList::from(v))
    }
}

impl IUnknown for D3D12GraphicsCommandList {
	fn as_unknown(&self) -> &Unknown { &self.0.0.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0.0.0 }
}

