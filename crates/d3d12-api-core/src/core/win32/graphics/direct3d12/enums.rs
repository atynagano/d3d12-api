#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CommandListType
{
	Direct               = 0x0i32,
	Bundle               = 0x1i32,
	Compute              = 0x2i32,
	Copy                 = 0x3i32,
	VideoDecode          = 0x4i32,
	VideoProcess         = 0x5i32,
	VideoEncode          = 0x6i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CommandQueueFlags
{
	None                 = 0x0u32,
	DisableGpuTimeout    = 0x1u32,
}

impl BitOr for D3D12CommandQueueFlags {
	type Output = D3D12CommandQueueFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12CommandQueueFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12CommandQueueFlags {
    pub fn contains(self, other: D3D12CommandQueueFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CommandQueuePriority
{
	Normal               = 0x0i32,
	High                 = 0x64i32,
	GlobalRealTime       = 0x2710i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12PrimitiveTopologyType
{
	Undefined            = 0x0i32,
	Point                = 0x1i32,
	Line                 = 0x2i32,
	Triangle             = 0x3i32,
	Patch                = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12InputClassification
{
	PerVertexData        = 0x0i32,
	PerInstanceData      = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12FillMode
{
	WireFrame            = 0x2i32,
	Solid                = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CullMode
{
	None                 = 0x1i32,
	Front                = 0x2i32,
	Back                 = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ComparisonFunc
{
	Never                = 0x1i32,
	Less                 = 0x2i32,
	Equal                = 0x3i32,
	LessEqual            = 0x4i32,
	Greater              = 0x5i32,
	NotEqual             = 0x6i32,
	GreaterEqual         = 0x7i32,
	Always               = 0x8i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DepthWriteMask
{
	Zero                 = 0x0i32,
	All                  = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12StencilOp
{
	Keep                 = 0x1i32,
	Zero                 = 0x2i32,
	Replace              = 0x3i32,
	IncrSat              = 0x4i32,
	DeCrSat              = 0x5i32,
	Invert               = 0x6i32,
	Incr                 = 0x7i32,
	DeCr                 = 0x8i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12Blend
{
	Zero                 = 0x1i32,
	One                  = 0x2i32,
	SrcColor             = 0x3i32,
	InvSrcColor          = 0x4i32,
	SrcAlpha             = 0x5i32,
	InvSrcAlpha          = 0x6i32,
	DestAlpha            = 0x7i32,
	InvDestAlpha         = 0x8i32,
	DestColor            = 0x9i32,
	InvDestColor         = 0xAi32,
	SrcAlphaSat          = 0xBi32,
	BlendFactor          = 0xEi32,
	InvBlendFactor       = 0xFi32,
	Src1Color            = 0x10i32,
	InvSrc1Color         = 0x11i32,
	Src1Alpha            = 0x12i32,
	InvSrc1Alpha         = 0x13i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12BlendOp
{
	Add                  = 0x1i32,
	Subtract             = 0x2i32,
	RevSubtract          = 0x3i32,
	Min                  = 0x4i32,
	Max                  = 0x5i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ColorWriteEnable
{
	Red                  = 0x1i32,
	Green                = 0x2i32,
	Blue                 = 0x4i32,
	Alpha                = 0x8i32,
	All                  = 0xFi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12LogicOp
{
	Clear                = 0x0i32,
	Set                  = 0x1i32,
	Copy                 = 0x2i32,
	CopyInverted         = 0x3i32,
	Noop                 = 0x4i32,
	Invert               = 0x5i32,
	And                  = 0x6i32,
	NAnd                 = 0x7i32,
	Or                   = 0x8i32,
	Nor                  = 0x9i32,
	XOr                  = 0xAi32,
	Equiv                = 0xBi32,
	AndReverse           = 0xCi32,
	AndInverted          = 0xDi32,
	OrReverse            = 0xEi32,
	OrInverted           = 0xFi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ConservativeRasterizationMode
{
	Off                  = 0x0i32,
	On                   = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12IndexBufferStripCutValue
{
	Disabled             = 0x0i32,
	_0XFfFf              = 0x1i32,
	_0XFfFfFfFf          = 0x2i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12PipelineStateFlags
{
	None                 = 0x0u32,
	ToolDebug            = 0x1u32,
}

impl BitOr for D3D12PipelineStateFlags {
	type Output = D3D12PipelineStateFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12PipelineStateFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12PipelineStateFlags {
    pub fn contains(self, other: D3D12PipelineStateFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12PipelineStateSubobjectType
{
	RootSignature        = 0x0i32,
	Vs                   = 0x1i32,
	Ps                   = 0x2i32,
	Ds                   = 0x3i32,
	Hs                   = 0x4i32,
	Gs                   = 0x5i32,
	Cs                   = 0x6i32,
	StreamOutput         = 0x7i32,
	Blend                = 0x8i32,
	SampleMask           = 0x9i32,
	Rasterizer           = 0xAi32,
	DepthStencil         = 0xBi32,
	InputLayout          = 0xCi32,
	IbStripCutValue      = 0xDi32,
	PrimitiveTopology    = 0xEi32,
	RenderTargetFormats  = 0xFi32,
	DepthStencilFormat   = 0x10i32,
	SampleDesc           = 0x11i32,
	NodeMask             = 0x12i32,
	CachedPSo            = 0x13i32,
	Flags                = 0x14i32,
	DepthStencil1        = 0x15i32,
	ViewInstancing       = 0x16i32,
	As                   = 0x18i32,
	Ms                   = 0x19i32,
	MaxValid             = 0x1Ai32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12Feature
{
	D3D12Options         = 0x0i32,
	Architecture         = 0x1i32,
	FeatureLevels        = 0x2i32,
	FormatSupport        = 0x3i32,
	MultiSampleQualityLevels = 0x4i32,
	FormatInfo           = 0x5i32,
	GpuVirtualAddressSupport = 0x6i32,
	ShaderModel          = 0x7i32,
	D3D12Options1        = 0x8i32,
	ProtectedResourceSessionSupport = 0xAi32,
	RootSignature        = 0xCi32,
	Architecture1        = 0x10i32,
	D3D12Options2        = 0x12i32,
	ShaderCache          = 0x13i32,
	CommandQueuePriority = 0x14i32,
	D3D12Options3        = 0x15i32,
	ExistingHeaps        = 0x16i32,
	D3D12Options4        = 0x17i32,
	Serialization        = 0x18i32,
	CrossNode            = 0x19i32,
	D3D12Options5        = 0x1Bi32,
	Displayable          = 0x1Ci32,
	D3D12Options6        = 0x1Ei32,
	QueryMetaCommand     = 0x1Fi32,
	D3D12Options7        = 0x20i32,
	ProtectedResourceSessionTypeCount = 0x21i32,
	ProtectedResourceSessionTypes = 0x22i32,
	D3D12Options8        = 0x24i32,
	D3D12Options9        = 0x25i32,
	D3D12Options10       = 0x27i32,
	D3D12Options11       = 0x28i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderMinPrecisionSupport
{
	None                 = 0x0u32,
	_10Bit               = 0x1u32,
	_16Bit               = 0x2u32,
}

impl BitOr for D3D12ShaderMinPrecisionSupport {
	type Output = D3D12ShaderMinPrecisionSupport;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ShaderMinPrecisionSupport {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ShaderMinPrecisionSupport {
    pub fn contains(self, other: D3D12ShaderMinPrecisionSupport) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12TiledResourcesTier
{
	NotSupported         = 0x0i32,
	_1                   = 0x1i32,
	_2                   = 0x2i32,
	_3                   = 0x3i32,
	_4                   = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResourceBindingTier
{
	_1                   = 0x1i32,
	_2                   = 0x2i32,
	_3                   = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ConservativeRasterizationTier
{
	NotSupported         = 0x0i32,
	_1                   = 0x1i32,
	_2                   = 0x2i32,
	_3                   = 0x3i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12FormatSupport1
{
	None                 = 0x0u32,
	Buffer               = 0x1u32,
	IaVertexBuffer       = 0x2u32,
	IaIndexBuffer        = 0x4u32,
	SoBuffer             = 0x8u32,
	Texture1D            = 0x10u32,
	Texture2D            = 0x20u32,
	Texture3D            = 0x40u32,
	TextureCube          = 0x80u32,
	ShaderLoad           = 0x100u32,
	ShaderSample         = 0x200u32,
	ShaderSampleComparison = 0x400u32,
	ShaderSampleMonoText = 0x800u32,
	Mip                  = 0x1000u32,
	RenderTarget         = 0x4000u32,
	BLendable            = 0x8000u32,
	DepthStencil         = 0x10000u32,
	MultiSampleResolve   = 0x40000u32,
	Display              = 0x80000u32,
	CastWithinBitLayout  = 0x100000u32,
	MultiSampleRenderTarget = 0x200000u32,
	MultiSampleLoad      = 0x400000u32,
	ShaderGather         = 0x800000u32,
	BackBufferCast       = 0x1000000u32,
	TypedUnorderedAccessView = 0x2000000u32,
	ShaderGatherComparison = 0x4000000u32,
	DecoderOutput        = 0x8000000u32,
	VideoProcessorOutput = 0x10000000u32,
	VideoProcessorInput  = 0x20000000u32,
	VideoEncoder         = 0x40000000u32,
}

impl BitOr for D3D12FormatSupport1 {
	type Output = D3D12FormatSupport1;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12FormatSupport1 {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12FormatSupport1 {
    pub fn contains(self, other: D3D12FormatSupport1) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12FormatSupport2
{
	None                 = 0x0u32,
	UavAtomicAdd         = 0x1u32,
	UavAtomicBitwiseOps  = 0x2u32,
	UavAtomicCompareStoreOrCompareExchange = 0x4u32,
	UavAtomicExchange    = 0x8u32,
	UavAtomicSignedMinOrMax = 0x10u32,
	UavAtomicUnsignedMinOrMax = 0x20u32,
	UavTypedLoad         = 0x40u32,
	UavTypedStore        = 0x80u32,
	OutputMergerLogicOp  = 0x100u32,
	Tiled                = 0x200u32,
	MultiplaneOverlay    = 0x4000u32,
	SamplerFeedback      = 0x8000u32,
}

impl BitOr for D3D12FormatSupport2 {
	type Output = D3D12FormatSupport2;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12FormatSupport2 {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12FormatSupport2 {
    pub fn contains(self, other: D3D12FormatSupport2) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MultiSampleQualityLevelFlags
{
	MultiSampleQualityLevelsFlagNone = 0x0u32,
	MultiSampleQualityLevelsFlagTiledResource = 0x1u32,
}

impl BitOr for D3D12MultiSampleQualityLevelFlags {
	type Output = D3D12MultiSampleQualityLevelFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12MultiSampleQualityLevelFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12MultiSampleQualityLevelFlags {
    pub fn contains(self, other: D3D12MultiSampleQualityLevelFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CrossNodeSharingTier
{
	NotSupported         = 0x0i32,
	_1Emulated           = 0x1i32,
	_1                   = 0x2i32,
	_2                   = 0x3i32,
	_3                   = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResourceHeapTier
{
	_1                   = 0x1i32,
	_2                   = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ProgrammableSamplePositionsTier
{
	NotSupported         = 0x0i32,
	_1                   = 0x1i32,
	_2                   = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ViewInstancingTier
{
	NotSupported         = 0x0i32,
	_1                   = 0x1i32,
	_2                   = 0x2i32,
	_3                   = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DRootSignatureVersion
{
	_1                   = 0x1i32,
	_1_1                 = 0x2i32,
}

impl D3DRootSignatureVersion {
	pub const _1_0                : Self = unsafe { transmute(0x1i32) };
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DShaderModel
{
	_5_1                 = 0x51i32,
	_6_0                 = 0x60i32,
	_6_1                 = 0x61i32,
	_6_2                 = 0x62i32,
	_6_3                 = 0x63i32,
	_6_4                 = 0x64i32,
	_6_5                 = 0x65i32,
	_6_6                 = 0x66i32,
	_6_7                 = 0x67i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderCacheSupportFlags
{
	ShaderCacheSupportNone = 0x0u32,
	ShaderCacheSupportSinglePSo = 0x1u32,
	ShaderCacheSupportLibrary = 0x2u32,
	ShaderCacheSupportAutomaticInProcCache = 0x4u32,
	ShaderCacheSupportAutomaticDiskCache = 0x8u32,
	ShaderCacheSupportDriverManagedCache = 0x10u32,
	ShaderCacheSupportShaderControlClear = 0x20u32,
	ShaderCacheSupportShaderSessionDelete = 0x40u32,
}

impl BitOr for D3D12ShaderCacheSupportFlags {
	type Output = D3D12ShaderCacheSupportFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ShaderCacheSupportFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ShaderCacheSupportFlags {
    pub fn contains(self, other: D3D12ShaderCacheSupportFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CommandListSupportFlags
{
	None                 = 0x0u32,
	Direct               = 0x1u32,
	Bundle               = 0x2u32,
	Compute              = 0x4u32,
	Copy                 = 0x8u32,
	VideoDecode          = 0x10u32,
	VideoProcess         = 0x20u32,
	VideoEncode          = 0x40u32,
}

impl BitOr for D3D12CommandListSupportFlags {
	type Output = D3D12CommandListSupportFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12CommandListSupportFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12CommandListSupportFlags {
    pub fn contains(self, other: D3D12CommandListSupportFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12SharedResourceCompatibilityTier
{
	_0                   = 0x0i32,
	_1                   = 0x1i32,
	_2                   = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12HeapSerializationTier
{
	_0                   = 0x0i32,
	_10                  = 0xAi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RenderPassTier
{
	_0                   = 0x0i32,
	_1                   = 0x1i32,
	_2                   = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingTier
{
	NotSupported         = 0x0i32,
	_1_0                 = 0xAi32,
	_1_1                 = 0xBi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12VariableShadingRateTier
{
	NotSupported         = 0x0i32,
	_1                   = 0x1i32,
	_2                   = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MeshShaderTier
{
	NotSupported         = 0x0i32,
	_1                   = 0xAi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12SamplerFeedbackTier
{
	NotSupported         = 0x0i32,
	_0_9                 = 0x5Ai32,
	_1_0                 = 0x64i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12WaveMMaTier
{
	NotSupported         = 0x0i32,
	_1_0                 = 0xAi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12HeapType
{
	Default              = 0x1i32,
	Upload               = 0x2i32,
	ReadBack             = 0x3i32,
	Custom               = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CpuPageProperty
{
	Unknown              = 0x0i32,
	NotAvailable         = 0x1i32,
	WriteCombine         = 0x2i32,
	WriteBack            = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MemoryPool
{
	Unknown              = 0x0i32,
	L0                   = 0x1i32,
	L1                   = 0x2i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12HeapFlags
{
	None                 = 0x0u32,
	Shared               = 0x1u32,
	DenyBuffers          = 0x4u32,
	AllowDisplay         = 0x8u32,
	SharedCrossAdapter   = 0x20u32,
	DenyRtDsTextures     = 0x40u32,
	DenyNonRtDsTextures  = 0x80u32,
	HardwareProtected    = 0x100u32,
	AllowWriteWatch      = 0x200u32,
	AllowShaderAtomics   = 0x400u32,
	CreateNotResident    = 0x800u32,
	CreateNotZeroed      = 0x1000u32,
	AllowOnlyBuffers     = 0xC0u32,
	AllowOnlyNonRtDsTextures = 0x44u32,
	AllowOnlyRtDsTextures = 0x84u32,
}

impl D3D12HeapFlags {
	pub const AllowAllBuffersAndTextures: Self = unsafe { transmute(0x0u32) };
}

impl BitOr for D3D12HeapFlags {
	type Output = D3D12HeapFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12HeapFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12HeapFlags {
    pub fn contains(self, other: D3D12HeapFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResourceDimension
{
	Unknown              = 0x0i32,
	Buffer               = 0x1i32,
	Texture1D            = 0x2i32,
	Texture2D            = 0x3i32,
	Texture3D            = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12TextureLayout
{
	Unknown              = 0x0i32,
	RowMajor             = 0x1i32,
	_64KbUndefinedSwizzle = 0x2i32,
	_64KbStandardSwizzle = 0x3i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResourceFlags
{
	None                 = 0x0u32,
	AllowRenderTarget    = 0x1u32,
	AllowDepthStencil    = 0x2u32,
	AllowUnorderedAccess = 0x4u32,
	DenyShaderResource   = 0x8u32,
	AllowCrossAdapter    = 0x10u32,
	AllowSimultaneousAccess = 0x20u32,
	VideoDecodeReferenceOnly = 0x40u32,
	VideoEncodeReferenceOnly = 0x80u32,
}

impl BitOr for D3D12ResourceFlags {
	type Output = D3D12ResourceFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ResourceFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ResourceFlags {
    pub fn contains(self, other: D3D12ResourceFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12TileRangeFlags
{
	None                 = 0x0i32,
	Null                 = 0x1i32,
	Skip                 = 0x2i32,
	ReuseSingleTile      = 0x4i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12TileMappingFlags
{
	None                 = 0x0u32,
	NoHazard             = 0x1u32,
}

impl BitOr for D3D12TileMappingFlags {
	type Output = D3D12TileMappingFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12TileMappingFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12TileMappingFlags {
    pub fn contains(self, other: D3D12TileMappingFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12TileCopyFlags
{
	None                 = 0x0u32,
	NoHazard             = 0x1u32,
	LinearBufferToSwizzledTiledResource = 0x2u32,
	SwizzledTiledResourceToLinearBuffer = 0x4u32,
}

impl BitOr for D3D12TileCopyFlags {
	type Output = D3D12TileCopyFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12TileCopyFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12TileCopyFlags {
    pub fn contains(self, other: D3D12TileCopyFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResourceStates
{
	Common               = 0x0u32,
	VertexAndConstantBuffer = 0x1u32,
	IndexBuffer          = 0x2u32,
	RenderTarget         = 0x4u32,
	UnorderedAccess      = 0x8u32,
	DepthWrite           = 0x10u32,
	DepthRead            = 0x20u32,
	NonPixelShaderResource = 0x40u32,
	PixelShaderResource  = 0x80u32,
	StreamOut            = 0x100u32,
	IndirectArgument     = 0x200u32,
	CopyDest             = 0x400u32,
	CopySource           = 0x800u32,
	ResolveDest          = 0x1000u32,
	ResolveSource        = 0x2000u32,
	RaytracingAccelerationStructure = 0x400000u32,
	ShadingRateSource    = 0x1000000u32,
	GenericRead          = 0xAC3u32,
	AllShaderResource    = 0xC0u32,
	VideoDecodeRead      = 0x10000u32,
	VideoDecodeWrite     = 0x20000u32,
	VideoProcessRead     = 0x40000u32,
	VideoProcessWrite    = 0x80000u32,
	VideoEncodeRead      = 0x200000u32,
	VideoEncodeWrite     = 0x800000u32,
}

impl D3D12ResourceStates {
	pub const Present             : Self = unsafe { transmute(0x0u32) };
	pub const Predication         : Self = unsafe { transmute(0x200u32) };
}

impl BitOr for D3D12ResourceStates {
	type Output = D3D12ResourceStates;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ResourceStates {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ResourceStates {
    pub fn contains(self, other: D3D12ResourceStates) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResourceBarrierType
{
	Transition           = 0x0i32,
	Aliasing             = 0x1i32,
	Uav                  = 0x2i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResourceBarrierFlags
{
	None                 = 0x0u32,
	BeginOnly            = 0x1u32,
	EndOnly              = 0x2u32,
}

impl BitOr for D3D12ResourceBarrierFlags {
	type Output = D3D12ResourceBarrierFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ResourceBarrierFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ResourceBarrierFlags {
    pub fn contains(self, other: D3D12ResourceBarrierFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12TextureCopyType
{
	SubresourceIndex     = 0x0i32,
	PlacedFootprint      = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResolveMode
{
	Decompress           = 0x0i32,
	Min                  = 0x1i32,
	Max                  = 0x2i32,
	Average              = 0x3i32,
	EncodeSamplerFeedback = 0x4i32,
	DecodeSamplerFeedback = 0x5i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ViewInstancingFlags
{
	None                 = 0x0u32,
	EnableViewInstanceMasking = 0x1u32,
}

impl BitOr for D3D12ViewInstancingFlags {
	type Output = D3D12ViewInstancingFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ViewInstancingFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ViewInstancingFlags {
    pub fn contains(self, other: D3D12ViewInstancingFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderComponentMapping
{
	FromMemoryComponent0 = 0x0i32,
	FromMemoryComponent1 = 0x1i32,
	FromMemoryComponent2 = 0x2i32,
	FromMemoryComponent3 = 0x3i32,
	ForceValue0          = 0x4i32,
	ForceValue1          = 0x5i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12BufferSrvFlags
{
	None                 = 0x0u32,
	Raw                  = 0x1u32,
}

impl BitOr for D3D12BufferSrvFlags {
	type Output = D3D12BufferSrvFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12BufferSrvFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12BufferSrvFlags {
    pub fn contains(self, other: D3D12BufferSrvFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12SrvDimension
{
	Unknown              = 0x0i32,
	Buffer               = 0x1i32,
	Texture1D            = 0x2i32,
	Texture1DArray       = 0x3i32,
	Texture2D            = 0x4i32,
	Texture2DArray       = 0x5i32,
	Texture2DMs          = 0x6i32,
	Texture2DMsArray     = 0x7i32,
	Texture3D            = 0x8i32,
	TextureCube          = 0x9i32,
	TextureCubeArray     = 0xAi32,
	RaytracingAccelerationStructure = 0xBi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12Filter
{
	MinMagMipPoint       = 0x0i32,
	MinMagPointMipLinear = 0x1i32,
	MinPointMagLinearMipPoint = 0x4i32,
	MinPointMagMipLinear = 0x5i32,
	MinLinearMagMipPoint = 0x10i32,
	MinLinearMagPointMipLinear = 0x11i32,
	MinMagLinearMipPoint = 0x14i32,
	MinMagMipLinear      = 0x15i32,
	Anisotropic          = 0x55i32,
	ComparisonMinMagMipPoint = 0x80i32,
	ComparisonMinMagPointMipLinear = 0x81i32,
	ComparisonMinPointMagLinearMipPoint = 0x84i32,
	ComparisonMinPointMagMipLinear = 0x85i32,
	ComparisonMinLinearMagMipPoint = 0x90i32,
	ComparisonMinLinearMagPointMipLinear = 0x91i32,
	ComparisonMinMagLinearMipPoint = 0x94i32,
	ComparisonMinMagMipLinear = 0x95i32,
	ComparisonAnisotropic = 0xD5i32,
	MinimumMinMagMipPoint = 0x100i32,
	MinimumMinMagPointMipLinear = 0x101i32,
	MinimumMinPointMagLinearMipPoint = 0x104i32,
	MinimumMinPointMagMipLinear = 0x105i32,
	MinimumMinLinearMagMipPoint = 0x110i32,
	MinimumMinLinearMagPointMipLinear = 0x111i32,
	MinimumMinMagLinearMipPoint = 0x114i32,
	MinimumMinMagMipLinear = 0x115i32,
	MinimumAnisotropic   = 0x155i32,
	MaximumMinMagMipPoint = 0x180i32,
	MaximumMinMagPointMipLinear = 0x181i32,
	MaximumMinPointMagLinearMipPoint = 0x184i32,
	MaximumMinPointMagMipLinear = 0x185i32,
	MaximumMinLinearMagMipPoint = 0x190i32,
	MaximumMinLinearMagPointMipLinear = 0x191i32,
	MaximumMinMagLinearMipPoint = 0x194i32,
	MaximumMinMagMipLinear = 0x195i32,
	MaximumAnisotropic   = 0x1D5i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12FilterType
{
	Point                = 0x0i32,
	Linear               = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12FilterReductionType
{
	Standard             = 0x0i32,
	Comparison           = 0x1i32,
	Minimum              = 0x2i32,
	Maximum              = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12TextureAddressMode
{
	Wrap                 = 0x1i32,
	Mirror               = 0x2i32,
	Clamp                = 0x3i32,
	Border               = 0x4i32,
	MirrorOnce           = 0x5i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12BufferUavFlags
{
	None                 = 0x0u32,
	Raw                  = 0x1u32,
}

impl BitOr for D3D12BufferUavFlags {
	type Output = D3D12BufferUavFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12BufferUavFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12BufferUavFlags {
    pub fn contains(self, other: D3D12BufferUavFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12UavDimension
{
	Unknown              = 0x0i32,
	Buffer               = 0x1i32,
	Texture1D            = 0x2i32,
	Texture1DArray       = 0x3i32,
	Texture2D            = 0x4i32,
	Texture2DArray       = 0x5i32,
	Texture3D            = 0x8i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RtvDimension
{
	Unknown              = 0x0i32,
	Buffer               = 0x1i32,
	Texture1D            = 0x2i32,
	Texture1DArray       = 0x3i32,
	Texture2D            = 0x4i32,
	Texture2DArray       = 0x5i32,
	Texture2DMs          = 0x6i32,
	Texture2DMsArray     = 0x7i32,
	Texture3D            = 0x8i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DsvFlags
{
	None                 = 0x0u32,
	ReadOnlyDepth        = 0x1u32,
	ReadOnlyStencil      = 0x2u32,
}

impl BitOr for D3D12DsvFlags {
	type Output = D3D12DsvFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12DsvFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12DsvFlags {
    pub fn contains(self, other: D3D12DsvFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DsvDimension
{
	Unknown              = 0x0i32,
	Texture1D            = 0x1i32,
	Texture1DArray       = 0x2i32,
	Texture2D            = 0x3i32,
	Texture2DArray       = 0x4i32,
	Texture2DMs          = 0x5i32,
	Texture2DMsArray     = 0x6i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ClearFlags
{
	Depth                = 0x1u32,
	Stencil              = 0x2u32,
}

impl BitOr for D3D12ClearFlags {
	type Output = D3D12ClearFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ClearFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ClearFlags {
    pub fn contains(self, other: D3D12ClearFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12FenceFlags
{
	None                 = 0x0u32,
	Shared               = 0x1u32,
	SharedCrossAdapter   = 0x2u32,
	NonMonitored         = 0x4u32,
}

impl BitOr for D3D12FenceFlags {
	type Output = D3D12FenceFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12FenceFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12FenceFlags {
    pub fn contains(self, other: D3D12FenceFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DescriptorHeapType
{
	CbvSrvUav            = 0x0i32,
	Sampler              = 0x1i32,
	Rtv                  = 0x2i32,
	Dsv                  = 0x3i32,
	NumTypes             = 0x4i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DescriptorHeapFlags
{
	None                 = 0x0u32,
	ShaderVisible        = 0x1u32,
}

impl BitOr for D3D12DescriptorHeapFlags {
	type Output = D3D12DescriptorHeapFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12DescriptorHeapFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12DescriptorHeapFlags {
    pub fn contains(self, other: D3D12DescriptorHeapFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DescriptorRangeType
{
	Srv                  = 0x0i32,
	Uav                  = 0x1i32,
	Cbv                  = 0x2i32,
	Sampler              = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderVisibility
{
	All                  = 0x0i32,
	Vertex               = 0x1i32,
	Hull                 = 0x2i32,
	Domain               = 0x3i32,
	Geometry             = 0x4i32,
	Pixel                = 0x5i32,
	Amplification        = 0x6i32,
	Mesh                 = 0x7i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RootParameterType
{
	DescriptorTable      = 0x0i32,
	_32BitConstants      = 0x1i32,
	Cbv                  = 0x2i32,
	Srv                  = 0x3i32,
	Uav                  = 0x4i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RootSignatureFlags
{
	None                 = 0x0u32,
	AllowInputAssemblerInputLayout = 0x1u32,
	DenyVertexShaderRootAccess = 0x2u32,
	DenyHullShaderRootAccess = 0x4u32,
	DenyDomainShaderRootAccess = 0x8u32,
	DenyGeometryShaderRootAccess = 0x10u32,
	DenyPixelShaderRootAccess = 0x20u32,
	AllowStreamOutput    = 0x40u32,
	LocalRootSignature   = 0x80u32,
	DenyAmplificationShaderRootAccess = 0x100u32,
	DenyMeshShaderRootAccess = 0x200u32,
	CbvSrvUavHeapDirectlyIndexed = 0x400u32,
	SamplerHeapDirectlyIndexed = 0x800u32,
}

impl BitOr for D3D12RootSignatureFlags {
	type Output = D3D12RootSignatureFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12RootSignatureFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12RootSignatureFlags {
    pub fn contains(self, other: D3D12RootSignatureFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12StaticBorderColor
{
	TransparentBlack     = 0x0i32,
	OpaqueBlack          = 0x1i32,
	OpaqueWhite          = 0x2i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DescriptorRangeFlags
{
	None                 = 0x0u32,
	DescriptorsVolatile  = 0x1u32,
	DataVolatile         = 0x2u32,
	DataStaticWhileSetAtExecute = 0x4u32,
	DataStatic           = 0x8u32,
	DescriptorsStaticKeepingBufferBoundsChecks = 0x10000u32,
}

impl BitOr for D3D12DescriptorRangeFlags {
	type Output = D3D12DescriptorRangeFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12DescriptorRangeFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12DescriptorRangeFlags {
    pub fn contains(self, other: D3D12DescriptorRangeFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RootDescriptorFlags
{
	None                 = 0x0u32,
	DataVolatile         = 0x2u32,
	DataStaticWhileSetAtExecute = 0x4u32,
	DataStatic           = 0x8u32,
}

impl BitOr for D3D12RootDescriptorFlags {
	type Output = D3D12RootDescriptorFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12RootDescriptorFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12RootDescriptorFlags {
    pub fn contains(self, other: D3D12RootDescriptorFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12QueryHeapType
{
	Occlusion            = 0x0i32,
	Timestamp            = 0x1i32,
	PipelineStatistics   = 0x2i32,
	SoStatistics         = 0x3i32,
	VideoDecodeStatistics = 0x4i32,
	CopyQueueTimestamp   = 0x5i32,
	PipelineStatistics1  = 0x7i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12QueryType
{
	Occlusion            = 0x0i32,
	BinaryOcclusion      = 0x1i32,
	Timestamp            = 0x2i32,
	PipelineStatistics   = 0x3i32,
	SoStatisticsStream0  = 0x4i32,
	SoStatisticsStream1  = 0x5i32,
	SoStatisticsStream2  = 0x6i32,
	SoStatisticsStream3  = 0x7i32,
	VideoDecodeStatistics = 0x8i32,
	PipelineStatistics1  = 0xAi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12PredicationOp
{
	EqualZero            = 0x0i32,
	NotEqualZero         = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12IndirectArgumentType
{
	Draw                 = 0x0i32,
	DrawIndexed          = 0x1i32,
	Dispatch             = 0x2i32,
	VertexBufferView     = 0x3i32,
	IndexBufferView      = 0x4i32,
	Constant             = 0x5i32,
	ConstantBufferView   = 0x6i32,
	ShaderResourceView   = 0x7i32,
	UnorderedAccessView  = 0x8i32,
	DispatchRays         = 0x9i32,
	DispatchMesh         = 0xAi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12WriteBufferImmediateMode
{
	Default              = 0x0i32,
	MarkerIn             = 0x1i32,
	MarkerOut            = 0x2i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MultipleFenceWaitFlags
{
	None                 = 0x0u32,
	Any                  = 0x1u32,
}

impl D3D12MultipleFenceWaitFlags {
	pub const All                 : Self = unsafe { transmute(0x0u32) };
}

impl BitOr for D3D12MultipleFenceWaitFlags {
	type Output = D3D12MultipleFenceWaitFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12MultipleFenceWaitFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12MultipleFenceWaitFlags {
    pub fn contains(self, other: D3D12MultipleFenceWaitFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResidencyPriority
{
	Minimum              = 0x28000000i32,
	Low                  = 0x50000000i32,
	Normal               = 0x78000000i32,
	High                 = -0x5FFF0000i32,
	Maximum              = -0x38000000i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ResidencyFlags
{
	None                 = 0x0u32,
	DenyOverBudget       = 0x1u32,
}

impl BitOr for D3D12ResidencyFlags {
	type Output = D3D12ResidencyFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ResidencyFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ResidencyFlags {
    pub fn contains(self, other: D3D12ResidencyFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CommandListFlags
{
	None                 = 0x0u32,
}

impl BitOr for D3D12CommandListFlags {
	type Output = D3D12CommandListFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12CommandListFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12CommandListFlags {
    pub fn contains(self, other: D3D12CommandListFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CommandPoolFlags
{
	None                 = 0x0u32,
}

impl BitOr for D3D12CommandPoolFlags {
	type Output = D3D12CommandPoolFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12CommandPoolFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12CommandPoolFlags {
    pub fn contains(self, other: D3D12CommandPoolFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12CommandRecorderFlags
{
	None                 = 0x0u32,
}

impl BitOr for D3D12CommandRecorderFlags {
	type Output = D3D12CommandRecorderFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12CommandRecorderFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12CommandRecorderFlags {
    pub fn contains(self, other: D3D12CommandRecorderFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ProtectedSessionStatus
{
	Ok                   = 0x0i32,
	Invalid              = 0x1i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ProtectedResourceSessionSupportFlags
{
	None                 = 0x0u32,
	Supported            = 0x1u32,
}

impl BitOr for D3D12ProtectedResourceSessionSupportFlags {
	type Output = D3D12ProtectedResourceSessionSupportFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ProtectedResourceSessionSupportFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ProtectedResourceSessionSupportFlags {
    pub fn contains(self, other: D3D12ProtectedResourceSessionSupportFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ProtectedResourceSessionFlags
{
	None                 = 0x0u32,
}

impl BitOr for D3D12ProtectedResourceSessionFlags {
	type Output = D3D12ProtectedResourceSessionFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ProtectedResourceSessionFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ProtectedResourceSessionFlags {
    pub fn contains(self, other: D3D12ProtectedResourceSessionFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12LifetimeState
{
	InUse                = 0x0i32,
	NotInUse             = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MetaCommandParameterType
{
	Float                = 0x0i32,
	UInt64               = 0x1i32,
	GpuVirtualAddress    = 0x2i32,
	CpuDescriptorHandleHeapTypeCbvSrvUav = 0x3i32,
	GpuDescriptorHandleHeapTypeCbvSrvUav = 0x4i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MetaCommandParameterFlags
{
	Input                = 0x1u32,
	Output               = 0x2u32,
}

impl BitOr for D3D12MetaCommandParameterFlags {
	type Output = D3D12MetaCommandParameterFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12MetaCommandParameterFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12MetaCommandParameterFlags {
    pub fn contains(self, other: D3D12MetaCommandParameterFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MetaCommandParameterStage
{
	Creation             = 0x0i32,
	Initialization       = 0x1i32,
	Execution            = 0x2i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12GraphicsStates
{
	None                 = 0x0u32,
	IaVertexBuffers      = 0x1u32,
	IaIndexBuffer        = 0x2u32,
	IaPrimitiveTopology  = 0x4u32,
	DescriptorHeap       = 0x8u32,
	GraphicsRootSignature = 0x10u32,
	ComputeRootSignature = 0x20u32,
	RsViewPorts          = 0x40u32,
	RsScissorReCts       = 0x80u32,
	Predication          = 0x100u32,
	OmRenderTargets      = 0x200u32,
	OmStencilRef         = 0x400u32,
	OmBlendFactor        = 0x800u32,
	PipelineState        = 0x1000u32,
	SoTargets            = 0x2000u32,
	OmDepthBounds        = 0x4000u32,
	SamplePositions      = 0x8000u32,
	ViewInstanceMask     = 0x10000u32,
}

impl BitOr for D3D12GraphicsStates {
	type Output = D3D12GraphicsStates;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12GraphicsStates {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12GraphicsStates {
    pub fn contains(self, other: D3D12GraphicsStates) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12StateSubobjectType
{
	StateObjectConfig    = 0x0i32,
	GlobalRootSignature  = 0x1i32,
	LocalRootSignature   = 0x2i32,
	NodeMask             = 0x3i32,
	DxilLibrary          = 0x5i32,
	ExistingCollection   = 0x6i32,
	SubobjectToExportsAssociation = 0x7i32,
	DxilSubobjectToExportsAssociation = 0x8i32,
	RaytracingShaderConfig = 0x9i32,
	RaytracingPipelineConfig = 0xAi32,
	HitGroup             = 0xBi32,
	RaytracingPipelineConfig1 = 0xCi32,
	MaxValid             = 0xDi32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12StateObjectFlags
{
	None                 = 0x0u32,
	AllowLocalDependenciesOnExternalDefinitions = 0x1u32,
	AllowExternalDependenciesOnLocalDefinitions = 0x2u32,
	AllowStateObjectAdditions = 0x4u32,
}

impl BitOr for D3D12StateObjectFlags {
	type Output = D3D12StateObjectFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12StateObjectFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12StateObjectFlags {
    pub fn contains(self, other: D3D12StateObjectFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ExportFlags
{
	None                 = 0x0u32,
}

impl BitOr for D3D12ExportFlags {
	type Output = D3D12ExportFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ExportFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ExportFlags {
    pub fn contains(self, other: D3D12ExportFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12HitGroupType
{
	Triangles            = 0x0i32,
	ProceduralPrimitive  = 0x1i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingPipelineFlags
{
	None                 = 0x0u32,
	SkipTriangles        = 0x100u32,
	SkipProceduralPrimitives = 0x200u32,
}

impl BitOr for D3D12RaytracingPipelineFlags {
	type Output = D3D12RaytracingPipelineFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12RaytracingPipelineFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12RaytracingPipelineFlags {
    pub fn contains(self, other: D3D12RaytracingPipelineFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12StateObjectType
{
	Collection           = 0x0i32,
	RaytracingPipeline   = 0x3i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingGeometryFlags
{
	None                 = 0x0u32,
	Opaque               = 0x1u32,
	NoDuplicateAnyHitInvocation = 0x2u32,
}

impl BitOr for D3D12RaytracingGeometryFlags {
	type Output = D3D12RaytracingGeometryFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12RaytracingGeometryFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12RaytracingGeometryFlags {
    pub fn contains(self, other: D3D12RaytracingGeometryFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingGeometryType
{
	Triangles            = 0x0i32,
	ProceduralPrimitiveAabbs = 0x1i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingInstanceFlags
{
	None                 = 0x0u32,
	TriangleCullDisable  = 0x1u32,
	TriangleFrontCounterclockwise = 0x2u32,
	ForceOpaque          = 0x4u32,
	ForceNonOpaque       = 0x8u32,
}

impl BitOr for D3D12RaytracingInstanceFlags {
	type Output = D3D12RaytracingInstanceFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12RaytracingInstanceFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12RaytracingInstanceFlags {
    pub fn contains(self, other: D3D12RaytracingInstanceFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingAccelerationStructureBuildFlags
{
	None                 = 0x0u32,
	AllowUpdate          = 0x1u32,
	AllowCompaction      = 0x2u32,
	PreferFastTrace      = 0x4u32,
	PreferFastBuild      = 0x8u32,
	MinimizeMemory       = 0x10u32,
	PerformUpdate        = 0x20u32,
}

impl BitOr for D3D12RaytracingAccelerationStructureBuildFlags {
	type Output = D3D12RaytracingAccelerationStructureBuildFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12RaytracingAccelerationStructureBuildFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12RaytracingAccelerationStructureBuildFlags {
    pub fn contains(self, other: D3D12RaytracingAccelerationStructureBuildFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingAccelerationStructureCopyMode
{
	Clone                = 0x0i32,
	Compact              = 0x1i32,
	VisualizationDecodeForTools = 0x2i32,
	Serialize            = 0x3i32,
	DeSerialize          = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingAccelerationStructureType
{
	TopLevel             = 0x0i32,
	BottomLevel          = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ElementsLayout
{
	Array                = 0x0i32,
	ArrayOfPointers      = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RaytracingAccelerationStructurePostBuildInfoType
{
	RaytracingAccelerationStructurePostBuildInfoCompactedSize = 0x0i32,
	RaytracingAccelerationStructurePostBuildInfoToolsVisualization = 0x1i32,
	RaytracingAccelerationStructurePostBuildInfoSerialization = 0x2i32,
	RaytracingAccelerationStructurePostBuildInfoCurrentSize = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12SerializedDataType
{
	SerializedDataRaytracingAccelerationStructure = 0x0i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DriverMatchingIdentifierStatus
{
	DriverMatchingIdentifierCompatibleWithDevice = 0x0i32,
	DriverMatchingIdentifierUnsupportedType = 0x1i32,
	DriverMatchingIdentifierUnrecognized = 0x2i32,
	DriverMatchingIdentifierIncompatibleVersion = 0x3i32,
	DriverMatchingIdentifierIncompatibleType = 0x4i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RayFlags
{
	None                 = 0x0u32,
	ForceOpaque          = 0x1u32,
	ForceNonOpaque       = 0x2u32,
	AcceptFirstHitAndEndSearch = 0x4u32,
	SkipClosestHitShader = 0x8u32,
	CullBackFacingTriangles = 0x10u32,
	CullFrontFacingTriangles = 0x20u32,
	CullOpaque           = 0x40u32,
	CullNonOpaque        = 0x80u32,
	SkipTriangles        = 0x100u32,
	SkipProceduralPrimitives = 0x200u32,
}

impl BitOr for D3D12RayFlags {
	type Output = D3D12RayFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12RayFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12RayFlags {
    pub fn contains(self, other: D3D12RayFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12HitKind
{
	TriangleFrontFace    = 0xFEi32,
	TriangleBackFace     = 0xFFi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12AutoBreadcrumbOp
{
	SetMarker            = 0x0i32,
	BeginEvent           = 0x1i32,
	EndEvent             = 0x2i32,
	DrawInstanced        = 0x3i32,
	DrawIndexedInstanced = 0x4i32,
	ExecuteIndirect      = 0x5i32,
	Dispatch             = 0x6i32,
	CopyBufferRegion     = 0x7i32,
	CopyTextureRegion    = 0x8i32,
	CopyResource         = 0x9i32,
	CopyTiles            = 0xAi32,
	ResolveSubresource   = 0xBi32,
	ClearRenderTargetView = 0xCi32,
	ClearUnorderedAccessView = 0xDi32,
	ClearDepthStencilView = 0xEi32,
	ResourceBarrier      = 0xFi32,
	ExecuteBundle        = 0x10i32,
	Present              = 0x11i32,
	ResolveQueryData     = 0x12i32,
	BegInsubmission      = 0x13i32,
	EndSubmission        = 0x14i32,
	DecodeFrame          = 0x15i32,
	ProcessFrames        = 0x16i32,
	AtomicCopyBuFFeruInt = 0x17i32,
	AtomicCopyBuFFeruInt64 = 0x18i32,
	ResolveSubresourceRegion = 0x19i32,
	WriteBufferImmediate = 0x1Ai32,
	DecodeFrame1         = 0x1Bi32,
	SetProtectedResourceSession = 0x1Ci32,
	DecodeFrame2         = 0x1Di32,
	ProcessFrames1       = 0x1Ei32,
	BuildRaytracingAccelerationStructure = 0x1Fi32,
	EmitRaytracingAccelerationStrUcTurEposTbUiLdinfo = 0x20i32,
	CopyRaytracingAccelerationStructure = 0x21i32,
	DispatchRays         = 0x22i32,
	InitializeMetaCommand = 0x23i32,
	ExecuteMetaCommand   = 0x24i32,
	EsTiMatEmotion       = 0x25i32,
	ReSolvEmotionVectorHeap = 0x26i32,
	SetPipeLinEstate1    = 0x27i32,
	InitializeExtensionCommand = 0x28i32,
	ExecuteExtensionCommand = 0x29i32,
	DispatchMesh         = 0x2Ai32,
	EncodeFrame          = 0x2Bi32,
	ResolveEncoderOutputMetaData = 0x2Ci32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DredVersion
{
	_1_0                 = 0x1i32,
	_1_1                 = 0x2i32,
	_1_2                 = 0x3i32,
	_1_3                 = 0x4i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DredFlags
{
	None                 = 0x0u32,
	ForceEnable          = 0x1u32,
	DisableAutoBreadcrumbs = 0x2u32,
}

impl BitOr for D3D12DredFlags {
	type Output = D3D12DredFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12DredFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12DredFlags {
    pub fn contains(self, other: D3D12DredFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DredEnablement
{
	SystemControlled     = 0x0i32,
	ForcedOff            = 0x1i32,
	ForcedOn             = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DredAllocationType
{
	CommandQueue         = 0x13i32,
	CommandAllocator     = 0x14i32,
	PipelineState        = 0x15i32,
	CommandList          = 0x16i32,
	Fence                = 0x17i32,
	DescriptorHeap       = 0x18i32,
	Heap                 = 0x19i32,
	QueryHeap            = 0x1Bi32,
	CommandSignature     = 0x1Ci32,
	PipelineLibrary      = 0x1Di32,
	VideoDecoder         = 0x1Ei32,
	VideoProcessor       = 0x20i32,
	Resource             = 0x22i32,
	Pass                 = 0x23i32,
	CryptoSession        = 0x24i32,
	CryptoSessionPolicy  = 0x25i32,
	ProtectedResourceSession = 0x26i32,
	VideoDecoderHeap     = 0x27i32,
	CommandPool          = 0x28i32,
	CommandRecorder      = 0x29i32,
	StateObject          = 0x2Ai32,
	MetaCommand          = 0x2Bi32,
	SchedulingGroup      = 0x2Ci32,
	VideoMotionEstimator = 0x2Di32,
	VideoMotionVectorHeap = 0x2Ei32,
	VideoExtensionCommand = 0x2Fi32,
	VideoEncoder         = 0x30i32,
	VideoEncoderHeap     = 0x31i32,
	Invalid              = -0x1i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DredPageFaultFlags
{
	None                 = 0x0u32,
}

impl BitOr for D3D12DredPageFaultFlags {
	type Output = D3D12DredPageFaultFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12DredPageFaultFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12DredPageFaultFlags {
    pub fn contains(self, other: D3D12DredPageFaultFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DredDeviceState
{
	Unknown              = 0x0i32,
	Hung                 = 0x3i32,
	Fault                = 0x6i32,
	PageFault            = 0x7i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12BackgroundProcessingMode
{
	Allowed              = 0x0i32,
	AllowIntrusiveMeasurements = 0x1i32,
	DisableBackgroundWork = 0x2i32,
	DisableProfilingBySystem = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MeasurementsAction
{
	KeepAll              = 0x0i32,
	CommitResults        = 0x1i32,
	CommitResultsHighPriority = 0x2i32,
	DiscardPrevious      = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RenderPassBeginningAccessType
{
	Discard              = 0x0i32,
	Preserve             = 0x1i32,
	Clear                = 0x2i32,
	NoAccess             = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RenderPassEndingAccessType
{
	Discard              = 0x0i32,
	Preserve             = 0x1i32,
	Resolve              = 0x2i32,
	NoAccess             = 0x3i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RenderPassFlags
{
	None                 = 0x0u32,
	AllowUavWrites       = 0x1u32,
	SuspendingPass       = 0x2u32,
	ResumingPass         = 0x4u32,
}

impl BitOr for D3D12RenderPassFlags {
	type Output = D3D12RenderPassFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12RenderPassFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12RenderPassFlags {
    pub fn contains(self, other: D3D12RenderPassFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderCacheMode
{
	Memory               = 0x0i32,
	Disk                 = 0x1i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderCacheFlags
{
	None                 = 0x0u32,
	DriverVersioned      = 0x1u32,
	UseWorkingDir        = 0x2u32,
}

impl BitOr for D3D12ShaderCacheFlags {
	type Output = D3D12ShaderCacheFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ShaderCacheFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ShaderCacheFlags {
    pub fn contains(self, other: D3D12ShaderCacheFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderCacheKindFlags
{
	ImplicitD3DCacheForDriver = 0x1u32,
	ImplicitD3DConversions = 0x2u32,
	ImplicitDriverManaged = 0x4u32,
	ApplicationManaged   = 0x8u32,
}

impl BitOr for D3D12ShaderCacheKindFlags {
	type Output = D3D12ShaderCacheKindFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ShaderCacheKindFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ShaderCacheKindFlags {
    pub fn contains(self, other: D3D12ShaderCacheKindFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderCacheControlFlags
{
	Disable              = 0x1u32,
	Enable               = 0x2u32,
	Clear                = 0x4u32,
}

impl BitOr for D3D12ShaderCacheControlFlags {
	type Output = D3D12ShaderCacheControlFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for D3D12ShaderCacheControlFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl D3D12ShaderCacheControlFlags {
    pub fn contains(self, other: D3D12ShaderCacheControlFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12GpuBasedValidationFlags
{
	None                 = 0x0i32,
	DisableStateTracking = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12RldoFlags
{
	RldoNone             = 0x0i32,
	RldoSummary          = 0x1i32,
	RldoDetail           = 0x2i32,
	RldoIgnoreInternal   = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DebugDeviceParameterType
{
	DebugDeviceParameterFeatureFlags = 0x0i32,
	DebugDeviceParameterGpuBasedValidationSettings = 0x1i32,
	DebugDeviceParameterGpuSlowdownPerformanceFactor = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DebugFeature
{
	None                 = 0x0i32,
	AllowBehaviorChangingDebugAids = 0x1i32,
	ConservativeResourceStateTracking = 0x2i32,
	DisableVIrTuaLiZedBundlesValidation = 0x4i32,
	EmulateWindows7      = 0x8i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12GpuBasedValidationShaderPatchMode
{
	None                 = 0x0i32,
	StateTrackingOnly    = 0x1i32,
	UnguardedValidation  = 0x2i32,
	GuardedValidation    = 0x3i32,
	Modes                = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12GpuBasedValidationPipelineStateCreateFlags
{
	None                 = 0x0i32,
	FrontLoadCreateTrackingOnlyShaders = 0x1i32,
	FrontLoadCreateUnguardedValidationShaders = 0x2i32,
	FrontLoadCreateGuardedValidationShaders = 0x4i32,
	SValidMask           = 0x7i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12DebugCommandListParameterType
{
	DebugCommandListParameterGpuBasedValidationSettings = 0x0i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MessageCategory
{
	ApplicationDefined   = 0x0i32,
	Miscellaneous        = 0x1i32,
	Initialization       = 0x2i32,
	Cleanup              = 0x3i32,
	Compilation          = 0x4i32,
	StateCreation        = 0x5i32,
	StateSetting         = 0x6i32,
	StateGetting         = 0x7i32,
	ResourceManipulation = 0x8i32,
	Execution            = 0x9i32,
	Shader               = 0xAi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MessageSeverity
{
	Corruption           = 0x0i32,
	Error                = 0x1i32,
	Warning              = 0x2i32,
	Info                 = 0x3i32,
	Message              = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MessageId
{
	Unknown              = 0x0i32,
	StringFromApplication = 0x1i32,
	CorruptedThis        = 0x2i32,
	CorruptedParameter1  = 0x3i32,
	CorruptedParameter2  = 0x4i32,
	CorruptedParameter3  = 0x5i32,
	CorruptedParameter4  = 0x6i32,
	CorruptedParameter5  = 0x7i32,
	CorruptedParameter6  = 0x8i32,
	CorruptedParameter7  = 0x9i32,
	CorruptedParameter8  = 0xAi32,
	CorruptedParameter9  = 0xBi32,
	CorruptedParameter10 = 0xCi32,
	CorruptedParameter11 = 0xDi32,
	CorruptedParameter12 = 0xEi32,
	CorruptedParameter13 = 0xFi32,
	CorruptedParameter14 = 0x10i32,
	CorruptedParameter15 = 0x11i32,
	CorruptedMultiThreading = 0x12i32,
	MessageReportingOutOfMemory = 0x13i32,
	GetPrivateDataMoreData = 0x14i32,
	SetPrivateDataInvalidFreeData = 0x15i32,
	SetPrivateDataChangingParams = 0x18i32,
	SetPrivateDataOutOfMemory = 0x19i32,
	CreateShaderResourceViewUnrecognizedFormat = 0x1Ai32,
	CreateShaderResourceViewInvalidDesc = 0x1Bi32,
	CreateShaderResourceViewInvalidFormat = 0x1Ci32,
	CreateShaderResourceViewInvalidVideoPlaneSlice = 0x1Di32,
	CreateShaderResourceViewInvalidPlaneSlice = 0x1Ei32,
	CreateShaderResourceViewInvalidDimensions = 0x1Fi32,
	CreateShaderResourceViewInvalidResource = 0x20i32,
	CreateRenderTargetViewUnrecognizedFormat = 0x23i32,
	CreateRenderTargetViewUnsupportedFormat = 0x24i32,
	CreateRenderTargetViewInvalidDesc = 0x25i32,
	CreateRenderTargetViewInvalidFormat = 0x26i32,
	CreateRenderTargetViewInvalidVideoPlaneSlice = 0x27i32,
	CreateRenderTargetViewInvalidPlaneSlice = 0x28i32,
	CreateRenderTargetViewInvalidDimensions = 0x29i32,
	CreateRenderTargetViewInvalidResource = 0x2Ai32,
	CreateDepthStencilViewUnrecognizedFormat = 0x2Di32,
	CreateDepthStencilViewInvalidDesc = 0x2Ei32,
	CreateDepthStencilViewInvalidFormat = 0x2Fi32,
	CreateDepthStencilViewInvalidDimensions = 0x30i32,
	CreateDepthStencilViewInvalidResource = 0x31i32,
	CreateInputLayoutOutOfMemory = 0x34i32,
	CreateInputLayoutTooManyElements = 0x35i32,
	CreateInputLayoutInvalidFormat = 0x36i32,
	CreateInputLayoutIncompatibleFormat = 0x37i32,
	CreateInputLayoutInvalidSlot = 0x38i32,
	CreateInputLayoutInvalidInputSlotClass = 0x39i32,
	CreateInputLayoutSTePrateSlotClassMismatch = 0x3Ai32,
	CreateInputLayoutInvalidSlotClassChange = 0x3Bi32,
	CreateInputLayoutInvalidsTePrateChange = 0x3Ci32,
	CreateInputLayoutInvalidAlignment = 0x3Di32,
	CreateInputLayoutDuplicateSemantic = 0x3Ei32,
	CreateInputLayoutUnParseAbleInputSignature = 0x3Fi32,
	CreateInputLayoutNullSemantic = 0x40i32,
	CreateInputLayoutMissingElement = 0x41i32,
	CreateVertexShaderOutOfMemory = 0x42i32,
	CreateVertexShaderInvalidShaderByteCode = 0x43i32,
	CreateVertexShaderInvalidShaderType = 0x44i32,
	CreateGeometryShaderOutOfMemory = 0x45i32,
	CreateGeometryShaderInvalidShaderByteCode = 0x46i32,
	CreateGeometryShaderInvalidShaderType = 0x47i32,
	CreateGeometryShaderWithStreamOutputOutOfMemory = 0x48i32,
	CreateGeometryShaderWithStreamOutputInvalidShaderByteCode = 0x49i32,
	CreateGeometryShaderWithStreamOutputInvalidShaderType = 0x4Ai32,
	CreateGeometryShaderWithStreamOutputInvalidNumEntries = 0x4Bi32,
	CreateGeometryShaderWithStreamOutputOutputStreamStrideUnused = 0x4Ci32,
	CreateGeometryShaderWithStreamOutputOutputSlot0Expected = 0x4Fi32,
	CreateGeometryShaderWithStreamOutputInvalidOutputSlot = 0x50i32,
	CreateGeometryShaderWithStreamOutputOnlyOneElementPerSlot = 0x51i32,
	CreateGeometryShaderWithStreamOutputInvalidComponentCount = 0x52i32,
	CreateGeometryShaderWithStreamOutputInvalidStartComponentAndComponentCount = 0x53i32,
	CreateGeometryShaderWithStreamOutputInvalidGapDefinition = 0x54i32,
	CreateGeometryShaderWithStreamOutputRepeatedOutput = 0x55i32,
	CreateGeometryShaderWithStreamOutputInvalidOutputStreamStride = 0x56i32,
	CreateGeometryShaderWithStreamOutputMissingSemantic = 0x57i32,
	CreateGeometryShaderWithStreamOutputMaskMismatch = 0x58i32,
	CreateGeometryShaderWithStreamOutputCanThaveOnlyGaps = 0x59i32,
	CreateGeometryShaderWithStreamOutputDeclTooComplex = 0x5Ai32,
	CreateGeometryShaderWithStreamOutputMissingOutputSignature = 0x5Bi32,
	CreatePixelShaderOutOfMemory = 0x5Ci32,
	CreatePixelShaderInvalidShaderByteCode = 0x5Di32,
	CreatePixelShaderInvalidShaderType = 0x5Ei32,
	CreateRasterizerStateInvalidFillMode = 0x5Fi32,
	CreateRasterizerStateInvalidCullMode = 0x60i32,
	CreateRasterizerStateInvalidDepthBiasClamp = 0x61i32,
	CreateRasterizerStateInvalidSlopeScaledDepthBias = 0x62i32,
	CreateDepthStencilStateInvalidDepthWriteMask = 0x64i32,
	CreateDepthStencilStateInvalidDepthFunc = 0x65i32,
	CreateDepthStencilStateInvalidFrontFaceStencilFailOp = 0x66i32,
	CreateDepthStencilStateInvalidFrontFaceStencilZFailOp = 0x67i32,
	CreateDepthStencilStateInvalidFrontFaceStencilPasSop = 0x68i32,
	CreateDepthStencilStateInvalidFrontFaceStencilFunc = 0x69i32,
	CreateDepthStencilStateInvalidBackFaceStencilFailOp = 0x6Ai32,
	CreateDepthStencilStateInvalidBackFaceStencilZFailOp = 0x6Bi32,
	CreateDepthStencilStateInvalidBackFaceStencilPasSop = 0x6Ci32,
	CreateDepthStencilStateInvalidBackFaceStencilFunc = 0x6Di32,
	CreateBlendStateInvalidSrcBlend = 0x6Fi32,
	CreateBlendStateInvalidDestBlend = 0x70i32,
	CreateBlendStateInValiDbLenDop = 0x71i32,
	CreateBlendStateInvalidSrcBlendAlpha = 0x72i32,
	CreateBlendStateInvalidDestBlendAlpha = 0x73i32,
	CreateBlendStateInValiDbLenDopAlpha = 0x74i32,
	CreateBlendStateInvalidRenderTargetWriteMask = 0x75i32,
	ClearDepthStencilViewInvalid = 0x87i32,
	CommandListDrawRootSignatureNotSet = 0xC8i32,
	CommandListDrawRootSignatureMismatch = 0xC9i32,
	CommandListDrawVertexBufferNotSet = 0xCAi32,
	CommandListDrawVertexBufferStrideTooSmall = 0xD1i32,
	CommandListDrawVertexBufferTooSmall = 0xD2i32,
	CommandListDrawIndexBufferNotSet = 0xD3i32,
	CommandListDrawIndexBufferFormatInvalid = 0xD4i32,
	CommandListDrawIndexBufferTooSmall = 0xD5i32,
	CommandListDrawInvalidPrimitiveTopology = 0xDBi32,
	CommandListDrawVertexStrideUnaligned = 0xDDi32,
	CommandListDrawIndexOffsetUnaligned = 0xDEi32,
	DeviceRemovalProcessAtFault = 0xE8i32,
	DeviceRemovalProcessPossiblyAtFault = 0xE9i32,
	DeviceRemovalProcessNotAtFault = 0xEAi32,
	CreateInputLayoutTrailingDigitInSemantic = 0xEFi32,
	CreateGeometryShaderWithStreamOutputTrailingDigitInSemantic = 0xF0i32,
	CreateInputLayoutTypeMismatch = 0xF5i32,
	CreateInputLayoutEmptyLayout = 0xFDi32,
	LiveObjectSummary    = 0xFFi32,
	LiveDevice           = 0x112i32,
	LiveSwapChain        = 0x113i32,
	CreateDepthStencilViewInvalidFlags = 0x114i32,
	CreateVertexShaderInvalidClassLinkage = 0x115i32,
	CreateGeometryShaderInvalidClassLinkage = 0x116i32,
	CreateGeometryShaderWithStreamOutputInvalidStreamToRasterizer = 0x118i32,
	CreatePixelShaderInvalidClassLinkage = 0x11Bi32,
	CreateGeometryShaderWithStreamOutputInvalidStream = 0x11Ci32,
	CreateGeometryShaderWithStreamOutputUnexpectedEntries = 0x11Di32,
	CreateGeometryShaderWithStreamOutputUnexpectedStrides = 0x11Ei32,
	CreateGeometryShaderWithStreamOutputInvalidNumStrides = 0x11Fi32,
	CreateHullShaderOutOfMemory = 0x121i32,
	CreateHullShaderInvalidShaderByteCode = 0x122i32,
	CreateHullShaderInvalidShaderType = 0x123i32,
	CreateHullShaderInvalidClassLinkage = 0x124i32,
	CreateDomainShaderOutOfMemory = 0x126i32,
	CreateDomainShaderInvalidShaderByteCode = 0x127i32,
	CreateDomainShaderInvalidShaderType = 0x128i32,
	CreateDomainShaderInvalidClassLinkage = 0x129i32,
	ResourceUnmapNotMapped = 0x136i32,
	DeviceCheckFeATuResupportMismatchedDataSize = 0x13Ei32,
	CreateComputeShaderOutOfMemory = 0x141i32,
	CreateComputeShaderInvalidShaderByteCode = 0x142i32,
	CreateComputeShaderInvalidClassLinkage = 0x143i32,
	DeviceCreateVertexShaderDoubleFloAtopSnotSupported = 0x14Bi32,
	DeviceCreateHullShaderDoubleFloAtopSnotSupported = 0x14Ci32,
	DeviceCreateDomainShaderDoubleFloAtopSnotSupported = 0x14Di32,
	DeviceCreateGeometryShaderDoubleFloAtopSnotSupported = 0x14Ei32,
	DeviceCreateGeometryShaderWithStreamOutputDoubleFloAtopSnotSupported = 0x14Fi32,
	DeviceCreatePixelShaderDoubleFloAtopSnotSupported = 0x150i32,
	DeviceCreateComputeShaderDoubleFloAtopSnotSupported = 0x151i32,
	CreateUnorderedAccessViewInvalidResource = 0x154i32,
	CreateUnorderedAccessViewInvalidDesc = 0x155i32,
	CreateUnorderedAccessViewInvalidFormat = 0x156i32,
	CreateUnorderedAccessViewInvalidVideoPlaneSlice = 0x157i32,
	CreateUnorderedAccessViewInvalidPlaneSlice = 0x158i32,
	CreateUnorderedAccessViewInvalidDimensions = 0x159i32,
	CreateUnorderedAccessViewUnrecognizedFormat = 0x15Ai32,
	CreateUnorderedAccessViewInvalidFlags = 0x162i32,
	CreateRasterizerStateInvalidForcedSampleCount = 0x191i32,
	CreateBlendStateInvalidLoGiCops = 0x193i32,
	DeviceCreateVertexShaderDoubleExtensionSnotSupported = 0x19Ai32,
	DeviceCreateHullShaderDoubleExtensionSnotSupported = 0x19Ci32,
	DeviceCreateDomainShaderDoubleExtensionSnotSupported = 0x19Ei32,
	DeviceCreateGeometryShaderDoubleExtensionSnotSupported = 0x1A0i32,
	DeviceCreateGeometryShaderWithStreamOutputDoubleExtensionSnotSupported = 0x1A2i32,
	DeviceCreatePixelShaderDoubleExtensionSnotSupported = 0x1A4i32,
	DeviceCreateComputeShaderDoubleExtensionSnotSupported = 0x1A6i32,
	DeviceCreateVertexShaderUavSnotSupported = 0x1A9i32,
	DeviceCreateHullShaderUavSnotSupported = 0x1AAi32,
	DeviceCreateDomainShaderUavSnotSupported = 0x1ABi32,
	DeviceCreateGeometryShaderUavSnotSupported = 0x1ACi32,
	DeviceCreateGeometryShaderWithStreamOutputUavSnotSupported = 0x1ADi32,
	DeviceCreatePixelShaderUavSnotSupported = 0x1AEi32,
	DeviceCreateComputeShaderUavSnotSupported = 0x1AFi32,
	DeviceClearViewInvalidSouRcErect = 0x1BFi32,
	DeviceClearViewEmptyRect = 0x1C0i32,
	UpdateTileMappingsInvalidParameter = 0x1EDi32,
	CopyTileMappingsInvalidParameter = 0x1EEi32,
	CreateDeviceInvalidArgs = 0x1FAi32,
	CreateDeviceWarning  = 0x1FBi32,
	ResourceBarrierInvalidType = 0x207i32,
	ResourceBarrierNullPointer = 0x208i32,
	ResourceBarrierInvalidSubresource = 0x209i32,
	ResourceBarrierReservedBits = 0x20Ai32,
	ResourceBarrierMissingBindFlags = 0x20Bi32,
	ResourceBarrierMismatchingMiscFlags = 0x20Ci32,
	ResourceBarrierMatchingStates = 0x20Di32,
	ResourceBarrierInvalidCombination = 0x20Ei32,
	ResourceBarrierBeforeAfterMismatch = 0x20Fi32,
	ResourceBarrierInvalidResource = 0x210i32,
	ResourceBarrierSampleCount = 0x211i32,
	ResourceBarrierInvalidFlags = 0x212i32,
	ResourceBarrierInvalidCombinedFlags = 0x213i32,
	ResourceBarrierInvalidFlagsForFormat = 0x214i32,
	ResourceBarrierInvalidSplitBarrier = 0x215i32,
	ResourceBarrierUnmatchedEnd = 0x216i32,
	ResourceBarrierUnmatchedBegin = 0x217i32,
	ResourceBarrierInvalidFlag = 0x218i32,
	ResourceBarrierInvalidCommandListType = 0x219i32,
	InvalidSubresourceState = 0x21Ai32,
	CommandAllocatorContention = 0x21Ci32,
	CommandAllocatorReset = 0x21Di32,
	CommandAllocatorResetBundle = 0x21Ei32,
	CommandAllocatorCannotReset = 0x21Fi32,
	CommandListOpen      = 0x220i32,
	InvalidBundleApi     = 0x222i32,
	CommandListClosed    = 0x223i32,
	WrongCommandAllocatorType = 0x225i32,
	CommandAllocatorSync = 0x228i32,
	CommandListSync      = 0x229i32,
	SetDescriptorHeapInvalid = 0x22Ai32,
	CreateCommandQueue   = 0x22Di32,
	CreateCommandAllocator = 0x22Ei32,
	CreatePipeLinEstate  = 0x22Fi32,
	CreateCommandList12  = 0x230i32,
	CreateResource       = 0x232i32,
	CreateDescriptorHeap = 0x233i32,
	CreateRootSignature  = 0x234i32,
	CreateLibrary        = 0x235i32,
	CreateHeap           = 0x236i32,
	CreateMonitoredFence = 0x237i32,
	CreateQueryHeap      = 0x238i32,
	CreateCommandSignature = 0x239i32,
	LiveCommandQueue     = 0x23Ai32,
	LiveCommandAllocator = 0x23Bi32,
	LivePipeLinEstate    = 0x23Ci32,
	LiveCommandList12    = 0x23Di32,
	LiveResource         = 0x23Fi32,
	LiveDescriptorHeap   = 0x240i32,
	LiveRootSignature    = 0x241i32,
	LiveLibrary          = 0x242i32,
	LiveHeap             = 0x243i32,
	LiveMonitoredFence   = 0x244i32,
	LiveQueryHeap        = 0x245i32,
	LiveCommandSignature = 0x246i32,
	DestroyCommandQueue  = 0x247i32,
	DestroyCommandAllocator = 0x248i32,
	DestroyPipeLinEstate = 0x249i32,
	DestroyCommandList12 = 0x24Ai32,
	DestroyResource      = 0x24Ci32,
	DestroyDescriptorHeap = 0x24Di32,
	DestroyRootSignature = 0x24Ei32,
	DestroyLibrary       = 0x24Fi32,
	DestroyHeap          = 0x250i32,
	DestroyMonitoredFence = 0x251i32,
	DestroyQueryHeap     = 0x252i32,
	DestroyCommandSignature = 0x253i32,
	CreateResourceInvalidDimensions = 0x255i32,
	CreateResourceInvalidMiscFlags = 0x257i32,
	CreateResourceInValiDargReturn = 0x25Ai32,
	CreateResourceOutOfMemoryReturn = 0x25Bi32,
	CreateResourceInvalidDesc = 0x25Ci32,
	PossiblyInvalidSubresourceState = 0x25Fi32,
	InvalidUseOfNonResidentResource = 0x260i32,
	PossibleInvalidUseOfNonResidentResource = 0x261i32,
	BundlePipelineStateMismatch = 0x262i32,
	PrimitiveTopologyMismatchPipelineState = 0x263i32,
	RenderTargetFormatMismatchPipelineState = 0x265i32,
	RenderTargetSampleDescMismatchPipelineState = 0x266i32,
	DepthStencilFormatMismatchPipelineState = 0x267i32,
	DepthStencilSampleDescMismatchPipelineState = 0x268i32,
	CreateShaderInvalidByteCode = 0x26Ei32,
	CreateHeapNullDesc   = 0x26Fi32,
	CreateHeapInvalidSize = 0x270i32,
	CreateHeapUnrecognizedHeapType = 0x271i32,
	CreateHeapUnrecognizedCpuPageProperties = 0x272i32,
	CreateHeapUnrecognizedMemoryPool = 0x273i32,
	CreateHeapInvalidProperties = 0x274i32,
	CreateHeapInvalidAlignment = 0x275i32,
	CreateHeapUnrecognizedMiscFlags = 0x276i32,
	CreateHeapInvalidMiscFlags = 0x277i32,
	CreateHeapInValiDargReturn = 0x278i32,
	CreateHeapOutOfMemoryReturn = 0x279i32,
	CreateResourceAndHeapNullHeapProperties = 0x27Ai32,
	CreateResourceAndHeapUnrecognizedHeapType = 0x27Bi32,
	CreateResourceAndHeapUnrecognizedCpuPageProperties = 0x27Ci32,
	CreateResourceAndHeapUnrecognizedMemoryPool = 0x27Di32,
	CreateResourceAndHeapInvalidHeapProperties = 0x27Ei32,
	CreateResourceAndHeapUnrecognizedHeapMiscFlags = 0x27Fi32,
	CreateResourceAndHeapInvalidHeapMiscFlags = 0x280i32,
	CreateResourceAndHeapInValiDargReturn = 0x281i32,
	CreateResourceAndHeapOutOfMemoryReturn = 0x282i32,
	GetCustomHeapPropertiesUnrecognizedHeapType = 0x283i32,
	GetCustomHeapPropertiesInvalidHeapType = 0x284i32,
	CreateDescriptorHeapInvalidDesc = 0x285i32,
	InvalidDescriptorHandle = 0x286i32,
	CreateRasterizerStateInvalidConservativeRasterMode = 0x287i32,
	CreateConstantBufferViewInvalidResource = 0x289i32,
	CreateConstantBufferViewInvalidDesc = 0x28Ai32,
	CreateUnorderedAccessViewInvalidCounterUsage = 0x28Ci32,
	CopyDescriptorsInvalidRanges = 0x28Di32,
	CopyDescriptorsWriteOnlyDescriptor = 0x28Ei32,
	CreateGraphicsPipeLinEstateRtvFormatNotUnknown = 0x28Fi32,
	CreateGraphicsPipeLinEstateInvalidRenderTargetCount = 0x290i32,
	CreateGraphicsPipeLinEstateVertexShaderNotSet = 0x291i32,
	CreateGraphicsPipeLinEstateInputLayoutNotSet = 0x292i32,
	CreateGraphicsPipeLinEstateShaderLinkageHsDsSignatureMismatch = 0x293i32,
	CreateGraphicsPipeLinEstateShaderLinkageRegisterIndex = 0x294i32,
	CreateGraphicsPipeLinEstateShaderLinkageComponentType = 0x295i32,
	CreateGraphicsPipeLinEstateShaderLinkageRegisterMask = 0x296i32,
	CreateGraphicsPipeLinEstateShaderLinkageSystemValue = 0x297i32,
	CreateGraphicsPipeLinEstateShaderLinkageNeverWrittenAlwaysReads = 0x298i32,
	CreateGraphicsPipeLinEstateShaderLinkageMinPrecision = 0x299i32,
	CreateGraphicsPipeLinEstateShaderLinkageSemanticNameNotFound = 0x29Ai32,
	CreateGraphicsPipeLinEstateHsXOrDsMismatch = 0x29Bi32,
	CreateGraphicsPipeLinEstateHullShaderInputTopologyMismatch = 0x29Ci32,
	CreateGraphicsPipeLinEstateHsDsControlPointCountMismatch = 0x29Di32,
	CreateGraphicsPipeLinEstateHsDsTessellatorDomainMismatch = 0x29Ei32,
	CreateGraphicsPipeLinEstateInvalidUseOfCenterMultiSamplePattern = 0x29Fi32,
	CreateGraphicsPipeLinEstateInvalidUseOfForcedSampleCount = 0x2A0i32,
	CreateGraphicsPipeLinEstateInvalidPrimitiveTopology = 0x2A1i32,
	CreateGraphicsPipeLinEstateInvalidSystemValue = 0x2A2i32,
	CreateGraphicsPipeLinEstateOmDualSourceBlendingCanOnlyHaveRenderTarget0 = 0x2A3i32,
	CreateGraphicsPipeLinEstateOmRenderTargetDoesNotSupportBlending = 0x2A4i32,
	CreateGraphicsPipeLinEstatePsOutputTypeMismatch = 0x2A5i32,
	CreateGraphicsPipeLinEstateOmRenderTargetDoesNotSupportLogicOps = 0x2A6i32,
	CreateGraphicsPipeLinEstateRenderTargetViewNotSet = 0x2A7i32,
	CreateGraphicsPipeLinEstateDepthStencilViewNotSet = 0x2A8i32,
	CreateGraphicsPipeLinEstateGsInputPrimitiveMismatch = 0x2A9i32,
	CreateGraphicsPipeLinEstatePositionNotPresent = 0x2AAi32,
	CreateGraphicsPipeLinEstateMissingRootSignatureFlags = 0x2ABi32,
	CreateGraphicsPipeLinEstateInvalidIndexBufferProperties = 0x2ACi32,
	CreateGraphicsPipeLinEstateInvalidSampleDesc = 0x2ADi32,
	CreateGraphicsPipeLinEstateHsRootSignatureMismatch = 0x2AEi32,
	CreateGraphicsPipeLinEstateDsRootSignatureMismatch = 0x2AFi32,
	CreateGraphicsPipeLinEstateVsRootSignatureMismatch = 0x2B0i32,
	CreateGraphicsPipeLinEstateGsRootSignatureMismatch = 0x2B1i32,
	CreateGraphicsPipeLinEstatePsRootSignatureMismatch = 0x2B2i32,
	CreateGraphicsPipeLinEstateMissingRootSignature = 0x2B3i32,
	ExecuteBundleOpenBundle = 0x2B4i32,
	ExecuteBundleDescriptorHeapMismatch = 0x2B5i32,
	ExecuteBundleType    = 0x2B6i32,
	DrawEmptyScissorRectangle = 0x2B7i32,
	CreateRootSignatureBlobNotFound = 0x2B8i32,
	CreateRootSignatureDeSerializeFailed = 0x2B9i32,
	CreateRootSignatureInvalidConfiguration = 0x2BAi32,
	CreateRootSignatureNotSupportedOnDevice = 0x2BBi32,
	CreateResourceAndHeapNullResourceProperties = 0x2BCi32,
	CreateResourceAndHeapNullHeap = 0x2BDi32,
	GetResourceAllocationInfoInValiDrDesCs = 0x2BEi32,
	MakeResidentNullObjectArray = 0x2BFi32,
	EvictNullObjectArray = 0x2C1i32,
	SetDescriptorTableInvalid = 0x2C4i32,
	SetRootConstantInvalid = 0x2C5i32,
	SetRootConstantBufferViewInvalid = 0x2C6i32,
	SetRootShaderResourceViewInvalid = 0x2C7i32,
	SetRootUnorderedAccessViewInvalid = 0x2C8i32,
	SetVertexBuffersInvalidDesc = 0x2C9i32,
	SetIndexBufferInvalidDesc = 0x2CBi32,
	SetStreamOutputBuffersInvalidDesc = 0x2CDi32,
	CreateResourceUnrecognizedDimensionality = 0x2CEi32,
	CreateResourceUnrecognizedLayout = 0x2CFi32,
	CreateResourceInvalidDimensionality = 0x2D0i32,
	CreateResourceInvalidAlignment = 0x2D1i32,
	CreateResourceInvalidMipLevels = 0x2D2i32,
	CreateResourceInvalidSampleDesc = 0x2D3i32,
	CreateResourceInvalidLayout = 0x2D4i32,
	SetIndexBufferInvalid = 0x2D5i32,
	SetVertexBuffersInvalid = 0x2D6i32,
	SetStreamOutputBuffersInvalid = 0x2D7i32,
	SetRenderTargetsInvalid = 0x2D8i32,
	CreateQueryHeapInvalidParameters = 0x2D9i32,
	BeginEndQueryInvalidParameters = 0x2DBi32,
	CloseCommandListOpenQuery = 0x2DCi32,
	ResolveQueryDataInvalidParameters = 0x2DDi32,
	SetPredicationInvalidParameters = 0x2DEi32,
	TimestampsNotSupported = 0x2DFi32,
	CreateResourceUnrecognizedFormat = 0x2E1i32,
	CreateResourceInvalidFormat = 0x2E2i32,
	GetCopyAbleFootprintsInvalidSubresourceRange = 0x2E3i32,
	GetCopyAbleFootprintsInvalidBaseOffset = 0x2E4i32,
	ResourceBarrierInvalidHeap = 0x2E5i32,
	CreateSamplerInvalid = 0x2E6i32,
	CreateCommandSignatureInvalid = 0x2E7i32,
	ExecuteIndirectInvalidParameters = 0x2E8i32,
	GetGpuVirtualAddressInvalidResourceDimension = 0x2E9i32,
	CreateResourceInvalidClearValue = 0x32Fi32,
	CreateResourceUnrecognizedClearValueFormat = 0x330i32,
	CreateResourceInvalidClearValueFormat = 0x331i32,
	CreateResourceClearValuedEnormFlush = 0x332i32,
	ClearRenderTargetViewMismatchingClearValue = 0x334i32,
	ClearDepthStencilViewMismatchingClearValue = 0x335i32,
	MapInvalidHeap       = 0x336i32,
	UnmapInvalidHeap     = 0x337i32,
	MapInvalidResource   = 0x338i32,
	UnmapInvalidResource = 0x339i32,
	MapInvalidSubresource = 0x33Ai32,
	UnmapInvalidSubresource = 0x33Bi32,
	MapInvalidRange      = 0x33Ci32,
	UnmapInvalidRange    = 0x33Di32,
	MapInvalidDataPointer = 0x340i32,
	MapInValiDargReturn  = 0x341i32,
	MapOutOfMemoryReturn = 0x342i32,
	ExecuteCommandListsBundleNotSupported = 0x343i32,
	ExecuteCommandListsCommandListMismatch = 0x344i32,
	ExecuteCommandListsOpenCommandList = 0x345i32,
	ExecuteCommandListsFailedCommandList = 0x346i32,
	CopyBufferRegionNulLdSt = 0x347i32,
	CopyBufferRegionInValiDdStResourceDimension = 0x348i32,
	CopyBufferRegionDStrangeOutOfBounds = 0x349i32,
	CopyBufferRegionNullSrc = 0x34Ai32,
	CopyBufferRegionInvalidSrcResourceDimension = 0x34Bi32,
	CopyBufferRegionSrcRangeOutOfBounds = 0x34Ci32,
	CopyBufferRegionInvalidCopyFlags = 0x34Di32,
	CopyTextureRegionNulLdSt = 0x34Ei32,
	CopyTextureRegionUnRecognizeDdStType = 0x34Fi32,
	CopyTextureRegionInValiDdStResourceDimension = 0x350i32,
	CopyTextureRegionInValiDdStResource = 0x351i32,
	CopyTextureRegionInValiDdStSubresource = 0x352i32,
	CopyTextureRegionInValiDdStOffset = 0x353i32,
	CopyTextureRegionUnRecognizeDdStFormat = 0x354i32,
	CopyTextureRegionInValiDdStFormat = 0x355i32,
	CopyTextureRegionInValiDdStDimensions = 0x356i32,
	CopyTextureRegionInValiDdStrowPitch = 0x357i32,
	CopyTextureRegionInValiDdStPlacement = 0x358i32,
	CopyTextureRegionInValiDdStDsPlacedFootprintFormat = 0x359i32,
	CopyTextureRegionDStRegionOutOfBounds = 0x35Ai32,
	CopyTextureRegionNullSrc = 0x35Bi32,
	CopyTextureRegionUnrecognizedSrcType = 0x35Ci32,
	CopyTextureRegionInvalidSrcResourceDimension = 0x35Di32,
	CopyTextureRegionInvalidSrcResource = 0x35Ei32,
	CopyTextureRegionInvalidSrcSubresource = 0x35Fi32,
	CopyTextureRegionInvalidSrcOffset = 0x360i32,
	CopyTextureRegionUnrecognizedSrcFormat = 0x361i32,
	CopyTextureRegionInvalidSrcFormat = 0x362i32,
	CopyTextureRegionInvalidSrcDimensions = 0x363i32,
	CopyTextureRegionInValiDsrCrowPitch = 0x364i32,
	CopyTextureRegionInvalidSrcPlacement = 0x365i32,
	CopyTextureRegionInvalidSrcDsPlacedFootprintFormat = 0x366i32,
	CopyTextureRegionSrcRegionOutOfBounds = 0x367i32,
	CopyTextureRegionInValiDdStCoordinates = 0x368i32,
	CopyTextureRegionInvalidSrcBox = 0x369i32,
	CopyTextureRegionFormatMismatch = 0x36Ai32,
	CopyTextureRegionEmptyBox = 0x36Bi32,
	CopyTextureRegionInvalidCopyFlags = 0x36Ci32,
	ResolveSubresourceInvalidSubresourceIndex = 0x36Di32,
	ResolveSubresourceInvalidFormat = 0x36Ei32,
	ResolveSubresourceResourceMismatch = 0x36Fi32,
	ResolveSubresourceInvalidSampleCount = 0x370i32,
	CreateComputePipeLinEstateInvalidShader = 0x371i32,
	CreateComputePipeLinEstateCsRootSignatureMismatch = 0x372i32,
	CreateComputePipeLinEstateMissingRootSignature = 0x373i32,
	CreatePipeLinEstateInvalidCachedBlob = 0x374i32,
	CreatePipeLinEstateCachedBlobAdapterMismatch = 0x375i32,
	CreatePipeLinEstateCachedBlobDriverVersionMismatch = 0x376i32,
	CreatePipeLinEstateCachedBlobDescMismatch = 0x377i32,
	CreatePipeLinEstateCachedBlobIgnored = 0x378i32,
	WriteToSubresourceInvalidHeap = 0x379i32,
	WriteToSubresourceInvalidResource = 0x37Ai32,
	WriteToSubresourceInvalidBox = 0x37Bi32,
	WriteToSubresourceInvalidSubresource = 0x37Ci32,
	WriteToSubresourceEmptyBox = 0x37Di32,
	ReadFromSubresourceInvalidHeap = 0x37Ei32,
	ReadFromSubresourceInvalidResource = 0x37Fi32,
	ReadFromSubresourceInvalidBox = 0x380i32,
	ReadFromSubresourceInvalidSubresource = 0x381i32,
	ReadFromSubresourceEmptyBox = 0x382i32,
	TooManyNodesSpecified = 0x383i32,
	InvalidNodeIndex     = 0x384i32,
	GetHeapPropertiesInvalidResource = 0x385i32,
	NodeMaskMismatch     = 0x386i32,
	CommandListOutOfMemory = 0x387i32,
	CommandListMultipleSwapChainBufferReferences = 0x388i32,
	CommandListTooManySwapChainReferences = 0x389i32,
	CommandQueueTooManySwapChainReferences = 0x38Ai32,
	ExecuteCommandListsWrongSwapChainBufferReference = 0x38Bi32,
	CommandListSetRenderTargetsInvalidNumRenderTargets = 0x38Ci32,
	CreateQueueInvalidType = 0x38Di32,
	CreateQueueInvalidFlags = 0x38Ei32,
	CreateSharedResourceInvalidFlags = 0x38Fi32,
	CreateSharedResourceInvalidFormat = 0x390i32,
	CreateSharedHeapInvalidFlags = 0x391i32,
	ReflectSharedPropertiesUnrecognizedProperties = 0x392i32,
	ReflectSharedPropertiesInvalidSize = 0x393i32,
	ReflectSharedPropertiesInvalidObject = 0x394i32,
	KeyedMuTexInvalidObject = 0x395i32,
	KeyedMuTexInvalidKey = 0x396i32,
	KeyedMuTexWrongState = 0x397i32,
	CreateQueueInvalidPriority = 0x398i32,
	ObjectDeletedWhileStillInUse = 0x399i32,
	CreatePipeLinEstateInvalidFlags = 0x39Ai32,
	HeapAddressRangeHasNoResource = 0x39Bi32,
	CommandListDrawRenderTargetDeleted = 0x39Ci32,
	CreateGraphicsPipeLinEstateAllRenderTargetsHaveUnknownFormat = 0x39Di32,
	HeapAddressRangeIntersectsMultipleBuffers = 0x39Ei32,
	ExecuteCommandListsGpuWrittenReadBackResourceMapped = 0x39Fi32,
	UnmapRangeNotEmpty   = 0x3A1i32,
	MapInvalidNullRange  = 0x3A2i32,
	UnmapInvalidNullRange = 0x3A3i32,
	NoGraphicsApiSupport = 0x3A4i32,
	NoComputeApiSupport  = 0x3A5i32,
	ResolveSubresourceResourceFlagsNotSupported = 0x3A6i32,
	GpuBasedValidationRootArgumentUninitialized = 0x3A7i32,
	GpuBasedValidationDescriptorHeapIndexOutOfBounds = 0x3A8i32,
	GpuBasedValidationDescriptorTableRegisterIndexOutOfBounds = 0x3A9i32,
	GpuBasedValidationDescriptorUninitialized = 0x3AAi32,
	GpuBasedValidationDescriptorTypeMismatch = 0x3ABi32,
	GpuBasedValidationSrvResourceDimensionMismatch = 0x3ACi32,
	GpuBasedValidationUavResourceDimensionMismatch = 0x3ADi32,
	GpuBasedValidationIncompatibleResourceState = 0x3AEi32,
	CopyResourceNulLdSt  = 0x3AFi32,
	CopyResourceInValiDdStResource = 0x3B0i32,
	CopyResourceNullSrc  = 0x3B1i32,
	CopyResourceInvalidSrcResource = 0x3B2i32,
	ResolveSubresourceNulLdSt = 0x3B3i32,
	ResolveSubresourceInValiDdStResource = 0x3B4i32,
	ResolveSubresourceNullSrc = 0x3B5i32,
	ResolveSubresourceInvalidSrcResource = 0x3B6i32,
	PipelineStateTypeMismatch = 0x3B7i32,
	CommandListDispatchRootSignatureNotSet = 0x3B8i32,
	CommandListDispatchRootSignatureMismatch = 0x3B9i32,
	ResourceBarrierZeroBarriers = 0x3BAi32,
	BeginEndEventMismatch = 0x3BBi32,
	ResourceBarrierPossibleBeforeAfterMismatch = 0x3BCi32,
	ResourceBarrierMismatchingBeginEnd = 0x3BDi32,
	GpuBasedValidationInvalidResource = 0x3BEi32,
	UseOfZeroRefCountObject = 0x3BFi32,
	ObjectEvictedWhileStillInUse = 0x3C0i32,
	GpuBasedValidationRootDescriptorAccessOutOfBounds = 0x3C1i32,
	CreatePipelineLibraryInvalidLibraryBlob = 0x3C2i32,
	CreatePipelineLibraryDriverVersionMismatch = 0x3C3i32,
	CreatePipelineLibraryAdapterVersionMismatch = 0x3C4i32,
	CreatePipelineLibraryUnsupported = 0x3C5i32,
	CreatePipelineLibrary = 0x3C6i32,
	LivePipelineLibrary  = 0x3C7i32,
	DestroyPipelineLibrary = 0x3C8i32,
	StorePipelineNoName  = 0x3C9i32,
	StorePipelineDuplicateName = 0x3CAi32,
	LoadPipelineNameNotFound = 0x3CBi32,
	LoadPipelineInvalidDesc = 0x3CCi32,
	PipelineLibrarySerializeNotEnoughMemory = 0x3CDi32,
	CreateGraphicsPipeLinEstatePsOutputRtOutputMismatch = 0x3CEi32,
	SetEvenTonMultipleFenceCompletionInvalidFlags = 0x3CFi32,
	CreateQueueVideoNotSupported = 0x3D0i32,
	CreateCommandAllocatorVideoNotSupported = 0x3D1i32,
	CreateQueryHeapVideoDecodeStatisticsNotSupported = 0x3D2i32,
	CreateVideoDecodeCommandList = 0x3D3i32,
	CreateVideoDecoder   = 0x3D4i32,
	CreateVideoDecodeStream = 0x3D5i32,
	LiveVideoDecodeCommandList = 0x3D6i32,
	LiveVideoDecoder     = 0x3D7i32,
	LiveVideoDecodeStream = 0x3D8i32,
	DestroyVideoDecodeCommandList = 0x3D9i32,
	DestroyVideoDecoder  = 0x3DAi32,
	DestroyVideoDecodeStream = 0x3DBi32,
	DecodeFrameInvalidParameters = 0x3DCi32,
	DeprecatedApi        = 0x3DDi32,
	ResourceBarrierMismatchingCommandListType = 0x3DEi32,
	CommandListDescriptorTableNotSet = 0x3DFi32,
	CommandListRootConstantBufferViewNotSet = 0x3E0i32,
	CommandListRootShaderResourceViewNotSet = 0x3E1i32,
	CommandListRootUnorderedAccessViewNotSet = 0x3E2i32,
	DiscardInvalidSubresourceRange = 0x3E3i32,
	DiscardOneSubresourceForMipsWithReCts = 0x3E4i32,
	DiscardNoReCtsForNonTexture2D = 0x3E5i32,
	CopyOnSameSubresource = 0x3E6i32,
	SetResidencyPriorityInvalidPageable = 0x3E7i32,
	GpuBasedValidationUnsupported = 0x3E8i32,
	StaticDescriptorInvalidDescriptorChange = 0x3E9i32,
	DataStaticDescriptorInvalidDataChange = 0x3EAi32,
	DataStaticWhileSetAtExecuteDescriptorInvalidDataChange = 0x3EBi32,
	ExecuteBundleStaticDescriptorDataStaticNotSet = 0x3ECi32,
	GpuBasedValidationResourceAccessOutOfBounds = 0x3EDi32,
	GpuBasedValidationSamplerModeMismatch = 0x3EEi32,
	CreateFenceInvalidFlags = 0x3EFi32,
	ResourceBarrierDuplicateSubresourceTransitions = 0x3F0i32,
	SetResidencyPriorityInvalidPriority = 0x3F1i32,
	CreateDescriptorHeapLargeNumDescriptors = 0x3F5i32,
	BeginEvent           = 0x3F6i32,
	EndEvent             = 0x3F7i32,
	CreateDeviceDebugLayerStartupOptions = 0x3F8i32,
	CreateDepthStencilStateDepthBoundsTestUnsupported = 0x3F9i32,
	CreatePipeLinEstateDuplicateSubobject = 0x3FAi32,
	CreatePipeLinEstateUnknownSubobject = 0x3FBi32,
	CreatePipeLinEstateZeroSizeStream = 0x3FCi32,
	CreatePipeLinEstateInvalidStream = 0x3FDi32,
	CreatePipeLinEstateCannotDeduceType = 0x3FEi32,
	CommandListStaticDescriptorResourceDimensionMismatch = 0x3FFi32,
	CreateCommandQueueInsufficientPrivilegeForGlobalRealTime = 0x400i32,
	CreateCommandQueueInsufficientHardwareSupportForGlobalRealTime = 0x401i32,
	AtomicCopyBufferInvalidArchitecture = 0x402i32,
	AtomicCopyBufferNullDSt = 0x403i32,
	AtomicCopyBufferInvalidDStResourceDimension = 0x404i32,
	AtomicCopyBufferDStRangeOutOfBounds = 0x405i32,
	AtomicCopyBufferNullSrc = 0x406i32,
	AtomicCopyBufferInvalidSrcResourceDimension = 0x407i32,
	AtomicCopyBufferSrcRangeOutOfBounds = 0x408i32,
	AtomicCopyBufferInvalidOffsetAlignment = 0x409i32,
	AtomicCopyBufferNullDependentResources = 0x40Ai32,
	AtomicCopyBufferNullDependentSubresourceRanges = 0x40Bi32,
	AtomicCopyBufferInvalidDependentResource = 0x40Ci32,
	AtomicCopyBufferInvalidDependentSubresourceRange = 0x40Di32,
	AtomicCopyBufferDependentSubresourceOutOfBounds = 0x40Ei32,
	AtomicCopyBufferDependentRangeOutOfBounds = 0x40Fi32,
	AtomicCopyBufferZeroDependencies = 0x410i32,
	DeviceCreateSharedHandleInValiDarg = 0x411i32,
	DescriptorHandleWithInvalidResource = 0x412i32,
	SetDepthBoundsInvalidArgs = 0x413i32,
	GpuBasedValidationResourceStateImprecise = 0x414i32,
	CommandListPipelineStateNotSet = 0x415i32,
	CreateGraphicsPipeLinEstateShaderModelMismatch = 0x416i32,
	ObjectAccessedWhileStillInUse = 0x417i32,
	ProgrammableMSaaUnsupported = 0x418i32,
	SetSamplePositionsInvalidArgs = 0x419i32,
	ResolveSubresourceRegionInvalidRect = 0x41Ai32,
	CreateVideoDecodeCommandQueue = 0x41Bi32,
	CreateVideoProcessCommandList = 0x41Ci32,
	CreateVideoProcessCommandQueue = 0x41Di32,
	LiveVideoDecodeCommandQueue = 0x41Ei32,
	LiveVideoProcessCommandList = 0x41Fi32,
	LiveVideoProcessCommandQueue = 0x420i32,
	DestroyVideoDecodeCommandQueue = 0x421i32,
	DestroyVideoProcessCommandList = 0x422i32,
	DestroyVideoProcessCommandQueue = 0x423i32,
	CreateVideoProcessor = 0x424i32,
	CreateVideoProcessStream = 0x425i32,
	LiveVideoProcessor   = 0x426i32,
	LiveVideoProcessStream = 0x427i32,
	DestroyVideoProcessor = 0x428i32,
	DestroyVideoProcessStream = 0x429i32,
	ProcessFrameInvalidParameters = 0x42Ai32,
	CopyInvalidLayout    = 0x42Bi32,
	CreateCryptoSession  = 0x42Ci32,
	CreateCryptoSessionPolicy = 0x42Di32,
	CreateProtectedResourceSession = 0x42Ei32,
	LiveCryptoSession    = 0x42Fi32,
	LiveCryptoSessionPolicy = 0x430i32,
	LiveProtectedResourceSession = 0x431i32,
	DestroyCryptoSession = 0x432i32,
	DestroyCryptoSessionPolicy = 0x433i32,
	DestroyProtectedResourceSession = 0x434i32,
	ProtectedResourceSessionUnsupported = 0x435i32,
	FenceInvalidOperation = 0x436i32,
	CreateQueryHeapCopyQueueTimestampsNotSupported = 0x437i32,
	SamplePositionsMismatchDeferred = 0x438i32,
	SamplePositionsMismatchRecordTimeAssumedFromFirstUse = 0x439i32,
	SamplePositionsMismatchRecordTimeAssumedFromClear = 0x43Ai32,
	CreateVideoDecoderHeap = 0x43Bi32,
	LiveVideoDecoderHeap = 0x43Ci32,
	DestroyVideoDecoderHeap = 0x43Di32,
	OpenExistingHeapInValiDargReturn = 0x43Ei32,
	OpenExistingHeapOutOfMemoryReturn = 0x43Fi32,
	OpenExistingHeapInvalidAddress = 0x440i32,
	OpenExistingHeapInvalidHandle = 0x441i32,
	WriteBufferImmediateInvalidDest = 0x442i32,
	WriteBufferImmediateInvalidMode = 0x443i32,
	WriteBufferImmediateInvalidAlignment = 0x444i32,
	WriteBufferImmediateNotSupported = 0x445i32,
	SetViewInstanceMaskInvalidArgs = 0x446i32,
	ViewInstancingUnsupported = 0x447i32,
	ViewInstancingInvalidArgs = 0x448i32,
	CopyTextureRegionMismatchDecodeReferenceOnlyFlag = 0x449i32,
	CopyResourceMismatchDecodeReferenceOnlyFlag = 0x44Ai32,
	CreateVideoDecodeHeapCapsFailure = 0x44Bi32,
	CreateVideoDecodeHeapCapsUnsupported = 0x44Ci32,
	VideoDecodeSupportInvalidInput = 0x44Di32,
	CreateVideoDecoderUnsupported = 0x44Ei32,
	CreateGraphicsPipeLinEstateMetaDataError = 0x44Fi32,
	CreateGraphicsPipeLinEstateViewInstancingVertexSizeExceeded = 0x450i32,
	CreateGraphicsPipeLinEstateRuntimeInternalError = 0x451i32,
	NoVideoApiSupport    = 0x452i32,
	VideoProcessSupportInvalidInput = 0x453i32,
	CreateVideoProcessorCapsFailure = 0x454i32,
	VideoProcessSupportUnsupportedFormat = 0x455i32,
	VideoDecodeFrameInvalidArgument = 0x456i32,
	EnqueueMakeResidentInvalidFlags = 0x457i32,
	OpenExistingHeapUnsupported = 0x458i32,
	VideoProcessFramesInvalidArgument = 0x459i32,
	VideoDecodeSupportUnsupported = 0x45Ai32,
	CreateCommandRecorder = 0x45Bi32,
	LiveCommandRecorder  = 0x45Ci32,
	DestroyCommandRecorder = 0x45Di32,
	CreateCommandRecorderVideoNotSupported = 0x45Ei32,
	CreateCommandRecorderInvalidSupportFlags = 0x45Fi32,
	CreateCommandRecorderInvalidFlags = 0x460i32,
	CreateCommandRecorderMoreRecordersThanLogicalProcessors = 0x461i32,
	CreateCommandPool    = 0x462i32,
	LiveCommandPool      = 0x463i32,
	DestroyCommandPool   = 0x464i32,
	CreateCommandPoolInvalidFlags = 0x465i32,
	CreateCommandListVideoNotSupported = 0x466i32,
	CommandRecorderSupportFlagsMismatch = 0x467i32,
	CommandRecorderContention = 0x468i32,
	CommandRecorderUsageWithCreateCommandListCommandList = 0x469i32,
	CommandAllocatorUsageWithCreateCommandList1CommandList = 0x46Ai32,
	CannotExecuteEmptyCommandList = 0x46Bi32,
	CannotResetCommandPoolWithOpenCommandLists = 0x46Ci32,
	CannotUseCommandRecorderWithoutCurrentTarget = 0x46Di32,
	CannotChangeCommandRecorderTargetWhileRecording = 0x46Ei32,
	CommandPoolSync      = 0x46Fi32,
	EvictUnderflow       = 0x470i32,
	CreateMetaCommand    = 0x471i32,
	LiveMetaCommand      = 0x472i32,
	DestroyMetaCommand   = 0x473i32,
	CopyBufferRegionInvalidDStResource = 0x474i32,
	CopyBufferRegionInvalidSrcResource = 0x475i32,
	AtomicCopyBufferInvalidDStResource = 0x476i32,
	AtomicCopyBufferInvalidSrcResource = 0x477i32,
	CreatEpLaCedreSouRcEonBufferNullBuffer = 0x478i32,
	CreatEpLaCedreSouRcEonBufferNullResourceDesc = 0x479i32,
	CreatEpLaCedreSouRcEonBufferUnsupported = 0x47Ai32,
	CreatEpLaCedreSouRcEonBufferInvalidBufferDimension = 0x47Bi32,
	CreatEpLaCedreSouRcEonBufferInvalidBufferFlags = 0x47Ci32,
	CreatEpLaCedreSouRcEonBufferInvalidBufferOffset = 0x47Di32,
	CreatEpLaCedreSouRcEonBufferInvalidResourceDimension = 0x47Ei32,
	CreatEpLaCedreSouRcEonBufferInvalidResourceFlags = 0x47Fi32,
	CreatEpLaCedreSouRcEonBufferOutOfMemoryReturn = 0x480i32,
	CannotCreateGraphicsAndVideoCommandRecorder = 0x481i32,
	UpdateTileMappingsPossiblyMismatchingProperties = 0x482i32,
	CreateCommandListInvalidCommandListType = 0x483i32,
	ClearUnorderedAccessViewIncompatibleWithStructuredBuffers = 0x484i32,
	ComputeOnlyDeviceOperationUnsupported = 0x485i32,
	BuildRaytracingAccelerationStructureInvalid = 0x486i32,
	EmitRaytracingAccelerationStructurePostBuildInfoInvalid = 0x487i32,
	CopyRaytracingAccelerationStructureInvalid = 0x488i32,
	DispatchRaysInvalid  = 0x489i32,
	GetRaytracingAccelerationStructurePrebuildInfoInvalid = 0x48Ai32,
	CreateLifetimeTracker = 0x48Bi32,
	LiveLifetimeTracker  = 0x48Ci32,
	DestroyLifetimeTracker = 0x48Di32,
	DestroyOwnedObjectObjectNoTowned = 0x48Ei32,
	CreateTrackedWorkload = 0x48Fi32,
	LiveTrackedWorkload  = 0x490i32,
	DestroyTrackedWorkload = 0x491i32,
	RenderPassError      = 0x492i32,
	MetaCommandIdInvalid = 0x493i32,
	MetaCommandUnsupportedParams = 0x494i32,
	MetaCommandFailedEnumeration = 0x495i32,
	MetaCommandParameterSizeMismatch = 0x496i32,
	UninitializedMetaCommand = 0x497i32,
	MetaCommandInvalidGpuVirtualAddress = 0x498i32,
	CreateVideoEncodeCommandList = 0x499i32,
	LiveVideoEncodeCommandList = 0x49Ai32,
	DestroyVideoEncodeCommandList = 0x49Bi32,
	CreateVideoEncodeCommandQueue = 0x49Ci32,
	LiveVideoEncodeCommandQueue = 0x49Di32,
	DestroyVideoEncodeCommandQueue = 0x49Ei32,
	CreateVIdeomotionEstimator = 0x49Fi32,
	LiveVIdeomotionEstimator = 0x4A0i32,
	DestroyVIdeomotionEstimator = 0x4A1i32,
	CreateVIdeomotionVectorHeap = 0x4A2i32,
	LiveVIdeomotionVectorHeap = 0x4A3i32,
	DestroyVIdeomotionVectorHeap = 0x4A4i32,
	MultipleTrackedWorkloads = 0x4A5i32,
	MultipleTrackedWorkloadPairs = 0x4A6i32,
	OutOfOrderTrackedWorkloadPair = 0x4A7i32,
	CannotAddTrackedWorkload = 0x4A8i32,
	IncompleteTrackedWorkloadPair = 0x4A9i32,
	CreateStateObjectError = 0x4AAi32,
	GetShaderIdentifierError = 0x4ABi32,
	GetShaderStackSizeError = 0x4ACi32,
	GetPipelineStackSizeError = 0x4ADi32,
	SetPipelineStackSizeError = 0x4AEi32,
	GetShaderIdentifierSizeInvalid = 0x4AFi32,
	CheckDriverMatchingIdentifierInvalid = 0x4B0i32,
	CheckDriverMatchingIdentifierDriverReportedIssue = 0x4B1i32,
	RenderPassInvalidResourceBarrier = 0x4B2i32,
	RenderPassDisallowedApiCalled = 0x4B3i32,
	RenderPassCannotNestRenderPasses = 0x4B4i32,
	RenderPassCannotEndWithoutBegin = 0x4B5i32,
	RenderPassCannotCloseCommandList = 0x4B6i32,
	RenderPassGpuWorkWhileSuspended = 0x4B7i32,
	RenderPassMismatchingSuspendResume = 0x4B8i32,
	RenderPassNoPriorSuspendWithinExecuteCommandLists = 0x4B9i32,
	RenderPassNoSubsequentResumeWithinExecuteCommandLists = 0x4BAi32,
	TrackedWorkloadCommandQueueMismatch = 0x4BBi32,
	TrackedWorkloadNotSupported = 0x4BCi32,
	RenderPassMismatchingNoAccess = 0x4BDi32,
	RenderPassUnsupportedResolve = 0x4BEi32,
	ClearUnorderedAccessViewInvalidResourcePtr = 0x4BFi32,
	Windows7FenceOutOfOrderSignal = 0x4C0i32,
	Windows7FenceOutOfOrderWait = 0x4C1i32,
	VideoCreateMotionEstimatorInvalidArgument = 0x4C2i32,
	VideoCreateMotionVectorHeapInvalidArgument = 0x4C3i32,
	EstimateMotionInvalidArgument = 0x4C4i32,
	ResolveMotionVectorHeapInvalidArgument = 0x4C5i32,
	GetGpuVirtualAddressInvalidHeapType = 0x4C6i32,
	SetBackgroundProcessingModeInvalidArgument = 0x4C7i32,
	CreateCommandListInvalidCommandListTypeForFeatureLevel = 0x4C8i32,
	CreateVideoExtensionCommand = 0x4C9i32,
	LiveVideoExtensionCommand = 0x4CAi32,
	DestroyVideoExtensionCommand = 0x4CBi32,
	InvalidVideoExtensionCommandId = 0x4CCi32,
	VideoExtensionCommandInvalidArgument = 0x4CDi32,
	CreateRootSignatureNotUniqueInDxilLibrary = 0x4CEi32,
	VariableShadingRateNotAllowedWithTIr = 0x4CFi32,
	GeometryShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = 0x4D0i32,
	RsSetShadingRateInvalidShadingRate = 0x4D1i32,
	RsSetShadingRateShadingRateNotPermittedByCap = 0x4D2i32,
	RsSetShadingRateInvalidCombiner = 0x4D3i32,
	RsSetShadIngrateImageRequiresTier2 = 0x4D4i32,
	RsSetShadIngrateRequiresTier1 = 0x4D5i32,
	ShadingRateImageIncorrectFormat = 0x4D6i32,
	ShadingRateImageIncorrectArraySize = 0x4D7i32,
	ShadingRateImageIncorrectMipLevel = 0x4D8i32,
	ShadingRateImageIncorrectSampleCount = 0x4D9i32,
	ShadingRateImageIncorrectSampleQuality = 0x4DAi32,
	NonRetailShaderModelWontValidate = 0x4DBi32,
	CreateGraphicsPipeLinEstateAsRootSignatureMismatch = 0x4DCi32,
	CreateGraphicsPipeLinEstateMsRootSignatureMismatch = 0x4DDi32,
	AddToStateObjectError = 0x4DEi32,
	CreateProtectedResourceSessionInvalidArgument = 0x4DFi32,
	CreateGraphicsPipeLinEstateMsPSoDescMismatch = 0x4E0i32,
	CreatePipeLinEstateMsIncompleteType = 0x4E1i32,
	CreateGraphicsPipeLinEstateAsNotMsMismatch = 0x4E2i32,
	CreateGraphicsPipeLinEstateMsNotPsMismatch = 0x4E3i32,
	NonzeroSamplerFeedbackMipRegionWithIncompatibleFormat = 0x4E4i32,
	CreateGraphicsPipeLinEstateInputLayoutShaderMismatch = 0x4E5i32,
	EmptyDispatch        = 0x4E6i32,
	ResourceFormatRequiresSamplerFeedbackCapability = 0x4E7i32,
	SamplerFeedbackMapInvalidMipRegion = 0x4E8i32,
	SamplerFeedbackMapInvalidDimension = 0x4E9i32,
	SamplerFeedbackMapInvalidSampleCount = 0x4EAi32,
	SamplerFeedbackMapInvalidSampleQuality = 0x4EBi32,
	SamplerFeedbackMapInvalidLayout = 0x4ECi32,
	SamplerFeedbackMapRequiresUnorderedAccessFlag = 0x4EDi32,
	SamplerFeedbackCreateUavNullArguments = 0x4EEi32,
	SamplerFeedbackUavRequiresSamplerFeedbackCapability = 0x4EFi32,
	SamplerFeedbackCreateUavRequiresFeedbackMapFormat = 0x4F0i32,
	CreateMeshShaderInvalidShaderByteCode = 0x4F1i32,
	CreateMeshShaderOutOfMemory = 0x4F2i32,
	CreateMeshShaderWithStreamOutputInvalidShaderType = 0x4F3i32,
	ResolveSubresourceSamplerFeedbackTransCodeInvalidFormat = 0x4F4i32,
	ResolveSubresourceSamplerFeedbackInvalidMipLevelCount = 0x4F5i32,
	ResolveSubresourceSamplerFeedbackTransCodeArraySizeMismatch = 0x4F6i32,
	SamplerFeedbackCreateUavMismatchingTargetedResource = 0x4F7i32,
	CreateMeshShaderOutputExceedsMaxSize = 0x4F8i32,
	CreateMeshShaderGroupSharedExceedsMaxSize = 0x4F9i32,
	VertexShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = 0x4FAi32,
	MeshShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = 0x4FBi32,
	CreateMeshShaderMiSmaTcheDasMsPayloadSize = 0x4FCi32,
	CreateRootSignatureUnboundedStaticDescriptors = 0x4FDi32,
	CreateAmplificationShaderInvalidShaderByteCode = 0x4FEi32,
	CreateAmplificationShaderOutOfMemory = 0x4FFi32,
	CreateShaderCacheSession = 0x500i32,
	LiveShaderCacheSession = 0x501i32,
	DestroyShaderCacheSession = 0x502i32,
	CreateShaderCacheSessionInvalidArgs = 0x503i32,
	CreateShaderCacheSessionDisabled = 0x504i32,
	CreateShaderCacheSessionAlreadyOpen = 0x505i32,
	ShaderCacheControlDeveloperMode = 0x506i32,
	ShaderCacheControlInvalidFlags = 0x507i32,
	ShaderCacheControlStateAlreadySet = 0x508i32,
	ShaderCacheControlIgnoredFlag = 0x509i32,
	ShaderCacheSessionSToRevalueAlreadyPresent = 0x50Ai32,
	ShaderCacheSessionSToRevalueHashCollision = 0x50Bi32,
	ShaderCacheSessionSToRevalueCacheFull = 0x50Ci32,
	ShaderCacheSessionFindValueNotFound = 0x50Di32,
	ShaderCacheSessionCorrupt = 0x50Ei32,
	ShaderCacheSessionDisabled = 0x50Fi32,
	OversizedDispatch    = 0x510i32,
	CreateVideoEncoder   = 0x511i32,
	LiveVideoEncoder     = 0x512i32,
	DestroyVideoEncoder  = 0x513i32,
	CreateVideoEncoderHeap = 0x514i32,
	LiveVideoEncoderHeap = 0x515i32,
	DestroyVideoEncoderHeap = 0x516i32,
	CopyTextureRegionMismatchEncodeReferenceOnlyFlag = 0x517i32,
	CopyResourceMismatchEncodeReferenceOnlyFlag = 0x518i32,
	EncodeFrameInvalidParameters = 0x519i32,
	EncodeFrameUnsupportedParameters = 0x51Ai32,
	ResolveEncoderOutputMetaDataInvalidParameters = 0x51Bi32,
	ResolveEncoderOutputMetaDataUnsupportedParameters = 0x51Ci32,
	CreateVideoEncoderInvalidParameters = 0x51Di32,
	CreateVideoEncoderUnsupportedParameters = 0x51Ei32,
	CreateVideoEncoderHeapInvalidParameters = 0x51Fi32,
	CreateVideoEncoderHeapUnsupportedParameters = 0x520i32,
	CreateCommandListNullCommandAllocator = 0x521i32,
	ClearUnorderedAccessViewInvalidDescriptorHandle = 0x522i32,
	DescriptorHeapNotShaderVisible = 0x523i32,
	CreateBlendStateBLenDopWarning = 0x524i32,
	CreateBlendStateBLenDopAlphaWarning = 0x525i32,
	WriteCombinePerformanceWarning = 0x526i32,
	ResolveQueryInvalidQueryState = 0x527i32,
	SetPrivateDataNoAccess = 0x528i32,
	CommandListStaticDescriptorSamplerModeMismatch = 0x529i32,
	GetCopyAbleFootprintsUnsupportedBufferWidth = 0x52Ai32,
	CreateMeshShaderTopologyMismatch = 0x52Bi32,
	VRsSumCombinerRequiresCapability = 0x52Ci32,
	SettingShadingRateFromMsRequiresCapability = 0x52Di32,
	ShaderCacheSessionShaderCacheDeleteNotSupported = 0x52Ei32,
	ShaderCacheControlShaderCacheClearNotSupported = 0x52Fi32,
	D3D12MessagesEnd     = 0x530i32,
}

impl D3D12MessageId {
	pub const GetCopyAbleLayoutInvalidSubresourceRange: Self = unsafe { transmute(0x2E3i32) };
	pub const GetCopyAbleLayoutInvalidBaseOffset: Self = unsafe { transmute(0x2E4i32) };
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12MessageCallbackFlags
{
	None                 = 0x0i32,
	ReFilters            = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12AxisShadingRate
{
	_1X                  = 0x0i32,
	_2X                  = 0x1i32,
	_4X                  = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShadingRate
{
	_1X1                 = 0x0i32,
	_1X2                 = 0x1i32,
	_2X1                 = 0x4i32,
	_2X2                 = 0x5i32,
	_2X4                 = 0x6i32,
	_4X2                 = 0x9i32,
	_4X4                 = 0xAi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShadingRateCombiner
{
	PassThrough          = 0x0i32,
	Override             = 0x1i32,
	Min                  = 0x2i32,
	Max                  = 0x3i32,
	Sum                  = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3D12ShaderVersionType
{
	ShVerPixelShader     = 0x0i32,
	ShVerVertexShader    = 0x1i32,
	ShVerGeometryShader  = 0x2i32,
	ShVerHullShader      = 0x3i32,
	ShVerDomainShader    = 0x4i32,
	ShVerComputeShader   = 0x5i32,
	ShVerReserved0       = 0xFFF0i32,
}

