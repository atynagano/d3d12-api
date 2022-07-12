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

	fn GetNodeCount(&self, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> u32
				= transmute(vt[7]);
			let _ret_ = f(vt, );
			_ret_
		}
	}

	fn CreateCommandQueue<T: IUnknown>(&self, desc: &D3D12CommandQueueDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_command_queue: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12CommandQueueDesc, riid: &GUID, _out_command_queue: *mut c_void, ) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut _out_command_queue), );
			if _ret_.is_ok() {
				if let Some(_out_command_queue) = _out_command_queue {
					return Ok(T::from(_out_command_queue));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateCommandAllocator<T: IUnknown>(&self, ty: D3D12CommandListType, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_command_allocator: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, ty: D3D12CommandListType, riid: &GUID, _out_command_allocator: *mut c_void, ) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, ty, T::IID, transmute(&mut _out_command_allocator), );
			if _ret_.is_ok() {
				if let Some(_out_command_allocator) = _out_command_allocator {
					return Ok(T::from(_out_command_allocator));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateGraphicsPipelineState<T: IUnknown>(&self, desc: &D3D12GraphicsPipelineStateDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pipeline_state: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12GraphicsPipelineStateDesc, riid: &GUID, _out_pipeline_state: *mut c_void, ) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut _out_pipeline_state), );
			if _ret_.is_ok() {
				if let Some(_out_pipeline_state) = _out_pipeline_state {
					return Ok(T::from(_out_pipeline_state));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateComputePipelineState<T: IUnknown>(&self, desc: &D3D12ComputePipelineStateDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_pipeline_state: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12ComputePipelineStateDesc, riid: &GUID, _out_pipeline_state: *mut c_void, ) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut _out_pipeline_state), );
			if _ret_.is_ok() {
				if let Some(_out_pipeline_state) = _out_pipeline_state {
					return Ok(T::from(_out_pipeline_state));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateCommandList<T: IUnknown>(&self, node_mask: u32, ty: D3D12CommandListType, command_allocator: &(impl ID3D12CommandAllocator + ?Sized), initial_state: Option<&D3D12PipelineState>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_command_list: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, node_mask: u32, ty: D3D12CommandListType, command_allocator: VTable, initial_state: *const c_void, riid: &GUID, _out_command_list: *mut c_void, ) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, node_mask, ty, command_allocator.vtable(), option_to_vtable(initial_state), T::IID, transmute(&mut _out_command_list), );
			if _ret_.is_ok() {
				if let Some(_out_command_list) = _out_command_list {
					return Ok(T::from(_out_command_list));
				}
			}
			Err(_ret_)
		}
	}

	fn CheckFeatureSupport<U>(&self, feature: D3D12Feature, feature_support_data: &mut U) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, D3D12Feature, *mut c_void, u32) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, feature, unsafe { transmute(feature_support_data as *const U) }, size_of_val(feature_support_data) as u32);
		ret.ok()
	}

	fn CreateDescriptorHeap<T: IUnknown>(&self, descriptor_heap_desc: &D3D12DescriptorHeapDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_heap: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, descriptor_heap_desc: &D3D12DescriptorHeapDesc, riid: &GUID, _out_heap: *mut c_void, ) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, descriptor_heap_desc, T::IID, transmute(&mut _out_heap), );
			if _ret_.is_ok() {
				if let Some(_out_heap) = _out_heap {
					return Ok(T::from(_out_heap));
				}
			}
			Err(_ret_)
		}
	}

	fn GetDescriptorHandleIncrementSize(&self, descriptor_heap_type: D3D12DescriptorHeapType, ) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, descriptor_heap_type: D3D12DescriptorHeapType, ) -> u32
				= transmute(vt[15]);
			let _ret_ = f(vt, descriptor_heap_type, );
			_ret_
		}
	}

	fn CreateRootSignature<T: IUnknown>(&self, node_mask: u32, blob_with_root_signature: &[u8], ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_blob_with_root_signature, _len_blob_with_root_signature) = blob_with_root_signature.deconstruct();
			let mut _out_root_signature: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, node_mask: u32, blob_with_root_signature: *const u8, blob_length_in_bytes: usize, riid: &GUID, _out_root_signature: *mut c_void, ) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, node_mask, _ptr_blob_with_root_signature, _len_blob_with_root_signature as usize, T::IID, transmute(&mut _out_root_signature), );
			if _ret_.is_ok() {
				if let Some(_out_root_signature) = _out_root_signature {
					return Ok(T::from(_out_root_signature));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateConstantBufferView(&self, desc: Option<&D3D12ConstantBufferViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, desc: *const c_void, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, transmute(desc), dest_descriptor, );
		}
	}

	fn CreateShaderResourceView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12ShaderResourceViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, resource: *const c_void, desc: *const c_void, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
				= transmute(vt[18]);
			let _ret_ = f(vt, option_to_vtable(resource), transmute(desc), dest_descriptor, );
		}
	}

	fn CreateUnorderedAccessView(&self, resource: Option<&D3D12Resource>, counter_resource: Option<&D3D12Resource>, desc: Option<&D3D12UnorderedAccessViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, resource: *const c_void, counter_resource: *const c_void, desc: *const c_void, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
				= transmute(vt[19]);
			let _ret_ = f(vt, option_to_vtable(resource), option_to_vtable(counter_resource), transmute(desc), dest_descriptor, );
		}
	}

	fn CreateRenderTargetView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12RenderTargetViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, resource: *const c_void, desc: *const c_void, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
				= transmute(vt[20]);
			let _ret_ = f(vt, option_to_vtable(resource), transmute(desc), dest_descriptor, );
		}
	}

	fn CreateDepthStencilView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12DepthStencilViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, resource: *const c_void, desc: *const c_void, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
				= transmute(vt[21]);
			let _ret_ = f(vt, option_to_vtable(resource), transmute(desc), dest_descriptor, );
		}
	}

	fn CreateSampler(&self, desc: &D3D12SamplerDesc, dest_descriptor: D3D12CpuDescriptorHandle, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, desc: &D3D12SamplerDesc, dest_descriptor: D3D12CpuDescriptorHandle, ) -> ()
				= transmute(vt[22]);
			let _ret_ = f(vt, desc, dest_descriptor, );
		}
	}

	fn CopyDescriptorsSimple(&self, num_descriptors: u32, dest_descriptor_range_start: D3D12CpuDescriptorHandle, src_descriptor_range_start: D3D12CpuDescriptorHandle, descriptor_heaps_type: D3D12DescriptorHeapType, ) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, num_descriptors: u32, dest_descriptor_range_start: D3D12CpuDescriptorHandle, src_descriptor_range_start: D3D12CpuDescriptorHandle, descriptor_heaps_type: D3D12DescriptorHeapType, ) -> ()
				= transmute(vt[24]);
			let _ret_ = f(vt, num_descriptors, dest_descriptor_range_start, src_descriptor_range_start, descriptor_heaps_type, );
		}
	}

	fn GetResourceAllocationInfo(&self, visible_mask: u32, resource_descs: &[D3D12ResourceDesc], ) -> D3D12ResourceAllocationInfo {
		unsafe {
			let vt = self.as_param();
			let (_ptr_resource_descs, _len_resource_descs) = resource_descs.deconstruct();
			let f: extern "system" fn(Param<Self>, visible_mask: u32, num_resource_descs: u32, resource_descs: *const D3D12ResourceDesc, ) -> D3D12ResourceAllocationInfo
				= transmute(vt[25]);
			let _ret_ = f(vt, visible_mask, _len_resource_descs as u32, _ptr_resource_descs, );
			_ret_
		}
	}

	fn GetCustomHeapProperties(&self, node_mask: u32, heap_type: D3D12HeapType, ) -> D3D12HeapProperties {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, node_mask: u32, heap_type: D3D12HeapType, ) -> D3D12HeapProperties
				= transmute(vt[26]);
			let _ret_ = f(vt, node_mask, heap_type, );
			_ret_
		}
	}

	fn CreateCommittedResource<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: *const c_void, riid_resource: &GUID, resource: *mut c_void, ) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), T::IID, transmute(if resource.is_some() { Some(&mut _out_resource) } else { None }), );
			if let Some(_out_resource) = _out_resource { *resource.unwrap_unchecked() = Some(T::from(_out_resource)); }
			_ret_.ok()
		}
	}

	fn create_committed_resource<T: IUnknown>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: *const c_void, riid_resource: &GUID, _out_resource: *mut c_void, ) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), T::IID, transmute(&mut _out_resource), );
			if _ret_.is_ok() {
				if let Some(_out_resource) = _out_resource {
					return Ok(T::from(_out_resource));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateHeap<T: IUnknown>(&self, desc: &D3D12HeapDesc, heap: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_heap: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12HeapDesc, riid: &GUID, heap: *mut c_void, ) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, desc, T::IID, transmute(if heap.is_some() { Some(&mut _out_heap) } else { None }), );
			if let Some(_out_heap) = _out_heap { *heap.unwrap_unchecked() = Some(T::from(_out_heap)); }
			_ret_.ok()
		}
	}

	fn create_heap<T: IUnknown>(&self, desc: &D3D12HeapDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_heap: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12HeapDesc, riid: &GUID, _out_heap: *mut c_void, ) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut _out_heap), );
			if _ret_.is_ok() {
				if let Some(_out_heap) = _out_heap {
					return Ok(T::from(_out_heap));
				}
			}
			Err(_ret_)
		}
	}

	fn CreatePlacedResource<T: IUnknown>(&self, heap: &(impl ID3D12Heap + ?Sized), heap_offset: u64, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap: VTable, heap_offset: u64, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: *const c_void, riid: &GUID, resource: *mut c_void, ) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, heap.vtable(), heap_offset, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(if resource.is_some() { Some(&mut _out_resource) } else { None }), );
			if let Some(_out_resource) = _out_resource { *resource.unwrap_unchecked() = Some(T::from(_out_resource)); }
			_ret_.ok()
		}
	}

	fn create_placed_resource<T: IUnknown>(&self, heap: &(impl ID3D12Heap + ?Sized), heap_offset: u64, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, heap: VTable, heap_offset: u64, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: *const c_void, riid: &GUID, _out_resource: *mut c_void, ) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, heap.vtable(), heap_offset, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(&mut _out_resource), );
			if _ret_.is_ok() {
				if let Some(_out_resource) = _out_resource {
					return Ok(T::from(_out_resource));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateReservedResource<T: IUnknown>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: *const c_void, riid: &GUID, resource: *mut c_void, ) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(if resource.is_some() { Some(&mut _out_resource) } else { None }), );
			if let Some(_out_resource) = _out_resource { *resource.unwrap_unchecked() = Some(T::from(_out_resource)); }
			_ret_.ok()
		}
	}

	fn create_reserved_resource<T: IUnknown>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_resource: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: *const c_void, riid: &GUID, _out_resource: *mut c_void, ) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(&mut _out_resource), );
			if _ret_.is_ok() {
				if let Some(_out_resource) = _out_resource {
					return Ok(T::from(_out_resource));
				}
			}
			Err(_ret_)
		}
	}

	fn CreateSharedHandle(&self, object: &(impl ID3D12DeviceChild + ?Sized), attributes: Option<&SecurityAttributes>, access: u32, name: Option<&str>, ) -> Result<Handle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_handle: Option<Handle> = None;
			let f: extern "system" fn(Param<Self>, object: VTable, attributes: *const c_void, access: u32, name: *const u16, _out_handle: *mut c_void, ) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, object.vtable(), transmute(attributes), access, name.map(str::to_utf16).as_ptr_or_null(), transmute(&mut _out_handle), );
			if _ret_.is_ok() {
				if let Some(_out_handle) = _out_handle {
					return Ok(_out_handle);
				}
			}
			Err(_ret_)
		}
	}

	fn OpenSharedHandle<T: IUnknown>(&self, nt_handle: Handle, obj: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_obj: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, nt_handle: Handle, riid: &GUID, obj: *mut c_void, ) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, nt_handle, T::IID, transmute(if obj.is_some() { Some(&mut _out_obj) } else { None }), );
			if let Some(_out_obj) = _out_obj { *obj.unwrap_unchecked() = Some(T::from(_out_obj)); }
			_ret_.ok()
		}
	}

	fn open_shared_handle<T: IUnknown>(&self, nt_handle: Handle, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_obj: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, nt_handle: Handle, riid: &GUID, _out_obj: *mut c_void, ) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, nt_handle, T::IID, transmute(&mut _out_obj), );
			if _ret_.is_ok() {
				if let Some(_out_obj) = _out_obj {
					return Ok(T::from(_out_obj));
				}
			}
			Err(_ret_)
		}
	}

	fn OpenSharedHandleByName(&self, name: &str, access: u32, ) -> Result<Handle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_nt_handle: Option<Handle> = None;
			let f: extern "system" fn(Param<Self>, name: *const u16, access: u32, _out_nt_handle: *mut c_void, ) -> HResult
				= transmute(vt[33]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), access, transmute(&mut _out_nt_handle), );
			if _ret_.is_ok() {
				if let Some(_out_nt_handle) = _out_nt_handle {
					return Ok(_out_nt_handle);
				}
			}
			Err(_ret_)
		}
	}

	fn MakeResident(&self, objects: &[Param<D3D12Pageable>], ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_objects, _len_objects) = objects.deconstruct();
			let f: extern "system" fn(Param<Self>, num_objects: u32, objects: *const Param<D3D12Pageable>, ) -> HResult
				= transmute(vt[34]);
			let _ret_ = f(vt, _len_objects as u32, _ptr_objects, );
			_ret_.ok()
		}
	}

	fn Evict(&self, objects: &[Param<D3D12Pageable>], ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (_ptr_objects, _len_objects) = objects.deconstruct();
			let f: extern "system" fn(Param<Self>, num_objects: u32, objects: *const Param<D3D12Pageable>, ) -> HResult
				= transmute(vt[35]);
			let _ret_ = f(vt, _len_objects as u32, _ptr_objects, );
			_ret_.ok()
		}
	}

	fn CreateFence<T: IUnknown>(&self, initial_value: u64, flags: D3D12FenceFlags, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_fence: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, initial_value: u64, flags: D3D12FenceFlags, riid: &GUID, _out_fence: *mut c_void, ) -> HResult
				= transmute(vt[36]);
			let _ret_ = f(vt, initial_value, flags, T::IID, transmute(&mut _out_fence), );
			if _ret_.is_ok() {
				if let Some(_out_fence) = _out_fence {
					return Ok(T::from(_out_fence));
				}
			}
			Err(_ret_)
		}
	}

	fn GetDeviceRemovedReason(&self, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> HResult
				= transmute(vt[37]);
			let _ret_ = f(vt, );
			_ret_.ok()
		}
	}

	fn GetCopyableFootprints(
		&self,
		resource_desc: &D3D12ResourceDesc,
		first_subresource: u32,
		num_subresources: u32,
		base_offset: u64,
		layouts: *mut D3D12PlacedSubresourceFootprint,
		num_rows: *mut u32,
		row_size_in_bytes: *mut u64,
		total_bytes: *mut u64,
	) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D3D12ResourceDesc, u32, u32, u64, *mut D3D12PlacedSubresourceFootprint, *mut u32, *mut u64, *mut u64) -> ()
				= transmute(vt[38]);
			f(vt, resource_desc, first_subresource, num_subresources, base_offset, layouts, num_rows, row_size_in_bytes, total_bytes);
		}
	}

	fn get_copyable_footprint(&self, resource_desc: &D3D12ResourceDesc) -> D3D12CopyableFootprint {
		unsafe {
			let mut out = MaybeUninit::<D3D12CopyableFootprint>::zeroed();
			let ptr = &mut *out.as_mut_ptr();
			self.GetCopyableFootprints(resource_desc, 0, 1, 0, &mut ptr.layout, &mut ptr.num_row, &mut ptr.row_size_in_bytes, &mut ptr.total_bytes);
			out.assume_init()
		}
	}


	fn CreateQueryHeap<T: IUnknown>(&self, desc: &D3D12QueryHeapDesc, heap: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_heap: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12QueryHeapDesc, riid: &GUID, heap: *mut c_void, ) -> HResult
				= transmute(vt[39]);
			let _ret_ = f(vt, desc, T::IID, transmute(if heap.is_some() { Some(&mut _out_heap) } else { None }), );
			if let Some(_out_heap) = _out_heap { *heap.unwrap_unchecked() = Some(T::from(_out_heap)); }
			_ret_.ok()
		}
	}

	fn create_query_heap<T: IUnknown>(&self, desc: &D3D12QueryHeapDesc, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_heap: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12QueryHeapDesc, riid: &GUID, _out_heap: *mut c_void, ) -> HResult
				= transmute(vt[39]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut _out_heap), );
			if _ret_.is_ok() {
				if let Some(_out_heap) = _out_heap {
					return Ok(T::from(_out_heap));
				}
			}
			Err(_ret_)
		}
	}

	fn SetStablePowerState(&self, enable: bool, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, enable: Bool, ) -> HResult
				= transmute(vt[40]);
			let _ret_ = f(vt, enable.to_bool(), );
			_ret_.ok()
		}
	}

	fn CreateCommandSignature<T: IUnknown>(&self, desc: &D3D12CommandSignatureDesc, root_signature: Option<&D3D12RootSignature>, command_signature: Option<&mut Option<T>>, ) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_command_signature: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12CommandSignatureDesc, root_signature: *const c_void, riid: &GUID, command_signature: *mut c_void, ) -> HResult
				= transmute(vt[41]);
			let _ret_ = f(vt, desc, option_to_vtable(root_signature), T::IID, transmute(if command_signature.is_some() { Some(&mut _out_command_signature) } else { None }), );
			if let Some(_out_command_signature) = _out_command_signature { *command_signature.unwrap_unchecked() = Some(T::from(_out_command_signature)); }
			_ret_.ok()
		}
	}

	fn create_command_signature<T: IUnknown>(&self, desc: &D3D12CommandSignatureDesc, root_signature: Option<&D3D12RootSignature>, ) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut _out_command_signature: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, desc: &D3D12CommandSignatureDesc, root_signature: *const c_void, riid: &GUID, _out_command_signature: *mut c_void, ) -> HResult
				= transmute(vt[41]);
			let _ret_ = f(vt, desc, option_to_vtable(root_signature), T::IID, transmute(&mut _out_command_signature), );
			if _ret_.is_ok() {
				if let Some(_out_command_signature) = _out_command_signature {
					return Ok(T::from(_out_command_signature));
				}
			}
			Err(_ret_)
		}
	}

	fn GetAdapterLuid(&self, ) -> Luid {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, ) -> Luid
				= transmute(vt[43]);
			let _ret_ = f(vt, );
			_ret_
		}
	}
}

impl ID3D12Device for D3D12Device {
	fn as_device(&self) -> &D3D12Device { self }
	fn into_device(self) -> D3D12Device { self }
}

impl ID3D12Object for D3D12Device {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl From<Unknown> for D3D12Device {
    fn from(v: Unknown) -> Self {
        Self(D3D12Object::from(v))
    }
}

impl IUnknown for D3D12Device {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

