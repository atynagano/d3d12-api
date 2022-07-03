use crate::core::win32::graphics::direct3d12::*;

pub type CommandListType = D3D12CommandListType;
pub type CommandQueueFlags = D3D12CommandQueueFlags;
pub type CommandQueuePriority = D3D12CommandQueuePriority;
pub type CommandQueueDesc = D3D12CommandQueueDesc;
pub type PrimitiveTopologyType = D3D12PrimitiveTopologyType;
pub type InputClassification = D3D12InputClassification;
pub type InputElementDesc<'a> = D3D12InputElementDesc<'a>;
pub type FillMode = D3D12FillMode;
pub type CullMode = D3D12CullMode;
pub type SoDeclarationEntry<'a> = D3D12SoDeclarationEntry<'a>;
pub type Viewport = D3D12Viewport;
pub type ComparisonFunc = D3D12ComparisonFunc;
pub type DepthWriteMask = D3D12DepthWriteMask;
pub type StencilOp = D3D12StencilOp;
pub type DepthStencilOpDesc = D3D12DepthStencilOpDesc;
pub type DepthStencilDesc = D3D12DepthStencilDesc;
pub type DepthStencilDesc1 = D3D12DepthStencilDesc1;
pub type Blend = D3D12Blend;
pub type BlendOp = D3D12BlendOp;
pub type ColorWriteEnable = D3D12ColorWriteEnable;
pub type LogicOp = D3D12LogicOp;
pub type RenderTargetBlendDesc = D3D12RenderTargetBlendDesc;
pub type BlendDesc = D3D12BlendDesc;
pub type ConservativeRasterizationMode = D3D12ConservativeRasterizationMode;
pub type RasterizerDesc = D3D12RasterizerDesc;
pub type Object = D3D12Object;
pub trait IObject: ID3D12Object {}
impl<T: ID3D12Object> IObject for T {}
pub type DeviceChild = D3D12DeviceChild;
pub trait IDeviceChild: ID3D12DeviceChild {}
impl<T: ID3D12DeviceChild> IDeviceChild for T {}
pub type RootSignature = D3D12RootSignature;
pub trait IRootSignature: ID3D12RootSignature {}
impl<T: ID3D12RootSignature> IRootSignature for T {}
pub type ShaderByteCode<'a> = D3D12ShaderByteCode<'a>;
pub type StreamOutputDesc<'a> = D3D12StreamOutputDesc<'a>;
pub type InputLayoutDesc<'a> = D3D12InputLayoutDesc<'a>;
pub type IndexBufferStripCutValue = D3D12IndexBufferStripCutValue;
pub type CachedPipelineState<'a> = D3D12CachedPipelineState<'a>;
pub type PipelineStateFlags = D3D12PipelineStateFlags;
pub type GraphicsPipelineStateDesc<'a> = D3D12GraphicsPipelineStateDesc<'a>;
pub type ComputePipelineStateDesc<'a> = D3D12ComputePipelineStateDesc<'a>;
pub type RtFormatArray = D3D12RtFormatArray;
pub type PipelineStateStreamDesc<'a> = D3D12PipelineStateStreamDesc<'a>;
pub type PipelineStateSubobjectType = D3D12PipelineStateSubobjectType;
pub type Feature = D3D12Feature;
pub type ShaderMinPrecisionSupport = D3D12ShaderMinPrecisionSupport;
pub type TiledResourcesTier = D3D12TiledResourcesTier;
pub type ResourceBindingTier = D3D12ResourceBindingTier;
pub type ConservativeRasterizationTier = D3D12ConservativeRasterizationTier;
pub type FormatSupport1 = D3D12FormatSupport1;
pub type FormatSupport2 = D3D12FormatSupport2;
pub type MultiSampleQualityLevelFlags = D3D12MultiSampleQualityLevelFlags;
pub type CrossNodeSharingTier = D3D12CrossNodeSharingTier;
pub type ResourceHeapTier = D3D12ResourceHeapTier;
pub type ProgrammableSamplePositionsTier = D3D12ProgrammableSamplePositionsTier;
pub type ViewInstancingTier = D3D12ViewInstancingTier;
pub type FeatureDataD3D12Options = D3D12FeatureDataD3D12Options;
pub type FeatureDataD3D12Options1 = D3D12FeatureDataD3D12Options1;
pub type FeatureDataD3D12Options2 = D3D12FeatureDataD3D12Options2;
pub type RootSignatureVersion = D3DRootSignatureVersion;
pub type FeatureDataRootSignature = D3D12FeatureDataRootSignature;
pub type FeatureDataArchitecture = D3D12FeatureDataArchitecture;
pub type FeatureDataArchitecture1 = D3D12FeatureDataArchitecture1;
pub type FeatureDataFeatureLevels<'a> = D3D12FeatureDataFeatureLevels<'a>;
pub type ShaderModel = D3DShaderModel;
pub type FeatureDataShaderModel = D3D12FeatureDataShaderModel;
pub type FeatureDataFormatSupport = D3D12FeatureDataFormatSupport;
pub type FeatureDataMultiSampleQualityLevels = D3D12FeatureDataMultiSampleQualityLevels;
pub type FeatureDataFormatInfo = D3D12FeatureDataFormatInfo;
pub type FeatureDataGpuVirtualAddressSupport = D3D12FeatureDataGpuVirtualAddressSupport;
pub type ShaderCacheSupportFlags = D3D12ShaderCacheSupportFlags;
pub type FeatureDataShaderCache = D3D12FeatureDataShaderCache;
pub type FeatureDataCommandQueuePriority = D3D12FeatureDataCommandQueuePriority;
pub type CommandListSupportFlags = D3D12CommandListSupportFlags;
pub type FeatureDataD3D12Options3 = D3D12FeatureDataD3D12Options3;
pub type FeatureDataExistingHeaps = D3D12FeatureDataExistingHeaps;
pub type SharedResourceCompatibilityTier = D3D12SharedResourceCompatibilityTier;
pub type FeatureDataDisplayable = D3D12FeatureDataDisplayable;
pub type FeatureDataD3D12Options4 = D3D12FeatureDataD3D12Options4;
pub type HeapSerializationTier = D3D12HeapSerializationTier;
pub type FeatureDataSerialization = D3D12FeatureDataSerialization;
pub type FeatureDataCrossNode = D3D12FeatureDataCrossNode;
pub type RenderPassTier = D3D12RenderPassTier;
pub type RaytracingTier = D3D12RaytracingTier;
pub type FeatureDataD3D12Options5 = D3D12FeatureDataD3D12Options5;
pub type VariableShadingRateTier = D3D12VariableShadingRateTier;
pub type FeatureDataD3D12Options6 = D3D12FeatureDataD3D12Options6;
pub type MeshShaderTier = D3D12MeshShaderTier;
pub type SamplerFeedbackTier = D3D12SamplerFeedbackTier;
pub type FeatureDataD3D12Options7 = D3D12FeatureDataD3D12Options7;
pub type FeatureDataQueryMetaCommand<'a> = D3D12FeatureDataQueryMetaCommand<'a>;
pub type FeatureDataD3D12Options8 = D3D12FeatureDataD3D12Options8;
pub type WaveMMaTier = D3D12WaveMMaTier;
pub type FeatureDataD3D12Options9 = D3D12FeatureDataD3D12Options9;
pub type FeatureDataD3D12Options10 = D3D12FeatureDataD3D12Options10;
pub type FeatureDataD3D12Options11 = D3D12FeatureDataD3D12Options11;
pub type ResourceAllocationInfo = D3D12ResourceAllocationInfo;
pub type ResourceAllocationInfo1 = D3D12ResourceAllocationInfo1;
pub type HeapType = D3D12HeapType;
pub type CpuPageProperty = D3D12CpuPageProperty;
pub type MemoryPool = D3D12MemoryPool;
pub type HeapProperties = D3D12HeapProperties;
pub type HeapFlags = D3D12HeapFlags;
pub type HeapDesc = D3D12HeapDesc;
pub type ResourceDimension = D3D12ResourceDimension;
pub type TextureLayout = D3D12TextureLayout;
pub type ResourceFlags = D3D12ResourceFlags;
pub type MipRegion = D3D12MipRegion;
pub type ResourceDesc = D3D12ResourceDesc;
pub type ResourceDesc1 = D3D12ResourceDesc1;
pub type DepthStencilValue = D3D12DepthStencilValue;
pub type ClearValueAnonymousUnion = D3D12ClearValueAnonymousUnion;
pub type ClearValue = D3D12ClearValue;
pub type Range = D3D12Range;
pub type RangeUInt64 = D3D12RangeUInt64;
pub type SubresourceRangeUInt64 = D3D12SubresourceRangeUInt64;
pub type SubresourceInfo = D3D12SubresourceInfo;
pub type TiledResourceCoordinate = D3D12TiledResourceCoordinate;
pub type TileRegionSize = D3D12TileRegionSize;
pub type TileRangeFlags = D3D12TileRangeFlags;
pub type SubresourceTiling = D3D12SubresourceTiling;
pub type TileShape = D3D12TileShape;
pub type PackedMipInfo = D3D12PackedMipInfo;
pub type TileMappingFlags = D3D12TileMappingFlags;
pub type TileCopyFlags = D3D12TileCopyFlags;
pub type ResourceStates = D3D12ResourceStates;
pub type ResourceBarrierType = D3D12ResourceBarrierType;
pub type ResourceTransitionBarrier<'a> = D3D12ResourceTransitionBarrier<'a>;
pub type ResourceAliasingBarrier<'a> = D3D12ResourceAliasingBarrier<'a>;
pub type ResourceUavBarrier<'a> = D3D12ResourceUavBarrier<'a>;
pub type ResourceBarrierFlags = D3D12ResourceBarrierFlags;
pub type ResourceBarrierAnonymousUnion<'a> = D3D12ResourceBarrierAnonymousUnion<'a>;
pub type ResourceBarrier<'a> = D3D12ResourceBarrier<'a>;
pub type SubresourceFootprint = D3D12SubresourceFootprint;
pub type PlacedSubresourceFootprint = D3D12PlacedSubresourceFootprint;
pub type TextureCopyType = D3D12TextureCopyType;
pub type TextureCopyLocationAnonymousUnion = D3D12TextureCopyLocationAnonymousUnion;
pub type TextureCopyLocation<'a> = D3D12TextureCopyLocation<'a>;
pub type ResolveMode = D3D12ResolveMode;
pub type SamplePosition = D3D12SamplePosition;
pub type ViewInstanceLocation = D3D12ViewInstanceLocation;
pub type ViewInstancingFlags = D3D12ViewInstancingFlags;
pub type ViewInstancingDesc<'a> = D3D12ViewInstancingDesc<'a>;
pub type ShaderComponentMapping = D3D12ShaderComponentMapping;
pub type BufferSrvFlags = D3D12BufferSrvFlags;
pub type BufferSrv = D3D12BufferSrv;
pub type Tex1DSrv = D3D12Tex1DSrv;
pub type Tex1DArraySrv = D3D12Tex1DArraySrv;
pub type Tex2DSrv = D3D12Tex2DSrv;
pub type Tex2DArraySrv = D3D12Tex2DArraySrv;
pub type Tex3DSrv = D3D12Tex3DSrv;
pub type TexCubeSrv = D3D12TexCubeSrv;
pub type TexCubeArraySrv = D3D12TexCubeArraySrv;
pub type Tex2DMsSrv = D3D12Tex2DMsSrv;
pub type Tex2DMsArraySrv = D3D12Tex2DMsArraySrv;
pub type RaytracingAccelerationStructureSrv = D3D12RaytracingAccelerationStructureSrv;
pub type SrvDimension = D3D12SrvDimension;
pub type ShaderResourceViewDescAnonymousUnion = D3D12ShaderResourceViewDescAnonymousUnion;
pub type ShaderResourceViewDesc = D3D12ShaderResourceViewDesc;
pub type ConstantBufferViewDesc = D3D12ConstantBufferViewDesc;
pub type Filter = D3D12Filter;
pub type FilterType = D3D12FilterType;
pub type FilterReductionType = D3D12FilterReductionType;
pub type TextureAddressMode = D3D12TextureAddressMode;
pub type SamplerDesc = D3D12SamplerDesc;
pub type BufferUavFlags = D3D12BufferUavFlags;
pub type BufferUav = D3D12BufferUav;
pub type Tex1DUav = D3D12Tex1DUav;
pub type Tex1DArrayUav = D3D12Tex1DArrayUav;
pub type Tex2DUav = D3D12Tex2DUav;
pub type Tex2DArrayUav = D3D12Tex2DArrayUav;
pub type Tex3DUav = D3D12Tex3DUav;
pub type UavDimension = D3D12UavDimension;
pub type UnorderedAccessViewDescAnonymousUnion = D3D12UnorderedAccessViewDescAnonymousUnion;
pub type UnorderedAccessViewDesc = D3D12UnorderedAccessViewDesc;
pub type BufferRtv = D3D12BufferRtv;
pub type Tex1DRtv = D3D12Tex1DRtv;
pub type Tex1DArrayRtv = D3D12Tex1DArrayRtv;
pub type Tex2DRtv = D3D12Tex2DRtv;
pub type Tex2DMsRtv = D3D12Tex2DMsRtv;
pub type Tex2DArrayRtv = D3D12Tex2DArrayRtv;
pub type Tex2DMsArrayRtv = D3D12Tex2DMsArrayRtv;
pub type Tex3DRtv = D3D12Tex3DRtv;
pub type RtvDimension = D3D12RtvDimension;
pub type RenderTargetViewDescAnonymousUnion = D3D12RenderTargetViewDescAnonymousUnion;
pub type RenderTargetViewDesc = D3D12RenderTargetViewDesc;
pub type Tex1DDsv = D3D12Tex1DDsv;
pub type Tex1DArrayDsv = D3D12Tex1DArrayDsv;
pub type Tex2DDsv = D3D12Tex2DDsv;
pub type Tex2DArrayDsv = D3D12Tex2DArrayDsv;
pub type Tex2DMsDsv = D3D12Tex2DMsDsv;
pub type Tex2DMsArrayDsv = D3D12Tex2DMsArrayDsv;
pub type DsvFlags = D3D12DsvFlags;
pub type DsvDimension = D3D12DsvDimension;
pub type DepthStencilViewDescAnonymousUnion = D3D12DepthStencilViewDescAnonymousUnion;
pub type DepthStencilViewDesc = D3D12DepthStencilViewDesc;
pub type ClearFlags = D3D12ClearFlags;
pub type FenceFlags = D3D12FenceFlags;
pub type DescriptorHeapType = D3D12DescriptorHeapType;
pub type DescriptorHeapFlags = D3D12DescriptorHeapFlags;
pub type DescriptorHeapDesc = D3D12DescriptorHeapDesc;
pub type DescriptorRangeType = D3D12DescriptorRangeType;
pub type DescriptorRange = D3D12DescriptorRange;
pub type RootDescriptorTable<'a> = D3D12RootDescriptorTable<'a>;
pub type RootConstants = D3D12RootConstants;
pub type RootDescriptor = D3D12RootDescriptor;
pub type ShaderVisibility = D3D12ShaderVisibility;
pub type RootParameterType = D3D12RootParameterType;
pub type RootParameterAnonymousUnion<'a> = D3D12RootParameterAnonymousUnion<'a>;
pub type RootParameter<'a> = D3D12RootParameter<'a>;
pub type RootSignatureFlags = D3D12RootSignatureFlags;
pub type StaticBorderColor = D3D12StaticBorderColor;
pub type StaticSamplerDesc = D3D12StaticSamplerDesc;
pub type RootSignatureDesc<'a> = D3D12RootSignatureDesc<'a>;
pub type DescriptorRangeFlags = D3D12DescriptorRangeFlags;
pub type DescriptorRange1 = D3D12DescriptorRange1;
pub type RootDescriptorTable1<'a> = D3D12RootDescriptorTable1<'a>;
pub type RootDescriptorFlags = D3D12RootDescriptorFlags;
pub type RootDescriptor1 = D3D12RootDescriptor1;
pub type RootParameter1AnonymousUnion<'a> = D3D12RootParameter1AnonymousUnion<'a>;
pub type RootParameter1<'a> = D3D12RootParameter1<'a>;
pub type RootSignatureDesc1<'a> = D3D12RootSignatureDesc1<'a>;
pub type VersionedRootSignatureDescAnonymousUnion<'a> = D3D12VersionedRootSignatureDescAnonymousUnion<'a>;
pub type VersionedRootSignatureDesc<'a> = D3D12VersionedRootSignatureDesc<'a>;
pub type RootSignatureDeserializer = D3D12RootSignatureDeserializer;
pub trait IRootSignatureDeserializer: ID3D12RootSignatureDeserializer {}
impl<T: ID3D12RootSignatureDeserializer> IRootSignatureDeserializer for T {}
pub type VersionedRootSignatureDeserializer = D3D12VersionedRootSignatureDeserializer;
pub trait IVersionedRootSignatureDeserializer: ID3D12VersionedRootSignatureDeserializer {}
impl<T: ID3D12VersionedRootSignatureDeserializer> IVersionedRootSignatureDeserializer for T {}
pub type CpuDescriptorHandle = D3D12CpuDescriptorHandle;
pub type GpuDescriptorHandle = D3D12GpuDescriptorHandle;
pub type DiscardRegion<'a> = D3D12DiscardRegion<'a>;
pub type QueryHeapType = D3D12QueryHeapType;
pub type QueryHeapDesc = D3D12QueryHeapDesc;
pub type QueryType = D3D12QueryType;
pub type PredicationOp = D3D12PredicationOp;
pub type QueryDataPipelineStatistics = D3D12QueryDataPipelineStatistics;
pub type QueryDataPipelineStatistics1 = D3D12QueryDataPipelineStatistics1;
pub type QueryDataSoStatistics = D3D12QueryDataSoStatistics;
pub type StreamOutputBufferView = D3D12StreamOutputBufferView;
pub type DrawArguments = D3D12DrawArguments;
pub type DrawIndexedArguments = D3D12DrawIndexedArguments;
pub type DispatchArguments = D3D12DispatchArguments;
pub type VertexBufferView = D3D12VertexBufferView;
pub type IndexBufferView = D3D12IndexBufferView;
pub type IndirectArgumentType = D3D12IndirectArgumentType;
pub type IndirectArgumentDescAnonymousUnion = D3D12IndirectArgumentDescAnonymousUnion;
pub type IndirectArgumentDesc = D3D12IndirectArgumentDesc;
pub type CommandSignatureDesc<'a> = D3D12CommandSignatureDesc<'a>;
pub type Pageable = D3D12Pageable;
pub trait IPageable: ID3D12Pageable {}
impl<T: ID3D12Pageable> IPageable for T {}
pub type Heap = D3D12Heap;
pub trait IHeap: ID3D12Heap {}
impl<T: ID3D12Heap> IHeap for T {}
pub type Resource = D3D12Resource;
pub trait IResource: ID3D12Resource {}
impl<T: ID3D12Resource> IResource for T {}
pub type CommandAllocator = D3D12CommandAllocator;
pub trait ICommandAllocator: ID3D12CommandAllocator {}
impl<T: ID3D12CommandAllocator> ICommandAllocator for T {}
pub type Fence = D3D12Fence;
pub trait IFence: ID3D12Fence {}
impl<T: ID3D12Fence> IFence for T {}
pub type Fence1 = D3D12Fence1;
pub trait IFence1: ID3D12Fence1 {}
impl<T: ID3D12Fence1> IFence1 for T {}
pub type PipelineState = D3D12PipelineState;
pub trait IPipelineState: ID3D12PipelineState {}
impl<T: ID3D12PipelineState> IPipelineState for T {}
pub type DescriptorHeap = D3D12DescriptorHeap;
pub trait IDescriptorHeap: ID3D12DescriptorHeap {}
impl<T: ID3D12DescriptorHeap> IDescriptorHeap for T {}
pub type QueryHeap = D3D12QueryHeap;
pub trait IQueryHeap: ID3D12QueryHeap {}
impl<T: ID3D12QueryHeap> IQueryHeap for T {}
pub type CommandSignature = D3D12CommandSignature;
pub trait ICommandSignature: ID3D12CommandSignature {}
impl<T: ID3D12CommandSignature> ICommandSignature for T {}
pub type CommandList = D3D12CommandList;
pub trait ICommandList: ID3D12CommandList {}
impl<T: ID3D12CommandList> ICommandList for T {}
pub type GraphicsCommandList = D3D12GraphicsCommandList;
pub trait IGraphicsCommandList: ID3D12GraphicsCommandList {}
impl<T: ID3D12GraphicsCommandList> IGraphicsCommandList for T {}
pub type GraphicsCommandList1 = D3D12GraphicsCommandList1;
pub trait IGraphicsCommandList1: ID3D12GraphicsCommandList1 {}
impl<T: ID3D12GraphicsCommandList1> IGraphicsCommandList1 for T {}
pub type WriteBufferImmediateParameter = D3D12WriteBufferImmediateParameter;
pub type WriteBufferImmediateMode = D3D12WriteBufferImmediateMode;
pub type GraphicsCommandList2 = D3D12GraphicsCommandList2;
pub trait IGraphicsCommandList2: ID3D12GraphicsCommandList2 {}
impl<T: ID3D12GraphicsCommandList2> IGraphicsCommandList2 for T {}
pub type CommandQueue = D3D12CommandQueue;
pub trait ICommandQueue: ID3D12CommandQueue {}
impl<T: ID3D12CommandQueue> ICommandQueue for T {}
pub type Device = D3D12Device;
pub trait IDevice: ID3D12Device {}
impl<T: ID3D12Device> IDevice for T {}
pub type PipelineLibrary = D3D12PipelineLibrary;
pub trait IPipelineLibrary: ID3D12PipelineLibrary {}
impl<T: ID3D12PipelineLibrary> IPipelineLibrary for T {}
pub type PipelineLibrary1 = D3D12PipelineLibrary1;
pub trait IPipelineLibrary1: ID3D12PipelineLibrary1 {}
impl<T: ID3D12PipelineLibrary1> IPipelineLibrary1 for T {}
pub type MultipleFenceWaitFlags = D3D12MultipleFenceWaitFlags;
pub type ResidencyPriority = D3D12ResidencyPriority;
pub type Device1 = D3D12Device1;
pub trait IDevice1: ID3D12Device1 {}
impl<T: ID3D12Device1> IDevice1 for T {}
pub type Device2 = D3D12Device2;
pub trait IDevice2: ID3D12Device2 {}
impl<T: ID3D12Device2> IDevice2 for T {}
pub type ResidencyFlags = D3D12ResidencyFlags;
pub type Device3 = D3D12Device3;
pub trait IDevice3: ID3D12Device3 {}
impl<T: ID3D12Device3> IDevice3 for T {}
pub type CommandListFlags = D3D12CommandListFlags;
pub type CommandPoolFlags = D3D12CommandPoolFlags;
pub type CommandRecorderFlags = D3D12CommandRecorderFlags;
pub type ProtectedSessionStatus = D3D12ProtectedSessionStatus;
pub type ProtectedSession = D3D12ProtectedSession;
pub trait IProtectedSession: ID3D12ProtectedSession {}
impl<T: ID3D12ProtectedSession> IProtectedSession for T {}
pub type ProtectedResourceSessionSupportFlags = D3D12ProtectedResourceSessionSupportFlags;
pub type FeatureDataProtectedResourceSessionSupport = D3D12FeatureDataProtectedResourceSessionSupport;
pub type ProtectedResourceSessionFlags = D3D12ProtectedResourceSessionFlags;
pub type ProtectedResourceSessionDesc = D3D12ProtectedResourceSessionDesc;
pub type ProtectedResourceSession = D3D12ProtectedResourceSession;
pub trait IProtectedResourceSession: ID3D12ProtectedResourceSession {}
impl<T: ID3D12ProtectedResourceSession> IProtectedResourceSession for T {}
pub type Device4 = D3D12Device4;
pub trait IDevice4: ID3D12Device4 {}
impl<T: ID3D12Device4> IDevice4 for T {}
pub type LifetimeState = D3D12LifetimeState;
pub type LifetimeOwner = D3D12LifetimeOwner;
pub trait ILifetimeOwner: ID3D12LifetimeOwner {}
impl<T: ID3D12LifetimeOwner> ILifetimeOwner for T {}
pub type SwapChainAssistant = D3D12SwapChainAssistant;
pub trait ISwapChainAssistant: ID3D12SwapChainAssistant {}
impl<T: ID3D12SwapChainAssistant> ISwapChainAssistant for T {}
pub type LifetimeTracker = D3D12LifetimeTracker;
pub trait ILifetimeTracker: ID3D12LifetimeTracker {}
impl<T: ID3D12LifetimeTracker> ILifetimeTracker for T {}
pub type MetaCommandParameterType = D3D12MetaCommandParameterType;
pub type MetaCommandParameterFlags = D3D12MetaCommandParameterFlags;
pub type MetaCommandParameterStage = D3D12MetaCommandParameterStage;
pub type MetaCommandParameterDesc<'a> = D3D12MetaCommandParameterDesc<'a>;
pub type GraphicsStates = D3D12GraphicsStates;
pub type MetaCommandDesc<'a> = D3D12MetaCommandDesc<'a>;
pub type StateObject = D3D12StateObject;
pub trait IStateObject: ID3D12StateObject {}
impl<T: ID3D12StateObject> IStateObject for T {}
pub type StateObjectProperties = D3D12StateObjectProperties;
pub trait IStateObjectProperties: ID3D12StateObjectProperties {}
impl<T: ID3D12StateObjectProperties> IStateObjectProperties for T {}
pub type StateSubobjectType = D3D12StateSubobjectType;
pub type StateSubobject<'a> = D3D12StateSubobject<'a>;
pub type StateObjectFlags = D3D12StateObjectFlags;
pub type StateObjectConfig = D3D12StateObjectConfig;
pub type GlobalRootSignature<'a> = D3D12GlobalRootSignature<'a>;
pub type LocalRootSignature<'a> = D3D12LocalRootSignature<'a>;
pub type NodeMask = D3D12NodeMask;
pub type ExportFlags = D3D12ExportFlags;
pub type DxilLibraryDesc<'a> = D3D12DxilLibraryDesc<'a>;
pub type ExistingCollectionDesc<'a> = D3D12ExistingCollectionDesc<'a>;
pub type SubobjectToExportsAssociation<'a> = D3D12SubobjectToExportsAssociation<'a>;
pub type DxilSubobjectToExportsAssociation<'a> = D3D12DxilSubobjectToExportsAssociation<'a>;
pub type HitGroupType = D3D12HitGroupType;
pub type HitGroupDesc<'a> = D3D12HitGroupDesc<'a>;
pub type RaytracingShaderConfig = D3D12RaytracingShaderConfig;
pub type RaytracingPipelineConfig = D3D12RaytracingPipelineConfig;
pub type RaytracingPipelineFlags = D3D12RaytracingPipelineFlags;
pub type RaytracingPipelineConfig1 = D3D12RaytracingPipelineConfig1;
pub type StateObjectType = D3D12StateObjectType;
pub type RaytracingGeometryFlags = D3D12RaytracingGeometryFlags;
pub type RaytracingGeometryType = D3D12RaytracingGeometryType;
pub type RaytracingInstanceFlags = D3D12RaytracingInstanceFlags;
pub type GpuVirtualAddressAndStride = D3D12GpuVirtualAddressAndStride;
pub type GpuVirtualAddressRange = D3D12GpuVirtualAddressRange;
pub type GpuVirtualAddressRangeAndStride = D3D12GpuVirtualAddressRangeAndStride;
pub type RaytracingGeometryTrianglesDesc = D3D12RaytracingGeometryTrianglesDesc;
pub type RaytracingAabb = D3D12RaytracingAabb;
pub type RaytracingGeometryAabbsDesc = D3D12RaytracingGeometryAabbsDesc;
pub type RaytracingAccelerationStructureBuildFlags = D3D12RaytracingAccelerationStructureBuildFlags;
pub type RaytracingAccelerationStructureCopyMode = D3D12RaytracingAccelerationStructureCopyMode;
pub type RaytracingAccelerationStructureType = D3D12RaytracingAccelerationStructureType;
pub type ElementsLayout = D3D12ElementsLayout;
pub type RaytracingAccelerationStructurePostBuildInfoType = D3D12RaytracingAccelerationStructurePostBuildInfoType;
pub type RaytracingAccelerationStructurePostBuildInfoDesc = D3D12RaytracingAccelerationStructurePostBuildInfoDesc;
pub type RaytracingAccelerationStructurePostBuildInfoCompactedSizeDesc = D3D12RaytracingAccelerationStructurePostBuildInfoCompactedSizeDesc;
pub type RaytracingAccelerationStructurePostBuildInfoToolsVisualizationDesc = D3D12RaytracingAccelerationStructurePostBuildInfoToolsVisualizationDesc;
pub type BuildRaytracingAccelerationStructureToolsVisualizationHeader = D3D12BuildRaytracingAccelerationStructureToolsVisualizationHeader;
pub type RaytracingAccelerationStructurePostBuildInfoSerializationDesc = D3D12RaytracingAccelerationStructurePostBuildInfoSerializationDesc;
pub type SerializedDataDriverMatchingIdentifier = D3D12SerializedDataDriverMatchingIdentifier;
pub type SerializedDataType = D3D12SerializedDataType;
pub type DriverMatchingIdentifierStatus = D3D12DriverMatchingIdentifierStatus;
pub type SerializedRaytracingAccelerationStructureHeader = D3D12SerializedRaytracingAccelerationStructureHeader;
pub type RaytracingAccelerationStructurePostBuildInfoCurrentSizeDesc = D3D12RaytracingAccelerationStructurePostBuildInfoCurrentSizeDesc;
pub type RaytracingGeometryDescAnonymousUnion = D3D12RaytracingGeometryDescAnonymousUnion;
pub type RaytracingGeometryDesc = D3D12RaytracingGeometryDesc;
pub type BuildRaytracingAccelerationStructureInputs<'a> = D3D12BuildRaytracingAccelerationStructureInputs<'a>;
pub type BuildRaytracingAccelerationStructureDesc<'a> = D3D12BuildRaytracingAccelerationStructureDesc<'a>;
pub type RaytracingAccelerationStructurePrebuildInfo = D3D12RaytracingAccelerationStructurePrebuildInfo;
pub type RayFlags = D3D12RayFlags;
pub type HitKind = D3D12HitKind;
pub type Device5 = D3D12Device5;
pub trait IDevice5: ID3D12Device5 {}
impl<T: ID3D12Device5> IDevice5 for T {}
pub type AutoBreadcrumbOp = D3D12AutoBreadcrumbOp;
pub type DredVersion = D3D12DredVersion;
pub type DredFlags = D3D12DredFlags;
pub type DredEnablement = D3D12DredEnablement;
pub type DredAllocationType = D3D12DredAllocationType;
pub type DredAllocationNode<'a> = D3D12DredAllocationNode<'a>;
pub type DredAllocationNode1<'a> = D3D12DredAllocationNode1<'a>;
pub type DredPageFaultOutput<'a> = D3D12DredPageFaultOutput<'a>;
pub type DredPageFaultOutput1<'a> = D3D12DredPageFaultOutput1<'a>;
pub type DredPageFaultFlags = D3D12DredPageFaultFlags;
pub type DredDeviceState = D3D12DredDeviceState;
pub type DredPageFaultOutput2<'a> = D3D12DredPageFaultOutput2<'a>;
pub type DeviceRemovedExtendedDataSettings = D3D12DeviceRemovedExtendedDataSettings;
pub trait IDeviceRemovedExtendedDataSettings: ID3D12DeviceRemovedExtendedDataSettings {}
impl<T: ID3D12DeviceRemovedExtendedDataSettings> IDeviceRemovedExtendedDataSettings for T {}
pub type DeviceRemovedExtendedDataSettings1 = D3D12DeviceRemovedExtendedDataSettings1;
pub trait IDeviceRemovedExtendedDataSettings1: ID3D12DeviceRemovedExtendedDataSettings1 {}
impl<T: ID3D12DeviceRemovedExtendedDataSettings1> IDeviceRemovedExtendedDataSettings1 for T {}
pub type DeviceRemovedExtendedData = D3D12DeviceRemovedExtendedData;
pub trait IDeviceRemovedExtendedData: ID3D12DeviceRemovedExtendedData {}
impl<T: ID3D12DeviceRemovedExtendedData> IDeviceRemovedExtendedData for T {}
pub type DeviceRemovedExtendedData1 = D3D12DeviceRemovedExtendedData1;
pub trait IDeviceRemovedExtendedData1: ID3D12DeviceRemovedExtendedData1 {}
impl<T: ID3D12DeviceRemovedExtendedData1> IDeviceRemovedExtendedData1 for T {}
pub type DeviceRemovedExtendedData2 = D3D12DeviceRemovedExtendedData2;
pub trait IDeviceRemovedExtendedData2: ID3D12DeviceRemovedExtendedData2 {}
impl<T: ID3D12DeviceRemovedExtendedData2> IDeviceRemovedExtendedData2 for T {}
pub type BackgroundProcessingMode = D3D12BackgroundProcessingMode;
pub type MeasurementsAction = D3D12MeasurementsAction;
pub type Device6 = D3D12Device6;
pub trait IDevice6: ID3D12Device6 {}
impl<T: ID3D12Device6> IDevice6 for T {}
pub type FeatureDataProtectedResourceSessionTypeCount = D3D12FeatureDataProtectedResourceSessionTypeCount;
pub type FeatureDataProtectedResourceSessionTypes<'a> = D3D12FeatureDataProtectedResourceSessionTypes<'a>;
pub type ProtectedResourceSessionDesc1 = D3D12ProtectedResourceSessionDesc1;
pub type ProtectedResourceSession1 = D3D12ProtectedResourceSession1;
pub trait IProtectedResourceSession1: ID3D12ProtectedResourceSession1 {}
impl<T: ID3D12ProtectedResourceSession1> IProtectedResourceSession1 for T {}
pub type Device7 = D3D12Device7;
pub trait IDevice7: ID3D12Device7 {}
impl<T: ID3D12Device7> IDevice7 for T {}
pub type Device8 = D3D12Device8;
pub trait IDevice8: ID3D12Device8 {}
impl<T: ID3D12Device8> IDevice8 for T {}
pub type Resource1 = D3D12Resource1;
pub trait IResource1: ID3D12Resource1 {}
impl<T: ID3D12Resource1> IResource1 for T {}
pub type Resource2 = D3D12Resource2;
pub trait IResource2: ID3D12Resource2 {}
impl<T: ID3D12Resource2> IResource2 for T {}
pub type Heap1 = D3D12Heap1;
pub trait IHeap1: ID3D12Heap1 {}
impl<T: ID3D12Heap1> IHeap1 for T {}
pub type GraphicsCommandList3 = D3D12GraphicsCommandList3;
pub trait IGraphicsCommandList3: ID3D12GraphicsCommandList3 {}
impl<T: ID3D12GraphicsCommandList3> IGraphicsCommandList3 for T {}
pub type RenderPassBeginningAccessType = D3D12RenderPassBeginningAccessType;
pub type RenderPassBeginningAccessClearParameters = D3D12RenderPassBeginningAccessClearParameters;
pub type RenderPassBeginningAccessAnonymousUnion = D3D12RenderPassBeginningAccessAnonymousUnion;
pub type RenderPassBeginningAccess = D3D12RenderPassBeginningAccess;
pub type RenderPassEndingAccessType = D3D12RenderPassEndingAccessType;
pub type RenderPassEndingAccessResolveSubresourceParameters = D3D12RenderPassEndingAccessResolveSubresourceParameters;
pub type RenderPassEndingAccessResolveParameters<'a> = D3D12RenderPassEndingAccessResolveParameters<'a>;
pub type RenderPassEndingAccessAnonymousUnion<'a> = D3D12RenderPassEndingAccessAnonymousUnion<'a>;
pub type RenderPassEndingAccess<'a> = D3D12RenderPassEndingAccess<'a>;
pub type RenderPassRenderTargetDesc<'a> = D3D12RenderPassRenderTargetDesc<'a>;
pub type RenderPassDepthStencilDesc<'a> = D3D12RenderPassDepthStencilDesc<'a>;
pub type RenderPassFlags = D3D12RenderPassFlags;
pub type MetaCommand = D3D12MetaCommand;
pub trait IMetaCommand: ID3D12MetaCommand {}
impl<T: ID3D12MetaCommand> IMetaCommand for T {}
pub type DispatchRaysDesc = D3D12DispatchRaysDesc;
pub type GraphicsCommandList4 = D3D12GraphicsCommandList4;
pub trait IGraphicsCommandList4: ID3D12GraphicsCommandList4 {}
impl<T: ID3D12GraphicsCommandList4> IGraphicsCommandList4 for T {}
pub type ShaderCacheMode = D3D12ShaderCacheMode;
pub type ShaderCacheFlags = D3D12ShaderCacheFlags;
pub type ShaderCacheSessionDesc = D3D12ShaderCacheSessionDesc;
pub type ShaderCacheSession = D3D12ShaderCacheSession;
pub trait IShaderCacheSession: ID3D12ShaderCacheSession {}
impl<T: ID3D12ShaderCacheSession> IShaderCacheSession for T {}
pub type ShaderCacheKindFlags = D3D12ShaderCacheKindFlags;
pub type ShaderCacheControlFlags = D3D12ShaderCacheControlFlags;
pub type Device9 = D3D12Device9;
pub trait IDevice9: ID3D12Device9 {}
impl<T: ID3D12Device9> IDevice9 for T {}
pub type Tools = D3D12Tools;
pub trait ITools: ID3D12Tools {}
impl<T: ID3D12Tools> ITools for T {}
pub type SubresourceData<'a> = D3D12SubresourceData<'a>;
pub type MemcpyDest<'a> = D3D12MemcpyDest<'a>;
pub type Debug = D3D12Debug;
pub trait IDebug: ID3D12Debug {}
impl<T: ID3D12Debug> IDebug for T {}
pub type GpuBasedValidationFlags = D3D12GpuBasedValidationFlags;
pub type Debug1 = D3D12Debug1;
pub trait IDebug1: ID3D12Debug1 {}
impl<T: ID3D12Debug1> IDebug1 for T {}
pub type Debug2 = D3D12Debug2;
pub trait IDebug2: ID3D12Debug2 {}
impl<T: ID3D12Debug2> IDebug2 for T {}
pub type Debug3 = D3D12Debug3;
pub trait IDebug3: ID3D12Debug3 {}
impl<T: ID3D12Debug3> IDebug3 for T {}
pub type Debug4 = D3D12Debug4;
pub trait IDebug4: ID3D12Debug4 {}
impl<T: ID3D12Debug4> IDebug4 for T {}
pub type Debug5 = D3D12Debug5;
pub trait IDebug5: ID3D12Debug5 {}
impl<T: ID3D12Debug5> IDebug5 for T {}
pub type RldoFlags = D3D12RldoFlags;
pub type DebugDeviceParameterType = D3D12DebugDeviceParameterType;
pub type DebugFeature = D3D12DebugFeature;
pub type GpuBasedValidationShaderPatchMode = D3D12GpuBasedValidationShaderPatchMode;
pub type GpuBasedValidationPipelineStateCreateFlags = D3D12GpuBasedValidationPipelineStateCreateFlags;
pub type DebugDeviceGpuBasedValidationSettings = D3D12DebugDeviceGpuBasedValidationSettings;
pub type DebugDeviceGpuSlowdownPerformanceFactor = D3D12DebugDeviceGpuSlowdownPerformanceFactor;
pub type DebugDevice1 = D3D12DebugDevice1;
pub trait IDebugDevice1: ID3D12DebugDevice1 {}
impl<T: ID3D12DebugDevice1> IDebugDevice1 for T {}
pub type DebugDevice = D3D12DebugDevice;
pub trait IDebugDevice: ID3D12DebugDevice {}
impl<T: ID3D12DebugDevice> IDebugDevice for T {}
pub type DebugDevice2 = D3D12DebugDevice2;
pub trait IDebugDevice2: ID3D12DebugDevice2 {}
impl<T: ID3D12DebugDevice2> IDebugDevice2 for T {}
pub type DebugCommandQueue = D3D12DebugCommandQueue;
pub trait IDebugCommandQueue: ID3D12DebugCommandQueue {}
impl<T: ID3D12DebugCommandQueue> IDebugCommandQueue for T {}
pub type DebugCommandListParameterType = D3D12DebugCommandListParameterType;
pub type DebugCommandListGpuBasedValidationSettings = D3D12DebugCommandListGpuBasedValidationSettings;
pub type DebugCommandList1 = D3D12DebugCommandList1;
pub trait IDebugCommandList1: ID3D12DebugCommandList1 {}
impl<T: ID3D12DebugCommandList1> IDebugCommandList1 for T {}
pub type DebugCommandList = D3D12DebugCommandList;
pub trait IDebugCommandList: ID3D12DebugCommandList {}
impl<T: ID3D12DebugCommandList> IDebugCommandList for T {}
pub type DebugCommandList2 = D3D12DebugCommandList2;
pub trait IDebugCommandList2: ID3D12DebugCommandList2 {}
impl<T: ID3D12DebugCommandList2> IDebugCommandList2 for T {}
pub type SharingContract = D3D12SharingContract;
pub trait ISharingContract: ID3D12SharingContract {}
impl<T: ID3D12SharingContract> ISharingContract for T {}
pub type MessageCategory = D3D12MessageCategory;
pub type MessageSeverity = D3D12MessageSeverity;
pub type MessageId = D3D12MessageId;
pub type InfoQueueFilterDesc<'a> = D3D12InfoQueueFilterDesc<'a>;
pub type InfoQueueFilter<'a> = D3D12InfoQueueFilter<'a>;
pub type InfoQueue = D3D12InfoQueue;
pub trait IInfoQueue: ID3D12InfoQueue {}
impl<T: ID3D12InfoQueue> IInfoQueue for T {}
pub type MessageCallbackFlags = D3D12MessageCallbackFlags;
pub type InfoQueue1 = D3D12InfoQueue1;
pub trait IInfoQueue1: ID3D12InfoQueue1 {}
impl<T: ID3D12InfoQueue1> IInfoQueue1 for T {}
pub type SDKConfiguration = D3D12SDKConfiguration;
pub trait ISDKConfiguration: ID3D12SDKConfiguration {}
impl<T: ID3D12SDKConfiguration> ISDKConfiguration for T {}
pub type AxisShadingRate = D3D12AxisShadingRate;
pub type ShadingRate = D3D12ShadingRate;
pub type ShadingRateCombiner = D3D12ShadingRateCombiner;
pub type GraphicsCommandList5 = D3D12GraphicsCommandList5;
pub trait IGraphicsCommandList5: ID3D12GraphicsCommandList5 {}
impl<T: ID3D12GraphicsCommandList5> IGraphicsCommandList5 for T {}
pub type DispatchMeshArguments = D3D12DispatchMeshArguments;
pub type GraphicsCommandList6 = D3D12GraphicsCommandList6;
pub trait IGraphicsCommandList6: ID3D12GraphicsCommandList6 {}
impl<T: ID3D12GraphicsCommandList6> IGraphicsCommandList6 for T {}
pub type ShaderVersionType = D3D12ShaderVersionType;
pub type SignatureParameterDesc<'a> = D3D12SignatureParameterDesc<'a>;
pub type ShaderBufferDesc<'a> = D3D12ShaderBufferDesc<'a>;
pub type ShaderVariableDesc<'a> = D3D12ShaderVariableDesc<'a>;
pub type ShaderTypeDesc<'a> = D3D12ShaderTypeDesc<'a>;
pub type ShaderDesc<'a> = D3D12ShaderDesc<'a>;
pub type ShaderInputBindDesc<'a> = D3D12ShaderInputBindDesc<'a>;
pub type LibraryDesc<'a> = D3D12LibraryDesc<'a>;
pub type FunctionDesc<'a> = D3D12FunctionDesc<'a>;
pub type ParameterDesc<'a> = D3D12ParameterDesc<'a>;
