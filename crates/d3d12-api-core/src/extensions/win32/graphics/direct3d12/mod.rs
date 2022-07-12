use std::mem::transmute;
use std::ptr::{NonNull, null_mut};
use std::slice;
use crate::core::win32::foundation::HResult;
use crate::core::win32::graphics::direct3d12::*;
use crate::core::win32::system::com::{AsParam, Param};

// pub trait ID3D12ObjectEx {}
// impl<T: ID3D12Object> ID3D12ObjectEx for T {}
// pub trait ID3D12DeviceChildEx: ID3D12DeviceChild {}
// impl<T:  ID3D12DeviceChild> ID3D12DeviceChildEx for T {}
// pub trait ID3D12RootSignatureEx: ID3D12RootSignature {}
// impl<T:  ID3D12RootSignature> ID3D12RootSignatureEx for T {}
// pub trait ID3D12RootSignatureDeserializerEx: ID3D12RootSignatureDeserializer {}
// impl<T:  ID3D12RootSignatureDeserializer> ID3D12RootSignatureDeserializerEx for T {}
// pub trait ID3D12VersionedRootSignatureDeserializerEx: ID3D12VersionedRootSignatureDeserializer {}
// impl<T:  ID3D12VersionedRootSignatureDeserializer> ID3D12VersionedRootSignatureDeserializerEx for T {}
// pub trait ID3D12PageableEx: ID3D12Pageable {}
// impl<T:  ID3D12Pageable> ID3D12PageableEx for T {}
// pub trait ID3D12HeapEx: ID3D12Heap {}
// impl<T:  ID3D12Heap> ID3D12HeapEx for T {}

pub trait ID3D12ResourceEx: ID3D12Resource {
    fn map(&self, subresource: u32, read_range: Option<&D3D12Range>) -> Result<NonNull<()>, HResult>;
}

impl<T: ID3D12Resource> ID3D12ResourceEx for T {
    fn map(&self, subresource: u32, read_range: Option<&D3D12Range>) -> Result<NonNull<()>, HResult> {
        let mut out: Option<NonNull<()>> = None;
        self.Map(subresource, read_range, Some(&mut out))?;
        if let Some(out) = out {
            return Ok(out);
        }
        Err(HResult::E_FAIL)
    }
}

// pub trait ID3D12CommandAllocatorEx: ID3D12CommandAllocator {}
// impl<T:  ID3D12CommandAllocator> ID3D12CommandAllocatorEx for T {}
// pub trait ID3D12FenceEx: ID3D12Fence {}
// impl<T:  ID3D12Fence> ID3D12FenceEx for T {}
// pub trait ID3D12Fence1Ex: ID3D12Fence1 {}
// impl<T:  ID3D12Fence1> ID3D12Fence1Ex for T {}
// pub trait ID3D12PipelineStateEx: ID3D12PipelineState {}
// impl<T:  ID3D12PipelineState> ID3D12PipelineStateEx for T {}
// pub trait ID3D12DescriptorHeapEx: ID3D12DescriptorHeap {}
// impl<T: ID3D12DescriptorHeap> ID3D12DescriptorHeapEx for T {}
// pub trait ID3D12QueryHeapEx: ID3D12QueryHeap {}
// impl<T:  ID3D12QueryHeap> ID3D12QueryHeapEx for T {}
// pub trait ID3D12CommandSignatureEx: ID3D12CommandSignature {}
// impl<T:  ID3D12CommandSignature> ID3D12CommandSignatureEx for T {}
// pub trait ID3D12CommandListEx: ID3D12CommandList {}
// impl<T:  ID3D12CommandList> ID3D12CommandListEx for T {}
pub trait ID3D12GraphicsCommandListEx: ID3D12GraphicsCommandList {
    fn resource_barrier_transition_all(
        &self,
        resource: &impl ID3D12Resource,
        before: D3D12ResourceStates,
        after: D3D12ResourceStates,
    );
    // todo: fn resource_barrier_aliasing(&self);
    fn resource_barrier_uav(&self, resource: &impl ID3D12Resource);
}

impl<T: ID3D12GraphicsCommandList> ID3D12GraphicsCommandListEx for T {
    fn resource_barrier_transition_all(
        &self,
        resource: &impl ID3D12Resource,
        before: D3D12ResourceStates,
        after: D3D12ResourceStates,
    ) {
        self.ResourceBarrier(&[
            D3D12ResourceBarrier::Transition(resource, D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES, before, after)])
    }

    fn resource_barrier_uav(&self, resource: &impl ID3D12Resource) {
        self.ResourceBarrier(&[D3D12ResourceBarrier::Uav(resource)])
    }
}
// pub trait ID3D12GraphicsCommandList1Ex: ID3D12GraphicsCommandList1 {}
// impl<T:  ID3D12GraphicsCommandList1> ID3D12GraphicsCommandList1Ex for T {}
// pub trait ID3D12GraphicsCommandList2Ex: ID3D12GraphicsCommandList2 {}
// impl<T:  ID3D12GraphicsCommandList2> ID3D12GraphicsCommandList2Ex for T {}

pub trait ID3D12CommandQueueEx {
    fn execute_command_list(&self, command_list: &impl ID3D12CommandList) -> ();
}

impl<T: ID3D12CommandQueue> ID3D12CommandQueueEx for T {
    fn execute_command_list(&self, command_list: &impl ID3D12CommandList) -> () {
        self.ExecuteCommandLists(&[command_list.as_command_list().as_param()])
    }
}

pub trait ID3D12DeviceEx: ID3D12Device {
    fn create_root_sig(&self, blob: &[u8]) -> Result<D3D12RootSignature, HResult>;
    fn enable_break_on_error(&self) -> Result<(), HResult>;
    fn enable_break_on_corruption(&self) -> Result<(), HResult>;
    fn create_desc_heap_rtv<T: ID3D12DescriptorHeap>(&self, count: u32) -> Result<T, HResult>;
    fn create_desc_heap_sampler<T: ID3D12DescriptorHeap>(&self, count: u32) -> Result<T, HResult>;
    fn create_desc_heap_dsv<T: ID3D12DescriptorHeap>(&self, count: u32) -> Result<T, HResult>;
    fn create_desc_heap_cbv_srv_uav<T: ID3D12DescriptorHeap>(&self, count: u32) -> Result<T, HResult>;
}

// todo: ?Sized
impl<T: ID3D12Device + ?Sized> ID3D12DeviceEx for T {
    fn create_root_sig(&self, blob: &[u8]) -> Result<D3D12RootSignature, HResult> {
        self.CreateRootSignature(0, blob)
    }

    fn enable_break_on_error(&self) -> Result<(), HResult> {
        self.QueryInterface::<D3D12InfoQueue>()?.enable_break_on_error()
    }

    fn enable_break_on_corruption(&self) -> Result<(), HResult> {
        self.QueryInterface::<D3D12InfoQueue>()?.enable_break_on_corruption()
    }

    fn create_desc_heap_rtv<U: ID3D12DescriptorHeap>(&self, count: u32) -> Result<U, HResult> {
        self.CreateDescriptorHeap(&D3D12DescriptorHeapDesc::Rtv(count))
    }

    fn create_desc_heap_sampler<U: ID3D12DescriptorHeap>(&self, count: u32) -> Result<U, HResult> {
        self.CreateDescriptorHeap(&D3D12DescriptorHeapDesc::Sampler(count))
    }

    fn create_desc_heap_dsv<U: ID3D12DescriptorHeap>(&self, count: u32) -> Result<U, HResult> {
        self.CreateDescriptorHeap(&D3D12DescriptorHeapDesc::Dsv(count))
    }

    fn create_desc_heap_cbv_srv_uav<U: ID3D12DescriptorHeap>(&self, count: u32) -> Result<U, HResult> {
        self.CreateDescriptorHeap(&D3D12DescriptorHeapDesc::CbvSrvUav(count))
    }
}

// pub trait ID3D12PipelineLibraryEx: ID3D12PipelineLibrary {}
// impl<T:  ID3D12PipelineLibrary> ID3D12PipelineLibraryEx for T {}
// pub trait ID3D12PipelineLibrary1Ex: ID3D12PipelineLibrary1 {}
// impl<T:  ID3D12PipelineLibrary1> ID3D12PipelineLibrary1Ex for T {}
// pub trait ID3D12Device1Ex: ID3D12Device1 {}
// impl<T:  ID3D12Device1> ID3D12Device1Ex for T {}
// pub trait ID3D12Device2Ex: ID3D12Device2 {}
// impl<T:  ID3D12Device2> ID3D12Device2Ex for T {}
// pub trait ID3D12Device3Ex: ID3D12Device3 {}
// impl<T:  ID3D12Device3> ID3D12Device3Ex for T {}
// pub trait ID3D12ProtectedSessionEx: ID3D12ProtectedSession {}
// impl<T:  ID3D12ProtectedSession> ID3D12ProtectedSessionEx for T {}
// pub trait ID3D12ProtectedResourceSessionEx: ID3D12ProtectedResourceSession {}
// impl<T:  ID3D12ProtectedResourceSession> ID3D12ProtectedResourceSessionEx for T {}
// pub trait ID3D12Device4Ex: ID3D12Device4 {}
// impl<T:  ID3D12Device4> ID3D12Device4Ex for T {}
// pub trait ID3D12LifetimeOwnerEx: ID3D12LifetimeOwner {}
// impl<T:  ID3D12LifetimeOwner> ID3D12LifetimeOwnerEx for T {}
// pub trait ID3D12SwapChainAssistantEx: ID3D12SwapChainAssistant {}
// impl<T:  ID3D12SwapChainAssistant> ID3D12SwapChainAssistantEx for T {}
// pub trait ID3D12LifetimeTrackerEx: ID3D12LifetimeTracker {}
// impl<T:  ID3D12LifetimeTracker> ID3D12LifetimeTrackerEx for T {}
// pub trait ID3D12StateObjectEx: ID3D12StateObject {}
// impl<T:  ID3D12StateObject> ID3D12StateObjectEx for T {}

pub trait ID3D12StateObjectPropertiesEx {
    fn get_shader_identifier(&self, export_name: &str) -> Option<&[u8; D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES as usize]>;
}

impl<T: ID3D12StateObjectProperties> ID3D12StateObjectPropertiesEx for T {
    fn get_shader_identifier(&self, export_name: &str) -> Option<&[u8; D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES as usize]> {
        unsafe { transmute(self.GetShaderIdentifier(export_name)) }
    }
}

pub trait ID3D12Device5Ex {
    fn create_state_object(&self, obj_type: D3D12StateObjectType, objs: &[D3D12StateSubobject]) -> Result<D3D12StateObject, HResult>;
}

impl<T: ID3D12Device5> ID3D12Device5Ex for T {
    fn create_state_object(&self, obj_type: D3D12StateObjectType, objs: &[D3D12StateSubobject]) -> Result<D3D12StateObject, HResult> {
        self.CreateStateObject(&D3D12StateObjectDesc::new(obj_type, objs))
    }
}

// pub trait ID3D12DeviceRemovedExtendedDataSettingsEx: ID3D12DeviceRemovedExtendedDataSettings {}
// impl<T:  ID3D12DeviceRemovedExtendedDataSettings> ID3D12DeviceRemovedExtendedDataSettingsEx for T {}
// pub trait ID3D12DeviceRemovedExtendedDataSettings1Ex: ID3D12DeviceRemovedExtendedDataSettings1 {}
// impl<T:  ID3D12DeviceRemovedExtendedDataSettings1> ID3D12DeviceRemovedExtendedDataSettings1Ex for T {}
// pub trait ID3D12DeviceRemovedExtendedDataEx: ID3D12DeviceRemovedExtendedData {}
// impl<T:  ID3D12DeviceRemovedExtendedData> ID3D12DeviceRemovedExtendedDataEx for T {}
// pub trait ID3D12DeviceRemovedExtendedData1Ex: ID3D12DeviceRemovedExtendedData1 {}
// impl<T:  ID3D12DeviceRemovedExtendedData1> ID3D12DeviceRemovedExtendedData1Ex for T {}
// pub trait ID3D12DeviceRemovedExtendedData2Ex: ID3D12DeviceRemovedExtendedData2 {}
// impl<T:  ID3D12DeviceRemovedExtendedData2> ID3D12DeviceRemovedExtendedData2Ex for T {}
// pub trait ID3D12Device6Ex: ID3D12Device6 {}
// impl<T:  ID3D12Device6> ID3D12Device6Ex for T {}
// pub trait ID3D12ProtectedResourceSession1Ex: ID3D12ProtectedResourceSession1 {}
// impl<T:  ID3D12ProtectedResourceSession1> ID3D12ProtectedResourceSession1Ex for T {}
// pub trait ID3D12Device7Ex: ID3D12Device7 {}
// impl<T:  ID3D12Device7> ID3D12Device7Ex for T {}
// pub trait ID3D12Device8Ex: ID3D12Device8 {}
// impl<T:  ID3D12Device8> ID3D12Device8Ex for T {}
// pub trait ID3D12Resource1Ex: ID3D12Resource1 {}
// impl<T:  ID3D12Resource1> ID3D12Resource1Ex for T {}
// pub trait ID3D12Resource2Ex: ID3D12Resource2 {}
// impl<T:  ID3D12Resource2> ID3D12Resource2Ex for T {}
// pub trait ID3D12Heap1Ex: ID3D12Heap1 {}
// impl<T:  ID3D12Heap1> ID3D12Heap1Ex for T {}
// pub trait ID3D12GraphicsCommandList3Ex: ID3D12GraphicsCommandList3 {}
// impl<T:  ID3D12GraphicsCommandList3> ID3D12GraphicsCommandList3Ex for T {}
// pub trait ID3D12MetaCommandEx: ID3D12MetaCommand {}
// impl<T:  ID3D12MetaCommand> ID3D12MetaCommandEx for T {}
// pub trait ID3D12GraphicsCommandList4Ex: ID3D12GraphicsCommandList4 {}
// impl<T:  ID3D12GraphicsCommandList4> ID3D12GraphicsCommandList4Ex for T {}
// pub trait ID3D12ShaderCacheSessionEx: ID3D12ShaderCacheSession {}
// impl<T:  ID3D12ShaderCacheSession> ID3D12ShaderCacheSessionEx for T {}
// pub trait ID3D12Device9Ex: ID3D12Device9 {}
// impl<T:  ID3D12Device9> ID3D12Device9Ex for T {}
// pub trait ID3D12ToolsEx: ID3D12Tools {}
// impl<T:  ID3D12Tools> ID3D12ToolsEx for T {}
// pub trait ID3D12DebugEx: ID3D12Debug {}
// impl<T:  ID3D12Debug> ID3D12DebugEx for T {}
// pub trait ID3D12Debug1Ex: ID3D12Debug1 {}
// impl<T:  ID3D12Debug1> ID3D12Debug1Ex for T {}
// pub trait ID3D12Debug2Ex: ID3D12Debug2 {}
// impl<T:  ID3D12Debug2> ID3D12Debug2Ex for T {}
// pub trait ID3D12Debug3Ex: ID3D12Debug3 {}
// impl<T:  ID3D12Debug3> ID3D12Debug3Ex for T {}
// pub trait ID3D12Debug4Ex: ID3D12Debug4 {}
// impl<T:  ID3D12Debug4> ID3D12Debug4Ex for T {}
// pub trait ID3D12Debug5Ex: ID3D12Debug5 {}
// impl<T:  ID3D12Debug5> ID3D12Debug5Ex for T {}
// pub trait ID3D12DebugDevice1Ex: ID3D12DebugDevice1 {}
// impl<T:  ID3D12DebugDevice1> ID3D12DebugDevice1Ex for T {}
// pub trait ID3D12DebugDeviceEx: ID3D12DebugDevice {}
// impl<T:  ID3D12DebugDevice> ID3D12DebugDeviceEx for T {}
// pub trait ID3D12DebugDevice2Ex: ID3D12DebugDevice2 {}
// impl<T:  ID3D12DebugDevice2> ID3D12DebugDevice2Ex for T {}
// pub trait ID3D12DebugCommandQueueEx: ID3D12DebugCommandQueue {}
// impl<T:  ID3D12DebugCommandQueue> ID3D12DebugCommandQueueEx for T {}
// pub trait ID3D12DebugCommandList1Ex: ID3D12DebugCommandList1 {}
// impl<T:  ID3D12DebugCommandList1> ID3D12DebugCommandList1Ex for T {}
// pub trait ID3D12DebugCommandListEx: ID3D12DebugCommandList {}
// impl<T:  ID3D12DebugCommandList> ID3D12DebugCommandListEx for T {}
// pub trait ID3D12DebugCommandList2Ex: ID3D12DebugCommandList2 {}
// impl<T:  ID3D12DebugCommandList2> ID3D12DebugCommandList2Ex for T {}
// pub trait ID3D12SharingContractEx: ID3D12SharingContract {}
// impl<T:  ID3D12SharingContract> ID3D12SharingContractEx for T {}

pub struct Message {
    pub category: D3D12MessageCategory,
    pub severity: D3D12MessageSeverity,
    pub id: D3D12MessageId,
    pub description: String,
}

pub trait ID3D12InfoQueueEx {
    fn get_message(&self, index: u64) -> Result<Message, HResult>;
    fn enable_break_on_error(&self) -> Result<(), HResult>;
    fn enable_break_on_corruption(&self) -> Result<(), HResult>;
}

impl<T: ID3D12InfoQueue> ID3D12InfoQueueEx for T {
    // 5 HRESULT GetMessage([In] ulong MessageIndex, [MemorySize(BytesParamIndex = 2), Out, Optional] D3D12_MESSAGE* pMessage, [In, Out] UIntPtr* pMessageByteLength);
    // 13 HRESULT GetStorageFilter([MemorySize(BytesParamIndex = 1), Out, Optional] D3D12_INFO_QUEUE_FILTER* pFilter, [In, Out] UIntPtr* pFilterByteLength);
    // 21 HRESULT GetRetrievalFilter([MemorySize(BytesParamIndex = 1), Out, Optional] D3D12_INFO_QUEUE_FILTER* pFilter, [In, Out] UIntPtr* pFilterByteLength);

    fn get_message(&self, index: u64) -> Result<Message, HResult> {
        let vt = self.as_param();
        let f: extern "system" fn(Param<Self>, message_index: u64, p_message: *mut u8, p_message_byte_length: &mut usize) -> HResult
            = unsafe { transmute(vt[5]) };

        let mut length: usize = 0;
        f(vt, index, null_mut(), &mut length).ok()?;

        let mut vec = Vec::<u8>::with_capacity(length);
        let vec = vec.as_mut_ptr();
        f(vt, index, vec, &mut length).ok()?;

        let msg: &D3D12Message = unsafe { transmute(vec) };
        let size = msg.description_byte_length;
        let size = if size > 0 { size - 1 } else { 0 };
        let ptr: *const u8 = unsafe { transmute(msg.description) };
        let desc = String::from_utf8_lossy(unsafe { slice::from_raw_parts(ptr, size) }).to_string();

        Ok(Message {
            category: msg.category,
            severity: msg.severity,
            id: msg.id,
            description: desc,
        })
    }

    fn enable_break_on_error(&self) -> Result<(), HResult> {
        self.SetBreakOnSeverity(D3D12MessageSeverity::Error, true)
    }

    fn enable_break_on_corruption(&self) -> Result<(), HResult> {
        self.SetBreakOnSeverity(D3D12MessageSeverity::Corruption, true)
    }
}

// pub trait ID3D12InfoQueue1Ex: ID3D12InfoQueue1 {}
// impl<T:  ID3D12InfoQueue1> ID3D12InfoQueue1Ex for T {}
// pub trait ID3D12SdkConfigurationEx: ID3D12SdkConfiguration {}
// impl<T:  ID3D12SdkConfiguration> ID3D12SdkConfigurationEx for T {}
// pub trait ID3D12GraphicsCommandList5Ex: ID3D12GraphicsCommandList5 {}
// impl<T:  ID3D12GraphicsCommandList5> ID3D12GraphicsCommandList5Ex for T {}
// pub trait ID3D12GraphicsCommandList6Ex: ID3D12GraphicsCommandList6 {}
// impl<T:  ID3D12GraphicsCommandList6> ID3D12GraphicsCommandList6Ex for T {}