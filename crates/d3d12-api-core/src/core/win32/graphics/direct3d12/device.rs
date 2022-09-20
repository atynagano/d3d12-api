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
use crate::core::win32::security::*;

#[repr(C)]
#[derive(Clone, Hash)]
pub struct D3D12Device(pub(crate) D3D12Object);

impl Deref for D3D12Device {
	type Target = D3D12Object;
	fn deref(&self) -> &Self::Target { &self.0	}
}

impl Guid for D3D12Device {
	const IID: &'static GUID = &GUID::from_u128(0x189819f11db64b57be541821339b85f7u128);
}

impl Com for D3D12Device {
	fn vtable(&self) -> VTable { self.0.vtable() }
}

impl D3D12Device {
	pub fn GetNodeCount(&self) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> u32
				= transmute(vt[7]);
			let _ret_ = f(vt);
			_ret_
		}
	}

	pub fn CreateCommandQueue<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12CommandQueueDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut command_queue_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12CommandQueueDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[8]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut command_queue_out_));
			if _ret_.is_ok() { if let Some(command_queue_out_) = command_queue_out_ { return Ok(T::from(UnknownWrapper(command_queue_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateCommandAllocator<T: IUnknown + From<UnknownWrapper>>(&self, r#type: D3D12CommandListType) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut command_allocator_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, D3D12CommandListType, &GUID, *mut c_void) -> HResult
				= transmute(vt[9]);
			let _ret_ = f(vt, r#type, T::IID, transmute(&mut command_allocator_out_));
			if _ret_.is_ok() { if let Some(command_allocator_out_) = command_allocator_out_ { return Ok(T::from(UnknownWrapper(command_allocator_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateGraphicsPipelineState<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12GraphicsPipelineStateDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pipeline_state_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12GraphicsPipelineStateDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[10]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut pipeline_state_out_));
			if _ret_.is_ok() { if let Some(pipeline_state_out_) = pipeline_state_out_ { return Ok(T::from(UnknownWrapper(pipeline_state_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateComputePipelineState<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12ComputePipelineStateDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut pipeline_state_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12ComputePipelineStateDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[11]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut pipeline_state_out_));
			if _ret_.is_ok() { if let Some(pipeline_state_out_) = pipeline_state_out_ { return Ok(T::from(UnknownWrapper(pipeline_state_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateCommandList<T: IUnknown + From<UnknownWrapper>>(&self, node_mask: u32, r#type: D3D12CommandListType, command_allocator: &D3D12CommandAllocator, initial_state: Option<&D3D12PipelineState>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut command_list_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, u32, D3D12CommandListType, VTable, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[12]);
			let _ret_ = f(vt, node_mask, r#type, command_allocator.vtable(), option_to_vtable(initial_state), T::IID, transmute(&mut command_list_out_));
			if _ret_.is_ok() { if let Some(command_list_out_) = command_list_out_ { return Ok(T::from(UnknownWrapper(command_list_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CheckFeatureSupport<U>(&self, feature: D3D12Feature, feature_support_data: &mut U) -> Result<(), HResult> {
		let vt = self.as_param();
		let f: extern "system" fn(Param<Self>, D3D12Feature, *mut c_void, u32) -> HResult
			= unsafe { transmute(vt[13]) };
		let ret = f(vt, feature, unsafe { transmute(feature_support_data as *const U) }, size_of_val(feature_support_data) as u32);
		ret.ok()
	}

	pub fn CreateDescriptorHeap<T: IUnknown + From<UnknownWrapper>>(&self, descriptor_heap_desc: &D3D12DescriptorHeapDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12DescriptorHeapDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[14]);
			let _ret_ = f(vt, descriptor_heap_desc, T::IID, transmute(&mut heap_out_));
			if _ret_.is_ok() { if let Some(heap_out_) = heap_out_ { return Ok(T::from(UnknownWrapper(heap_out_))); } }
			Err(_ret_)
		}
	}

	pub fn GetDescriptorHandleIncrementSize(&self, descriptor_heap_type: D3D12DescriptorHeapType) -> u32 {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, D3D12DescriptorHeapType) -> u32
				= transmute(vt[15]);
			let _ret_ = f(vt, descriptor_heap_type);
			_ret_
		}
	}

	pub fn CreateRootSignature<T: IUnknown + From<UnknownWrapper>>(&self, node_mask: u32, blob_with_root_signature: &[u8]) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let (blob_with_root_signature_ptr_, blob_with_root_signature_len_) = blob_with_root_signature.deconstruct();
			let mut root_signature_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, u32, *const u8, usize, &GUID, *mut c_void) -> HResult
				= transmute(vt[16]);
			let _ret_ = f(vt, node_mask, blob_with_root_signature_ptr_, blob_with_root_signature_len_ as usize, T::IID, transmute(&mut root_signature_out_));
			if _ret_.is_ok() { if let Some(root_signature_out_) = root_signature_out_ { return Ok(T::from(UnknownWrapper(root_signature_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateConstantBufferView(&self, desc: Option<&D3D12ConstantBufferViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, D3D12CpuDescriptorHandle) -> ()
				= transmute(vt[17]);
			let _ret_ = f(vt, transmute(desc), dest_descriptor);
		}
	}

	pub fn CreateShaderResourceView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12ShaderResourceViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, D3D12CpuDescriptorHandle) -> ()
				= transmute(vt[18]);
			let _ret_ = f(vt, option_to_vtable(resource), transmute(desc), dest_descriptor);
		}
	}

	pub fn CreateUnorderedAccessView(&self, resource: Option<&D3D12Resource>, counter_resource: Option<&D3D12Resource>, desc: Option<&D3D12UnorderedAccessViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, *const c_void, D3D12CpuDescriptorHandle) -> ()
				= transmute(vt[19]);
			let _ret_ = f(vt, option_to_vtable(resource), option_to_vtable(counter_resource), transmute(desc), dest_descriptor);
		}
	}

	pub fn CreateRenderTargetView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12RenderTargetViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, D3D12CpuDescriptorHandle) -> ()
				= transmute(vt[20]);
			let _ret_ = f(vt, option_to_vtable(resource), transmute(desc), dest_descriptor);
		}
	}

	pub fn CreateDepthStencilView(&self, resource: Option<&D3D12Resource>, desc: Option<&D3D12DepthStencilViewDesc>, dest_descriptor: D3D12CpuDescriptorHandle) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, *const c_void, *const c_void, D3D12CpuDescriptorHandle) -> ()
				= transmute(vt[21]);
			let _ret_ = f(vt, option_to_vtable(resource), transmute(desc), dest_descriptor);
		}
	}

	pub fn CreateSampler(&self, desc: &D3D12SamplerDesc, dest_descriptor: D3D12CpuDescriptorHandle) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, &D3D12SamplerDesc, D3D12CpuDescriptorHandle) -> ()
				= transmute(vt[22]);
			let _ret_ = f(vt, desc, dest_descriptor);
		}
	}

	pub unsafe fn CopyDescriptors() { todo!() }

	pub fn CopyDescriptorsSimple(&self, num_descriptors: u32, dest_descriptor_range_start: D3D12CpuDescriptorHandle, src_descriptor_range_start: D3D12CpuDescriptorHandle, descriptor_heaps_type: D3D12DescriptorHeapType) -> () {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12CpuDescriptorHandle, D3D12CpuDescriptorHandle, D3D12DescriptorHeapType) -> ()
				= transmute(vt[24]);
			let _ret_ = f(vt, num_descriptors, dest_descriptor_range_start, src_descriptor_range_start, descriptor_heaps_type);
		}
	}

	pub fn GetResourceAllocationInfo(&self, visible_mask: u32, resource_descs: &[D3D12ResourceDesc]) -> D3D12ResourceAllocationInfo {
		unsafe {
			let vt = self.as_param();
			let mut _out_: MaybeUninit<D3D12ResourceAllocationInfo> = MaybeUninit::zeroed();
			let (resource_descs_ptr_, resource_descs_len_) = resource_descs.deconstruct();
			let f: extern "system" fn(Param<Self>, *mut D3D12ResourceAllocationInfo, u32, u32, *const D3D12ResourceDesc) -> ()
				= transmute(vt[25]);
			let _ret_ = f(vt, _out_.as_mut_ptr(), visible_mask, resource_descs_len_ as u32, resource_descs_ptr_);
			_out_.assume_init()
		}
	}

	pub fn GetCustomHeapProperties(&self, node_mask: u32, heap_type: D3D12HeapType) -> D3D12HeapProperties {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, u32, D3D12HeapType) -> D3D12HeapProperties
				= transmute(vt[26]);
			let _ret_ = f(vt, node_mask, heap_type);
			_ret_
		}
	}

	pub fn CreateCommittedResource<T: IUnknown + From<UnknownWrapper>>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12HeapProperties, D3D12HeapFlags, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), T::IID, transmute(if resource_out_.is_some() { Some(&mut resource_out_) } else { None }));
			if let Some(resource_out_) = resource_out_ { *resource.unwrap_unchecked() = Some(T::from(UnknownWrapper(resource_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_committed_resource<T: IUnknown + From<UnknownWrapper>>(&self, heap_properties: &D3D12HeapProperties, heap_flags: D3D12HeapFlags, desc: &D3D12ResourceDesc, initial_resource_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12HeapProperties, D3D12HeapFlags, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[27]);
			let _ret_ = f(vt, heap_properties, heap_flags, desc, initial_resource_state, transmute(optimized_clear_value), T::IID, transmute(&mut resource_out_));
			if _ret_.is_ok() { if let Some(resource_out_) = resource_out_ { return Ok(T::from(UnknownWrapper(resource_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateHeap<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12HeapDesc, heap: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12HeapDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, desc, T::IID, transmute(if heap_out_.is_some() { Some(&mut heap_out_) } else { None }));
			if let Some(heap_out_) = heap_out_ { *heap.unwrap_unchecked() = Some(T::from(UnknownWrapper(heap_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_heap<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12HeapDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12HeapDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[28]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut heap_out_));
			if _ret_.is_ok() { if let Some(heap_out_) = heap_out_ { return Ok(T::from(UnknownWrapper(heap_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreatePlacedResource<T: IUnknown + From<UnknownWrapper>>(&self, heap: &D3D12Heap, heap_offset: u64, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, VTable, u64, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, heap.vtable(), heap_offset, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(if resource_out_.is_some() { Some(&mut resource_out_) } else { None }));
			if let Some(resource_out_) = resource_out_ { *resource.unwrap_unchecked() = Some(T::from(UnknownWrapper(resource_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_placed_resource<T: IUnknown + From<UnknownWrapper>>(&self, heap: &D3D12Heap, heap_offset: u64, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, VTable, u64, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[29]);
			let _ret_ = f(vt, heap.vtable(), heap_offset, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(&mut resource_out_));
			if _ret_.is_ok() { if let Some(resource_out_) = resource_out_ { return Ok(T::from(UnknownWrapper(resource_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateReservedResource<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>, resource: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(if resource_out_.is_some() { Some(&mut resource_out_) } else { None }));
			if let Some(resource_out_) = resource_out_ { *resource.unwrap_unchecked() = Some(T::from(UnknownWrapper(resource_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_reserved_resource<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12ResourceDesc, initial_state: D3D12ResourceStates, optimized_clear_value: Option<&D3D12ClearValue>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut resource_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12ResourceDesc, D3D12ResourceStates, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[30]);
			let _ret_ = f(vt, desc, initial_state, transmute(optimized_clear_value), T::IID, transmute(&mut resource_out_));
			if _ret_.is_ok() { if let Some(resource_out_) = resource_out_ { return Ok(T::from(UnknownWrapper(resource_out_))); } }
			Err(_ret_)
		}
	}

	pub fn CreateSharedHandle(&self, object: &D3D12DeviceChild, attributes: Option<&SecurityAttributes>, access: u32, name: Option<&str>) -> Result<Handle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut handle_out_: Option<Handle> = None;
			let f: extern "system" fn(Param<Self>, VTable, *const c_void, u32, *const u16, *mut c_void) -> HResult
				= transmute(vt[31]);
			let _ret_ = f(vt, object.vtable(), transmute(attributes), access, name.map(str::to_utf16).as_ptr_or_null(), transmute(&mut handle_out_));
			if _ret_.is_ok() { if let Some(handle_out_) = handle_out_ { return Ok(handle_out_); } }
			Err(_ret_)
		}
	}

	pub fn OpenSharedHandle<T: IUnknown + From<UnknownWrapper>>(&self, nt_handle: Handle, obj: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut obj_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, Handle, &GUID, *mut c_void) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, nt_handle, T::IID, transmute(if obj_out_.is_some() { Some(&mut obj_out_) } else { None }));
			if let Some(obj_out_) = obj_out_ { *obj.unwrap_unchecked() = Some(T::from(UnknownWrapper(obj_out_))); }
			_ret_.ok()
		}
	}

	pub fn open_shared_handle<T: IUnknown + From<UnknownWrapper>>(&self, nt_handle: Handle) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut obj_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, Handle, &GUID, *mut c_void) -> HResult
				= transmute(vt[32]);
			let _ret_ = f(vt, nt_handle, T::IID, transmute(&mut obj_out_));
			if _ret_.is_ok() { if let Some(obj_out_) = obj_out_ { return Ok(T::from(UnknownWrapper(obj_out_))); } }
			Err(_ret_)
		}
	}

	pub fn OpenSharedHandleByName(&self, name: &str, access: u32) -> Result<Handle, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut nt_handle_out_: Option<Handle> = None;
			let f: extern "system" fn(Param<Self>, *const u16, u32, *mut c_void) -> HResult
				= transmute(vt[33]);
			let _ret_ = f(vt, name.to_utf16().as_ptr_or_null(), access, transmute(&mut nt_handle_out_));
			if _ret_.is_ok() { if let Some(nt_handle_out_) = nt_handle_out_ { return Ok(nt_handle_out_); } }
			Err(_ret_)
		}
	}

	pub fn MakeResident(&self, objects: &[Param<D3D12Pageable>]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (objects_ptr_, objects_len_) = objects.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Param<D3D12Pageable>) -> HResult
				= transmute(vt[34]);
			let _ret_ = f(vt, objects_len_ as u32, objects_ptr_);
			_ret_.ok()
		}
	}

	pub fn Evict(&self, objects: &[Param<D3D12Pageable>]) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let (objects_ptr_, objects_len_) = objects.deconstruct();
			let f: extern "system" fn(Param<Self>, u32, *const Param<D3D12Pageable>) -> HResult
				= transmute(vt[35]);
			let _ret_ = f(vt, objects_len_ as u32, objects_ptr_);
			_ret_.ok()
		}
	}

	pub fn CreateFence<T: IUnknown + From<UnknownWrapper>>(&self, initial_value: u64, flags: D3D12FenceFlags) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut fence_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, u64, D3D12FenceFlags, &GUID, *mut c_void) -> HResult
				= transmute(vt[36]);
			let _ret_ = f(vt, initial_value, flags, T::IID, transmute(&mut fence_out_));
			if _ret_.is_ok() { if let Some(fence_out_) = fence_out_ { return Ok(T::from(UnknownWrapper(fence_out_))); } }
			Err(_ret_)
		}
	}

	pub fn GetDeviceRemovedReason(&self) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> HResult
				= transmute(vt[37]);
			let _ret_ = f(vt);
			_ret_.ok()
		}
	}

	pub fn GetCopyableFootprints(
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

	pub fn get_copyable_footprint(&self, resource_desc: &D3D12ResourceDesc) -> D3D12CopyableFootprint {
		unsafe {
			let mut out = MaybeUninit::<D3D12CopyableFootprint>::zeroed();
			let ptr = &mut *out.as_mut_ptr();
			self.GetCopyableFootprints(resource_desc, 0, 1, 0, &mut ptr.layout, &mut ptr.num_row, &mut ptr.row_size_in_bytes, &mut ptr.total_bytes);
			out.assume_init()
		}
	}


	pub fn CreateQueryHeap<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12QueryHeapDesc, heap: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12QueryHeapDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[39]);
			let _ret_ = f(vt, desc, T::IID, transmute(if heap_out_.is_some() { Some(&mut heap_out_) } else { None }));
			if let Some(heap_out_) = heap_out_ { *heap.unwrap_unchecked() = Some(T::from(UnknownWrapper(heap_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_query_heap<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12QueryHeapDesc) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut heap_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12QueryHeapDesc, &GUID, *mut c_void) -> HResult
				= transmute(vt[39]);
			let _ret_ = f(vt, desc, T::IID, transmute(&mut heap_out_));
			if _ret_.is_ok() { if let Some(heap_out_) = heap_out_ { return Ok(T::from(UnknownWrapper(heap_out_))); } }
			Err(_ret_)
		}
	}

	pub fn SetStablePowerState(&self, enable: bool) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>, Bool) -> HResult
				= transmute(vt[40]);
			let _ret_ = f(vt, enable.to_bool());
			_ret_.ok()
		}
	}

	pub fn CreateCommandSignature<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12CommandSignatureDesc, root_signature: Option<&D3D12RootSignature>, command_signature: Option<&mut Option<T>>) -> Result<(), HResult> {
		unsafe {
			let vt = self.as_param();
			let mut command_signature_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12CommandSignatureDesc, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[41]);
			let _ret_ = f(vt, desc, option_to_vtable(root_signature), T::IID, transmute(if command_signature_out_.is_some() { Some(&mut command_signature_out_) } else { None }));
			if let Some(command_signature_out_) = command_signature_out_ { *command_signature.unwrap_unchecked() = Some(T::from(UnknownWrapper(command_signature_out_))); }
			_ret_.ok()
		}
	}

	pub fn create_command_signature<T: IUnknown + From<UnknownWrapper>>(&self, desc: &D3D12CommandSignatureDesc, root_signature: Option<&D3D12RootSignature>) -> Result<T, HResult> {
		unsafe {
			let vt = self.as_param();
			let mut command_signature_out_: Option<Unknown> = None;
			let f: extern "system" fn(Param<Self>, &D3D12CommandSignatureDesc, *const c_void, &GUID, *mut c_void) -> HResult
				= transmute(vt[41]);
			let _ret_ = f(vt, desc, option_to_vtable(root_signature), T::IID, transmute(&mut command_signature_out_));
			if _ret_.is_ok() { if let Some(command_signature_out_) = command_signature_out_ { return Ok(T::from(UnknownWrapper(command_signature_out_))); } }
			Err(_ret_)
		}
	}

	pub unsafe fn GetResourceTiling(&self) { todo!() }

	pub fn GetAdapterLuid(&self) -> Luid {
		unsafe {
			let vt = self.as_param();
			let f: extern "system" fn(Param<Self>) -> Luid
				= transmute(vt[43]);
			let _ret_ = f(vt);
			_ret_
		}
	}
}

pub trait ID3D12Device: ID3D12Object {
	fn as_device(&self) -> &D3D12Device;
	fn into_device(self) -> D3D12Device;
}

impl ID3D12Device for D3D12Device {
	fn as_device(&self) -> &D3D12Device { self }
	fn into_device(self) -> D3D12Device { self }
}
impl ID3D12Object for D3D12Device {
	fn as_object(&self) -> &D3D12Object { &self.0.as_object() }
	fn into_object(self) -> D3D12Object { self.0.into_object() }
}

impl IUnknown for D3D12Device {
	fn as_unknown(&self) -> &Unknown { &self.0.as_unknown() }
	fn into_unknown(self) -> Unknown { self.0.into_unknown() }
}

impl From<UnknownWrapper> for D3D12Device {
    fn from(v: UnknownWrapper) -> Self {
        Self(D3D12Object::from(v))
    }
}

