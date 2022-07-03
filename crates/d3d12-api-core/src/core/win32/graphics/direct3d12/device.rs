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
use crate::core::win32::security::*;
#[repr(C)]
pub struct D3D12Device(pub(crate) D3D12Object);

impl Guid for D3D12Device {
	const IID: &'static GUID = &GUID::from_u128(0x189819f11db64b57be541821339b85f7u128);
}

impl Clone for D3D12Device {
	fn clone(&self) -> Self { D3D12Device(self.0.clone()) }
}

pub trait ID3D12Device: ID3D12Object {
	fn as_device(&self) -> &D3D12Device;
	fn into_device(self) -> D3D12Device;

	fn GetNodeCount(&self, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> u32
			= unsafe { transmute(vt[7]) };
		let ret = f(vt, );
		return (ret);
	}

	fn CreateCommandQueue<T: IUnknown>(&self, desc: &D3D12CommandQueueDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _command_queue: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12CommandQueueDesc, riid: &GUID, _command_queue: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[8]) };
		let ret = f(vt, desc, T::IID, &mut _command_queue, );
		if ret.is_ok() {
			if let (Some(_command_queue)) = (_command_queue) {
				return Ok((T::from(_command_queue)));
			}
		}
		Err(ret)
	}

	fn CreateCommandAllocator<T: IUnknown>(&self, ty: D3D12CommandListType, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _command_allocator: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, ty: D3D12CommandListType, riid: &GUID, _command_allocator: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[9]) };
		let ret = f(vt, ty, T::IID, &mut _command_allocator, );
		if ret.is_ok() {
			if let (Some(_command_allocator)) = (_command_allocator) {
				return Ok((T::from(_command_allocator)));
			}
		}
		Err(ret)
	}

	fn CreateGraphicsPipelineState<T: IUnknown>(&self, desc: &D3D12GraphicsPipelineStateDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _pipeline_state: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12GraphicsPipelineStateDesc, riid: &GUID, _pipeline_state: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[10]) };
		let ret = f(vt, desc, T::IID, &mut _pipeline_state, );
		if ret.is_ok() {
			if let (Some(_pipeline_state)) = (_pipeline_state) {
				return Ok((T::from(_pipeline_state)));
			}
		}
		Err(ret)
	}

	fn CreateComputePipelineState<T: IUnknown>(&self, desc: &D3D12ComputePipelineStateDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _pipeline_state: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12ComputePipelineStateDesc, riid: &GUID, _pipeline_state: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[11]) };
		let ret = f(vt, desc, T::IID, &mut _pipeline_state, );
		if ret.is_ok() {
			if let (Some(_pipeline_state)) = (_pipeline_state) {
				return Ok((T::from(_pipeline_state)));
			}
		}
		Err(ret)
	}

	fn CreateCommandList<T: IUnknown>(&self, node_mask: u32, ty: D3D12CommandListType, command_allocator: &(impl ID3D12CommandAllocator + ?Sized), initial_state: Option<&D3D12PipelineState>, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _command_list: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, node_mask: u32, ty: D3D12CommandListType, command_allocator: VTable, initial_state: Option<VTable>, riid: &GUID, _command_list: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[12]) };
		let ret = f(vt, node_mask, ty, command_allocator.vtable(), option_to_vtable(initial_state), T::IID, &mut _command_list, );
		if ret.is_ok() {
			if let (Some(_command_list)) = (_command_list) {
				return Ok((T::from(_command_list)));
			}
		}
		Err(ret)
	}

	fn CheckFeatureSupport<U>(&self, feature: D3D12Feature, feature_support_data: &mut U) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, D3D12Feature, *mut c_void, u32) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, feature, unsafe { transmute(feature_support_data as *const U) }, size_of_val(feature_support_data) as u32);
		ret.ok()
	}

	fn CreateDescriptorHeap<T: IUnknown>(&self, descriptor_heap_desc: &D3D12DescriptorHeapDesc, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _heap: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, descriptor_heap_desc: &D3D12DescriptorHeapDesc, riid: &GUID, _heap: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[14]) };
		let ret = f(vt, descriptor_heap_desc, T::IID, &mut _heap, );
		if ret.is_ok() {
			if let (Some(_heap)) = (_heap) {
				return Ok((T::from(_heap)));
			}
		}
		Err(ret)
	}

	fn GetDescriptorHandleIncrementSize(&self, descriptor_heap_type: D3D12DescriptorHeapType, ) -> (u32) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, descriptor_heap_type: D3D12DescriptorHeapType, ) -> u32
			= unsafe { transmute(vt[15]) };
		let ret = f(vt, descriptor_heap_type, );
		return (ret);
	}

	fn CreateRootSignature<T: IUnknown>(&self, node_mask: u32, blob_with_root_signature: &[u8], ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _root_signature: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, node_mask: u32, blob_with_root_signature: *const u8, blob_length_in_bytes: usize, riid: &GUID, _root_signature: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[16]) };
		let ret = f(vt, node_mask, blob_with_root_signature.as_ptr_or_null(), blob_with_root_signature.len() as usize, T::IID, &mut _root_signature, );
		if ret.is_ok() {
			if let (Some(_root_signature)) = (_root_signature) {
				return Ok((T::from(_root_signature)));
			}
		}
		Err(ret)
	}

	fn CreateConstantBufferView(&self, desc: Option<&D3D12ConstantBufferViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, desc: Option<&D3D12ConstantBufferViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[17]) };
		let ret = f(vt, desc, dest_descriptor, );
	}

	fn CreateShaderResourceView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12ShaderResourceViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resource: Option<VTable>, desc: Option<&D3D12ShaderResourceViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[18]) };
		let ret = f(vt, option_to_vtable(resource), desc, dest_descriptor, );
	}

	fn CreateUnorderedAccessView(&self, resource: Option<&D3D12Resource>, counter_resource: Option<&D3D12Resource>, desc: Option<&D3D12UnorderedAccessViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resource: Option<VTable>, counter_resource: Option<VTable>, desc: Option<&D3D12UnorderedAccessViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[19]) };
		let ret = f(vt, option_to_vtable(resource), option_to_vtable(counter_resource), desc, dest_descriptor, );
	}

	fn CreateRenderTargetView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12RenderTargetViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resource: Option<VTable>, desc: Option<&D3D12RenderTargetViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[20]) };
		let ret = f(vt, option_to_vtable(resource), desc, dest_descriptor, );
	}

	fn CreateDepthStencilView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12DepthStencilViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resource: Option<VTable>, desc: Option<&D3D12DepthStencilViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[21]) };
		let ret = f(vt, option_to_vtable(resource), desc, dest_descriptor, );
	}

	fn CreateSampler(&self, desc: &D3D12SamplerDesc, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, desc: &D3D12SamplerDesc, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
			= unsafe { transmute(vt[22]) };
		let ret = f(vt, desc, dest_descriptor, );
	}

	fn CopyDescriptors(&self, dest_descriptor_range_starts: &[D3D12CpuDescriptorHandle], dest_descriptor_range_sizes: Option<&[u32]>, src_descriptor_range_starts: &[D3D12CpuDescriptorHandle], src_descriptor_range_sizes: Option<&[u32]>, descriptor_heaps_type: D3D12DescriptorHeapType, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_dest_descriptor_ranges: u32, dest_descriptor_range_starts: *const D3D12CpuDescriptorHandle, dest_descriptor_range_sizes: *const u32, num_src_descriptor_ranges: u32, src_descriptor_range_starts: *const D3D12CpuDescriptorHandle, src_descriptor_range_sizes: *const u32, descriptor_heaps_type: D3D12DescriptorHeapType, ) -> ()
			= unsafe { transmute(vt[23]) };
		let ret = f(vt, dest_descriptor_range_starts.len() as u32, dest_descriptor_range_starts.as_ptr_or_null(), dest_descriptor_range_sizes.as_ptr_or_null(), src_descriptor_range_starts.len() as u32, src_descriptor_range_starts.as_ptr_or_null(), src_descriptor_range_sizes.as_ptr_or_null(), descriptor_heaps_type, );
	}

	fn CopyDescriptorsSimple(&self, num_descriptors: u32, dest_descriptor_range_start: D3D12CpuDescriptorHandle, src_descriptor_range_start: D3D12CpuDescriptorHandle, descriptor_heaps_type: D3D12DescriptorHeapType, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_descriptors: u32, dest_descriptor_range_start: D3D12CpuDescriptorHandle, src_descriptor_range_start: D3D12CpuDescriptorHandle, descriptor_heaps_type: D3D12DescriptorHeapType, ) -> ()
			= unsafe { transmute(vt[24]) };
		let ret = f(vt, num_descriptors, dest_descriptor_range_start, src_descriptor_range_start, descriptor_heaps_type, );
	}

	fn GetResourceAllocationInfo(&self, visible_mask: u32, resource_descs: &[D3D12ResourceDesc], ) -> (D3D12ResourceAllocationInfo) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, visible_mask: u32, num_resource_descs: u32, resource_descs: *const D3D12ResourceDesc, ) -> D3D12ResourceAllocationInfo
			= unsafe { transmute(vt[25]) };
		let ret = f(vt, visible_mask, resource_descs.len() as u32, resource_descs.as_ptr_or_null(), );
		return (ret);
	}

	fn GetCustomHeapProperties(&self, node_mask: u32, heap_type: D3D12HeapType, ) -> (D3D12HeapProperties) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, node_mask: u32, heap_type: D3D12HeapType, ) -> D3D12HeapProperties
			= unsafe { transmute(vt[26]) };
		let ret = f(vt, node_mask, heap_type, );
		return (ret);
	}

	fn CreateCommittedResource<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_resource: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, riid_resource: &GUID, resource: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[27]) };
		let ret = f(vt, heap_properties, heap_flags, desc, initial_resource_state, optimized_clear_value, T::IID, if resource.is_some() { Some(&mut out_resource) } else { None }, );
		if let (Some(resource), Some(out_resource)) = (resource, out_resource) { *resource = Some(T::from(out_resource)); }
		ret.ok()
	}

	fn CreateHeap<T: IUnknown>(&self, desc: &D3D12HeapDesc, heap: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_heap: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12HeapDesc, riid: &GUID, heap: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[28]) };
		let ret = f(vt, desc, T::IID, if heap.is_some() { Some(&mut out_heap) } else { None }, );
		if let (Some(heap), Some(out_heap)) = (heap, out_heap) { *heap = Some(T::from(out_heap)); }
		ret.ok()
	}

	fn CreatePlacedResource<T: IUnknown>(&self, heap: &(impl ID3D12Heap + ?Sized), heap_offset: u64, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_resource: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, heap: VTable, heap_offset: u64, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, riid: &GUID, resource: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[29]) };
		let ret = f(vt, heap.vtable(), heap_offset, desc, initial_state, optimized_clear_value, T::IID, if resource.is_some() { Some(&mut out_resource) } else { None }, );
		if let (Some(resource), Some(out_resource)) = (resource, out_resource) { *resource = Some(T::from(out_resource)); }
		ret.ok()
	}

	fn CreateReservedResource<T: IUnknown>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_resource: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, riid: &GUID, resource: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[30]) };
		let ret = f(vt, desc, initial_state, optimized_clear_value, T::IID, if resource.is_some() { Some(&mut out_resource) } else { None }, );
		if let (Some(resource), Some(out_resource)) = (resource, out_resource) { *resource = Some(T::from(out_resource)); }
		ret.ok()
	}

	fn CreateSharedHandle(&self, object: &(impl ID3D12DeviceChild + ?Sized), attributes: Option<&SecurityAttributes>, access: u32, name: Option<&str>, ) -> Result<(Handle), HResult> {
		let vt = self.as_param();
		let mut _handle: Handle = Handle::zeroed();
		let f: extern "system" fn(Param<Self>, object: VTable, attributes: Option<&SecurityAttributes>, access: u32, name: *const u16, _handle: &mut Handle, ) -> HResult
			= unsafe { transmute(vt[31]) };
		let ret = f(vt, object.vtable(), attributes, access, name.map(str::to_utf16).as_ptr_or_null(), &mut _handle, );
		if ret.is_ok() {
			return Ok((_handle));
		}
		Err(ret)
	}

	fn OpenSharedHandle<T: IUnknown>(&self, nt_handle: Handle, obj: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_obj: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, nt_handle: Handle, riid: &GUID, obj: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[32]) };
		let ret = f(vt, nt_handle, T::IID, if obj.is_some() { Some(&mut out_obj) } else { None }, );
		if let (Some(obj), Some(out_obj)) = (obj, out_obj) { *obj = Some(T::from(out_obj)); }
		ret.ok()
	}

	fn OpenSharedHandleByName(&self, name: &str, access: u32, ) -> Result<(Handle), HResult> {
		let vt = self.as_param();
		let mut _nt_handle: Handle = Handle::zeroed();
		let f: extern "system" fn(Param<Self>, name: *const u16, access: u32, _nt_handle: &mut Handle, ) -> HResult
			= unsafe { transmute(vt[33]) };
		let ret = f(vt, name.to_utf16().as_ptr_or_null(), access, &mut _nt_handle, );
		if ret.is_ok() {
			return Ok((_nt_handle));
		}
		Err(ret)
	}

	fn MakeResident(&self, objects: &[Param<D3D12Pageable>], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_objects: u32, objects: *const Param<D3D12Pageable>, ) -> HResult
			= unsafe { transmute(vt[34]) };
		let ret = f(vt, objects.len() as u32, objects.as_ptr_or_null(), );
		ret.ok()
	}

	fn Evict(&self, objects: &[Param<D3D12Pageable>], ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, num_objects: u32, objects: *const Param<D3D12Pageable>, ) -> HResult
			= unsafe { transmute(vt[35]) };
		let ret = f(vt, objects.len() as u32, objects.as_ptr_or_null(), );
		ret.ok()
	}

	fn CreateFence<T: IUnknown>(&self, initial_value: u64, flags: D3D12FenceFlags, ) -> Result<(T), HResult> {
		let vt = self.as_param();
		let mut _fence: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, initial_value: u64, flags: D3D12FenceFlags, riid: &GUID, _fence: &mut Option<Unknown>, ) -> HResult
			= unsafe { transmute(vt[36]) };
		let ret = f(vt, initial_value, flags, T::IID, &mut _fence, );
		if ret.is_ok() {
			if let (Some(_fence)) = (_fence) {
				return Ok((T::from(_fence)));
			}
		}
		Err(ret)
	}

	fn GetDeviceRemovedReason(&self, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> HResult
			= unsafe { transmute(vt[37]) };
		let ret = f(vt, );
		ret.ok()
	}

	fn GetCopyableFootprints(&self, resource_desc: &D3D12ResourceDesc, first_subresource: u32, base_offset: u64, mut layouts: Option<&mut [D3D12PlacedSubresourceFootprint]>, mut num_rows: Option<&mut [u32]>, mut row_size_in_bytes: Option<&mut [u64]>, total_bytes: Option<&mut u64>, ) -> () {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, resource_desc: &D3D12ResourceDesc, first_subresource: u32, num_subresources: u32, base_offset: u64, layouts: *mut D3D12PlacedSubresourceFootprint, num_rows: *mut u32, row_size_in_bytes: *mut u64, total_bytes: Option<&mut u64>, ) -> ()
			= unsafe { transmute(vt[38]) };
		let ret = f(vt, resource_desc, first_subresource, layouts.len() as u32, base_offset, layouts.as_mut_ptr_or_null(), num_rows.as_mut_ptr_or_null(), row_size_in_bytes.as_mut_ptr_or_null(), total_bytes, );
	}

	fn CreateQueryHeap<T: IUnknown>(&self, desc: &D3D12QueryHeapDesc, heap: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_heap: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12QueryHeapDesc, riid: &GUID, heap: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[39]) };
		let ret = f(vt, desc, T::IID, if heap.is_some() { Some(&mut out_heap) } else { None }, );
		if let (Some(heap), Some(out_heap)) = (heap, out_heap) { *heap = Some(T::from(out_heap)); }
		ret.ok()
	}

	fn SetStablePowerState(&self, enable: bool, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, enable: Bool, ) -> HResult
			= unsafe { transmute(vt[40]) };
		let ret = f(vt, enable.to_bool(), );
		ret.ok()
	}

	fn CreateCommandSignature<T: IUnknown>(&self, desc: &D3D12CommandSignatureDesc, root_signature: Option<&D3D12RootSignature>, command_signature: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		let vt = self.as_param();
		let mut out_command_signature: Option<Unknown> = None;
		let f: extern "system" fn(Param<Self>, desc: &D3D12CommandSignatureDesc, root_signature: Option<VTable>, riid: &GUID, command_signature: Option<&mut Option<Unknown>>, ) -> HResult
			= unsafe { transmute(vt[41]) };
		let ret = f(vt, desc, option_to_vtable(root_signature), T::IID, if command_signature.is_some() { Some(&mut out_command_signature) } else { None }, );
		if let (Some(command_signature), Some(out_command_signature)) = (command_signature, out_command_signature) { *command_signature = Some(T::from(out_command_signature)); }
		ret.ok()
	}

	fn GetResourceTiling<'a>(&self, tiled_resource: &(impl ID3D12Resource + ?Sized), num_tiles_for_entire_resource: Option<&mut u32>, packed_mip_desc: Option<&mut D3D12PackedMipInfo>, standard_tile_shape_for_non_packed_mips: Option<&mut D3D12TileShape>, first_subresource_tiling_to_get: u32, mut subresource_tilings_for_non_packed_mips: &'a mut [D3D12SubresourceTiling], ) -> (&'a mut [D3D12SubresourceTiling]) {
		let vt = self.as_param();
		let mut num_subresource_tilings = subresource_tilings_for_non_packed_mips.len() as u32;
		let f: extern "system" fn(Param<Self>, tiled_resource: VTable, num_tiles_for_entire_resource: Option<&mut u32>, packed_mip_desc: Option<&mut D3D12PackedMipInfo>, standard_tile_shape_for_non_packed_mips: Option<&mut D3D12TileShape>, num_subresource_tilings: &mut u32, first_subresource_tiling_to_get: u32, subresource_tilings_for_non_packed_mips: *mut D3D12SubresourceTiling, ) -> ()
			= unsafe { transmute(vt[42]) };
		let ret = f(vt, tiled_resource.vtable(), num_tiles_for_entire_resource, packed_mip_desc, standard_tile_shape_for_non_packed_mips, &mut num_subresource_tilings, first_subresource_tiling_to_get, subresource_tilings_for_non_packed_mips.as_mut_ptr_or_null(), );
		return (&mut subresource_tilings_for_non_packed_mips[..(num_subresource_tilings as usize)]);
	}

	fn GetAdapterLuid(&self, ) -> (Luid) {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, ) -> Luid
			= unsafe { transmute(vt[43]) };
		let ret = f(vt, );
		return (ret);
	}
}

impl ID3D12Device for D3D12Device {
	fn as_device(&self) -> &D3D12Device { self }
	fn into_device(self) -> D3D12Device { self }
}

impl ID3D12Object for D3D12Device {
	fn as_object(&self) -> &D3D12Object { &self.0 }
	fn into_object(self) -> D3D12Object { self.0 }
}

impl From<Unknown> for D3D12Device {
    fn from(v: Unknown) -> Self {
        Self(D3D12Object::from(v))
    }
}

impl IUnknown for D3D12Device {
	fn as_unknown(&self) -> &Unknown { &self.0.0 }
	fn into_unknown(self) -> Unknown { self.0.0 }
}

