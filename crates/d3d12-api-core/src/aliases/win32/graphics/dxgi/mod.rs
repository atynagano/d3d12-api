#![allow(unused_imports)]

pub mod common;

use crate::core::win32::graphics::dxgi::*;

pub type Rgba = DxgiRgba;
pub type ResourcePriority = DxgiResourcePriority;
pub type FrameStatistics = DxgiFrameStatistics;
pub type MappedRect<'a> = DxgiMappedRect<'a>;
pub type AdapterDesc = DxgiAdapterDesc;
pub type OutputDesc = DxgiOutputDesc;
pub type SharedResource = DxgiSharedResource;
pub type Residency = DxgiResidency;
pub type SurfaceDesc = DxgiSurfaceDesc;
pub type SwapEffect = DxgiSwapEffect;
pub type SwapChainFlag = DxgiSwapChainFlag;
pub type SwapChainDesc = DxgiSwapChainDesc;
pub type Object = DxgiObject;
pub trait IObject: IDxgiObject {}
impl<T: IDxgiObject> IObject for T {}
pub type DeviceSubObject = DxgiDeviceSubObject;
pub trait IDeviceSubObject: IDxgiDeviceSubObject {}
impl<T: IDxgiDeviceSubObject> IDeviceSubObject for T {}
pub type Resource = DxgiResource;
pub trait IResource: IDxgiResource {}
impl<T: IDxgiResource> IResource for T {}
pub type KeyedMutex = DxgiKeyedMutex;
pub trait IKeyedMutex: IDxgiKeyedMutex {}
impl<T: IDxgiKeyedMutex> IKeyedMutex for T {}
pub type Surface = DxgiSurface;
pub trait ISurface: IDxgiSurface {}
impl<T: IDxgiSurface> ISurface for T {}
pub type Surface1 = DxgiSurface1;
pub trait ISurface1: IDxgiSurface1 {}
impl<T: IDxgiSurface1> ISurface1 for T {}
pub type Adapter = DxgiAdapter;
pub trait IAdapter: IDxgiAdapter {}
impl<T: IDxgiAdapter> IAdapter for T {}
pub type Output = DxgiOutput;
pub trait IOutput: IDxgiOutput {}
impl<T: IDxgiOutput> IOutput for T {}
pub type SwapChain = DxgiSwapChain;
pub trait ISwapChain: IDxgiSwapChain {}
impl<T: IDxgiSwapChain> ISwapChain for T {}
pub type Factory = DxgiFactory;
pub trait IFactory: IDxgiFactory {}
impl<T: IDxgiFactory> IFactory for T {}
pub type Device = DxgiDevice;
pub trait IDevice: IDxgiDevice {}
impl<T: IDxgiDevice> IDevice for T {}
pub type AdapterFlag = DxgiAdapterFlag;
pub type AdapterDesc1 = DxgiAdapterDesc1;
pub type DisplayColorSpace = DxgiDisplayColorSpace;
pub type Factory1 = DxgiFactory1;
pub trait IFactory1: IDxgiFactory1 {}
impl<T: IDxgiFactory1> IFactory1 for T {}
pub type Adapter1 = DxgiAdapter1;
pub trait IAdapter1: IDxgiAdapter1 {}
impl<T: IDxgiAdapter1> IAdapter1 for T {}
pub type Device1 = DxgiDevice1;
pub trait IDevice1: IDxgiDevice1 {}
impl<T: IDxgiDevice1> IDevice1 for T {}
pub type DisplayControl = DxgiDisplayControl;
pub trait IDisplayControl: IDxgiDisplayControl {}
impl<T: IDxgiDisplayControl> IDisplayControl for T {}
pub type OutDuplMoveRect = DxgiOutDuplMoveRect;
pub type OutDuplDesc = DxgiOutDuplDesc;
pub type OutDuplPointerPosition = DxgiOutDuplPointerPosition;
pub type OutDuplPointerShapeType = DxgiOutDuplPointerShapeType;
pub type OutDuplPointerShapeInfo = DxgiOutDuplPointerShapeInfo;
pub type OutDuplFrameInfo = DxgiOutDuplFrameInfo;
pub type OutputDuplication = DxgiOutputDuplication;
pub trait IOutputDuplication: IDxgiOutputDuplication {}
impl<T: IDxgiOutputDuplication> IOutputDuplication for T {}
pub type Surface2 = DxgiSurface2;
pub trait ISurface2: IDxgiSurface2 {}
impl<T: IDxgiSurface2> ISurface2 for T {}
pub type Resource1 = DxgiResource1;
pub trait IResource1: IDxgiResource1 {}
impl<T: IDxgiResource1> IResource1 for T {}
pub type OfferResourcePriority = DxgiOfferResourcePriority;
pub type Device2 = DxgiDevice2;
pub trait IDevice2: IDxgiDevice2 {}
impl<T: IDxgiDevice2> IDevice2 for T {}
pub type ModeDesc1 = DxgiModeDesc1;
pub type Scaling = DxgiScaling;
pub type SwapChainDesc1 = DxgiSwapChainDesc1;
pub type SwapChainFullScreenDesc = DxgiSwapChainFullScreenDesc;
pub type PresentParameters<'a> = DxgiPresentParameters<'a>;
pub type SwapChain1 = DxgiSwapChain1;
pub trait ISwapChain1: IDxgiSwapChain1 {}
impl<T: IDxgiSwapChain1> ISwapChain1 for T {}
pub type Factory2 = DxgiFactory2;
pub trait IFactory2: IDxgiFactory2 {}
impl<T: IDxgiFactory2> IFactory2 for T {}
pub type GraphicsPreemptionGranularity = DxgiGraphicsPreemptionGranularity;
pub type ComputePreemptionGranularity = DxgiComputePreemptionGranularity;
pub type AdapterDesc2 = DxgiAdapterDesc2;
pub type Adapter2 = DxgiAdapter2;
pub trait IAdapter2: IDxgiAdapter2 {}
impl<T: IDxgiAdapter2> IAdapter2 for T {}
pub type Output1 = DxgiOutput1;
pub trait IOutput1: IDxgiOutput1 {}
impl<T: IDxgiOutput1> IOutput1 for T {}
pub type Device3 = DxgiDevice3;
pub trait IDevice3: IDxgiDevice3 {}
impl<T: IDxgiDevice3> IDevice3 for T {}
pub type Matrix3X2F = DxgiMatrix3X2F;
pub type SwapChain2 = DxgiSwapChain2;
pub trait ISwapChain2: IDxgiSwapChain2 {}
impl<T: IDxgiSwapChain2> ISwapChain2 for T {}
pub type Output2 = DxgiOutput2;
pub trait IOutput2: IDxgiOutput2 {}
impl<T: IDxgiOutput2> IOutput2 for T {}
pub type Factory3 = DxgiFactory3;
pub trait IFactory3: IDxgiFactory3 {}
impl<T: IDxgiFactory3> IFactory3 for T {}
pub type DecodeSwapChainDesc = DxgiDecodeSwapChainDesc;
pub type MultiplaneOverlayYcbcrFlags = DxgiMultiplaneOverlayYcbcrFlags;
pub type DecodeSwapChain = DxgiDecodeSwapChain;
pub trait IDecodeSwapChain: IDxgiDecodeSwapChain {}
impl<T: IDxgiDecodeSwapChain> IDecodeSwapChain for T {}
pub type FactoryMedia = DxgiFactoryMedia;
pub trait IFactoryMedia: IDxgiFactoryMedia {}
impl<T: IDxgiFactoryMedia> IFactoryMedia for T {}
pub type FramePresentationMode = DxgiFramePresentationMode;
pub type FrameStatisticsMedia = DxgiFrameStatisticsMedia;
pub type SwapChainMedia = DxgiSwapChainMedia;
pub trait ISwapChainMedia: IDxgiSwapChainMedia {}
impl<T: IDxgiSwapChainMedia> ISwapChainMedia for T {}
pub type OverlaySupportFlag = DxgiOverlaySupportFlag;
pub type Output3 = DxgiOutput3;
pub trait IOutput3: IDxgiOutput3 {}
impl<T: IDxgiOutput3> IOutput3 for T {}
pub type SwapChainColorSpaceSupportFlag = DxgiSwapChainColorSpaceSupportFlag;
pub type SwapChain3 = DxgiSwapChain3;
pub trait ISwapChain3: IDxgiSwapChain3 {}
impl<T: IDxgiSwapChain3> ISwapChain3 for T {}
pub type OverlayColorSpaceSupportFlag = DxgiOverlayColorSpaceSupportFlag;
pub type Output4 = DxgiOutput4;
pub trait IOutput4: IDxgiOutput4 {}
impl<T: IDxgiOutput4> IOutput4 for T {}
pub type Factory4 = DxgiFactory4;
pub trait IFactory4: IDxgiFactory4 {}
impl<T: IDxgiFactory4> IFactory4 for T {}
pub type MemorySegmentGroup = DxgiMemorySegmentGroup;
pub type QueryVideoMemoryInfo = DxgiQueryVideoMemoryInfo;
pub type Adapter3 = DxgiAdapter3;
pub trait IAdapter3: IDxgiAdapter3 {}
impl<T: IDxgiAdapter3> IAdapter3 for T {}
pub type OutDuplFlag = DxgiOutDuplFlag;
pub type Output5 = DxgiOutput5;
pub trait IOutput5: IDxgiOutput5 {}
impl<T: IDxgiOutput5> IOutput5 for T {}
pub type HdrMetaDataType = DxgiHdrMetaDataType;
pub type HdrMetaDataHdr10 = DxgiHdrMetaDataHdr10;
pub type HdrMetaDataHdr10Plus = DxgiHdrMetaDataHdr10Plus;
pub type SwapChain4 = DxgiSwapChain4;
pub trait ISwapChain4: IDxgiSwapChain4 {}
impl<T: IDxgiSwapChain4> ISwapChain4 for T {}
pub type OfferResourceFlags = DxgiOfferResourceFlags;
pub type ReclaimResourceResults = DxgiReclaimResourceResults;
pub type Device4 = DxgiDevice4;
pub trait IDevice4: IDxgiDevice4 {}
impl<T: IDxgiDevice4> IDevice4 for T {}
pub type Feature = DxgiFeature;
pub type Factory5 = DxgiFactory5;
pub trait IFactory5: IDxgiFactory5 {}
impl<T: IDxgiFactory5> IFactory5 for T {}
pub type AdapterFlag3 = DxgiAdapterFlag3;
pub type AdapterDesc3 = DxgiAdapterDesc3;
pub type Adapter4 = DxgiAdapter4;
pub trait IAdapter4: IDxgiAdapter4 {}
impl<T: IDxgiAdapter4> IAdapter4 for T {}
pub type OutputDesc1 = DxgiOutputDesc1;
pub type HardwareCompositionSupportFlags = DxgiHardwareCompositionSupportFlags;
pub type Output6 = DxgiOutput6;
pub trait IOutput6: IDxgiOutput6 {}
impl<T: IDxgiOutput6> IOutput6 for T {}
pub type GpuPreference = DxgiGpuPreference;
pub type Factory6 = DxgiFactory6;
pub trait IFactory6: IDxgiFactory6 {}
impl<T: IDxgiFactory6> IFactory6 for T {}
pub type Factory7 = DxgiFactory7;
pub trait IFactory7: IDxgiFactory7 {}
impl<T: IDxgiFactory7> IFactory7 for T {}
pub type DebugRLoFlags = DxgiDebugRLoFlags;
pub type InfoQueueMessageCategory = DxgiInfoQueueMessageCategory;
pub type InfoQueueMessageSeverity = DxgiInfoQueueMessageSeverity;
pub type InfoQueueMessage<'a> = DxgiInfoQueueMessage<'a>;
pub type InfoQueueFilterDesc<'a> = DxgiInfoQueueFilterDesc<'a>;
pub type InfoQueueFilter<'a> = DxgiInfoQueueFilter<'a>;
pub type InfoQueue = DxgiInfoQueue;
pub trait IInfoQueue: IDxgiInfoQueue {}
impl<T: IDxgiInfoQueue> IInfoQueue for T {}
pub type Debug = DxgiDebug;
pub trait IDebug: IDxgiDebug {}
impl<T: IDxgiDebug> IDebug for T {}
pub type Debug1 = DxgiDebug1;
pub trait IDebug1: IDxgiDebug1 {}
impl<T: IDxgiDebug1> IDebug1 for T {}
pub type MessageId = DxgiMessageId;
pub type GraphicsAnalysis = DXGraphicsAnalysis;
pub trait IGraphicsAnalysis: IDXGraphicsAnalysis {}
impl<T: IDXGraphicsAnalysis> IGraphicsAnalysis for T {}
