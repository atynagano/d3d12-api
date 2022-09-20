#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};


/// D3D12_COMMAND_LIST_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12CommandListType
{
	/// D3D12_COMMAND_LIST_TYPE_DIRECT = 0x0i32
	Direct               = 0x0i32,
	/// D3D12_COMMAND_LIST_TYPE_BUNDLE = 0x1i32
	Bundle               = 0x1i32,
	/// D3D12_COMMAND_LIST_TYPE_COMPUTE = 0x2i32
	Compute              = 0x2i32,
	/// D3D12_COMMAND_LIST_TYPE_COPY = 0x3i32
	Copy                 = 0x3i32,
	/// D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE = 0x4i32
	VideoDecode          = 0x4i32,
	/// D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS = 0x5i32
	VideoProcess         = 0x5i32,
	/// D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE = 0x6i32
	VideoEncode          = 0x6i32,
}

bitflags::bitflags! {
	/// D3D12_COMMAND_QUEUE_FLAGS
	#[repr(transparent)]
	pub struct D3D12CommandQueueFlags: u32 {
		/// D3D12_COMMAND_QUEUE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT = 0x1u32
		const DisableGpuTimeout    = 0x1u32;
	}
}

/// D3D12_COMMAND_QUEUE_PRIORITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12CommandQueuePriority
{
	/// D3D12_COMMAND_QUEUE_PRIORITY_NORMAL = 0x0i32
	Normal               = 0x0i32,
	/// D3D12_COMMAND_QUEUE_PRIORITY_HIGH = 0x64i32
	High                 = 0x64i32,
	/// D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME = 0x2710i32
	GlobalRealTime       = 0x2710i32,
}

/// D3D12_PRIMITIVE_TOPOLOGY_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12PrimitiveTopologyType
{
	/// D3D12_PRIMITIVE_TOPOLOGY_TYPE_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT = 0x1i32
	Point                = 0x1i32,
	/// D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE = 0x2i32
	Line                 = 0x2i32,
	/// D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE = 0x3i32
	Triangle             = 0x3i32,
	/// D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH = 0x4i32
	Patch                = 0x4i32,
}

/// D3D12_INPUT_CLASSIFICATION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12InputClassification
{
	/// D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA = 0x0i32
	PerVertexData        = 0x0i32,
	/// D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA = 0x1i32
	PerInstanceData      = 0x1i32,
}

/// D3D12_FILL_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12FillMode
{
	/// D3D12_FILL_MODE_WIREFRAME = 0x2i32
	WireFrame            = 0x2i32,
	/// D3D12_FILL_MODE_SOLID = 0x3i32
	Solid                = 0x3i32,
}

/// D3D12_CULL_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12CullMode
{
	/// D3D12_CULL_MODE_NONE = 0x1i32
	None                 = 0x1i32,
	/// D3D12_CULL_MODE_FRONT = 0x2i32
	Front                = 0x2i32,
	/// D3D12_CULL_MODE_BACK = 0x3i32
	Back                 = 0x3i32,
}

/// D3D12_COMPARISON_FUNC
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ComparisonFunc
{
	/// D3D12_COMPARISON_FUNC_NEVER = 0x1i32
	Never                = 0x1i32,
	/// D3D12_COMPARISON_FUNC_LESS = 0x2i32
	Less                 = 0x2i32,
	/// D3D12_COMPARISON_FUNC_EQUAL = 0x3i32
	Equal                = 0x3i32,
	/// D3D12_COMPARISON_FUNC_LESS_EQUAL = 0x4i32
	LessEqual            = 0x4i32,
	/// D3D12_COMPARISON_FUNC_GREATER = 0x5i32
	Greater              = 0x5i32,
	/// D3D12_COMPARISON_FUNC_NOT_EQUAL = 0x6i32
	NotEqual             = 0x6i32,
	/// D3D12_COMPARISON_FUNC_GREATER_EQUAL = 0x7i32
	GreaterEqual         = 0x7i32,
	/// D3D12_COMPARISON_FUNC_ALWAYS = 0x8i32
	Always               = 0x8i32,
}

/// D3D12_DEPTH_WRITE_MASK
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DepthWriteMask
{
	/// D3D12_DEPTH_WRITE_MASK_ZERO = 0x0i32
	Zero                 = 0x0i32,
	/// D3D12_DEPTH_WRITE_MASK_ALL = 0x1i32
	All                  = 0x1i32,
}

/// D3D12_STENCIL_OP
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12StencilOp
{
	/// D3D12_STENCIL_OP_KEEP = 0x1i32
	Keep                 = 0x1i32,
	/// D3D12_STENCIL_OP_ZERO = 0x2i32
	Zero                 = 0x2i32,
	/// D3D12_STENCIL_OP_REPLACE = 0x3i32
	Replace              = 0x3i32,
	/// D3D12_STENCIL_OP_INCR_SAT = 0x4i32
	IncrSat              = 0x4i32,
	/// D3D12_STENCIL_OP_DECR_SAT = 0x5i32
	DecRSat              = 0x5i32,
	/// D3D12_STENCIL_OP_INVERT = 0x6i32
	Invert               = 0x6i32,
	/// D3D12_STENCIL_OP_INCR = 0x7i32
	Incr                 = 0x7i32,
	/// D3D12_STENCIL_OP_DECR = 0x8i32
	DecR                 = 0x8i32,
}

/// D3D12_BLEND
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12Blend
{
	/// D3D12_BLEND_ZERO = 0x1i32
	Zero                 = 0x1i32,
	/// D3D12_BLEND_ONE = 0x2i32
	One                  = 0x2i32,
	/// D3D12_BLEND_SRC_COLOR = 0x3i32
	SrcColor             = 0x3i32,
	/// D3D12_BLEND_INV_SRC_COLOR = 0x4i32
	InvSrcColor          = 0x4i32,
	/// D3D12_BLEND_SRC_ALPHA = 0x5i32
	SrcAlpha             = 0x5i32,
	/// D3D12_BLEND_INV_SRC_ALPHA = 0x6i32
	InvSrcAlpha          = 0x6i32,
	/// D3D12_BLEND_DEST_ALPHA = 0x7i32
	DestAlpha            = 0x7i32,
	/// D3D12_BLEND_INV_DEST_ALPHA = 0x8i32
	InvDestAlpha         = 0x8i32,
	/// D3D12_BLEND_DEST_COLOR = 0x9i32
	DestColor            = 0x9i32,
	/// D3D12_BLEND_INV_DEST_COLOR = 0xAi32
	InvDestColor         = 0xAi32,
	/// D3D12_BLEND_SRC_ALPHA_SAT = 0xBi32
	SrcAlphaSat          = 0xBi32,
	/// D3D12_BLEND_BLEND_FACTOR = 0xEi32
	BlendFactor          = 0xEi32,
	/// D3D12_BLEND_INV_BLEND_FACTOR = 0xFi32
	InvBlendFactor       = 0xFi32,
	/// D3D12_BLEND_SRC1_COLOR = 0x10i32
	Src1Color            = 0x10i32,
	/// D3D12_BLEND_INV_SRC1_COLOR = 0x11i32
	InvSrc1Color         = 0x11i32,
	/// D3D12_BLEND_SRC1_ALPHA = 0x12i32
	Src1Alpha            = 0x12i32,
	/// D3D12_BLEND_INV_SRC1_ALPHA = 0x13i32
	InvSrc1Alpha         = 0x13i32,
}

/// D3D12_BLEND_OP
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12BlendOp
{
	/// D3D12_BLEND_OP_ADD = 0x1i32
	Add                  = 0x1i32,
	/// D3D12_BLEND_OP_SUBTRACT = 0x2i32
	Subtract             = 0x2i32,
	/// D3D12_BLEND_OP_REV_SUBTRACT = 0x3i32
	RevSubtract          = 0x3i32,
	/// D3D12_BLEND_OP_MIN = 0x4i32
	Min                  = 0x4i32,
	/// D3D12_BLEND_OP_MAX = 0x5i32
	Max                  = 0x5i32,
}

/// D3D12_COLOR_WRITE_ENABLE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ColorWriteEnable
{
	/// D3D12_COLOR_WRITE_ENABLE_RED = 0x1i32
	Red                  = 0x1i32,
	/// D3D12_COLOR_WRITE_ENABLE_GREEN = 0x2i32
	Green                = 0x2i32,
	/// D3D12_COLOR_WRITE_ENABLE_BLUE = 0x4i32
	Blue                 = 0x4i32,
	/// D3D12_COLOR_WRITE_ENABLE_ALPHA = 0x8i32
	Alpha                = 0x8i32,
	/// D3D12_COLOR_WRITE_ENABLE_ALL = 0xFi32
	All                  = 0xFi32,
}

/// D3D12_LOGIC_OP
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12LogicOp
{
	/// D3D12_LOGIC_OP_CLEAR = 0x0i32
	Clear                = 0x0i32,
	/// D3D12_LOGIC_OP_SET = 0x1i32
	Set                  = 0x1i32,
	/// D3D12_LOGIC_OP_COPY = 0x2i32
	Copy                 = 0x2i32,
	/// D3D12_LOGIC_OP_COPY_INVERTED = 0x3i32
	CopyInverted         = 0x3i32,
	/// D3D12_LOGIC_OP_NOOP = 0x4i32
	Noop                 = 0x4i32,
	/// D3D12_LOGIC_OP_INVERT = 0x5i32
	Invert               = 0x5i32,
	/// D3D12_LOGIC_OP_AND = 0x6i32
	And                  = 0x6i32,
	/// D3D12_LOGIC_OP_NAND = 0x7i32
	NAnd                 = 0x7i32,
	/// D3D12_LOGIC_OP_OR = 0x8i32
	Or                   = 0x8i32,
	/// D3D12_LOGIC_OP_NOR = 0x9i32
	Nor                  = 0x9i32,
	/// D3D12_LOGIC_OP_XOR = 0xAi32
	Xor                  = 0xAi32,
	/// D3D12_LOGIC_OP_EQUIV = 0xBi32
	Equiv                = 0xBi32,
	/// D3D12_LOGIC_OP_AND_REVERSE = 0xCi32
	AndReverse           = 0xCi32,
	/// D3D12_LOGIC_OP_AND_INVERTED = 0xDi32
	AndInverted          = 0xDi32,
	/// D3D12_LOGIC_OP_OR_REVERSE = 0xEi32
	OrReverse            = 0xEi32,
	/// D3D12_LOGIC_OP_OR_INVERTED = 0xFi32
	OrInverted           = 0xFi32,
}

/// D3D12_CONSERVATIVE_RASTERIZATION_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ConservativeRasterizationMode
{
	/// D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF = 0x0i32
	Off                  = 0x0i32,
	/// D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON = 0x1i32
	On                   = 0x1i32,
}

/// D3D12_INDEX_BUFFER_STRIP_CUT_VALUE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12IndexBufferStripCutValue
{
	/// D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED = 0x0i32
	Disabled             = 0x0i32,
	/// D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF = 0x1i32
	_0xFFFF              = 0x1i32,
	/// D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF = 0x2i32
	_0xFFFFFFFF          = 0x2i32,
}

bitflags::bitflags! {
	/// D3D12_PIPELINE_STATE_FLAGS
	#[repr(transparent)]
	pub struct D3D12PipelineStateFlags: u32 {
		/// D3D12_PIPELINE_STATE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_PIPELINE_STATE_FLAG_TOOL_DEBUG = 0x1u32
		const ToolDebug            = 0x1u32;
	}
}

/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12PipelineStateSubobjectType
{
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE = 0x0i32
	RootSignature        = 0x0i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS = 0x1i32
	Vs                   = 0x1i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS = 0x2i32
	Ps                   = 0x2i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS = 0x3i32
	Ds                   = 0x3i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS = 0x4i32
	Hs                   = 0x4i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS = 0x5i32
	Gs                   = 0x5i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CS = 0x6i32
	Cs                   = 0x6i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT = 0x7i32
	StreamOutput         = 0x7i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND = 0x8i32
	Blend                = 0x8i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK = 0x9i32
	SampleMask           = 0x9i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER = 0xAi32
	Rasterizer           = 0xAi32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL = 0xBi32
	DepthStencil         = 0xBi32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT = 0xCi32
	InputLayout          = 0xCi32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE = 0xDi32
	IbStripCutValue      = 0xDi32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY = 0xEi32
	PrimitiveTopology    = 0xEi32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS = 0xFi32
	RenderTargetFormats  = 0xFi32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT = 0x10i32
	DepthStencilFormat   = 0x10i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC = 0x11i32
	SampleDesc           = 0x11i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK = 0x12i32
	NodeMask             = 0x12i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO = 0x13i32
	CachedPso            = 0x13i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS = 0x14i32
	Flags                = 0x14i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1 = 0x15i32
	DepthStencil1        = 0x15i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VIEW_INSTANCING = 0x16i32
	ViewInstancing       = 0x16i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_AS = 0x18i32
	As                   = 0x18i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MS = 0x19i32
	Ms                   = 0x19i32,
	/// D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MAX_VALID = 0x1Ai32
	MaxValid             = 0x1Ai32,
}

/// D3D12_FEATURE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12Feature
{
	/// D3D12_FEATURE_D3D12_OPTIONS = 0x0i32
	D3D12Options         = 0x0i32,
	/// D3D12_FEATURE_ARCHITECTURE = 0x1i32
	Architecture         = 0x1i32,
	/// D3D12_FEATURE_FEATURE_LEVELS = 0x2i32
	FeatureLevels        = 0x2i32,
	/// D3D12_FEATURE_FORMAT_SUPPORT = 0x3i32
	FormatSupport        = 0x3i32,
	/// D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS = 0x4i32
	MultiSampleQualityLevels = 0x4i32,
	/// D3D12_FEATURE_FORMAT_INFO = 0x5i32
	FormatInfo           = 0x5i32,
	/// D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT = 0x6i32
	GpuVirtualAddressSupport = 0x6i32,
	/// D3D12_FEATURE_SHADER_MODEL = 0x7i32
	ShaderModel          = 0x7i32,
	/// D3D12_FEATURE_D3D12_OPTIONS1 = 0x8i32
	D3D12Options1        = 0x8i32,
	/// D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_SUPPORT = 0xAi32
	ProtectedResourceSessionSupport = 0xAi32,
	/// D3D12_FEATURE_ROOT_SIGNATURE = 0xCi32
	RootSignature        = 0xCi32,
	/// D3D12_FEATURE_ARCHITECTURE1 = 0x10i32
	Architecture1        = 0x10i32,
	/// D3D12_FEATURE_D3D12_OPTIONS2 = 0x12i32
	D3D12Options2        = 0x12i32,
	/// D3D12_FEATURE_SHADER_CACHE = 0x13i32
	ShaderCache          = 0x13i32,
	/// D3D12_FEATURE_COMMAND_QUEUE_PRIORITY = 0x14i32
	CommandQueuePriority = 0x14i32,
	/// D3D12_FEATURE_D3D12_OPTIONS3 = 0x15i32
	D3D12Options3        = 0x15i32,
	/// D3D12_FEATURE_EXISTING_HEAPS = 0x16i32
	ExistingHeaps        = 0x16i32,
	/// D3D12_FEATURE_D3D12_OPTIONS4 = 0x17i32
	D3D12Options4        = 0x17i32,
	/// D3D12_FEATURE_SERIALIZATION = 0x18i32
	Serialization        = 0x18i32,
	/// D3D12_FEATURE_CROSS_NODE = 0x19i32
	CrossNode            = 0x19i32,
	/// D3D12_FEATURE_D3D12_OPTIONS5 = 0x1Bi32
	D3D12Options5        = 0x1Bi32,
	/// D3D12_FEATURE_DISPLAYABLE = 0x1Ci32
	Displayable          = 0x1Ci32,
	/// D3D12_FEATURE_D3D12_OPTIONS6 = 0x1Ei32
	D3D12Options6        = 0x1Ei32,
	/// D3D12_FEATURE_QUERY_META_COMMAND = 0x1Fi32
	QueryMetaCommand     = 0x1Fi32,
	/// D3D12_FEATURE_D3D12_OPTIONS7 = 0x20i32
	D3D12Options7        = 0x20i32,
	/// D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPE_COUNT = 0x21i32
	ProtectedResourceSessionTypeCount = 0x21i32,
	/// D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPES = 0x22i32
	ProtectedResourceSessionTypes = 0x22i32,
	/// D3D12_FEATURE_D3D12_OPTIONS8 = 0x24i32
	D3D12Options8        = 0x24i32,
	/// D3D12_FEATURE_D3D12_OPTIONS9 = 0x25i32
	D3D12Options9        = 0x25i32,
	/// D3D12_FEATURE_D3D12_OPTIONS10 = 0x27i32
	D3D12Options10       = 0x27i32,
	/// D3D12_FEATURE_D3D12_OPTIONS11 = 0x28i32
	D3D12Options11       = 0x28i32,
}

bitflags::bitflags! {
	/// D3D12_SHADER_MIN_PRECISION_SUPPORT
	#[repr(transparent)]
	pub struct D3D12ShaderMinPrecisionSupport: u32 {
		/// D3D12_SHADER_MIN_PRECISION_SUPPORT_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_SHADER_MIN_PRECISION_SUPPORT_10_BIT = 0x1u32
		const _10Bit               = 0x1u32;
		/// D3D12_SHADER_MIN_PRECISION_SUPPORT_16_BIT = 0x2u32
		const _16Bit               = 0x2u32;
	}
}

/// D3D12_TILED_RESOURCES_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12TiledResourcesTier
{
	/// D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_TILED_RESOURCES_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_TILED_RESOURCES_TIER_2 = 0x2i32
	_2                   = 0x2i32,
	/// D3D12_TILED_RESOURCES_TIER_3 = 0x3i32
	_3                   = 0x3i32,
	/// D3D12_TILED_RESOURCES_TIER_4 = 0x4i32
	_4                   = 0x4i32,
}

/// D3D12_RESOURCE_BINDING_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ResourceBindingTier
{
	/// D3D12_RESOURCE_BINDING_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_RESOURCE_BINDING_TIER_2 = 0x2i32
	_2                   = 0x2i32,
	/// D3D12_RESOURCE_BINDING_TIER_3 = 0x3i32
	_3                   = 0x3i32,
}

/// D3D12_CONSERVATIVE_RASTERIZATION_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ConservativeRasterizationTier
{
	/// D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_CONSERVATIVE_RASTERIZATION_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_CONSERVATIVE_RASTERIZATION_TIER_2 = 0x2i32
	_2                   = 0x2i32,
	/// D3D12_CONSERVATIVE_RASTERIZATION_TIER_3 = 0x3i32
	_3                   = 0x3i32,
}

bitflags::bitflags! {
	/// D3D12_FORMAT_SUPPORT1
	#[repr(transparent)]
	pub struct D3D12FormatSupport1: u32 {
		/// D3D12_FORMAT_SUPPORT1_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_FORMAT_SUPPORT1_BUFFER = 0x1u32
		const Buffer               = 0x1u32;
		/// D3D12_FORMAT_SUPPORT1_IA_VERTEX_BUFFER = 0x2u32
		const IaVertexBuffer       = 0x2u32;
		/// D3D12_FORMAT_SUPPORT1_IA_INDEX_BUFFER = 0x4u32
		const IaIndexBuffer        = 0x4u32;
		/// D3D12_FORMAT_SUPPORT1_SO_BUFFER = 0x8u32
		const SoBuffer             = 0x8u32;
		/// D3D12_FORMAT_SUPPORT1_TEXTURE1D = 0x10u32
		const Texture1d            = 0x10u32;
		/// D3D12_FORMAT_SUPPORT1_TEXTURE2D = 0x20u32
		const Texture2d            = 0x20u32;
		/// D3D12_FORMAT_SUPPORT1_TEXTURE3D = 0x40u32
		const Texture3d            = 0x40u32;
		/// D3D12_FORMAT_SUPPORT1_TEXTURECUBE = 0x80u32
		const TextureCube          = 0x80u32;
		/// D3D12_FORMAT_SUPPORT1_SHADER_LOAD = 0x100u32
		const ShaderLoad           = 0x100u32;
		/// D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE = 0x200u32
		const ShaderSample         = 0x200u32;
		/// D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_COMPARISON = 0x400u32
		const ShaderSampleComparison = 0x400u32;
		/// D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_MONO_TEXT = 0x800u32
		const ShaderSampleMonoText = 0x800u32;
		/// D3D12_FORMAT_SUPPORT1_MIP = 0x1000u32
		const Mip                  = 0x1000u32;
		/// D3D12_FORMAT_SUPPORT1_RENDER_TARGET = 0x4000u32
		const RenderTarget         = 0x4000u32;
		/// D3D12_FORMAT_SUPPORT1_BLENDABLE = 0x8000u32
		const BLendable            = 0x8000u32;
		/// D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL = 0x10000u32
		const DepthStencil         = 0x10000u32;
		/// D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RESOLVE = 0x40000u32
		const MultiSampleResolve   = 0x40000u32;
		/// D3D12_FORMAT_SUPPORT1_DISPLAY = 0x80000u32
		const Display              = 0x80000u32;
		/// D3D12_FORMAT_SUPPORT1_CAST_WITHIN_BIT_LAYOUT = 0x100000u32
		const CastWithinBitLayout  = 0x100000u32;
		/// D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RENDERTARGET = 0x200000u32
		const MultiSampleRenderTarget = 0x200000u32;
		/// D3D12_FORMAT_SUPPORT1_MULTISAMPLE_LOAD = 0x400000u32
		const MultiSampleLoad      = 0x400000u32;
		/// D3D12_FORMAT_SUPPORT1_SHADER_GATHER = 0x800000u32
		const ShaderGather         = 0x800000u32;
		/// D3D12_FORMAT_SUPPORT1_BACK_BUFFER_CAST = 0x1000000u32
		const BackBufferCast       = 0x1000000u32;
		/// D3D12_FORMAT_SUPPORT1_TYPED_UNORDERED_ACCESS_VIEW = 0x2000000u32
		const TypedUnorderedAccessView = 0x2000000u32;
		/// D3D12_FORMAT_SUPPORT1_TYPED_UNORDERED_ACCESS_VIEW
		const TypedUav             = Self::TypedUnorderedAccessView.bits;
		/// D3D12_FORMAT_SUPPORT1_SHADER_GATHER_COMPARISON = 0x4000000u32
		const ShaderGatherComparison = 0x4000000u32;
		/// D3D12_FORMAT_SUPPORT1_DECODER_OUTPUT = 0x8000000u32
		const DecoderOutput        = 0x8000000u32;
		/// D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_OUTPUT = 0x10000000u32
		const VideoProcessorOutput = 0x10000000u32;
		/// D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_INPUT = 0x20000000u32
		const VideoProcessorInput  = 0x20000000u32;
		/// D3D12_FORMAT_SUPPORT1_VIDEO_ENCODER = 0x40000000u32
		const VideoEncoder         = 0x40000000u32;
	}
}

bitflags::bitflags! {
	/// D3D12_FORMAT_SUPPORT2
	#[repr(transparent)]
	pub struct D3D12FormatSupport2: u32 {
		/// D3D12_FORMAT_SUPPORT2_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_ADD = 0x1u32
		const UavAtomicAdd         = 0x1u32;
		/// D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS = 0x2u32
		const UavAtomicBitwiseOps  = 0x2u32;
		/// D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE = 0x4u32
		const UavAtomicCompareStoreOrCompareExchange = 0x4u32;
		/// D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE = 0x8u32
		const UavAtomicExchange    = 0x8u32;
		/// D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX = 0x10u32
		const UavAtomicSignedMinOrMax = 0x10u32;
		/// D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX = 0x20u32
		const UavAtomicUnsignedMinOrMax = 0x20u32;
		/// D3D12_FORMAT_SUPPORT2_UAV_TYPED_LOAD = 0x40u32
		const UavTypedLoad         = 0x40u32;
		/// D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE = 0x80u32
		const UavTypedStore        = 0x80u32;
		/// D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP = 0x100u32
		const OutputMergerLogicOp  = 0x100u32;
		/// D3D12_FORMAT_SUPPORT2_TILED = 0x200u32
		const Tiled                = 0x200u32;
		/// D3D12_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY = 0x4000u32
		const MultiplaneOverlay    = 0x4000u32;
		/// D3D12_FORMAT_SUPPORT2_SAMPLER_FEEDBACK = 0x8000u32
		const SamplerFeedback      = 0x8000u32;
	}
}

bitflags::bitflags! {
	/// D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS
	#[repr(transparent)]
	pub struct D3D12MultiSampleQualityLevelFlags: u32 {
		/// D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_TILED_RESOURCE = 0x1u32
		const TiledResource        = 0x1u32;
	}
}

/// D3D12_CROSS_NODE_SHARING_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12CrossNodeSharingTier
{
	/// D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_CROSS_NODE_SHARING_TIER_1_EMULATED = 0x1i32
	_1Emulated           = 0x1i32,
	/// D3D12_CROSS_NODE_SHARING_TIER_1 = 0x2i32
	_1                   = 0x2i32,
	/// D3D12_CROSS_NODE_SHARING_TIER_2 = 0x3i32
	_2                   = 0x3i32,
	/// D3D12_CROSS_NODE_SHARING_TIER_3 = 0x4i32
	_3                   = 0x4i32,
}

/// D3D12_RESOURCE_HEAP_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ResourceHeapTier
{
	/// D3D12_RESOURCE_HEAP_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_RESOURCE_HEAP_TIER_2 = 0x2i32
	_2                   = 0x2i32,
}

/// D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ProgrammableSamplePositionsTier
{
	/// D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_2 = 0x2i32
	_2                   = 0x2i32,
}

/// D3D12_VIEW_INSTANCING_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ViewInstancingTier
{
	/// D3D12_VIEW_INSTANCING_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_VIEW_INSTANCING_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_VIEW_INSTANCING_TIER_2 = 0x2i32
	_2                   = 0x2i32,
	/// D3D12_VIEW_INSTANCING_TIER_3 = 0x3i32
	_3                   = 0x3i32,
}

/// D3D_ROOT_SIGNATURE_VERSION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DRootSignatureVersion
{
	/// D3D_ROOT_SIGNATURE_VERSION_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D_ROOT_SIGNATURE_VERSION_1_1 = 0x2i32
	_1_1                 = 0x2i32,
}

impl D3DRootSignatureVersion {
	/// D3D_ROOT_SIGNATURE_VERSION_1_0 = 0x1i32
	pub const _1_0                : Self = unsafe { transmute(0x1i32) };
}

/// D3D_SHADER_MODEL
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DShaderModel
{
	/// D3D_SHADER_MODEL_5_1 = 0x51i32
	_5_1                 = 0x51i32,
	/// D3D_SHADER_MODEL_6_0 = 0x60i32
	_6_0                 = 0x60i32,
	/// D3D_SHADER_MODEL_6_1 = 0x61i32
	_6_1                 = 0x61i32,
	/// D3D_SHADER_MODEL_6_2 = 0x62i32
	_6_2                 = 0x62i32,
	/// D3D_SHADER_MODEL_6_3 = 0x63i32
	_6_3                 = 0x63i32,
	/// D3D_SHADER_MODEL_6_4 = 0x64i32
	_6_4                 = 0x64i32,
	/// D3D_SHADER_MODEL_6_5 = 0x65i32
	_6_5                 = 0x65i32,
	/// D3D_SHADER_MODEL_6_6 = 0x66i32
	_6_6                 = 0x66i32,
	/// D3D_SHADER_MODEL_6_7 = 0x67i32
	_6_7                 = 0x67i32,
}

bitflags::bitflags! {
	/// D3D12_SHADER_CACHE_SUPPORT_FLAGS
	#[repr(transparent)]
	pub struct D3D12ShaderCacheSupportFlags: u32 {
		/// D3D12_SHADER_CACHE_SUPPORT_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_SHADER_CACHE_SUPPORT_SINGLE_PSO = 0x1u32
		const SinglePso            = 0x1u32;
		/// D3D12_SHADER_CACHE_SUPPORT_LIBRARY = 0x2u32
		const Library              = 0x2u32;
		/// D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE = 0x4u32
		const AutomaticInProcCache = 0x4u32;
		/// D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE = 0x8u32
		const AutomaticDiskCache   = 0x8u32;
		/// D3D12_SHADER_CACHE_SUPPORT_DRIVER_MANAGED_CACHE = 0x10u32
		const DriverManagedCache   = 0x10u32;
		/// D3D12_SHADER_CACHE_SUPPORT_SHADER_CONTROL_CLEAR = 0x20u32
		const ShaderControlClear   = 0x20u32;
		/// D3D12_SHADER_CACHE_SUPPORT_SHADER_SESSION_DELETE = 0x40u32
		const ShaderSessionDelete  = 0x40u32;
	}
}

bitflags::bitflags! {
	/// D3D12_COMMAND_LIST_SUPPORT_FLAGS
	#[repr(transparent)]
	pub struct D3D12CommandListSupportFlags: u32 {
		/// D3D12_COMMAND_LIST_SUPPORT_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_COMMAND_LIST_SUPPORT_FLAG_DIRECT = 0x1u32
		const Direct               = 0x1u32;
		/// D3D12_COMMAND_LIST_SUPPORT_FLAG_BUNDLE = 0x2u32
		const Bundle               = 0x2u32;
		/// D3D12_COMMAND_LIST_SUPPORT_FLAG_COMPUTE = 0x4u32
		const Compute              = 0x4u32;
		/// D3D12_COMMAND_LIST_SUPPORT_FLAG_COPY = 0x8u32
		const Copy                 = 0x8u32;
		/// D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_DECODE = 0x10u32
		const VideoDecode          = 0x10u32;
		/// D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_PROCESS = 0x20u32
		const VideoProcess         = 0x20u32;
		/// D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_ENCODE = 0x40u32
		const VideoEncode          = 0x40u32;
	}
}

/// D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12SharedResourceCompatibilityTier
{
	/// D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0 = 0x0i32
	_0                   = 0x0i32,
	/// D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_2 = 0x2i32
	_2                   = 0x2i32,
}

/// D3D12_HEAP_SERIALIZATION_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12HeapSerializationTier
{
	/// D3D12_HEAP_SERIALIZATION_TIER_0 = 0x0i32
	_0                   = 0x0i32,
	/// D3D12_HEAP_SERIALIZATION_TIER_10 = 0xAi32
	_10                  = 0xAi32,
}

/// D3D12_RENDER_PASS_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RenderPassTier
{
	/// D3D12_RENDER_PASS_TIER_0 = 0x0i32
	_0                   = 0x0i32,
	/// D3D12_RENDER_PASS_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_RENDER_PASS_TIER_2 = 0x2i32
	_2                   = 0x2i32,
}

/// D3D12_RAYTRACING_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RaytracingTier
{
	/// D3D12_RAYTRACING_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_RAYTRACING_TIER_1_0 = 0xAi32
	_1_0                 = 0xAi32,
	/// D3D12_RAYTRACING_TIER_1_1 = 0xBi32
	_1_1                 = 0xBi32,
}

/// D3D12_VARIABLE_SHADING_RATE_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12VariableShadingRateTier
{
	/// D3D12_VARIABLE_SHADING_RATE_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_VARIABLE_SHADING_RATE_TIER_1 = 0x1i32
	_1                   = 0x1i32,
	/// D3D12_VARIABLE_SHADING_RATE_TIER_2 = 0x2i32
	_2                   = 0x2i32,
}

/// D3D12_MESH_SHADER_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MeshShaderTier
{
	/// D3D12_MESH_SHADER_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_MESH_SHADER_TIER_1 = 0xAi32
	_1                   = 0xAi32,
}

/// D3D12_SAMPLER_FEEDBACK_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12SamplerFeedbackTier
{
	/// D3D12_SAMPLER_FEEDBACK_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_SAMPLER_FEEDBACK_TIER_0_9 = 0x5Ai32
	_0_9                 = 0x5Ai32,
	/// D3D12_SAMPLER_FEEDBACK_TIER_1_0 = 0x64i32
	_1_0                 = 0x64i32,
}

/// D3D12_WAVE_MMA_TIER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12WaveMmaTier
{
	/// D3D12_WAVE_MMA_TIER_NOT_SUPPORTED = 0x0i32
	NotSupported         = 0x0i32,
	/// D3D12_WAVE_MMA_TIER_1_0 = 0xAi32
	_1_0                 = 0xAi32,
}

/// D3D12_HEAP_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12HeapType
{
	/// D3D12_HEAP_TYPE_DEFAULT = 0x1i32
	Default              = 0x1i32,
	/// D3D12_HEAP_TYPE_UPLOAD = 0x2i32
	Upload               = 0x2i32,
	/// D3D12_HEAP_TYPE_READBACK = 0x3i32
	ReadBack             = 0x3i32,
	/// D3D12_HEAP_TYPE_CUSTOM = 0x4i32
	Custom               = 0x4i32,
}

/// D3D12_CPU_PAGE_PROPERTY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12CpuPageProperty
{
	/// D3D12_CPU_PAGE_PROPERTY_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE = 0x1i32
	NotAvailable         = 0x1i32,
	/// D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE = 0x2i32
	WriteCombine         = 0x2i32,
	/// D3D12_CPU_PAGE_PROPERTY_WRITE_BACK = 0x3i32
	WriteBack            = 0x3i32,
}

/// D3D12_MEMORY_POOL
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MemoryPool
{
	/// D3D12_MEMORY_POOL_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_MEMORY_POOL_L0 = 0x1i32
	L0                   = 0x1i32,
	/// D3D12_MEMORY_POOL_L1 = 0x2i32
	L1                   = 0x2i32,
}

bitflags::bitflags! {
	/// D3D12_HEAP_FLAGS
	#[repr(transparent)]
	pub struct D3D12HeapFlags: u32 {
		/// D3D12_HEAP_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_HEAP_FLAG_SHARED = 0x1u32
		const Shared               = 0x1u32;
		/// D3D12_HEAP_FLAG_DENY_BUFFERS = 0x4u32
		const DenyBuffers          = 0x4u32;
		/// D3D12_HEAP_FLAG_ALLOW_DISPLAY = 0x8u32
		const AllowDisplay         = 0x8u32;
		/// D3D12_HEAP_FLAG_SHARED_CROSS_ADAPTER = 0x20u32
		const SharedCrossAdapter   = 0x20u32;
		/// D3D12_HEAP_FLAG_DENY_RT_DS_TEXTURES = 0x40u32
		const DenyRtDsTextures     = 0x40u32;
		/// D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES = 0x80u32
		const DenyNonRtDsTextures  = 0x80u32;
		/// D3D12_HEAP_FLAG_HARDWARE_PROTECTED = 0x100u32
		const HardwareProtected    = 0x100u32;
		/// D3D12_HEAP_FLAG_ALLOW_WRITE_WATCH = 0x200u32
		const AllowWriteWatch      = 0x200u32;
		/// D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS = 0x400u32
		const AllowShaderAtomics   = 0x400u32;
		/// D3D12_HEAP_FLAG_CREATE_NOT_RESIDENT = 0x800u32
		const CreateNotResident    = 0x800u32;
		/// D3D12_HEAP_FLAG_CREATE_NOT_ZEROED = 0x1000u32
		const CreateNotZeroed      = 0x1000u32;
		/// D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES = 0x0u32
		const AllowAllBuffersAndTextures = 0x0u32;
		/// D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS = 0xC0u32
		const AllowOnlyBuffers     = 0xC0u32;
		/// D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES = 0x44u32
		const AllowOnlyNonRtDsTextures = 0x44u32;
		/// D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES = 0x84u32
		const AllowOnlyRtDsTextures = 0x84u32;
	}
}

/// D3D12_RESOURCE_DIMENSION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ResourceDimension
{
	/// D3D12_RESOURCE_DIMENSION_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_RESOURCE_DIMENSION_BUFFER = 0x1i32
	Buffer               = 0x1i32,
	/// D3D12_RESOURCE_DIMENSION_TEXTURE1D = 0x2i32
	Texture1d            = 0x2i32,
	/// D3D12_RESOURCE_DIMENSION_TEXTURE2D = 0x3i32
	Texture2d            = 0x3i32,
	/// D3D12_RESOURCE_DIMENSION_TEXTURE3D = 0x4i32
	Texture3d            = 0x4i32,
}

/// D3D12_TEXTURE_LAYOUT
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12TextureLayout
{
	/// D3D12_TEXTURE_LAYOUT_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_TEXTURE_LAYOUT_ROW_MAJOR = 0x1i32
	RowMajor             = 0x1i32,
	/// D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE = 0x2i32
	_64KBUndefinedSwizzle = 0x2i32,
	/// D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE = 0x3i32
	_64KBStandardSwizzle = 0x3i32,
}

bitflags::bitflags! {
	/// D3D12_RESOURCE_FLAGS
	#[repr(transparent)]
	pub struct D3D12ResourceFlags: u32 {
		/// D3D12_RESOURCE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET = 0x1u32
		const AllowRenderTarget    = 0x1u32;
		/// D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL = 0x2u32
		const AllowDepthStencil    = 0x2u32;
		/// D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS = 0x4u32
		const AllowUnorderedAccess = 0x4u32;
		/// D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE = 0x8u32
		const DenyShaderResource   = 0x8u32;
		/// D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER = 0x10u32
		const AllowCrossAdapter    = 0x10u32;
		/// D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS = 0x20u32
		const AllowSimultaneousAccess = 0x20u32;
		/// D3D12_RESOURCE_FLAG_VIDEO_DECODE_REFERENCE_ONLY = 0x40u32
		const VideoDecodeReferenceOnly = 0x40u32;
		/// D3D12_RESOURCE_FLAG_VIDEO_ENCODE_REFERENCE_ONLY = 0x80u32
		const VideoEncodeReferenceOnly = 0x80u32;
	}
}

/// D3D12_TILE_RANGE_FLAGS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12TileRangeFlags
{
	/// D3D12_TILE_RANGE_FLAG_NONE = 0x0i32
	None                 = 0x0i32,
	/// D3D12_TILE_RANGE_FLAG_NULL = 0x1i32
	Null                 = 0x1i32,
	/// D3D12_TILE_RANGE_FLAG_SKIP = 0x2i32
	Skip                 = 0x2i32,
	/// D3D12_TILE_RANGE_FLAG_REUSE_SINGLE_TILE = 0x4i32
	ReuseSingleTile      = 0x4i32,
}

bitflags::bitflags! {
	/// D3D12_TILE_MAPPING_FLAGS
	#[repr(transparent)]
	pub struct D3D12TileMappingFlags: u32 {
		/// D3D12_TILE_MAPPING_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_TILE_MAPPING_FLAG_NO_HAZARD = 0x1u32
		const NoHazard             = 0x1u32;
	}
}

bitflags::bitflags! {
	/// D3D12_TILE_COPY_FLAGS
	#[repr(transparent)]
	pub struct D3D12TileCopyFlags: u32 {
		/// D3D12_TILE_COPY_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_TILE_COPY_FLAG_NO_HAZARD = 0x1u32
		const NoHazard             = 0x1u32;
		/// D3D12_TILE_COPY_FLAG_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE = 0x2u32
		const LinearBufferToSwizzledTiledResource = 0x2u32;
		/// D3D12_TILE_COPY_FLAG_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER = 0x4u32
		const SwizzledTiledResourceToLinearBuffer = 0x4u32;
	}
}

bitflags::bitflags! {
	/// D3D12_RESOURCE_STATES
	#[repr(transparent)]
	pub struct D3D12ResourceStates: u32 {
		/// D3D12_RESOURCE_STATE_COMMON = 0x0u32
		const Common               = 0x0u32;
		/// D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER = 0x1u32
		const VertexAndConstantBuffer = 0x1u32;
		/// D3D12_RESOURCE_STATE_INDEX_BUFFER = 0x2u32
		const IndexBuffer          = 0x2u32;
		/// D3D12_RESOURCE_STATE_RENDER_TARGET = 0x4u32
		const RenderTarget         = 0x4u32;
		/// D3D12_RESOURCE_STATE_UNORDERED_ACCESS = 0x8u32
		const UnorderedAccess      = 0x8u32;
		/// D3D12_RESOURCE_STATE_DEPTH_WRITE = 0x10u32
		const DepthWrite           = 0x10u32;
		/// D3D12_RESOURCE_STATE_DEPTH_READ = 0x20u32
		const DepthRead            = 0x20u32;
		/// D3D12_RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE = 0x40u32
		const NonPixelShaderResource = 0x40u32;
		/// D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE = 0x80u32
		const PixelShaderResource  = 0x80u32;
		/// D3D12_RESOURCE_STATE_STREAM_OUT = 0x100u32
		const StreamOut            = 0x100u32;
		/// D3D12_RESOURCE_STATE_INDIRECT_ARGUMENT = 0x200u32
		const IndirectArgument     = 0x200u32;
		/// D3D12_RESOURCE_STATE_COPY_DEST = 0x400u32
		const CopyDest             = 0x400u32;
		/// D3D12_RESOURCE_STATE_COPY_SOURCE = 0x800u32
		const CopySource           = 0x800u32;
		/// D3D12_RESOURCE_STATE_RESOLVE_DEST = 0x1000u32
		const ResolveDest          = 0x1000u32;
		/// D3D12_RESOURCE_STATE_RESOLVE_SOURCE = 0x2000u32
		const ResolveSource        = 0x2000u32;
		/// D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE = 0x400000u32
		const RaytracingAccelerationStructure = 0x400000u32;
		/// D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE
		const RtAs                 = Self::RaytracingAccelerationStructure.bits;
		/// D3D12_RESOURCE_STATE_SHADING_RATE_SOURCE = 0x1000000u32
		const ShadingRateSource    = 0x1000000u32;
		/// D3D12_RESOURCE_STATE_GENERIC_READ = 0xAC3u32
		const GenericRead          = 0xAC3u32;
		/// D3D12_RESOURCE_STATE_ALL_SHADER_RESOURCE = 0xC0u32
		const AllShaderResource    = 0xC0u32;
		/// D3D12_RESOURCE_STATE_PRESENT = 0x0u32
		const Present              = 0x0u32;
		/// D3D12_RESOURCE_STATE_PREDICATION = 0x200u32
		const Predication          = 0x200u32;
		/// D3D12_RESOURCE_STATE_VIDEO_DECODE_READ = 0x10000u32
		const VideoDecodeRead      = 0x10000u32;
		/// D3D12_RESOURCE_STATE_VIDEO_DECODE_WRITE = 0x20000u32
		const VideoDecodeWrite     = 0x20000u32;
		/// D3D12_RESOURCE_STATE_VIDEO_PROCESS_READ = 0x40000u32
		const VideoProcessRead     = 0x40000u32;
		/// D3D12_RESOURCE_STATE_VIDEO_PROCESS_WRITE = 0x80000u32
		const VideoProcessWrite    = 0x80000u32;
		/// D3D12_RESOURCE_STATE_VIDEO_ENCODE_READ = 0x200000u32
		const VideoEncodeRead      = 0x200000u32;
		/// D3D12_RESOURCE_STATE_VIDEO_ENCODE_WRITE = 0x800000u32
		const VideoEncodeWrite     = 0x800000u32;
	}
}

/// D3D12_RESOURCE_BARRIER_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ResourceBarrierType
{
	/// D3D12_RESOURCE_BARRIER_TYPE_TRANSITION = 0x0i32
	Transition           = 0x0i32,
	/// D3D12_RESOURCE_BARRIER_TYPE_ALIASING = 0x1i32
	Aliasing             = 0x1i32,
	/// D3D12_RESOURCE_BARRIER_TYPE_UAV = 0x2i32
	Uav                  = 0x2i32,
}

bitflags::bitflags! {
	/// D3D12_RESOURCE_BARRIER_FLAGS
	#[repr(transparent)]
	pub struct D3D12ResourceBarrierFlags: u32 {
		/// D3D12_RESOURCE_BARRIER_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY = 0x1u32
		const BeginOnly            = 0x1u32;
		/// D3D12_RESOURCE_BARRIER_FLAG_END_ONLY = 0x2u32
		const EndOnly              = 0x2u32;
	}
}

/// D3D12_TEXTURE_COPY_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12TextureCopyType
{
	/// D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX = 0x0i32
	SubresourceIndex     = 0x0i32,
	/// D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT = 0x1i32
	PlacedFootprint      = 0x1i32,
}

/// D3D12_RESOLVE_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ResolveMode
{
	/// D3D12_RESOLVE_MODE_DECOMPRESS = 0x0i32
	Decompress           = 0x0i32,
	/// D3D12_RESOLVE_MODE_MIN = 0x1i32
	Min                  = 0x1i32,
	/// D3D12_RESOLVE_MODE_MAX = 0x2i32
	Max                  = 0x2i32,
	/// D3D12_RESOLVE_MODE_AVERAGE = 0x3i32
	Average              = 0x3i32,
	/// D3D12_RESOLVE_MODE_ENCODE_SAMPLER_FEEDBACK = 0x4i32
	EncodeSamplerFeedback = 0x4i32,
	/// D3D12_RESOLVE_MODE_DECODE_SAMPLER_FEEDBACK = 0x5i32
	DecodeSamplerFeedback = 0x5i32,
}

bitflags::bitflags! {
	/// D3D12_VIEW_INSTANCING_FLAGS
	#[repr(transparent)]
	pub struct D3D12ViewInstancingFlags: u32 {
		/// D3D12_VIEW_INSTANCING_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_VIEW_INSTANCING_FLAG_ENABLE_VIEW_INSTANCE_MASKING = 0x1u32
		const EnableViewInstanceMasking = 0x1u32;
	}
}

bitflags::bitflags! {
	/// D3D12_BUFFER_SRV_FLAGS
	#[repr(transparent)]
	pub struct D3D12BufferSrvFlags: u32 {
		/// D3D12_BUFFER_SRV_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_BUFFER_SRV_FLAG_RAW = 0x1u32
		const Raw                  = 0x1u32;
	}
}

/// D3D12_SRV_DIMENSION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12SrvDimension
{
	/// D3D12_SRV_DIMENSION_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_SRV_DIMENSION_BUFFER = 0x1i32
	Buffer               = 0x1i32,
	/// D3D12_SRV_DIMENSION_TEXTURE1D = 0x2i32
	Texture1d            = 0x2i32,
	/// D3D12_SRV_DIMENSION_TEXTURE1DARRAY = 0x3i32
	Texture1dArray       = 0x3i32,
	/// D3D12_SRV_DIMENSION_TEXTURE2D = 0x4i32
	Texture2d            = 0x4i32,
	/// D3D12_SRV_DIMENSION_TEXTURE2DARRAY = 0x5i32
	Texture2dArray       = 0x5i32,
	/// D3D12_SRV_DIMENSION_TEXTURE2DMS = 0x6i32
	Texture2dMs          = 0x6i32,
	/// D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY = 0x7i32
	Texture2dMsArray     = 0x7i32,
	/// D3D12_SRV_DIMENSION_TEXTURE3D = 0x8i32
	Texture3d            = 0x8i32,
	/// D3D12_SRV_DIMENSION_TEXTURECUBE = 0x9i32
	TextureCube          = 0x9i32,
	/// D3D12_SRV_DIMENSION_TEXTURECUBEARRAY = 0xAi32
	TextureCubeArray     = 0xAi32,
	/// D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE = 0xBi32
	RaytracingAccelerationStructure = 0xBi32,
}

impl D3D12SrvDimension {
	/// D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE
	pub const RtAs                : Self = Self::RaytracingAccelerationStructure;
}

/// D3D12_FILTER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12Filter
{
	/// D3D12_FILTER_MIN_MAG_MIP_POINT = 0x0i32
	MinMagMipPoint       = 0x0i32,
	/// D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR = 0x1i32
	MinMagPointMipLinear = 0x1i32,
	/// D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x4i32
	MinPointMagLinearMipPoint = 0x4i32,
	/// D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR = 0x5i32
	MinPointMagMipLinear = 0x5i32,
	/// D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT = 0x10i32
	MinLinearMagMipPoint = 0x10i32,
	/// D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x11i32
	MinLinearMagPointMipLinear = 0x11i32,
	/// D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT = 0x14i32
	MinMagLinearMipPoint = 0x14i32,
	/// D3D12_FILTER_MIN_MAG_MIP_LINEAR = 0x15i32
	MinMagMipLinear      = 0x15i32,
	/// D3D12_FILTER_ANISOTROPIC = 0x55i32
	Anisotropic          = 0x55i32,
	/// D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT = 0x80i32
	ComparisonMinMagMipPoint = 0x80i32,
	/// D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR = 0x81i32
	ComparisonMinMagPointMipLinear = 0x81i32,
	/// D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x84i32
	ComparisonMinPointMagLinearMipPoint = 0x84i32,
	/// D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR = 0x85i32
	ComparisonMinPointMagMipLinear = 0x85i32,
	/// D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT = 0x90i32
	ComparisonMinLinearMagMipPoint = 0x90i32,
	/// D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x91i32
	ComparisonMinLinearMagPointMipLinear = 0x91i32,
	/// D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT = 0x94i32
	ComparisonMinMagLinearMipPoint = 0x94i32,
	/// D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR = 0x95i32
	ComparisonMinMagMipLinear = 0x95i32,
	/// D3D12_FILTER_COMPARISON_ANISOTROPIC = 0xD5i32
	ComparisonAnisotropic = 0xD5i32,
	/// D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT = 0x100i32
	MinimumMinMagMipPoint = 0x100i32,
	/// D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR = 0x101i32
	MinimumMinMagPointMipLinear = 0x101i32,
	/// D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x104i32
	MinimumMinPointMagLinearMipPoint = 0x104i32,
	/// D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR = 0x105i32
	MinimumMinPointMagMipLinear = 0x105i32,
	/// D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT = 0x110i32
	MinimumMinLinearMagMipPoint = 0x110i32,
	/// D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x111i32
	MinimumMinLinearMagPointMipLinear = 0x111i32,
	/// D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT = 0x114i32
	MinimumMinMagLinearMipPoint = 0x114i32,
	/// D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR = 0x115i32
	MinimumMinMagMipLinear = 0x115i32,
	/// D3D12_FILTER_MINIMUM_ANISOTROPIC = 0x155i32
	MinimumAnisotropic   = 0x155i32,
	/// D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT = 0x180i32
	MaximumMinMagMipPoint = 0x180i32,
	/// D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR = 0x181i32
	MaximumMinMagPointMipLinear = 0x181i32,
	/// D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x184i32
	MaximumMinPointMagLinearMipPoint = 0x184i32,
	/// D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR = 0x185i32
	MaximumMinPointMagMipLinear = 0x185i32,
	/// D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT = 0x190i32
	MaximumMinLinearMagMipPoint = 0x190i32,
	/// D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x191i32
	MaximumMinLinearMagPointMipLinear = 0x191i32,
	/// D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT = 0x194i32
	MaximumMinMagLinearMipPoint = 0x194i32,
	/// D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR = 0x195i32
	MaximumMinMagMipLinear = 0x195i32,
	/// D3D12_FILTER_MAXIMUM_ANISOTROPIC = 0x1D5i32
	MaximumAnisotropic   = 0x1D5i32,
}

/// D3D12_FILTER_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12FilterType
{
	/// D3D12_FILTER_TYPE_POINT = 0x0i32
	Point                = 0x0i32,
	/// D3D12_FILTER_TYPE_LINEAR = 0x1i32
	Linear               = 0x1i32,
}

/// D3D12_FILTER_REDUCTION_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12FilterReductionType
{
	/// D3D12_FILTER_REDUCTION_TYPE_STANDARD = 0x0i32
	Standard             = 0x0i32,
	/// D3D12_FILTER_REDUCTION_TYPE_COMPARISON = 0x1i32
	Comparison           = 0x1i32,
	/// D3D12_FILTER_REDUCTION_TYPE_MINIMUM = 0x2i32
	Minimum              = 0x2i32,
	/// D3D12_FILTER_REDUCTION_TYPE_MAXIMUM = 0x3i32
	Maximum              = 0x3i32,
}

/// D3D12_TEXTURE_ADDRESS_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12TextureAddressMode
{
	/// D3D12_TEXTURE_ADDRESS_MODE_WRAP = 0x1i32
	Wrap                 = 0x1i32,
	/// D3D12_TEXTURE_ADDRESS_MODE_MIRROR = 0x2i32
	Mirror               = 0x2i32,
	/// D3D12_TEXTURE_ADDRESS_MODE_CLAMP = 0x3i32
	Clamp                = 0x3i32,
	/// D3D12_TEXTURE_ADDRESS_MODE_BORDER = 0x4i32
	Border               = 0x4i32,
	/// D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE = 0x5i32
	MirrorOnce           = 0x5i32,
}

bitflags::bitflags! {
	/// D3D12_BUFFER_UAV_FLAGS
	#[repr(transparent)]
	pub struct D3D12BufferUavFlags: u32 {
		/// D3D12_BUFFER_UAV_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_BUFFER_UAV_FLAG_RAW = 0x1u32
		const Raw                  = 0x1u32;
	}
}

/// D3D12_UAV_DIMENSION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12UavDimension
{
	/// D3D12_UAV_DIMENSION_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_UAV_DIMENSION_BUFFER = 0x1i32
	Buffer               = 0x1i32,
	/// D3D12_UAV_DIMENSION_TEXTURE1D = 0x2i32
	Texture1d            = 0x2i32,
	/// D3D12_UAV_DIMENSION_TEXTURE1DARRAY = 0x3i32
	Texture1dArray       = 0x3i32,
	/// D3D12_UAV_DIMENSION_TEXTURE2D = 0x4i32
	Texture2d            = 0x4i32,
	/// D3D12_UAV_DIMENSION_TEXTURE2DARRAY = 0x5i32
	Texture2dArray       = 0x5i32,
	/// D3D12_UAV_DIMENSION_TEXTURE3D = 0x8i32
	Texture3d            = 0x8i32,
}

/// D3D12_RTV_DIMENSION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RtvDimension
{
	/// D3D12_RTV_DIMENSION_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_RTV_DIMENSION_BUFFER = 0x1i32
	Buffer               = 0x1i32,
	/// D3D12_RTV_DIMENSION_TEXTURE1D = 0x2i32
	Texture1d            = 0x2i32,
	/// D3D12_RTV_DIMENSION_TEXTURE1DARRAY = 0x3i32
	Texture1dArray       = 0x3i32,
	/// D3D12_RTV_DIMENSION_TEXTURE2D = 0x4i32
	Texture2d            = 0x4i32,
	/// D3D12_RTV_DIMENSION_TEXTURE2DARRAY = 0x5i32
	Texture2dArray       = 0x5i32,
	/// D3D12_RTV_DIMENSION_TEXTURE2DMS = 0x6i32
	Texture2dMs          = 0x6i32,
	/// D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY = 0x7i32
	Texture2dMsArray     = 0x7i32,
	/// D3D12_RTV_DIMENSION_TEXTURE3D = 0x8i32
	Texture3d            = 0x8i32,
}

bitflags::bitflags! {
	/// D3D12_DSV_FLAGS
	#[repr(transparent)]
	pub struct D3D12DsvFlags: u32 {
		/// D3D12_DSV_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_DSV_FLAG_READ_ONLY_DEPTH = 0x1u32
		const ReadOnlyDepth        = 0x1u32;
		/// D3D12_DSV_FLAG_READ_ONLY_STENCIL = 0x2u32
		const ReadOnlyStencil      = 0x2u32;
	}
}

/// D3D12_DSV_DIMENSION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DsvDimension
{
	/// D3D12_DSV_DIMENSION_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_DSV_DIMENSION_TEXTURE1D = 0x1i32
	Texture1d            = 0x1i32,
	/// D3D12_DSV_DIMENSION_TEXTURE1DARRAY = 0x2i32
	Texture1dArray       = 0x2i32,
	/// D3D12_DSV_DIMENSION_TEXTURE2D = 0x3i32
	Texture2d            = 0x3i32,
	/// D3D12_DSV_DIMENSION_TEXTURE2DARRAY = 0x4i32
	Texture2dArray       = 0x4i32,
	/// D3D12_DSV_DIMENSION_TEXTURE2DMS = 0x5i32
	Texture2dMs          = 0x5i32,
	/// D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY = 0x6i32
	Texture2dMsArray     = 0x6i32,
}

bitflags::bitflags! {
	/// D3D12_CLEAR_FLAGS
	#[repr(transparent)]
	pub struct D3D12ClearFlags: u32 {
		/// D3D12_CLEAR_FLAG_DEPTH = 0x1u32
		const Depth                = 0x1u32;
		/// D3D12_CLEAR_FLAG_STENCIL = 0x2u32
		const Stencil              = 0x2u32;
	}
}

bitflags::bitflags! {
	/// D3D12_FENCE_FLAGS
	#[repr(transparent)]
	pub struct D3D12FenceFlags: u32 {
		/// D3D12_FENCE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_FENCE_FLAG_SHARED = 0x1u32
		const Shared               = 0x1u32;
		/// D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER = 0x2u32
		const SharedCrossAdapter   = 0x2u32;
		/// D3D12_FENCE_FLAG_NON_MONITORED = 0x4u32
		const NonMonitored         = 0x4u32;
	}
}

/// D3D12_DESCRIPTOR_HEAP_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DescriptorHeapType
{
	/// D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV = 0x0i32
	CbvSrvUav            = 0x0i32,
	/// D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER = 0x1i32
	Sampler              = 0x1i32,
	/// D3D12_DESCRIPTOR_HEAP_TYPE_RTV = 0x2i32
	Rtv                  = 0x2i32,
	/// D3D12_DESCRIPTOR_HEAP_TYPE_DSV = 0x3i32
	Dsv                  = 0x3i32,
	/// D3D12_DESCRIPTOR_HEAP_TYPE_NUM_TYPES = 0x4i32
	NumTypes             = 0x4i32,
}

bitflags::bitflags! {
	/// D3D12_DESCRIPTOR_HEAP_FLAGS
	#[repr(transparent)]
	pub struct D3D12DescriptorHeapFlags: u32 {
		/// D3D12_DESCRIPTOR_HEAP_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE = 0x1u32
		const ShaderVisible        = 0x1u32;
	}
}

/// D3D12_DESCRIPTOR_RANGE_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DescriptorRangeType
{
	/// D3D12_DESCRIPTOR_RANGE_TYPE_SRV = 0x0i32
	Srv                  = 0x0i32,
	/// D3D12_DESCRIPTOR_RANGE_TYPE_UAV = 0x1i32
	Uav                  = 0x1i32,
	/// D3D12_DESCRIPTOR_RANGE_TYPE_CBV = 0x2i32
	Cbv                  = 0x2i32,
	/// D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER = 0x3i32
	Sampler              = 0x3i32,
}

/// D3D12_SHADER_VISIBILITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ShaderVisibility
{
	/// D3D12_SHADER_VISIBILITY_ALL = 0x0i32
	All                  = 0x0i32,
	/// D3D12_SHADER_VISIBILITY_VERTEX = 0x1i32
	Vertex               = 0x1i32,
	/// D3D12_SHADER_VISIBILITY_HULL = 0x2i32
	Hull                 = 0x2i32,
	/// D3D12_SHADER_VISIBILITY_DOMAIN = 0x3i32
	Domain               = 0x3i32,
	/// D3D12_SHADER_VISIBILITY_GEOMETRY = 0x4i32
	Geometry             = 0x4i32,
	/// D3D12_SHADER_VISIBILITY_PIXEL = 0x5i32
	Pixel                = 0x5i32,
	/// D3D12_SHADER_VISIBILITY_AMPLIFICATION = 0x6i32
	Amplification        = 0x6i32,
	/// D3D12_SHADER_VISIBILITY_MESH = 0x7i32
	Mesh                 = 0x7i32,
}

/// D3D12_ROOT_PARAMETER_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RootParameterType
{
	/// D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE = 0x0i32
	DescriptorTable      = 0x0i32,
	/// D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS = 0x1i32
	_32BitConstants      = 0x1i32,
	/// D3D12_ROOT_PARAMETER_TYPE_CBV = 0x2i32
	Cbv                  = 0x2i32,
	/// D3D12_ROOT_PARAMETER_TYPE_SRV = 0x3i32
	Srv                  = 0x3i32,
	/// D3D12_ROOT_PARAMETER_TYPE_UAV = 0x4i32
	Uav                  = 0x4i32,
}

bitflags::bitflags! {
	/// D3D12_ROOT_SIGNATURE_FLAGS
	#[repr(transparent)]
	pub struct D3D12RootSignatureFlags: u32 {
		/// D3D12_ROOT_SIGNATURE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT = 0x1u32
		const AllowInputAssemblerInputLayout = 0x1u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS = 0x2u32
		const DenyVertexShaderRootAccess = 0x2u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS = 0x4u32
		const DenyHullShaderRootAccess = 0x4u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS = 0x8u32
		const DenyDomainShaderRootAccess = 0x8u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS = 0x10u32
		const DenyGeometryShaderRootAccess = 0x10u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS = 0x20u32
		const DenyPixelShaderRootAccess = 0x20u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT = 0x40u32
		const AllowStreamOutput    = 0x40u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE = 0x80u32
		const LocalRootSignature   = 0x80u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_DENY_AMPLIFICATION_SHADER_ROOT_ACCESS = 0x100u32
		const DenyAmplificationShaderRootAccess = 0x100u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_DENY_MESH_SHADER_ROOT_ACCESS = 0x200u32
		const DenyMeshShaderRootAccess = 0x200u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_CBV_SRV_UAV_HEAP_DIRECTLY_INDEXED = 0x400u32
		const CbvSrvUavHeapDirectlyIndexed = 0x400u32;
		/// D3D12_ROOT_SIGNATURE_FLAG_SAMPLER_HEAP_DIRECTLY_INDEXED = 0x800u32
		const SamplerHeapDirectlyIndexed = 0x800u32;
	}
}

/// D3D12_STATIC_BORDER_COLOR
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12StaticBorderColor
{
	/// D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK = 0x0i32
	TransparentBlack     = 0x0i32,
	/// D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK = 0x1i32
	OpaqueBlack          = 0x1i32,
	/// D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE = 0x2i32
	OpaqueWhite          = 0x2i32,
}

bitflags::bitflags! {
	/// D3D12_DESCRIPTOR_RANGE_FLAGS
	#[repr(transparent)]
	pub struct D3D12DescriptorRangeFlags: u32 {
		/// D3D12_DESCRIPTOR_RANGE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_VOLATILE = 0x1u32
		const DescriptorsVolatile  = 0x1u32;
		/// D3D12_DESCRIPTOR_RANGE_FLAG_DATA_VOLATILE = 0x2u32
		const DataVolatile         = 0x2u32;
		/// D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE = 0x4u32
		const DataStaticWhileSetAtExecute = 0x4u32;
		/// D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC = 0x8u32
		const DataStatic           = 0x8u32;
		/// D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_STATIC_KEEPING_BUFFER_BOUNDS_CHECKS = 0x10000u32
		const DescriptorsStaticKeepingBufferBoundsChecks = 0x10000u32;
	}
}

bitflags::bitflags! {
	/// D3D12_ROOT_DESCRIPTOR_FLAGS
	#[repr(transparent)]
	pub struct D3D12RootDescriptorFlags: u32 {
		/// D3D12_ROOT_DESCRIPTOR_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_ROOT_DESCRIPTOR_FLAG_DATA_VOLATILE = 0x2u32
		const DataVolatile         = 0x2u32;
		/// D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE = 0x4u32
		const DataStaticWhileSetAtExecute = 0x4u32;
		/// D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC = 0x8u32
		const DataStatic           = 0x8u32;
	}
}

/// D3D12_QUERY_HEAP_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12QueryHeapType
{
	/// D3D12_QUERY_HEAP_TYPE_OCCLUSION = 0x0i32
	Occlusion            = 0x0i32,
	/// D3D12_QUERY_HEAP_TYPE_TIMESTAMP = 0x1i32
	Timestamp            = 0x1i32,
	/// D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS = 0x2i32
	PipelineStatistics   = 0x2i32,
	/// D3D12_QUERY_HEAP_TYPE_SO_STATISTICS = 0x3i32
	SoStatistics         = 0x3i32,
	/// D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS = 0x4i32
	VideoDecodeStatistics = 0x4i32,
	/// D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP = 0x5i32
	CopyQueueTimestamp   = 0x5i32,
	/// D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS1 = 0x7i32
	PipelineStatistics1  = 0x7i32,
}

/// D3D12_QUERY_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12QueryType
{
	/// D3D12_QUERY_TYPE_OCCLUSION = 0x0i32
	Occlusion            = 0x0i32,
	/// D3D12_QUERY_TYPE_BINARY_OCCLUSION = 0x1i32
	BinaryOcclusion      = 0x1i32,
	/// D3D12_QUERY_TYPE_TIMESTAMP = 0x2i32
	Timestamp            = 0x2i32,
	/// D3D12_QUERY_TYPE_PIPELINE_STATISTICS = 0x3i32
	PipelineStatistics   = 0x3i32,
	/// D3D12_QUERY_TYPE_SO_STATISTICS_STREAM0 = 0x4i32
	SoStatisticsStream0  = 0x4i32,
	/// D3D12_QUERY_TYPE_SO_STATISTICS_STREAM1 = 0x5i32
	SoStatisticsStream1  = 0x5i32,
	/// D3D12_QUERY_TYPE_SO_STATISTICS_STREAM2 = 0x6i32
	SoStatisticsStream2  = 0x6i32,
	/// D3D12_QUERY_TYPE_SO_STATISTICS_STREAM3 = 0x7i32
	SoStatisticsStream3  = 0x7i32,
	/// D3D12_QUERY_TYPE_VIDEO_DECODE_STATISTICS = 0x8i32
	VideoDecodeStatistics = 0x8i32,
	/// D3D12_QUERY_TYPE_PIPELINE_STATISTICS1 = 0xAi32
	PipelineStatistics1  = 0xAi32,
}

/// D3D12_PREDICATION_OP
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12PredicationOp
{
	/// D3D12_PREDICATION_OP_EQUAL_ZERO = 0x0i32
	EqualZero            = 0x0i32,
	/// D3D12_PREDICATION_OP_NOT_EQUAL_ZERO = 0x1i32
	NotEqualZero         = 0x1i32,
}

/// D3D12_INDIRECT_ARGUMENT_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12IndirectArgumentType
{
	/// D3D12_INDIRECT_ARGUMENT_TYPE_DRAW = 0x0i32
	Draw                 = 0x0i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED = 0x1i32
	DrawIndexed          = 0x1i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH = 0x2i32
	Dispatch             = 0x2i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW = 0x3i32
	VertexBufferView     = 0x3i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_INDEX_BUFFER_VIEW = 0x4i32
	IndexBufferView      = 0x4i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT = 0x5i32
	Constant             = 0x5i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW = 0x6i32
	ConstantBufferView   = 0x6i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW = 0x7i32
	ShaderResourceView   = 0x7i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW = 0x8i32
	UnorderedAccessView  = 0x8i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_RAYS = 0x9i32
	DispatchRays         = 0x9i32,
	/// D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH_MESH = 0xAi32
	DispatchMesh         = 0xAi32,
}

impl D3D12IndirectArgumentType {
	/// D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW
	pub const Cbv                 : Self = Self::ConstantBufferView;
	/// D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW
	pub const Srv                 : Self = Self::ShaderResourceView;
	/// D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW
	pub const Uav                 : Self = Self::UnorderedAccessView;
}

/// D3D12_WRITEBUFFERIMMEDIATE_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12WriteBufferImmediateMode
{
	/// D3D12_WRITEBUFFERIMMEDIATE_MODE_DEFAULT = 0x0i32
	Default              = 0x0i32,
	/// D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_IN = 0x1i32
	MarkerIn             = 0x1i32,
	/// D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_OUT = 0x2i32
	MarkerOut            = 0x2i32,
}

bitflags::bitflags! {
	/// D3D12_MULTIPLE_FENCE_WAIT_FLAGS
	#[repr(transparent)]
	pub struct D3D12MultipleFenceWaitFlags: u32 {
		/// D3D12_MULTIPLE_FENCE_WAIT_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_MULTIPLE_FENCE_WAIT_FLAG_ANY = 0x1u32
		const Any                  = 0x1u32;
		/// D3D12_MULTIPLE_FENCE_WAIT_FLAG_ALL = 0x0u32
		const All                  = 0x0u32;
	}
}

/// D3D12_RESIDENCY_PRIORITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ResidencyPriority
{
	/// D3D12_RESIDENCY_PRIORITY_MINIMUM = 0x28000000i32
	Minimum              = 0x28000000i32,
	/// D3D12_RESIDENCY_PRIORITY_LOW = 0x50000000i32
	Low                  = 0x50000000i32,
	/// D3D12_RESIDENCY_PRIORITY_NORMAL = 0x78000000i32
	Normal               = 0x78000000i32,
	/// D3D12_RESIDENCY_PRIORITY_HIGH = -0x5FFF0000i32
	High                 = -0x5FFF0000i32,
	/// D3D12_RESIDENCY_PRIORITY_MAXIMUM = -0x38000000i32
	Maximum              = -0x38000000i32,
}

bitflags::bitflags! {
	/// D3D12_RESIDENCY_FLAGS
	#[repr(transparent)]
	pub struct D3D12ResidencyFlags: u32 {
		/// D3D12_RESIDENCY_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RESIDENCY_FLAG_DENY_OVERBUDGET = 0x1u32
		const DenyOverBudget       = 0x1u32;
	}
}

bitflags::bitflags! {
	/// D3D12_COMMAND_LIST_FLAGS
	#[repr(transparent)]
	pub struct D3D12CommandListFlags: u32 {
		/// D3D12_COMMAND_LIST_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
	}
}

bitflags::bitflags! {
	/// D3D12_COMMAND_POOL_FLAGS
	#[repr(transparent)]
	pub struct D3D12CommandPoolFlags: u32 {
		/// D3D12_COMMAND_POOL_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
	}
}

bitflags::bitflags! {
	/// D3D12_COMMAND_RECORDER_FLAGS
	#[repr(transparent)]
	pub struct D3D12CommandRecorderFlags: u32 {
		/// D3D12_COMMAND_RECORDER_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
	}
}

/// D3D12_PROTECTED_SESSION_STATUS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ProtectedSessionStatus
{
	/// D3D12_PROTECTED_SESSION_STATUS_OK = 0x0i32
	Ok                   = 0x0i32,
	/// D3D12_PROTECTED_SESSION_STATUS_INVALID = 0x1i32
	Invalid              = 0x1i32,
}

bitflags::bitflags! {
	/// D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS
	#[repr(transparent)]
	pub struct D3D12ProtectedResourceSessionSupportFlags: u32 {
		/// D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_SUPPORTED = 0x1u32
		const Supported            = 0x1u32;
	}
}

bitflags::bitflags! {
	/// D3D12_PROTECTED_RESOURCE_SESSION_FLAGS
	#[repr(transparent)]
	pub struct D3D12ProtectedResourceSessionFlags: u32 {
		/// D3D12_PROTECTED_RESOURCE_SESSION_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
	}
}

/// D3D12_LIFETIME_STATE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12LifetimeState
{
	/// D3D12_LIFETIME_STATE_IN_USE = 0x0i32
	InUse                = 0x0i32,
	/// D3D12_LIFETIME_STATE_NOT_IN_USE = 0x1i32
	NotInUse             = 0x1i32,
}

/// D3D12_META_COMMAND_PARAMETER_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MetaCommandParameterType
{
	/// D3D12_META_COMMAND_PARAMETER_TYPE_FLOAT = 0x0i32
	Float                = 0x0i32,
	/// D3D12_META_COMMAND_PARAMETER_TYPE_UINT64 = 0x1i32
	Uint64               = 0x1i32,
	/// D3D12_META_COMMAND_PARAMETER_TYPE_GPU_VIRTUAL_ADDRESS = 0x2i32
	GpuVirtualAddress    = 0x2i32,
	/// D3D12_META_COMMAND_PARAMETER_TYPE_CPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV = 0x3i32
	CpuDescriptorHandleHeapTypeCbvSrvUav = 0x3i32,
	/// D3D12_META_COMMAND_PARAMETER_TYPE_GPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV = 0x4i32
	GpuDescriptorHandleHeapTypeCbvSrvUav = 0x4i32,
}

bitflags::bitflags! {
	/// D3D12_META_COMMAND_PARAMETER_FLAGS
	#[repr(transparent)]
	pub struct D3D12MetaCommandParameterFlags: u32 {
		/// D3D12_META_COMMAND_PARAMETER_FLAG_INPUT = 0x1u32
		const Input                = 0x1u32;
		/// D3D12_META_COMMAND_PARAMETER_FLAG_OUTPUT = 0x2u32
		const Output               = 0x2u32;
	}
}

/// D3D12_META_COMMAND_PARAMETER_STAGE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MetaCommandParameterStage
{
	/// D3D12_META_COMMAND_PARAMETER_STAGE_CREATION = 0x0i32
	Creation             = 0x0i32,
	/// D3D12_META_COMMAND_PARAMETER_STAGE_INITIALIZATION = 0x1i32
	Initialization       = 0x1i32,
	/// D3D12_META_COMMAND_PARAMETER_STAGE_EXECUTION = 0x2i32
	Execution            = 0x2i32,
}

bitflags::bitflags! {
	/// D3D12_GRAPHICS_STATES
	#[repr(transparent)]
	pub struct D3D12GraphicsStates: u32 {
		/// D3D12_GRAPHICS_STATE_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_GRAPHICS_STATE_IA_VERTEX_BUFFERS = 0x1u32
		const IaVertexBuffers      = 0x1u32;
		/// D3D12_GRAPHICS_STATE_IA_INDEX_BUFFER = 0x2u32
		const IaIndexBuffer        = 0x2u32;
		/// D3D12_GRAPHICS_STATE_IA_PRIMITIVE_TOPOLOGY = 0x4u32
		const IaPrimitiveTopology  = 0x4u32;
		/// D3D12_GRAPHICS_STATE_DESCRIPTOR_HEAP = 0x8u32
		const DescriptorHeap       = 0x8u32;
		/// D3D12_GRAPHICS_STATE_GRAPHICS_ROOT_SIGNATURE = 0x10u32
		const GraphicsRootSignature = 0x10u32;
		/// D3D12_GRAPHICS_STATE_COMPUTE_ROOT_SIGNATURE = 0x20u32
		const ComputeRootSignature = 0x20u32;
		/// D3D12_GRAPHICS_STATE_RS_VIEWPORTS = 0x40u32
		const RsViewports          = 0x40u32;
		/// D3D12_GRAPHICS_STATE_RS_SCISSOR_RECTS = 0x80u32
		const RsScissorRects       = 0x80u32;
		/// D3D12_GRAPHICS_STATE_PREDICATION = 0x100u32
		const Predication          = 0x100u32;
		/// D3D12_GRAPHICS_STATE_OM_RENDER_TARGETS = 0x200u32
		const OmRenderTargets      = 0x200u32;
		/// D3D12_GRAPHICS_STATE_OM_STENCIL_REF = 0x400u32
		const OmStencilRef         = 0x400u32;
		/// D3D12_GRAPHICS_STATE_OM_BLEND_FACTOR = 0x800u32
		const OmBlendFactor        = 0x800u32;
		/// D3D12_GRAPHICS_STATE_PIPELINE_STATE = 0x1000u32
		const PipelineState        = 0x1000u32;
		/// D3D12_GRAPHICS_STATE_SO_TARGETS = 0x2000u32
		const SoTargets            = 0x2000u32;
		/// D3D12_GRAPHICS_STATE_OM_DEPTH_BOUNDS = 0x4000u32
		const OmDepthBounds        = 0x4000u32;
		/// D3D12_GRAPHICS_STATE_SAMPLE_POSITIONS = 0x8000u32
		const SamplePositions      = 0x8000u32;
		/// D3D12_GRAPHICS_STATE_VIEW_INSTANCE_MASK = 0x10000u32
		const ViewInstanceMask     = 0x10000u32;
	}
}

/// D3D12_STATE_SUBOBJECT_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12StateSubobjectType
{
	/// D3D12_STATE_SUBOBJECT_TYPE_STATE_OBJECT_CONFIG = 0x0i32
	StateObjectConfig    = 0x0i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_GLOBAL_ROOT_SIGNATURE = 0x1i32
	GlobalRootSignature  = 0x1i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_LOCAL_ROOT_SIGNATURE = 0x2i32
	LocalRootSignature   = 0x2i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_NODE_MASK = 0x3i32
	NodeMask             = 0x3i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_DXIL_LIBRARY = 0x5i32
	DxilLibrary          = 0x5i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_EXISTING_COLLECTION = 0x6i32
	ExistingCollection   = 0x6i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_SUBOBJECT_TO_EXPORTS_ASSOCIATION = 0x7i32
	SubobjectToExportsAssociation = 0x7i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION = 0x8i32
	DxilSubobjectToExportsAssociation = 0x8i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_SHADER_CONFIG = 0x9i32
	RaytracingShaderConfig = 0x9i32,
	/// D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG = 0xAi32
	RaytracingPipelineConfig = 0xAi32,
	/// D3D12_STATE_SUBOBJECT_TYPE_HIT_GROUP = 0xBi32
	HitGroup             = 0xBi32,
	/// D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG1 = 0xCi32
	RaytracingPipelineConfig1 = 0xCi32,
	/// D3D12_STATE_SUBOBJECT_TYPE_MAX_VALID = 0xDi32
	MaxValid             = 0xDi32,
}

impl D3D12StateSubobjectType {
	/// D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_SHADER_CONFIG
	pub const RtShaderConfig      : Self = Self::RaytracingShaderConfig;
	/// D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG
	pub const RtPipelineConfig    : Self = Self::RaytracingPipelineConfig;
	/// D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG1
	pub const RtPipelineConfig1   : Self = Self::RaytracingPipelineConfig1;
}

bitflags::bitflags! {
	/// D3D12_STATE_OBJECT_FLAGS
	#[repr(transparent)]
	pub struct D3D12StateObjectFlags: u32 {
		/// D3D12_STATE_OBJECT_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_STATE_OBJECT_FLAG_ALLOW_LOCAL_DEPENDENCIES_ON_EXTERNAL_DEFINITIONS = 0x1u32
		const AllowLocalDependenciesOnExternalDefinitions = 0x1u32;
		/// D3D12_STATE_OBJECT_FLAG_ALLOW_EXTERNAL_DEPENDENCIES_ON_LOCAL_DEFINITIONS = 0x2u32
		const AllowExternalDependenciesOnLocalDefinitions = 0x2u32;
		/// D3D12_STATE_OBJECT_FLAG_ALLOW_STATE_OBJECT_ADDITIONS = 0x4u32
		const AllowStateObjectAdditions = 0x4u32;
	}
}

bitflags::bitflags! {
	/// D3D12_EXPORT_FLAGS
	#[repr(transparent)]
	pub struct D3D12ExportFlags: u32 {
		/// D3D12_EXPORT_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
	}
}

/// D3D12_HIT_GROUP_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12HitGroupType
{
	/// D3D12_HIT_GROUP_TYPE_TRIANGLES = 0x0i32
	Triangles            = 0x0i32,
	/// D3D12_HIT_GROUP_TYPE_PROCEDURAL_PRIMITIVE = 0x1i32
	ProceduralPrimitive  = 0x1i32,
}

bitflags::bitflags! {
	/// D3D12_RAYTRACING_PIPELINE_FLAGS
	#[repr(transparent)]
	pub struct D3D12RaytracingPipelineFlags: u32 {
		/// D3D12_RAYTRACING_PIPELINE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RAYTRACING_PIPELINE_FLAG_SKIP_TRIANGLES = 0x100u32
		const SkipTriangles        = 0x100u32;
		/// D3D12_RAYTRACING_PIPELINE_FLAG_SKIP_PROCEDURAL_PRIMITIVES = 0x200u32
		const SkipProceduralPrimitives = 0x200u32;
	}
}

/// D3D12_STATE_OBJECT_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12StateObjectType
{
	/// D3D12_STATE_OBJECT_TYPE_COLLECTION = 0x0i32
	Collection           = 0x0i32,
	/// D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE = 0x3i32
	RaytracingPipeline   = 0x3i32,
}

impl D3D12StateObjectType {
	/// D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE
	pub const RtPipeline          : Self = Self::RaytracingPipeline;
}

bitflags::bitflags! {
	/// D3D12_RAYTRACING_GEOMETRY_FLAGS
	#[repr(transparent)]
	pub struct D3D12RaytracingGeometryFlags: u32 {
		/// D3D12_RAYTRACING_GEOMETRY_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RAYTRACING_GEOMETRY_FLAG_OPAQUE = 0x1u32
		const Opaque               = 0x1u32;
		/// D3D12_RAYTRACING_GEOMETRY_FLAG_NO_DUPLICATE_ANYHIT_INVOCATION = 0x2u32
		const NoDuplicateAnyHitInvocation = 0x2u32;
	}
}

/// D3D12_RAYTRACING_GEOMETRY_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RaytracingGeometryType
{
	/// D3D12_RAYTRACING_GEOMETRY_TYPE_TRIANGLES = 0x0i32
	Triangles            = 0x0i32,
	/// D3D12_RAYTRACING_GEOMETRY_TYPE_PROCEDURAL_PRIMITIVE_AABBS = 0x1i32
	ProceduralPrimitiveAabbs = 0x1i32,
}

bitflags::bitflags! {
	/// D3D12_RAYTRACING_INSTANCE_FLAGS
	#[repr(transparent)]
	pub struct D3D12RaytracingInstanceFlags: u32 {
		/// D3D12_RAYTRACING_INSTANCE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_CULL_DISABLE = 0x1u32
		const TriangleCullDisable  = 0x1u32;
		/// D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_FRONT_COUNTERCLOCKWISE = 0x2u32
		const TriangleFrontCounterclockwise = 0x2u32;
		/// D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_OPAQUE = 0x4u32
		const ForceOpaque          = 0x4u32;
		/// D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_NON_OPAQUE = 0x8u32
		const ForceNonOpaque       = 0x8u32;
	}
}

bitflags::bitflags! {
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS
	#[repr(transparent)]
	pub struct D3D12RaytracingAccelerationStructureBuildFlags: u32 {
		/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_UPDATE = 0x1u32
		const AllowUpdate          = 0x1u32;
		/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_COMPACTION = 0x2u32
		const AllowCompaction      = 0x2u32;
		/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_TRACE = 0x4u32
		const PreferFastTrace      = 0x4u32;
		/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_BUILD = 0x8u32
		const PreferFastBuild      = 0x8u32;
		/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_MINIMIZE_MEMORY = 0x10u32
		const MinimizeMemory       = 0x10u32;
		/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PERFORM_UPDATE = 0x20u32
		const PerformUpdate        = 0x20u32;
	}
}

/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RaytracingAccelerationStructureCopyMode
{
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_CLONE = 0x0i32
	Clone                = 0x0i32,
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_COMPACT = 0x1i32
	Compact              = 0x1i32,
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_VISUALIZATION_DECODE_FOR_TOOLS = 0x2i32
	VisualizationDecodeForTools = 0x2i32,
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_SERIALIZE = 0x3i32
	Serialize            = 0x3i32,
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_DESERIALIZE = 0x4i32
	Deserialize          = 0x4i32,
}

/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RaytracingAccelerationStructureType
{
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL = 0x0i32
	TopLevel             = 0x0i32,
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL = 0x1i32
	BottomLevel          = 0x1i32,
}

/// D3D12_ELEMENTS_LAYOUT
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ElementsLayout
{
	/// D3D12_ELEMENTS_LAYOUT_ARRAY = 0x0i32
	Array                = 0x0i32,
	/// D3D12_ELEMENTS_LAYOUT_ARRAY_OF_POINTERS = 0x1i32
	ArrayOfPointers      = 0x1i32,
}

/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RaytracingAccelerationStructurePostBuildInfoType
{
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE = 0x0i32
	CompactedSize        = 0x0i32,
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION = 0x1i32
	ToolsVisualization   = 0x1i32,
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION = 0x2i32
	Serialization        = 0x2i32,
	/// D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE = 0x3i32
	CurrentSize          = 0x3i32,
}

/// D3D12_SERIALIZED_DATA_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12SerializedDataType
{
	/// D3D12_SERIALIZED_DATA_RAYTRACING_ACCELERATION_STRUCTURE = 0x0i32
	RaytracingAccelerationStructure = 0x0i32,
}

impl D3D12SerializedDataType {
	/// D3D12_SERIALIZED_DATA_RAYTRACING_ACCELERATION_STRUCTURE
	pub const RtAs                : Self = Self::RaytracingAccelerationStructure;
}

/// D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DriverMatchingIdentifierStatus
{
	/// D3D12_DRIVER_MATCHING_IDENTIFIER_COMPATIBLE_WITH_DEVICE = 0x0i32
	CompatibleWithDevice = 0x0i32,
	/// D3D12_DRIVER_MATCHING_IDENTIFIER_UNSUPPORTED_TYPE = 0x1i32
	UnsupportedType      = 0x1i32,
	/// D3D12_DRIVER_MATCHING_IDENTIFIER_UNRECOGNIZED = 0x2i32
	Unrecognized         = 0x2i32,
	/// D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_VERSION = 0x3i32
	IncompatibleVersion  = 0x3i32,
	/// D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_TYPE = 0x4i32
	IncompatibleType     = 0x4i32,
}

bitflags::bitflags! {
	/// D3D12_RAY_FLAGS
	#[repr(transparent)]
	pub struct D3D12RayFlags: u32 {
		/// D3D12_RAY_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RAY_FLAG_FORCE_OPAQUE = 0x1u32
		const ForceOpaque          = 0x1u32;
		/// D3D12_RAY_FLAG_FORCE_NON_OPAQUE = 0x2u32
		const ForceNonOpaque       = 0x2u32;
		/// D3D12_RAY_FLAG_ACCEPT_FIRST_HIT_AND_END_SEARCH = 0x4u32
		const AcceptFirstHitAndEndSearch = 0x4u32;
		/// D3D12_RAY_FLAG_SKIP_CLOSEST_HIT_SHADER = 0x8u32
		const SkipClosestHitShader = 0x8u32;
		/// D3D12_RAY_FLAG_CULL_BACK_FACING_TRIANGLES = 0x10u32
		const CullBackFacingTriangles = 0x10u32;
		/// D3D12_RAY_FLAG_CULL_FRONT_FACING_TRIANGLES = 0x20u32
		const CullFrontFacingTriangles = 0x20u32;
		/// D3D12_RAY_FLAG_CULL_OPAQUE = 0x40u32
		const CullOpaque           = 0x40u32;
		/// D3D12_RAY_FLAG_CULL_NON_OPAQUE = 0x80u32
		const CullNonOpaque        = 0x80u32;
		/// D3D12_RAY_FLAG_SKIP_TRIANGLES = 0x100u32
		const SkipTriangles        = 0x100u32;
		/// D3D12_RAY_FLAG_SKIP_PROCEDURAL_PRIMITIVES = 0x200u32
		const SkipProceduralPrimitives = 0x200u32;
	}
}

/// D3D12_HIT_KIND
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12HitKind
{
	/// D3D12_HIT_KIND_TRIANGLE_FRONT_FACE = 0xFEi32
	TriangleFrontFace    = 0xFEi32,
	/// D3D12_HIT_KIND_TRIANGLE_BACK_FACE = 0xFFi32
	TriangleBackFace     = 0xFFi32,
}

/// D3D12_AUTO_BREADCRUMB_OP
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12AutoBreadcrumbOp
{
	/// D3D12_AUTO_BREADCRUMB_OP_SETMARKER = 0x0i32
	SetMarker            = 0x0i32,
	/// D3D12_AUTO_BREADCRUMB_OP_BEGINEVENT = 0x1i32
	BeginEvent           = 0x1i32,
	/// D3D12_AUTO_BREADCRUMB_OP_ENDEVENT = 0x2i32
	EndEvent             = 0x2i32,
	/// D3D12_AUTO_BREADCRUMB_OP_DRAWINSTANCED = 0x3i32
	DrawInstanced        = 0x3i32,
	/// D3D12_AUTO_BREADCRUMB_OP_DRAWINDEXEDINSTANCED = 0x4i32
	DrawIndexedInstanced = 0x4i32,
	/// D3D12_AUTO_BREADCRUMB_OP_EXECUTEINDIRECT = 0x5i32
	ExecuteIndirect      = 0x5i32,
	/// D3D12_AUTO_BREADCRUMB_OP_DISPATCH = 0x6i32
	Dispatch             = 0x6i32,
	/// D3D12_AUTO_BREADCRUMB_OP_COPYBUFFERREGION = 0x7i32
	CopyBufferRegion     = 0x7i32,
	/// D3D12_AUTO_BREADCRUMB_OP_COPYTEXTUREREGION = 0x8i32
	CopyTextureRegion    = 0x8i32,
	/// D3D12_AUTO_BREADCRUMB_OP_COPYRESOURCE = 0x9i32
	CopyResource         = 0x9i32,
	/// D3D12_AUTO_BREADCRUMB_OP_COPYTILES = 0xAi32
	CopyTiles            = 0xAi32,
	/// D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCE = 0xBi32
	ResolveSubresource   = 0xBi32,
	/// D3D12_AUTO_BREADCRUMB_OP_CLEARRENDERTARGETVIEW = 0xCi32
	ClearRenderTargetView = 0xCi32,
	/// D3D12_AUTO_BREADCRUMB_OP_CLEARUNORDEREDACCESSVIEW = 0xDi32
	ClearUnorderedAccessView = 0xDi32,
	/// D3D12_AUTO_BREADCRUMB_OP_CLEARDEPTHSTENCILVIEW = 0xEi32
	ClearDepthStencilView = 0xEi32,
	/// D3D12_AUTO_BREADCRUMB_OP_RESOURCEBARRIER = 0xFi32
	ResourceBarrier      = 0xFi32,
	/// D3D12_AUTO_BREADCRUMB_OP_EXECUTEBUNDLE = 0x10i32
	ExecuteBundle        = 0x10i32,
	/// D3D12_AUTO_BREADCRUMB_OP_PRESENT = 0x11i32
	Present              = 0x11i32,
	/// D3D12_AUTO_BREADCRUMB_OP_RESOLVEQUERYDATA = 0x12i32
	ResolveQueryData     = 0x12i32,
	/// D3D12_AUTO_BREADCRUMB_OP_BEGINSUBMISSION = 0x13i32
	BegInsubmission      = 0x13i32,
	/// D3D12_AUTO_BREADCRUMB_OP_ENDSUBMISSION = 0x14i32
	EndSubmission        = 0x14i32,
	/// D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME = 0x15i32
	DecodeFrame          = 0x15i32,
	/// D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES = 0x16i32
	ProcessFrames        = 0x16i32,
	/// D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT = 0x17i32
	AtomicCopyBufferUint = 0x17i32,
	/// D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT64 = 0x18i32
	AtomicCopyBufferUint64 = 0x18i32,
	/// D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCEREGION = 0x19i32
	ResolveSubresourceRegion = 0x19i32,
	/// D3D12_AUTO_BREADCRUMB_OP_WRITEBUFFERIMMEDIATE = 0x1Ai32
	WriteBufferImmediate = 0x1Ai32,
	/// D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME1 = 0x1Bi32
	DecodeFrame1         = 0x1Bi32,
	/// D3D12_AUTO_BREADCRUMB_OP_SETPROTECTEDRESOURCESESSION = 0x1Ci32
	SetProtectedResourceSession = 0x1Ci32,
	/// D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME2 = 0x1Di32
	DecodeFrame2         = 0x1Di32,
	/// D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES1 = 0x1Ei32
	ProcessFrames1       = 0x1Ei32,
	/// D3D12_AUTO_BREADCRUMB_OP_BUILDRAYTRACINGACCELERATIONSTRUCTURE = 0x1Fi32
	BuildRaytracingAccelerationStructure = 0x1Fi32,
	/// D3D12_AUTO_BREADCRUMB_OP_EMITRAYTRACINGACCELERATIONSTRUCTUREPOSTBUILDINFO = 0x20i32
	EmitRaytracingAccelerationStructurePostBuildInfo = 0x20i32,
	/// D3D12_AUTO_BREADCRUMB_OP_COPYRAYTRACINGACCELERATIONSTRUCTURE = 0x21i32
	CopyRaytracingAccelerationStructure = 0x21i32,
	/// D3D12_AUTO_BREADCRUMB_OP_DISPATCHRAYS = 0x22i32
	DispatchRays         = 0x22i32,
	/// D3D12_AUTO_BREADCRUMB_OP_INITIALIZEMETACOMMAND = 0x23i32
	InitializeMetaCommand = 0x23i32,
	/// D3D12_AUTO_BREADCRUMB_OP_EXECUTEMETACOMMAND = 0x24i32
	ExecuteMetaCommand   = 0x24i32,
	/// D3D12_AUTO_BREADCRUMB_OP_ESTIMATEMOTION = 0x25i32
	EstimateMotion       = 0x25i32,
	/// D3D12_AUTO_BREADCRUMB_OP_RESOLVEMOTIONVECTORHEAP = 0x26i32
	ResolveMotionVectorHeap = 0x26i32,
	/// D3D12_AUTO_BREADCRUMB_OP_SETPIPELINESTATE1 = 0x27i32
	SetPipelineState1    = 0x27i32,
	/// D3D12_AUTO_BREADCRUMB_OP_INITIALIZEEXTENSIONCOMMAND = 0x28i32
	InitializeExtensionCommand = 0x28i32,
	/// D3D12_AUTO_BREADCRUMB_OP_EXECUTEEXTENSIONCOMMAND = 0x29i32
	ExecuteExtensionCommand = 0x29i32,
	/// D3D12_AUTO_BREADCRUMB_OP_DISPATCHMESH = 0x2Ai32
	DispatchMesh         = 0x2Ai32,
	/// D3D12_AUTO_BREADCRUMB_OP_ENCODEFRAME = 0x2Bi32
	EncodeFrame          = 0x2Bi32,
	/// D3D12_AUTO_BREADCRUMB_OP_RESOLVEENCODEROUTPUTMETADATA = 0x2Ci32
	ResolveEncoderOutputMetaData = 0x2Ci32,
}

impl D3D12AutoBreadcrumbOp {
	/// D3D12_AUTO_BREADCRUMB_OP_CLEARUNORDEREDACCESSVIEW
	pub const ClearUav            : Self = Self::ClearUnorderedAccessView;
	/// D3D12_AUTO_BREADCRUMB_OP_BUILDRAYTRACINGACCELERATIONSTRUCTURE
	pub const BuildRtAs           : Self = Self::BuildRaytracingAccelerationStructure;
	/// D3D12_AUTO_BREADCRUMB_OP_EMITRAYTRACINGACCELERATIONSTRUCTUREPOSTBUILDINFO
	pub const EmitRtAsPostBuildInfo: Self = Self::EmitRaytracingAccelerationStructurePostBuildInfo;
	/// D3D12_AUTO_BREADCRUMB_OP_COPYRAYTRACINGACCELERATIONSTRUCTURE
	pub const CopyRtAs            : Self = Self::CopyRaytracingAccelerationStructure;
}

/// D3D12_DRED_VERSION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DredVersion
{
	/// D3D12_DRED_VERSION_1_0 = 0x1i32
	_1_0                 = 0x1i32,
	/// D3D12_DRED_VERSION_1_1 = 0x2i32
	_1_1                 = 0x2i32,
	/// D3D12_DRED_VERSION_1_2 = 0x3i32
	_1_2                 = 0x3i32,
	/// D3D12_DRED_VERSION_1_3 = 0x4i32
	_1_3                 = 0x4i32,
}

bitflags::bitflags! {
	/// D3D12_DRED_FLAGS
	#[repr(transparent)]
	pub struct D3D12DredFlags: u32 {
		/// D3D12_DRED_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_DRED_FLAG_FORCE_ENABLE = 0x1u32
		const ForceEnable          = 0x1u32;
		/// D3D12_DRED_FLAG_DISABLE_AUTOBREADCRUMBS = 0x2u32
		const DisableAutoBreadcrumbs = 0x2u32;
	}
}

/// D3D12_DRED_ENABLEMENT
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DredEnablement
{
	/// D3D12_DRED_ENABLEMENT_SYSTEM_CONTROLLED = 0x0i32
	SystemControlled     = 0x0i32,
	/// D3D12_DRED_ENABLEMENT_FORCED_OFF = 0x1i32
	ForcedOff            = 0x1i32,
	/// D3D12_DRED_ENABLEMENT_FORCED_ON = 0x2i32
	ForcedOn             = 0x2i32,
}

/// D3D12_DRED_ALLOCATION_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DredAllocationType
{
	/// D3D12_DRED_ALLOCATION_TYPE_COMMAND_QUEUE = 0x13i32
	CommandQueue         = 0x13i32,
	/// D3D12_DRED_ALLOCATION_TYPE_COMMAND_ALLOCATOR = 0x14i32
	CommandAllocator     = 0x14i32,
	/// D3D12_DRED_ALLOCATION_TYPE_PIPELINE_STATE = 0x15i32
	PipelineState        = 0x15i32,
	/// D3D12_DRED_ALLOCATION_TYPE_COMMAND_LIST = 0x16i32
	CommandList          = 0x16i32,
	/// D3D12_DRED_ALLOCATION_TYPE_FENCE = 0x17i32
	Fence                = 0x17i32,
	/// D3D12_DRED_ALLOCATION_TYPE_DESCRIPTOR_HEAP = 0x18i32
	DescriptorHeap       = 0x18i32,
	/// D3D12_DRED_ALLOCATION_TYPE_HEAP = 0x19i32
	Heap                 = 0x19i32,
	/// D3D12_DRED_ALLOCATION_TYPE_QUERY_HEAP = 0x1Bi32
	QueryHeap            = 0x1Bi32,
	/// D3D12_DRED_ALLOCATION_TYPE_COMMAND_SIGNATURE = 0x1Ci32
	CommandSignature     = 0x1Ci32,
	/// D3D12_DRED_ALLOCATION_TYPE_PIPELINE_LIBRARY = 0x1Di32
	PipelineLibrary      = 0x1Di32,
	/// D3D12_DRED_ALLOCATION_TYPE_VIDEO_DECODER = 0x1Ei32
	VideoDecoder         = 0x1Ei32,
	/// D3D12_DRED_ALLOCATION_TYPE_VIDEO_PROCESSOR = 0x20i32
	VideoProcessor       = 0x20i32,
	/// D3D12_DRED_ALLOCATION_TYPE_RESOURCE = 0x22i32
	Resource             = 0x22i32,
	/// D3D12_DRED_ALLOCATION_TYPE_PASS = 0x23i32
	Pass                 = 0x23i32,
	/// D3D12_DRED_ALLOCATION_TYPE_CRYPTOSESSION = 0x24i32
	CryptoSession        = 0x24i32,
	/// D3D12_DRED_ALLOCATION_TYPE_CRYPTOSESSIONPOLICY = 0x25i32
	CryptoSessionPolicy  = 0x25i32,
	/// D3D12_DRED_ALLOCATION_TYPE_PROTECTEDRESOURCESESSION = 0x26i32
	ProtectedResourceSession = 0x26i32,
	/// D3D12_DRED_ALLOCATION_TYPE_VIDEO_DECODER_HEAP = 0x27i32
	VideoDecoderHeap     = 0x27i32,
	/// D3D12_DRED_ALLOCATION_TYPE_COMMAND_POOL = 0x28i32
	CommandPool          = 0x28i32,
	/// D3D12_DRED_ALLOCATION_TYPE_COMMAND_RECORDER = 0x29i32
	CommandRecorder      = 0x29i32,
	/// D3D12_DRED_ALLOCATION_TYPE_STATE_OBJECT = 0x2Ai32
	StateObject          = 0x2Ai32,
	/// D3D12_DRED_ALLOCATION_TYPE_METACOMMAND = 0x2Bi32
	MetaCommand          = 0x2Bi32,
	/// D3D12_DRED_ALLOCATION_TYPE_SCHEDULINGGROUP = 0x2Ci32
	SchedulingGroup      = 0x2Ci32,
	/// D3D12_DRED_ALLOCATION_TYPE_VIDEO_MOTION_ESTIMATOR = 0x2Di32
	VideoMotionEstimator = 0x2Di32,
	/// D3D12_DRED_ALLOCATION_TYPE_VIDEO_MOTION_VECTOR_HEAP = 0x2Ei32
	VideoMotionVectorHeap = 0x2Ei32,
	/// D3D12_DRED_ALLOCATION_TYPE_VIDEO_EXTENSION_COMMAND = 0x2Fi32
	VideoExtensionCommand = 0x2Fi32,
	/// D3D12_DRED_ALLOCATION_TYPE_VIDEO_ENCODER = 0x30i32
	VideoEncoder         = 0x30i32,
	/// D3D12_DRED_ALLOCATION_TYPE_VIDEO_ENCODER_HEAP = 0x31i32
	VideoEncoderHeap     = 0x31i32,
	/// D3D12_DRED_ALLOCATION_TYPE_INVALID = -0x1i32
	Invalid              = -0x1i32,
}

bitflags::bitflags! {
	/// D3D12_DRED_PAGE_FAULT_FLAGS
	#[repr(transparent)]
	pub struct D3D12DredPageFaultFlags: u32 {
		/// D3D12_DRED_PAGE_FAULT_FLAGS_NONE = 0x0u32
		const None                 = 0x0u32;
	}
}

/// D3D12_DRED_DEVICE_STATE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DredDeviceState
{
	/// D3D12_DRED_DEVICE_STATE_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_DRED_DEVICE_STATE_HUNG = 0x3i32
	Hung                 = 0x3i32,
	/// D3D12_DRED_DEVICE_STATE_FAULT = 0x6i32
	Fault                = 0x6i32,
	/// D3D12_DRED_DEVICE_STATE_PAGEFAULT = 0x7i32
	PageFault            = 0x7i32,
}

/// D3D12_BACKGROUND_PROCESSING_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12BackgroundProcessingMode
{
	/// D3D12_BACKGROUND_PROCESSING_MODE_ALLOWED = 0x0i32
	Allowed              = 0x0i32,
	/// D3D12_BACKGROUND_PROCESSING_MODE_ALLOW_INTRUSIVE_MEASUREMENTS = 0x1i32
	AllowIntrusiveMeasurements = 0x1i32,
	/// D3D12_BACKGROUND_PROCESSING_MODE_DISABLE_BACKGROUND_WORK = 0x2i32
	DisableBackgroundWork = 0x2i32,
	/// D3D12_BACKGROUND_PROCESSING_MODE_DISABLE_PROFILING_BY_SYSTEM = 0x3i32
	DisableProfilingBySystem = 0x3i32,
}

/// D3D12_MEASUREMENTS_ACTION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MeasurementsAction
{
	/// D3D12_MEASUREMENTS_ACTION_KEEP_ALL = 0x0i32
	KeepAll              = 0x0i32,
	/// D3D12_MEASUREMENTS_ACTION_COMMIT_RESULTS = 0x1i32
	CommitResults        = 0x1i32,
	/// D3D12_MEASUREMENTS_ACTION_COMMIT_RESULTS_HIGH_PRIORITY = 0x2i32
	CommitResultsHighPriority = 0x2i32,
	/// D3D12_MEASUREMENTS_ACTION_DISCARD_PREVIOUS = 0x3i32
	DiscardPrevious      = 0x3i32,
}

/// D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RenderPassBeginningAccessType
{
	/// D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_DISCARD = 0x0i32
	Discard              = 0x0i32,
	/// D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE = 0x1i32
	Preserve             = 0x1i32,
	/// D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_CLEAR = 0x2i32
	Clear                = 0x2i32,
	/// D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_NO_ACCESS = 0x3i32
	NoAccess             = 0x3i32,
}

/// D3D12_RENDER_PASS_ENDING_ACCESS_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RenderPassEndingAccessType
{
	/// D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_DISCARD = 0x0i32
	Discard              = 0x0i32,
	/// D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE = 0x1i32
	Preserve             = 0x1i32,
	/// D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_RESOLVE = 0x2i32
	Resolve              = 0x2i32,
	/// D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_NO_ACCESS = 0x3i32
	NoAccess             = 0x3i32,
}

bitflags::bitflags! {
	/// D3D12_RENDER_PASS_FLAGS
	#[repr(transparent)]
	pub struct D3D12RenderPassFlags: u32 {
		/// D3D12_RENDER_PASS_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_RENDER_PASS_FLAG_ALLOW_UAV_WRITES = 0x1u32
		const AllowUavWrites       = 0x1u32;
		/// D3D12_RENDER_PASS_FLAG_SUSPENDING_PASS = 0x2u32
		const SuspendingPass       = 0x2u32;
		/// D3D12_RENDER_PASS_FLAG_RESUMING_PASS = 0x4u32
		const ResumingPass         = 0x4u32;
	}
}

/// D3D12_SHADER_CACHE_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ShaderCacheMode
{
	/// D3D12_SHADER_CACHE_MODE_MEMORY = 0x0i32
	Memory               = 0x0i32,
	/// D3D12_SHADER_CACHE_MODE_DISK = 0x1i32
	Disk                 = 0x1i32,
}

bitflags::bitflags! {
	/// D3D12_SHADER_CACHE_FLAGS
	#[repr(transparent)]
	pub struct D3D12ShaderCacheFlags: u32 {
		/// D3D12_SHADER_CACHE_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D3D12_SHADER_CACHE_FLAG_DRIVER_VERSIONED = 0x1u32
		const DriverVersioned      = 0x1u32;
		/// D3D12_SHADER_CACHE_FLAG_USE_WORKING_DIR = 0x2u32
		const UseWorkingDir        = 0x2u32;
	}
}

bitflags::bitflags! {
	/// D3D12_SHADER_CACHE_KIND_FLAGS
	#[repr(transparent)]
	pub struct D3D12ShaderCacheKindFlags: u32 {
		/// D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_D3D_CACHE_FOR_DRIVER = 0x1u32
		const ImplicitD3DCacheForDriver = 0x1u32;
		/// D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_D3D_CONVERSIONS = 0x2u32
		const ImplicitD3DConversions = 0x2u32;
		/// D3D12_SHADER_CACHE_KIND_FLAG_IMPLICIT_DRIVER_MANAGED = 0x4u32
		const ImplicitDriverManaged = 0x4u32;
		/// D3D12_SHADER_CACHE_KIND_FLAG_APPLICATION_MANAGED = 0x8u32
		const ApplicationManaged   = 0x8u32;
	}
}

bitflags::bitflags! {
	/// D3D12_SHADER_CACHE_CONTROL_FLAGS
	#[repr(transparent)]
	pub struct D3D12ShaderCacheControlFlags: u32 {
		/// D3D12_SHADER_CACHE_CONTROL_FLAG_DISABLE = 0x1u32
		const Disable              = 0x1u32;
		/// D3D12_SHADER_CACHE_CONTROL_FLAG_ENABLE = 0x2u32
		const Enable               = 0x2u32;
		/// D3D12_SHADER_CACHE_CONTROL_FLAG_CLEAR = 0x4u32
		const Clear                = 0x4u32;
	}
}

/// D3D12_GPU_BASED_VALIDATION_FLAGS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12GpuBasedValidationFlags
{
	/// D3D12_GPU_BASED_VALIDATION_FLAGS_NONE = 0x0i32
	None                 = 0x0i32,
	/// D3D12_GPU_BASED_VALIDATION_FLAGS_DISABLE_STATE_TRACKING = 0x1i32
	DisableStateTracking = 0x1i32,
}

/// D3D12_RLDO_FLAGS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12RldoFlags
{
	/// D3D12_RLDO_NONE = 0x0i32
	None                 = 0x0i32,
	/// D3D12_RLDO_SUMMARY = 0x1i32
	Summary              = 0x1i32,
	/// D3D12_RLDO_DETAIL = 0x2i32
	Detail               = 0x2i32,
	/// D3D12_RLDO_IGNORE_INTERNAL = 0x4i32
	IgnoreInternal       = 0x4i32,
}

/// D3D12_DEBUG_DEVICE_PARAMETER_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DebugDeviceParameterType
{
	/// D3D12_DEBUG_DEVICE_PARAMETER_FEATURE_FLAGS = 0x0i32
	FeatureFlags         = 0x0i32,
	/// D3D12_DEBUG_DEVICE_PARAMETER_GPU_BASED_VALIDATION_SETTINGS = 0x1i32
	GpuBasedValidationSettings = 0x1i32,
	/// D3D12_DEBUG_DEVICE_PARAMETER_GPU_SLOWDOWN_PERFORMANCE_FACTOR = 0x2i32
	GpuSlowdownPerformanceFactor = 0x2i32,
}

/// D3D12_DEBUG_FEATURE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DebugFeature
{
	/// D3D12_DEBUG_FEATURE_NONE = 0x0i32
	None                 = 0x0i32,
	/// D3D12_DEBUG_FEATURE_ALLOW_BEHAVIOR_CHANGING_DEBUG_AIDS = 0x1i32
	AllowBehaviorChangingDebugAids = 0x1i32,
	/// D3D12_DEBUG_FEATURE_CONSERVATIVE_RESOURCE_STATE_TRACKING = 0x2i32
	ConservativeResourceStateTracking = 0x2i32,
	/// D3D12_DEBUG_FEATURE_DISABLE_VIRTUALIZED_BUNDLES_VALIDATION = 0x4i32
	DisableVirtualizedBundlesValidation = 0x4i32,
	/// D3D12_DEBUG_FEATURE_EMULATE_WINDOWS7 = 0x8i32
	EmulateWindows7      = 0x8i32,
}

/// D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12GpuBasedValidationShaderPatchMode
{
	/// D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_NONE = 0x0i32
	None                 = 0x0i32,
	/// D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_STATE_TRACKING_ONLY = 0x1i32
	StateTrackingOnly    = 0x1i32,
	/// D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_UNGUARDED_VALIDATION = 0x2i32
	UnguardedValidation  = 0x2i32,
	/// D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE_GUARDED_VALIDATION = 0x3i32
	GuardedValidation    = 0x3i32,
	/// NUM_D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODES = 0x4i32
	D3D12GpuBasedValidationShaderPatchModes = 0x4i32,
}

/// D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12GpuBasedValidationPipelineStateCreateFlags
{
	/// D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_NONE = 0x0i32
	None                 = 0x0i32,
	/// D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_TRACKING_ONLY_SHADERS = 0x1i32
	FrontLoadCreateTrackingOnlyShaders = 0x1i32,
	/// D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_UNGUARDED_VALIDATION_SHADERS = 0x2i32
	FrontLoadCreateUnguardedValidationShaders = 0x2i32,
	/// D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAG_FRONT_LOAD_CREATE_GUARDED_VALIDATION_SHADERS = 0x4i32
	FrontLoadCreateGuardedValidationShaders = 0x4i32,
	/// D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS_VALID_MASK = 0x7i32
	ValidMask            = 0x7i32,
}

/// D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12DebugCommandListParameterType
{
	/// D3D12_DEBUG_COMMAND_LIST_PARAMETER_GPU_BASED_VALIDATION_SETTINGS = 0x0i32
	GpuBasedValidationSettings = 0x0i32,
}

/// D3D12_MESSAGE_CATEGORY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MessageCategory
{
	/// D3D12_MESSAGE_CATEGORY_APPLICATION_DEFINED = 0x0i32
	ApplicationDefined   = 0x0i32,
	/// D3D12_MESSAGE_CATEGORY_MISCELLANEOUS = 0x1i32
	Miscellaneous        = 0x1i32,
	/// D3D12_MESSAGE_CATEGORY_INITIALIZATION = 0x2i32
	Initialization       = 0x2i32,
	/// D3D12_MESSAGE_CATEGORY_CLEANUP = 0x3i32
	Cleanup              = 0x3i32,
	/// D3D12_MESSAGE_CATEGORY_COMPILATION = 0x4i32
	Compilation          = 0x4i32,
	/// D3D12_MESSAGE_CATEGORY_STATE_CREATION = 0x5i32
	StateCreation        = 0x5i32,
	/// D3D12_MESSAGE_CATEGORY_STATE_SETTING = 0x6i32
	StateSetting         = 0x6i32,
	/// D3D12_MESSAGE_CATEGORY_STATE_GETTING = 0x7i32
	StateGetting         = 0x7i32,
	/// D3D12_MESSAGE_CATEGORY_RESOURCE_MANIPULATION = 0x8i32
	ResourceManipulation = 0x8i32,
	/// D3D12_MESSAGE_CATEGORY_EXECUTION = 0x9i32
	Execution            = 0x9i32,
	/// D3D12_MESSAGE_CATEGORY_SHADER = 0xAi32
	Shader               = 0xAi32,
}

/// D3D12_MESSAGE_SEVERITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MessageSeverity
{
	/// D3D12_MESSAGE_SEVERITY_CORRUPTION = 0x0i32
	Corruption           = 0x0i32,
	/// D3D12_MESSAGE_SEVERITY_ERROR = 0x1i32
	Error                = 0x1i32,
	/// D3D12_MESSAGE_SEVERITY_WARNING = 0x2i32
	Warning              = 0x2i32,
	/// D3D12_MESSAGE_SEVERITY_INFO = 0x3i32
	Info                 = 0x3i32,
	/// D3D12_MESSAGE_SEVERITY_MESSAGE = 0x4i32
	Message              = 0x4i32,
}

/// D3D12_MESSAGE_ID
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MessageId
{
	/// D3D12_MESSAGE_ID_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D12_MESSAGE_ID_STRING_FROM_APPLICATION = 0x1i32
	StringFromApplication = 0x1i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_THIS = 0x2i32
	CorruptedThis        = 0x2i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER1 = 0x3i32
	CorruptedParameter1  = 0x3i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER2 = 0x4i32
	CorruptedParameter2  = 0x4i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER3 = 0x5i32
	CorruptedParameter3  = 0x5i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER4 = 0x6i32
	CorruptedParameter4  = 0x6i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER5 = 0x7i32
	CorruptedParameter5  = 0x7i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER6 = 0x8i32
	CorruptedParameter6  = 0x8i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER7 = 0x9i32
	CorruptedParameter7  = 0x9i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER8 = 0xAi32
	CorruptedParameter8  = 0xAi32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER9 = 0xBi32
	CorruptedParameter9  = 0xBi32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER10 = 0xCi32
	CorruptedParameter10 = 0xCi32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER11 = 0xDi32
	CorruptedParameter11 = 0xDi32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER12 = 0xEi32
	CorruptedParameter12 = 0xEi32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER13 = 0xFi32
	CorruptedParameter13 = 0xFi32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER14 = 0x10i32
	CorruptedParameter14 = 0x10i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_PARAMETER15 = 0x11i32
	CorruptedParameter15 = 0x11i32,
	/// D3D12_MESSAGE_ID_CORRUPTED_MULTITHREADING = 0x12i32
	CorruptedMultiThreading = 0x12i32,
	/// D3D12_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY = 0x13i32
	MessageReportingOutOfMemory = 0x13i32,
	/// D3D12_MESSAGE_ID_GETPRIVATEDATA_MOREDATA = 0x14i32
	GetPrivateDataMoreData = 0x14i32,
	/// D3D12_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA = 0x15i32
	SetPrivateDataInvalidFreeData = 0x15i32,
	/// D3D12_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS = 0x18i32
	SetPrivateDataChangingParams = 0x18i32,
	/// D3D12_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY = 0x19i32
	SetPrivateDataOutOfMemory = 0x19i32,
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT = 0x1Ai32
	CreateShaderResourceViewUnrecognizedFormat = 0x1Ai32,
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC = 0x1Bi32
	CreateShaderResourceViewInvalidDesc = 0x1Bi32,
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT = 0x1Ci32
	CreateShaderResourceViewInvalidFormat = 0x1Ci32,
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANESLICE = 0x1Di32
	CreateShaderResourceViewInvalidVideoPlaneSlice = 0x1Di32,
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANESLICE = 0x1Ei32
	CreateShaderResourceViewInvalidPlaneSlice = 0x1Ei32,
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS = 0x1Fi32
	CreateShaderResourceViewInvalidDimensions = 0x1Fi32,
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE = 0x20i32
	CreateShaderResourceViewInvalidResource = 0x20i32,
	/// D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT = 0x23i32
	CreateRenderTargetViewUnrecognizedFormat = 0x23i32,
	/// D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT = 0x24i32
	CreateRenderTargetViewUnsupportedFormat = 0x24i32,
	/// D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC = 0x25i32
	CreateRenderTargetViewInvalidDesc = 0x25i32,
	/// D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT = 0x26i32
	CreateRenderTargetViewInvalidFormat = 0x26i32,
	/// D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDVIDEOPLANESLICE = 0x27i32
	CreateRenderTargetViewInvalidVideoPlaneSlice = 0x27i32,
	/// D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDPLANESLICE = 0x28i32
	CreateRenderTargetViewInvalidPlaneSlice = 0x28i32,
	/// D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS = 0x29i32
	CreateRenderTargetViewInvalidDimensions = 0x29i32,
	/// D3D12_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE = 0x2Ai32
	CreateRenderTargetViewInvalidResource = 0x2Ai32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT = 0x2Di32
	CreateDepthStencilViewUnrecognizedFormat = 0x2Di32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC = 0x2Ei32
	CreateDepthStencilViewInvalidDesc = 0x2Ei32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT = 0x2Fi32
	CreateDepthStencilViewInvalidFormat = 0x2Fi32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS = 0x30i32
	CreateDepthStencilViewInvalidDimensions = 0x30i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE = 0x31i32
	CreateDepthStencilViewInvalidResource = 0x31i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY = 0x34i32
	CreateInputLayoutOutOfMemory = 0x34i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS = 0x35i32
	CreateInputLayoutTooManyElements = 0x35i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT = 0x36i32
	CreateInputLayoutInvalidFormat = 0x36i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT = 0x37i32
	CreateInputLayoutIncompatibleFormat = 0x37i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT = 0x38i32
	CreateInputLayoutInvalidSlot = 0x38i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS = 0x39i32
	CreateInputLayoutInvalidInputSlotClass = 0x39i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH = 0x3Ai32
	CreateInputLayoutStepRateSlotClassMismatch = 0x3Ai32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE = 0x3Bi32
	CreateInputLayoutInvalidSlotClassChange = 0x3Bi32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE = 0x3Ci32
	CreateInputLayoutInvalidStepRateChange = 0x3Ci32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT = 0x3Di32
	CreateInputLayoutInvalidAlignment = 0x3Di32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC = 0x3Ei32
	CreateInputLayoutDuplicateSemantic = 0x3Ei32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE = 0x3Fi32
	CreateInputLayoutUnparseableInputSignature = 0x3Fi32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC = 0x40i32
	CreateInputLayoutNullSemantic = 0x40i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT = 0x41i32
	CreateInputLayoutMissingElement = 0x41i32,
	/// D3D12_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY = 0x42i32
	CreateVertexShaderOutOfMemory = 0x42i32,
	/// D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE = 0x43i32
	CreateVertexShaderInvalidShaderByteCode = 0x43i32,
	/// D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE = 0x44i32
	CreateVertexShaderInvalidShaderType = 0x44i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY = 0x45i32
	CreateGeometryShaderOutOfMemory = 0x45i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE = 0x46i32
	CreateGeometryShaderInvalidShaderByteCode = 0x46i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE = 0x47i32
	CreateGeometryShaderInvalidShaderType = 0x47i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY = 0x48i32
	CreateGeometryShaderWithStreamOutputOutOfMemory = 0x48i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE = 0x49i32
	CreateGeometryShaderWithStreamOutputInvalidShaderByteCode = 0x49i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE = 0x4Ai32
	CreateGeometryShaderWithStreamOutputInvalidShaderType = 0x4Ai32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES = 0x4Bi32
	CreateGeometryShaderWithStreamOutputInvalidNumEntries = 0x4Bi32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED = 0x4Ci32
	CreateGeometryShaderWithStreamOutputOutputStreamStrideUnused = 0x4Ci32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED = 0x4Fi32
	CreateGeometryShaderWithStreamOutputOutputSlot0Expected = 0x4Fi32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT = 0x50i32
	CreateGeometryShaderWithStreamOutputInvalidOutputSlot = 0x50i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT = 0x51i32
	CreateGeometryShaderWithStreamOutputOnlyOneElementPerSlot = 0x51i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT = 0x52i32
	CreateGeometryShaderWithStreamOutputInvalidComponentCount = 0x52i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT = 0x53i32
	CreateGeometryShaderWithStreamOutputInvalidStartComponentAndComponentCount = 0x53i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION = 0x54i32
	CreateGeometryShaderWithStreamOutputInvalidGapDefinition = 0x54i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT = 0x55i32
	CreateGeometryShaderWithStreamOutputRepeatedOutput = 0x55i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE = 0x56i32
	CreateGeometryShaderWithStreamOutputInvalidOutputStreamStride = 0x56i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC = 0x57i32
	CreateGeometryShaderWithStreamOutputMissingSemantic = 0x57i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH = 0x58i32
	CreateGeometryShaderWithStreamOutputMaskMismatch = 0x58i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS = 0x59i32
	CreateGeometryShaderWithStreamOutputCanThaveOnlyGaps = 0x59i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX = 0x5Ai32
	CreateGeometryShaderWithStreamOutputDeclTooComplex = 0x5Ai32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE = 0x5Bi32
	CreateGeometryShaderWithStreamOutputMissingOutputSignature = 0x5Bi32,
	/// D3D12_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY = 0x5Ci32
	CreatePixelShaderOutOfMemory = 0x5Ci32,
	/// D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE = 0x5Di32
	CreatePixelShaderInvalidShaderByteCode = 0x5Di32,
	/// D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE = 0x5Ei32
	CreatePixelShaderInvalidShaderType = 0x5Ei32,
	/// D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE = 0x5Fi32
	CreateRasterizerStateInvalidFillMode = 0x5Fi32,
	/// D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE = 0x60i32
	CreateRasterizerStateInvalidCullMode = 0x60i32,
	/// D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP = 0x61i32
	CreateRasterizerStateInvalidDepthBiasClamp = 0x61i32,
	/// D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS = 0x62i32
	CreateRasterizerStateInvalidSlopeScaledDepthBias = 0x62i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK = 0x64i32
	CreateDepthStencilStateInvalidDepthWriteMask = 0x64i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC = 0x65i32
	CreateDepthStencilStateInvalidDepthFunc = 0x65i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP = 0x66i32
	CreateDepthStencilStateInvalidFrontFaceStencilFailOp = 0x66i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP = 0x67i32
	CreateDepthStencilStateInvalidFrontFaceStencilZFailOp = 0x67i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP = 0x68i32
	CreateDepthStencilStateInvalidFrontFaceStencilPasSop = 0x68i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC = 0x69i32
	CreateDepthStencilStateInvalidFrontFaceStencilFunc = 0x69i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP = 0x6Ai32
	CreateDepthStencilStateInvalidBackFaceStencilFailOp = 0x6Ai32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP = 0x6Bi32
	CreateDepthStencilStateInvalidBackFaceStencilZFailOp = 0x6Bi32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP = 0x6Ci32
	CreateDepthStencilStateInvalidBackFaceStencilPasSop = 0x6Ci32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC = 0x6Di32
	CreateDepthStencilStateInvalidBackFaceStencilFunc = 0x6Di32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND = 0x6Fi32
	CreateBlendStateInvalidSrcBlend = 0x6Fi32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND = 0x70i32
	CreateBlendStateInvalidDestBlend = 0x70i32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP = 0x71i32
	CreateBlendStateInvalidBlendOp = 0x71i32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA = 0x72i32
	CreateBlendStateInvalidSrcBlendAlpha = 0x72i32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA = 0x73i32
	CreateBlendStateInvalidDestBlendAlpha = 0x73i32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA = 0x74i32
	CreateBlendStateInvalidBlendOpAlpha = 0x74i32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK = 0x75i32
	CreateBlendStateInvalidRenderTargetWriteMask = 0x75i32,
	/// D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID = 0x87i32
	ClearDepthStencilViewInvalid = 0x87i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_NOT_SET = 0xC8i32
	CommandListDrawRootSignatureNotSet = 0xC8i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_ROOT_SIGNATURE_MISMATCH = 0xC9i32
	CommandListDrawRootSignatureMismatch = 0xC9i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_NOT_SET = 0xCAi32
	CommandListDrawVertexBufferNotSet = 0xCAi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL = 0xD1i32
	CommandListDrawVertexBufferStrideTooSmall = 0xD1i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_BUFFER_TOO_SMALL = 0xD2i32
	CommandListDrawVertexBufferTooSmall = 0xD2i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_NOT_SET = 0xD3i32
	CommandListDrawIndexBufferNotSet = 0xD3i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_FORMAT_INVALID = 0xD4i32
	CommandListDrawIndexBufferFormatInvalid = 0xD4i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_BUFFER_TOO_SMALL = 0xD5i32
	CommandListDrawIndexBufferTooSmall = 0xD5i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INVALID_PRIMITIVETOPOLOGY = 0xDBi32
	CommandListDrawInvalidPrimitiveTopology = 0xDBi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_VERTEX_STRIDE_UNALIGNED = 0xDDi32
	CommandListDrawVertexStrideUnaligned = 0xDDi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_INDEX_OFFSET_UNALIGNED = 0xDEi32
	CommandListDrawIndexOffsetUnaligned = 0xDEi32,
	/// D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT = 0xE8i32
	DeviceRemovalProcessAtFault = 0xE8i32,
	/// D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT = 0xE9i32
	DeviceRemovalProcessPossiblyAtFault = 0xE9i32,
	/// D3D12_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT = 0xEAi32
	DeviceRemovalProcessNotAtFault = 0xEAi32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC = 0xEFi32
	CreateInputLayoutTrailingDigitInSemantic = 0xEFi32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC = 0xF0i32
	CreateGeometryShaderWithStreamOutputTrailingDigitInSemantic = 0xF0i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH = 0xF5i32
	CreateInputLayoutTypeMismatch = 0xF5i32,
	/// D3D12_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT = 0xFDi32
	CreateInputLayoutEmptyLayout = 0xFDi32,
	/// D3D12_MESSAGE_ID_LIVE_OBJECT_SUMMARY = 0xFFi32
	LiveObjectSummary    = 0xFFi32,
	/// D3D12_MESSAGE_ID_LIVE_DEVICE = 0x112i32
	LiveDevice           = 0x112i32,
	/// D3D12_MESSAGE_ID_LIVE_SWAPCHAIN = 0x113i32
	LiveSwapChain        = 0x113i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFLAGS = 0x114i32
	CreateDepthStencilViewInvalidFlags = 0x114i32,
	/// D3D12_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDCLASSLINKAGE = 0x115i32
	CreateVertexShaderInvalidClassLinkage = 0x115i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDCLASSLINKAGE = 0x116i32
	CreateGeometryShaderInvalidClassLinkage = 0x116i32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAMTORASTERIZER = 0x118i32
	CreateGeometryShaderWithStreamOutputInvalidStreamToRasterizer = 0x118i32,
	/// D3D12_MESSAGE_ID_CREATEPIXELSHADER_INVALIDCLASSLINKAGE = 0x11Bi32
	CreatePixelShaderInvalidClassLinkage = 0x11Bi32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAM = 0x11Ci32
	CreateGeometryShaderWithStreamOutputInvalidStream = 0x11Ci32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDENTRIES = 0x11Di32
	CreateGeometryShaderWithStreamOutputUnexpectedEntries = 0x11Di32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTRIDES = 0x11Ei32
	CreateGeometryShaderWithStreamOutputUnexpectedStrides = 0x11Ei32,
	/// D3D12_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTRIDES = 0x11Fi32
	CreateGeometryShaderWithStreamOutputInvalidNumStrides = 0x11Fi32,
	/// D3D12_MESSAGE_ID_CREATEHULLSHADER_OUTOFMEMORY = 0x121i32
	CreateHullShaderOutOfMemory = 0x121i32,
	/// D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERBYTECODE = 0x122i32
	CreateHullShaderInvalidShaderByteCode = 0x122i32,
	/// D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERTYPE = 0x123i32
	CreateHullShaderInvalidShaderType = 0x123i32,
	/// D3D12_MESSAGE_ID_CREATEHULLSHADER_INVALIDCLASSLINKAGE = 0x124i32
	CreateHullShaderInvalidClassLinkage = 0x124i32,
	/// D3D12_MESSAGE_ID_CREATEDOMAINSHADER_OUTOFMEMORY = 0x126i32
	CreateDomainShaderOutOfMemory = 0x126i32,
	/// D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERBYTECODE = 0x127i32
	CreateDomainShaderInvalidShaderByteCode = 0x127i32,
	/// D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERTYPE = 0x128i32
	CreateDomainShaderInvalidShaderType = 0x128i32,
	/// D3D12_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCLASSLINKAGE = 0x129i32
	CreateDomainShaderInvalidClassLinkage = 0x129i32,
	/// D3D12_MESSAGE_ID_RESOURCE_UNMAP_NOTMAPPED = 0x136i32
	ResourceUnmapNotMapped = 0x136i32,
	/// D3D12_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_MISMATCHED_DATA_SIZE = 0x13Ei32
	DeviceCheckFeatureSupportMismatchedDataSize = 0x13Ei32,
	/// D3D12_MESSAGE_ID_CREATECOMPUTESHADER_OUTOFMEMORY = 0x141i32
	CreateComputeShaderOutOfMemory = 0x141i32,
	/// D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERBYTECODE = 0x142i32
	CreateComputeShaderInvalidShaderByteCode = 0x142i32,
	/// D3D12_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCLASSLINKAGE = 0x143i32
	CreateComputeShaderInvalidClassLinkage = 0x143i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEFLOATOPSNOTSUPPORTED = 0x14Bi32
	DeviceCreateVertexShaderDoubleFloAtopSnotSupported = 0x14Bi32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEFLOATOPSNOTSUPPORTED = 0x14Ci32
	DeviceCreateHullShaderDoubleFloAtopSnotSupported = 0x14Ci32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEFLOATOPSNOTSUPPORTED = 0x14Di32
	DeviceCreateDomainShaderDoubleFloAtopSnotSupported = 0x14Di32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEFLOATOPSNOTSUPPORTED = 0x14Ei32
	DeviceCreateGeometryShaderDoubleFloAtopSnotSupported = 0x14Ei32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEFLOATOPSNOTSUPPORTED = 0x14Fi32
	DeviceCreateGeometryShaderWithStreamOutputDoubleFloAtopSnotSupported = 0x14Fi32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEFLOATOPSNOTSUPPORTED = 0x150i32
	DeviceCreatePixelShaderDoubleFloAtopSnotSupported = 0x150i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEFLOATOPSNOTSUPPORTED = 0x151i32
	DeviceCreateComputeShaderDoubleFloAtopSnotSupported = 0x151i32,
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE = 0x154i32
	CreateUnorderedAccessViewInvalidResource = 0x154i32,
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC = 0x155i32
	CreateUnorderedAccessViewInvalidDesc = 0x155i32,
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT = 0x156i32
	CreateUnorderedAccessViewInvalidFormat = 0x156i32,
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANESLICE = 0x157i32
	CreateUnorderedAccessViewInvalidVideoPlaneSlice = 0x157i32,
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANESLICE = 0x158i32
	CreateUnorderedAccessViewInvalidPlaneSlice = 0x158i32,
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS = 0x159i32
	CreateUnorderedAccessViewInvalidDimensions = 0x159i32,
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT = 0x15Ai32
	CreateUnorderedAccessViewUnrecognizedFormat = 0x15Ai32,
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS = 0x162i32
	CreateUnorderedAccessViewInvalidFlags = 0x162i32,
	/// D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFORCEDSAMPLECOUNT = 0x191i32
	CreateRasterizerStateInvalidForcedSampleCount = 0x191i32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_INVALIDLOGICOPS = 0x193i32
	CreateBlendStateInvalidLogICops = 0x193i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEEXTENSIONSNOTSUPPORTED = 0x19Ai32
	DeviceCreateVertexShaderDoubleExtensionSnotSupported = 0x19Ai32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEEXTENSIONSNOTSUPPORTED = 0x19Ci32
	DeviceCreateHullShaderDoubleExtensionSnotSupported = 0x19Ci32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEEXTENSIONSNOTSUPPORTED = 0x19Ei32
	DeviceCreateDomainShaderDoubleExtensionSnotSupported = 0x19Ei32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEEXTENSIONSNOTSUPPORTED = 0x1A0i32
	DeviceCreateGeometryShaderDoubleExtensionSnotSupported = 0x1A0i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEEXTENSIONSNOTSUPPORTED = 0x1A2i32
	DeviceCreateGeometryShaderWithStreamOutputDoubleExtensionSnotSupported = 0x1A2i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEEXTENSIONSNOTSUPPORTED = 0x1A4i32
	DeviceCreatePixelShaderDoubleExtensionSnotSupported = 0x1A4i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEEXTENSIONSNOTSUPPORTED = 0x1A6i32
	DeviceCreateComputeShaderDoubleExtensionSnotSupported = 0x1A6i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_UAVSNOTSUPPORTED = 0x1A9i32
	DeviceCreateVertexShaderUavSnotSupported = 0x1A9i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEHULLSHADER_UAVSNOTSUPPORTED = 0x1AAi32
	DeviceCreateHullShaderUavSnotSupported = 0x1AAi32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_UAVSNOTSUPPORTED = 0x1ABi32
	DeviceCreateDomainShaderUavSnotSupported = 0x1ABi32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_UAVSNOTSUPPORTED = 0x1ACi32
	DeviceCreateGeometryShaderUavSnotSupported = 0x1ACi32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UAVSNOTSUPPORTED = 0x1ADi32
	DeviceCreateGeometryShaderWithStreamOutputUavSnotSupported = 0x1ADi32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_UAVSNOTSUPPORTED = 0x1AEi32
	DeviceCreatePixelShaderUavSnotSupported = 0x1AEi32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_UAVSNOTSUPPORTED = 0x1AFi32
	DeviceCreateComputeShaderUavSnotSupported = 0x1AFi32,
	/// D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDSOURCERECT = 0x1BFi32
	DeviceClearViewInvalidSourceRect = 0x1BFi32,
	/// D3D12_MESSAGE_ID_DEVICE_CLEARVIEW_EMPTYRECT = 0x1C0i32
	DeviceClearViewEmptyRect = 0x1C0i32,
	/// D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_INVALID_PARAMETER = 0x1EDi32
	UpdateTileMappingsInvalidParameter = 0x1EDi32,
	/// D3D12_MESSAGE_ID_COPYTILEMAPPINGS_INVALID_PARAMETER = 0x1EEi32
	CopyTileMappingsInvalidParameter = 0x1EEi32,
	/// D3D12_MESSAGE_ID_CREATEDEVICE_INVALIDARGS = 0x1FAi32
	CreateDeviceInvalidArgs = 0x1FAi32,
	/// D3D12_MESSAGE_ID_CREATEDEVICE_WARNING = 0x1FBi32
	CreateDeviceWarning  = 0x1FBi32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_TYPE = 0x207i32
	ResourceBarrierInvalidType = 0x207i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_NULL_POINTER = 0x208i32
	ResourceBarrierNullPointer = 0x208i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SUBRESOURCE = 0x209i32
	ResourceBarrierInvalidSubresource = 0x209i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_RESERVED_BITS = 0x20Ai32
	ResourceBarrierReservedBits = 0x20Ai32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISSING_BIND_FLAGS = 0x20Bi32
	ResourceBarrierMissingBindFlags = 0x20Bi32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_MISC_FLAGS = 0x20Ci32
	ResourceBarrierMismatchingMiscFlags = 0x20Ci32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_MATCHING_STATES = 0x20Di32
	ResourceBarrierMatchingStates = 0x20Di32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINATION = 0x20Ei32
	ResourceBarrierInvalidCombination = 0x20Ei32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_BEFORE_AFTER_MISMATCH = 0x20Fi32
	ResourceBarrierBeforeAfterMismatch = 0x20Fi32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_RESOURCE = 0x210i32
	ResourceBarrierInvalidResource = 0x210i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_SAMPLE_COUNT = 0x211i32
	ResourceBarrierSampleCount = 0x211i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS = 0x212i32
	ResourceBarrierInvalidFlags = 0x212i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMBINED_FLAGS = 0x213i32
	ResourceBarrierInvalidCombinedFlags = 0x213i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAGS_FOR_FORMAT = 0x214i32
	ResourceBarrierInvalidFlagsForFormat = 0x214i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_SPLIT_BARRIER = 0x215i32
	ResourceBarrierInvalidSplitBarrier = 0x215i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_END = 0x216i32
	ResourceBarrierUnmatchedEnd = 0x216i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_UNMATCHED_BEGIN = 0x217i32
	ResourceBarrierUnmatchedBegin = 0x217i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_FLAG = 0x218i32
	ResourceBarrierInvalidFlag = 0x218i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_COMMAND_LIST_TYPE = 0x219i32
	ResourceBarrierInvalidCommandListType = 0x219i32,
	/// D3D12_MESSAGE_ID_INVALID_SUBRESOURCE_STATE = 0x21Ai32
	InvalidSubresourceState = 0x21Ai32,
	/// D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CONTENTION = 0x21Ci32
	CommandAllocatorContention = 0x21Ci32,
	/// D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET = 0x21Di32
	CommandAllocatorReset = 0x21Di32,
	/// D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_RESET_BUNDLE = 0x21Ei32
	CommandAllocatorResetBundle = 0x21Ei32,
	/// D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_CANNOT_RESET = 0x21Fi32
	CommandAllocatorCannotReset = 0x21Fi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_OPEN = 0x220i32
	CommandListOpen      = 0x220i32,
	/// D3D12_MESSAGE_ID_INVALID_BUNDLE_API = 0x222i32
	InvalidBundleApi     = 0x222i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_CLOSED = 0x223i32
	CommandListClosed    = 0x223i32,
	/// D3D12_MESSAGE_ID_WRONG_COMMAND_ALLOCATOR_TYPE = 0x225i32
	WrongCommandAllocatorType = 0x225i32,
	/// D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_SYNC = 0x228i32
	CommandAllocatorSync = 0x228i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_SYNC = 0x229i32
	CommandListSync      = 0x229i32,
	/// D3D12_MESSAGE_ID_SET_DESCRIPTOR_HEAP_INVALID = 0x22Ai32
	SetDescriptorHeapInvalid = 0x22Ai32,
	/// D3D12_MESSAGE_ID_CREATE_COMMANDQUEUE = 0x22Di32
	CreateCommandQueue   = 0x22Di32,
	/// D3D12_MESSAGE_ID_CREATE_COMMANDALLOCATOR = 0x22Ei32
	CreateCommandAllocator = 0x22Ei32,
	/// D3D12_MESSAGE_ID_CREATE_PIPELINESTATE = 0x22Fi32
	CreatePipelineState  = 0x22Fi32,
	/// D3D12_MESSAGE_ID_CREATE_COMMANDLIST12 = 0x230i32
	CreateCommandList12  = 0x230i32,
	/// D3D12_MESSAGE_ID_CREATE_RESOURCE = 0x232i32
	CreateResource       = 0x232i32,
	/// D3D12_MESSAGE_ID_CREATE_DESCRIPTORHEAP = 0x233i32
	CreateDescriptorHeap = 0x233i32,
	/// D3D12_MESSAGE_ID_CREATE_ROOTSIGNATURE = 0x234i32
	CreateRootSignature  = 0x234i32,
	/// D3D12_MESSAGE_ID_CREATE_LIBRARY = 0x235i32
	CreateLibrary        = 0x235i32,
	/// D3D12_MESSAGE_ID_CREATE_HEAP = 0x236i32
	CreateHeap           = 0x236i32,
	/// D3D12_MESSAGE_ID_CREATE_MONITOREDFENCE = 0x237i32
	CreateMonitoredFence = 0x237i32,
	/// D3D12_MESSAGE_ID_CREATE_QUERYHEAP = 0x238i32
	CreateQueryHeap      = 0x238i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMANDSIGNATURE = 0x239i32
	CreateCommandSignature = 0x239i32,
	/// D3D12_MESSAGE_ID_LIVE_COMMANDQUEUE = 0x23Ai32
	LiveCommandQueue     = 0x23Ai32,
	/// D3D12_MESSAGE_ID_LIVE_COMMANDALLOCATOR = 0x23Bi32
	LiveCommandAllocator = 0x23Bi32,
	/// D3D12_MESSAGE_ID_LIVE_PIPELINESTATE = 0x23Ci32
	LivePipelineState    = 0x23Ci32,
	/// D3D12_MESSAGE_ID_LIVE_COMMANDLIST12 = 0x23Di32
	LiveCommandList12    = 0x23Di32,
	/// D3D12_MESSAGE_ID_LIVE_RESOURCE = 0x23Fi32
	LiveResource         = 0x23Fi32,
	/// D3D12_MESSAGE_ID_LIVE_DESCRIPTORHEAP = 0x240i32
	LiveDescriptorHeap   = 0x240i32,
	/// D3D12_MESSAGE_ID_LIVE_ROOTSIGNATURE = 0x241i32
	LiveRootSignature    = 0x241i32,
	/// D3D12_MESSAGE_ID_LIVE_LIBRARY = 0x242i32
	LiveLibrary          = 0x242i32,
	/// D3D12_MESSAGE_ID_LIVE_HEAP = 0x243i32
	LiveHeap             = 0x243i32,
	/// D3D12_MESSAGE_ID_LIVE_MONITOREDFENCE = 0x244i32
	LiveMonitoredFence   = 0x244i32,
	/// D3D12_MESSAGE_ID_LIVE_QUERYHEAP = 0x245i32
	LiveQueryHeap        = 0x245i32,
	/// D3D12_MESSAGE_ID_LIVE_COMMANDSIGNATURE = 0x246i32
	LiveCommandSignature = 0x246i32,
	/// D3D12_MESSAGE_ID_DESTROY_COMMANDQUEUE = 0x247i32
	DestroyCommandQueue  = 0x247i32,
	/// D3D12_MESSAGE_ID_DESTROY_COMMANDALLOCATOR = 0x248i32
	DestroyCommandAllocator = 0x248i32,
	/// D3D12_MESSAGE_ID_DESTROY_PIPELINESTATE = 0x249i32
	DestroyPipelineState = 0x249i32,
	/// D3D12_MESSAGE_ID_DESTROY_COMMANDLIST12 = 0x24Ai32
	DestroyCommandList12 = 0x24Ai32,
	/// D3D12_MESSAGE_ID_DESTROY_RESOURCE = 0x24Ci32
	DestroyResource      = 0x24Ci32,
	/// D3D12_MESSAGE_ID_DESTROY_DESCRIPTORHEAP = 0x24Di32
	DestroyDescriptorHeap = 0x24Di32,
	/// D3D12_MESSAGE_ID_DESTROY_ROOTSIGNATURE = 0x24Ei32
	DestroyRootSignature = 0x24Ei32,
	/// D3D12_MESSAGE_ID_DESTROY_LIBRARY = 0x24Fi32
	DestroyLibrary       = 0x24Fi32,
	/// D3D12_MESSAGE_ID_DESTROY_HEAP = 0x250i32
	DestroyHeap          = 0x250i32,
	/// D3D12_MESSAGE_ID_DESTROY_MONITOREDFENCE = 0x251i32
	DestroyMonitoredFence = 0x251i32,
	/// D3D12_MESSAGE_ID_DESTROY_QUERYHEAP = 0x252i32
	DestroyQueryHeap     = 0x252i32,
	/// D3D12_MESSAGE_ID_DESTROY_COMMANDSIGNATURE = 0x253i32
	DestroyCommandSignature = 0x253i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONS = 0x255i32
	CreateResourceInvalidDimensions = 0x255i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMISCFLAGS = 0x257i32
	CreateResourceInvalidMiscFlags = 0x257i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDARG_RETURN = 0x25Ai32
	CreateResourceInvalidArgReturn = 0x25Ai32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_OUTOFMEMORY_RETURN = 0x25Bi32
	CreateResourceOutOfMemoryReturn = 0x25Bi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDESC = 0x25Ci32
	CreateResourceInvalidDesc = 0x25Ci32,
	/// D3D12_MESSAGE_ID_POSSIBLY_INVALID_SUBRESOURCE_STATE = 0x25Fi32
	PossiblyInvalidSubresourceState = 0x25Fi32,
	/// D3D12_MESSAGE_ID_INVALID_USE_OF_NON_RESIDENT_RESOURCE = 0x260i32
	InvalidUseOfNonResidentResource = 0x260i32,
	/// D3D12_MESSAGE_ID_POSSIBLE_INVALID_USE_OF_NON_RESIDENT_RESOURCE = 0x261i32
	PossibleInvalidUseOfNonResidentResource = 0x261i32,
	/// D3D12_MESSAGE_ID_BUNDLE_PIPELINE_STATE_MISMATCH = 0x262i32
	BundlePipelineStateMismatch = 0x262i32,
	/// D3D12_MESSAGE_ID_PRIMITIVE_TOPOLOGY_MISMATCH_PIPELINE_STATE = 0x263i32
	PrimitiveTopologyMismatchPipelineState = 0x263i32,
	/// D3D12_MESSAGE_ID_RENDER_TARGET_FORMAT_MISMATCH_PIPELINE_STATE = 0x265i32
	RenderTargetFormatMismatchPipelineState = 0x265i32,
	/// D3D12_MESSAGE_ID_RENDER_TARGET_SAMPLE_DESC_MISMATCH_PIPELINE_STATE = 0x266i32
	RenderTargetSampleDescMismatchPipelineState = 0x266i32,
	/// D3D12_MESSAGE_ID_DEPTH_STENCIL_FORMAT_MISMATCH_PIPELINE_STATE = 0x267i32
	DepthStencilFormatMismatchPipelineState = 0x267i32,
	/// D3D12_MESSAGE_ID_DEPTH_STENCIL_SAMPLE_DESC_MISMATCH_PIPELINE_STATE = 0x268i32
	DepthStencilSampleDescMismatchPipelineState = 0x268i32,
	/// D3D12_MESSAGE_ID_CREATESHADER_INVALIDBYTECODE = 0x26Ei32
	CreateShaderInvalidByteCode = 0x26Ei32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_NULLDESC = 0x26Fi32
	CreateHeapNullDesc   = 0x26Fi32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_INVALIDSIZE = 0x270i32
	CreateHeapInvalidSize = 0x270i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDHEAPTYPE = 0x271i32
	CreateHeapUnrecognizedHeapType = 0x271i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES = 0x272i32
	CreateHeapUnrecognizedCpuPageProperties = 0x272i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMEMORYPOOL = 0x273i32
	CreateHeapUnrecognizedMemoryPool = 0x273i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_INVALIDPROPERTIES = 0x274i32
	CreateHeapInvalidProperties = 0x274i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_INVALIDALIGNMENT = 0x275i32
	CreateHeapInvalidAlignment = 0x275i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_UNRECOGNIZEDMISCFLAGS = 0x276i32
	CreateHeapUnrecognizedMiscFlags = 0x276i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_INVALIDMISCFLAGS = 0x277i32
	CreateHeapInvalidMiscFlags = 0x277i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_INVALIDARG_RETURN = 0x278i32
	CreateHeapInvalidArgReturn = 0x278i32,
	/// D3D12_MESSAGE_ID_CREATEHEAP_OUTOFMEMORY_RETURN = 0x279i32
	CreateHeapOutOfMemoryReturn = 0x279i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAPPROPERTIES = 0x27Ai32
	CreateResourceAndHeapNullHeapProperties = 0x27Ai32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPTYPE = 0x27Bi32
	CreateResourceAndHeapUnrecognizedHeapType = 0x27Bi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDCPUPAGEPROPERTIES = 0x27Ci32
	CreateResourceAndHeapUnrecognizedCpuPageProperties = 0x27Ci32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDMEMORYPOOL = 0x27Di32
	CreateResourceAndHeapUnrecognizedMemoryPool = 0x27Di32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPPROPERTIES = 0x27Ei32
	CreateResourceAndHeapInvalidHeapProperties = 0x27Ei32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_UNRECOGNIZEDHEAPMISCFLAGS = 0x27Fi32
	CreateResourceAndHeapUnrecognizedHeapMiscFlags = 0x27Fi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDHEAPMISCFLAGS = 0x280i32
	CreateResourceAndHeapInvalidHeapMiscFlags = 0x280i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_INVALIDARG_RETURN = 0x281i32
	CreateResourceAndHeapInvalidArgReturn = 0x281i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_OUTOFMEMORY_RETURN = 0x282i32
	CreateResourceAndHeapOutOfMemoryReturn = 0x282i32,
	/// D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_UNRECOGNIZEDHEAPTYPE = 0x283i32
	GetCustomHeapPropertiesUnrecognizedHeapType = 0x283i32,
	/// D3D12_MESSAGE_ID_GETCUSTOMHEAPPROPERTIES_INVALIDHEAPTYPE = 0x284i32
	GetCustomHeapPropertiesInvalidHeapType = 0x284i32,
	/// D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_INVALID_DESC = 0x285i32
	CreateDescriptorHeapInvalidDesc = 0x285i32,
	/// D3D12_MESSAGE_ID_INVALID_DESCRIPTOR_HANDLE = 0x286i32
	InvalidDescriptorHandle = 0x286i32,
	/// D3D12_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_CONSERVATIVERASTERMODE = 0x287i32
	CreateRasterizerStateInvalidConservativeRasterMode = 0x287i32,
	/// D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_RESOURCE = 0x289i32
	CreateConstantBufferViewInvalidResource = 0x289i32,
	/// D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_DESC = 0x28Ai32
	CreateConstantBufferViewInvalidDesc = 0x28Ai32,
	/// D3D12_MESSAGE_ID_CREATE_UNORDEREDACCESS_VIEW_INVALID_COUNTER_USAGE = 0x28Ci32
	CreateUnorderedAccessViewInvalidCounterUsage = 0x28Ci32,
	/// D3D12_MESSAGE_ID_COPY_DESCRIPTORS_INVALID_RANGES = 0x28Di32
	CopyDescriptorsInvalidRanges = 0x28Di32,
	/// D3D12_MESSAGE_ID_COPY_DESCRIPTORS_WRITE_ONLY_DESCRIPTOR = 0x28Ei32
	CopyDescriptorsWriteOnlyDescriptor = 0x28Ei32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RTV_FORMAT_NOT_UNKNOWN = 0x28Fi32
	CreateGraphicsPipelineStateRtvFormatNotUnknown = 0x28Fi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_RENDER_TARGET_COUNT = 0x290i32
	CreateGraphicsPipelineStateInvalidRenderTargetCount = 0x290i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VERTEX_SHADER_NOT_SET = 0x291i32
	CreateGraphicsPipelineStateVertexShaderNotSet = 0x291i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_NOT_SET = 0x292i32
	CreateGraphicsPipelineStateInputLayoutNotSet = 0x292i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_HS_DS_SIGNATURE_MISMATCH = 0x293i32
	CreateGraphicsPipelineStateShaderLinkageHsDsSignatureMismatch = 0x293i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERINDEX = 0x294i32
	CreateGraphicsPipelineStateShaderLinkageRegisterIndex = 0x294i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_COMPONENTTYPE = 0x295i32
	CreateGraphicsPipelineStateShaderLinkageComponentType = 0x295i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_REGISTERMASK = 0x296i32
	CreateGraphicsPipelineStateShaderLinkageRegisterMask = 0x296i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SYSTEMVALUE = 0x297i32
	CreateGraphicsPipelineStateShaderLinkageSystemValue = 0x297i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS = 0x298i32
	CreateGraphicsPipelineStateShaderLinkageNeverWrittenAlwaysReads = 0x298i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_MINPRECISION = 0x299i32
	CreateGraphicsPipelineStateShaderLinkageMinPrecision = 0x299i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND = 0x29Ai32
	CreateGraphicsPipelineStateShaderLinkageSemanticNameNotFound = 0x29Ai32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_XOR_DS_MISMATCH = 0x29Bi32
	CreateGraphicsPipelineStateHsXorDsMismatch = 0x29Bi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HULL_SHADER_INPUT_TOPOLOGY_MISMATCH = 0x29Ci32
	CreateGraphicsPipelineStateHullShaderInputTopologyMismatch = 0x29Ci32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_CONTROL_POINT_COUNT_MISMATCH = 0x29Di32
	CreateGraphicsPipelineStateHsDsControlPointCountMismatch = 0x29Di32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_DS_TESSELLATOR_DOMAIN_MISMATCH = 0x29Ei32
	CreateGraphicsPipelineStateHsDsTessellatorDomainMismatch = 0x29Ei32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN = 0x29Fi32
	CreateGraphicsPipelineStateInvalidUseOfCenterMultiSamplePattern = 0x29Fi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_USE_OF_FORCED_SAMPLE_COUNT = 0x2A0i32
	CreateGraphicsPipelineStateInvalidUseOfForcedSampleCount = 0x2A0i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_PRIMITIVETOPOLOGY = 0x2A1i32
	CreateGraphicsPipelineStateInvalidPrimitiveTopology = 0x2A1i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SYSTEMVALUE = 0x2A2i32
	CreateGraphicsPipelineStateInvalidSystemValue = 0x2A2i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0 = 0x2A3i32
	CreateGraphicsPipelineStateOmDualSourceBlendingCanOnlyHaveRenderTarget0 = 0x2A3i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING = 0x2A4i32
	CreateGraphicsPipelineStateOmRenderTargetDoesNotSupportBlending = 0x2A4i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_TYPE_MISMATCH = 0x2A5i32
	CreateGraphicsPipelineStatePsOutputTypeMismatch = 0x2A5i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_OM_RENDER_TARGET_DOES_NOT_SUPPORT_LOGIC_OPS = 0x2A6i32
	CreateGraphicsPipelineStateOmRenderTargetDoesNotSupportLogicOps = 0x2A6i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RENDERTARGETVIEW_NOT_SET = 0x2A7i32
	CreateGraphicsPipelineStateRenderTargetViewNotSet = 0x2A7i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DEPTHSTENCILVIEW_NOT_SET = 0x2A8i32
	CreateGraphicsPipelineStateDepthStencilViewNotSet = 0x2A8i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_INPUT_PRIMITIVE_MISMATCH = 0x2A9i32
	CreateGraphicsPipelineStateGsInputPrimitiveMismatch = 0x2A9i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_POSITION_NOT_PRESENT = 0x2AAi32
	CreateGraphicsPipelineStatePositionNotPresent = 0x2AAi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE_FLAGS = 0x2ABi32
	CreateGraphicsPipelineStateMissingRootSignatureFlags = 0x2ABi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_INDEX_BUFFER_PROPERTIES = 0x2ACi32
	CreateGraphicsPipelineStateInvalidIndexBufferProperties = 0x2ACi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INVALID_SAMPLE_DESC = 0x2ADi32
	CreateGraphicsPipelineStateInvalidSampleDesc = 0x2ADi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_HS_ROOT_SIGNATURE_MISMATCH = 0x2AEi32
	CreateGraphicsPipelineStateHsRootSignatureMismatch = 0x2AEi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_DS_ROOT_SIGNATURE_MISMATCH = 0x2AFi32
	CreateGraphicsPipelineStateDsRootSignatureMismatch = 0x2AFi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VS_ROOT_SIGNATURE_MISMATCH = 0x2B0i32
	CreateGraphicsPipelineStateVsRootSignatureMismatch = 0x2B0i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_GS_ROOT_SIGNATURE_MISMATCH = 0x2B1i32
	CreateGraphicsPipelineStateGsRootSignatureMismatch = 0x2B1i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_ROOT_SIGNATURE_MISMATCH = 0x2B2i32
	CreateGraphicsPipelineStatePsRootSignatureMismatch = 0x2B2i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MISSING_ROOT_SIGNATURE = 0x2B3i32
	CreateGraphicsPipelineStateMissingRootSignature = 0x2B3i32,
	/// D3D12_MESSAGE_ID_EXECUTE_BUNDLE_OPEN_BUNDLE = 0x2B4i32
	ExecuteBundleOpenBundle = 0x2B4i32,
	/// D3D12_MESSAGE_ID_EXECUTE_BUNDLE_DESCRIPTOR_HEAP_MISMATCH = 0x2B5i32
	ExecuteBundleDescriptorHeapMismatch = 0x2B5i32,
	/// D3D12_MESSAGE_ID_EXECUTE_BUNDLE_TYPE = 0x2B6i32
	ExecuteBundleType    = 0x2B6i32,
	/// D3D12_MESSAGE_ID_DRAW_EMPTY_SCISSOR_RECTANGLE = 0x2B7i32
	DrawEmptyScissorRectangle = 0x2B7i32,
	/// D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_BLOB_NOT_FOUND = 0x2B8i32
	CreateRootSignatureBlobNotFound = 0x2B8i32,
	/// D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_DESERIALIZE_FAILED = 0x2B9i32
	CreateRootSignatureDeserializeFailed = 0x2B9i32,
	/// D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_INVALID_CONFIGURATION = 0x2BAi32
	CreateRootSignatureInvalidConfiguration = 0x2BAi32,
	/// D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_SUPPORTED_ON_DEVICE = 0x2BBi32
	CreateRootSignatureNotSupportedOnDevice = 0x2BBi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLRESOURCEPROPERTIES = 0x2BCi32
	CreateResourceAndHeapNullResourceProperties = 0x2BCi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCEANDHEAP_NULLHEAP = 0x2BDi32
	CreateResourceAndHeapNullHeap = 0x2BDi32,
	/// D3D12_MESSAGE_ID_GETRESOURCEALLOCATIONINFO_INVALIDRDESCS = 0x2BEi32
	GetResourceAllocationInfoInvalidRDescs = 0x2BEi32,
	/// D3D12_MESSAGE_ID_MAKERESIDENT_NULLOBJECTARRAY = 0x2BFi32
	MakeResidentNullObjectArray = 0x2BFi32,
	/// D3D12_MESSAGE_ID_EVICT_NULLOBJECTARRAY = 0x2C1i32
	EvictNullObjectArray = 0x2C1i32,
	/// D3D12_MESSAGE_ID_SET_DESCRIPTOR_TABLE_INVALID = 0x2C4i32
	SetDescriptorTableInvalid = 0x2C4i32,
	/// D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_INVALID = 0x2C5i32
	SetRootConstantInvalid = 0x2C5i32,
	/// D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_BUFFER_VIEW_INVALID = 0x2C6i32
	SetRootConstantBufferViewInvalid = 0x2C6i32,
	/// D3D12_MESSAGE_ID_SET_ROOT_SHADER_RESOURCE_VIEW_INVALID = 0x2C7i32
	SetRootShaderResourceViewInvalid = 0x2C7i32,
	/// D3D12_MESSAGE_ID_SET_ROOT_UNORDERED_ACCESS_VIEW_INVALID = 0x2C8i32
	SetRootUnorderedAccessViewInvalid = 0x2C8i32,
	/// D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID_DESC = 0x2C9i32
	SetVertexBuffersInvalidDesc = 0x2C9i32,
	/// D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID_DESC = 0x2CBi32
	SetIndexBufferInvalidDesc = 0x2CBi32,
	/// D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID_DESC = 0x2CDi32
	SetStreamOutputBuffersInvalidDesc = 0x2CDi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDDIMENSIONALITY = 0x2CEi32
	CreateResourceUnrecognizedDimensionality = 0x2CEi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDLAYOUT = 0x2CFi32
	CreateResourceUnrecognizedLayout = 0x2CFi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDDIMENSIONALITY = 0x2D0i32
	CreateResourceInvalidDimensionality = 0x2D0i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDALIGNMENT = 0x2D1i32
	CreateResourceInvalidAlignment = 0x2D1i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDMIPLEVELS = 0x2D2i32
	CreateResourceInvalidMipLevels = 0x2D2i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDSAMPLEDESC = 0x2D3i32
	CreateResourceInvalidSampleDesc = 0x2D3i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDLAYOUT = 0x2D4i32
	CreateResourceInvalidLayout = 0x2D4i32,
	/// D3D12_MESSAGE_ID_SET_INDEX_BUFFER_INVALID = 0x2D5i32
	SetIndexBufferInvalid = 0x2D5i32,
	/// D3D12_MESSAGE_ID_SET_VERTEX_BUFFERS_INVALID = 0x2D6i32
	SetVertexBuffersInvalid = 0x2D6i32,
	/// D3D12_MESSAGE_ID_SET_STREAM_OUTPUT_BUFFERS_INVALID = 0x2D7i32
	SetStreamOutputBuffersInvalid = 0x2D7i32,
	/// D3D12_MESSAGE_ID_SET_RENDER_TARGETS_INVALID = 0x2D8i32
	SetRenderTargetsInvalid = 0x2D8i32,
	/// D3D12_MESSAGE_ID_CREATEQUERY_HEAP_INVALID_PARAMETERS = 0x2D9i32
	CreateQueryHeapInvalidParameters = 0x2D9i32,
	/// D3D12_MESSAGE_ID_BEGIN_END_QUERY_INVALID_PARAMETERS = 0x2DBi32
	BeginEndQueryInvalidParameters = 0x2DBi32,
	/// D3D12_MESSAGE_ID_CLOSE_COMMAND_LIST_OPEN_QUERY = 0x2DCi32
	CloseCommandListOpenQuery = 0x2DCi32,
	/// D3D12_MESSAGE_ID_RESOLVE_QUERY_DATA_INVALID_PARAMETERS = 0x2DDi32
	ResolveQueryDataInvalidParameters = 0x2DDi32,
	/// D3D12_MESSAGE_ID_SET_PREDICATION_INVALID_PARAMETERS = 0x2DEi32
	SetPredicationInvalidParameters = 0x2DEi32,
	/// D3D12_MESSAGE_ID_TIMESTAMPS_NOT_SUPPORTED = 0x2DFi32
	TimestampsNotSupported = 0x2DFi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDFORMAT = 0x2E1i32
	CreateResourceUnrecognizedFormat = 0x2E1i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDFORMAT = 0x2E2i32
	CreateResourceInvalidFormat = 0x2E2i32,
	/// D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDSUBRESOURCERANGE = 0x2E3i32
	GetCopyAbleFootprintsInvalidSubresourceRange = 0x2E3i32,
	/// D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_INVALIDBASEOFFSET = 0x2E4i32
	GetCopyAbleFootprintsInvalidBaseOffset = 0x2E4i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_INVALID_HEAP = 0x2E5i32
	ResourceBarrierInvalidHeap = 0x2E5i32,
	/// D3D12_MESSAGE_ID_CREATE_SAMPLER_INVALID = 0x2E6i32
	CreateSamplerInvalid = 0x2E6i32,
	/// D3D12_MESSAGE_ID_CREATECOMMANDSIGNATURE_INVALID = 0x2E7i32
	CreateCommandSignatureInvalid = 0x2E7i32,
	/// D3D12_MESSAGE_ID_EXECUTE_INDIRECT_INVALID_PARAMETERS = 0x2E8i32
	ExecuteIndirectInvalidParameters = 0x2E8i32,
	/// D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_RESOURCE_DIMENSION = 0x2E9i32
	GetGpuVirtualAddressInvalidResourceDimension = 0x2E9i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUE = 0x32Fi32
	CreateResourceInvalidClearValue = 0x32Fi32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_UNRECOGNIZEDCLEARVALUEFORMAT = 0x330i32
	CreateResourceUnrecognizedClearValueFormat = 0x330i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_INVALIDCLEARVALUEFORMAT = 0x331i32
	CreateResourceInvalidClearValueFormat = 0x331i32,
	/// D3D12_MESSAGE_ID_CREATERESOURCE_CLEARVALUEDENORMFLUSH = 0x332i32
	CreateResourceClearValuedEnormFlush = 0x332i32,
	/// D3D12_MESSAGE_ID_CLEARRENDERTARGETVIEW_MISMATCHINGCLEARVALUE = 0x334i32
	ClearRenderTargetViewMismatchingClearValue = 0x334i32,
	/// D3D12_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_MISMATCHINGCLEARVALUE = 0x335i32
	ClearDepthStencilViewMismatchingClearValue = 0x335i32,
	/// D3D12_MESSAGE_ID_MAP_INVALIDHEAP = 0x336i32
	MapInvalidHeap       = 0x336i32,
	/// D3D12_MESSAGE_ID_UNMAP_INVALIDHEAP = 0x337i32
	UnmapInvalidHeap     = 0x337i32,
	/// D3D12_MESSAGE_ID_MAP_INVALIDRESOURCE = 0x338i32
	MapInvalidResource   = 0x338i32,
	/// D3D12_MESSAGE_ID_UNMAP_INVALIDRESOURCE = 0x339i32
	UnmapInvalidResource = 0x339i32,
	/// D3D12_MESSAGE_ID_MAP_INVALIDSUBRESOURCE = 0x33Ai32
	MapInvalidSubresource = 0x33Ai32,
	/// D3D12_MESSAGE_ID_UNMAP_INVALIDSUBRESOURCE = 0x33Bi32
	UnmapInvalidSubresource = 0x33Bi32,
	/// D3D12_MESSAGE_ID_MAP_INVALIDRANGE = 0x33Ci32
	MapInvalidRange      = 0x33Ci32,
	/// D3D12_MESSAGE_ID_UNMAP_INVALIDRANGE = 0x33Di32
	UnmapInvalidRange    = 0x33Di32,
	/// D3D12_MESSAGE_ID_MAP_INVALIDDATAPOINTER = 0x340i32
	MapInvalidDataPointer = 0x340i32,
	/// D3D12_MESSAGE_ID_MAP_INVALIDARG_RETURN = 0x341i32
	MapInvalidArgReturn  = 0x341i32,
	/// D3D12_MESSAGE_ID_MAP_OUTOFMEMORY_RETURN = 0x342i32
	MapOutOfMemoryReturn = 0x342i32,
	/// D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_BUNDLENOTSUPPORTED = 0x343i32
	ExecuteCommandListsBundleNotSupported = 0x343i32,
	/// D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_COMMANDLISTMISMATCH = 0x344i32
	ExecuteCommandListsCommandListMismatch = 0x344i32,
	/// D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_OPENCOMMANDLIST = 0x345i32
	ExecuteCommandListsOpenCommandList = 0x345i32,
	/// D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_FAILEDCOMMANDLIST = 0x346i32
	ExecuteCommandListsFailedCommandList = 0x346i32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLDST = 0x347i32
	CopyBufferRegionNullDst = 0x347i32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDDSTRESOURCEDIMENSION = 0x348i32
	CopyBufferRegionInvalidDstResourceDimension = 0x348i32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_DSTRANGEOUTOFBOUNDS = 0x349i32
	CopyBufferRegionDStrangeOutOfBounds = 0x349i32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_NULLSRC = 0x34Ai32
	CopyBufferRegionNullSrc = 0x34Ai32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDSRCRESOURCEDIMENSION = 0x34Bi32
	CopyBufferRegionInvalidSrcResourceDimension = 0x34Bi32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_SRCRANGEOUTOFBOUNDS = 0x34Ci32
	CopyBufferRegionSrcRangeOutOfBounds = 0x34Ci32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALIDCOPYFLAGS = 0x34Di32
	CopyBufferRegionInvalidCopyFlags = 0x34Di32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLDST = 0x34Ei32
	CopyTextureRegionNullDst = 0x34Ei32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTTYPE = 0x34Fi32
	CopyTextureRegionUnrecognizedDstType = 0x34Fi32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCEDIMENSION = 0x350i32
	CopyTextureRegionInvalidDstResourceDimension = 0x350i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTRESOURCE = 0x351i32
	CopyTextureRegionInvalidDstResource = 0x351i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTSUBRESOURCE = 0x352i32
	CopyTextureRegionInvalidDstSubresource = 0x352i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTOFFSET = 0x353i32
	CopyTextureRegionInvalidDstOffset = 0x353i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDDSTFORMAT = 0x354i32
	CopyTextureRegionUnrecognizedDstFormat = 0x354i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTFORMAT = 0x355i32
	CopyTextureRegionInvalidDstFormat = 0x355i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDIMENSIONS = 0x356i32
	CopyTextureRegionInvalidDstDimensions = 0x356i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTROWPITCH = 0x357i32
	CopyTextureRegionInvalidDstRowPitch = 0x357i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTPLACEMENT = 0x358i32
	CopyTextureRegionInvalidDstPlacement = 0x358i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTDSPLACEDFOOTPRINTFORMAT = 0x359i32
	CopyTextureRegionInvalidDstDsPlacedFootprintFormat = 0x359i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_DSTREGIONOUTOFBOUNDS = 0x35Ai32
	CopyTextureRegionDstRegionOutOfBounds = 0x35Ai32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_NULLSRC = 0x35Bi32
	CopyTextureRegionNullSrc = 0x35Bi32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCTYPE = 0x35Ci32
	CopyTextureRegionUnrecognizedSrcType = 0x35Ci32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCEDIMENSION = 0x35Di32
	CopyTextureRegionInvalidSrcResourceDimension = 0x35Di32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCRESOURCE = 0x35Ei32
	CopyTextureRegionInvalidSrcResource = 0x35Ei32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCSUBRESOURCE = 0x35Fi32
	CopyTextureRegionInvalidSrcSubresource = 0x35Fi32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCOFFSET = 0x360i32
	CopyTextureRegionInvalidSrcOffset = 0x360i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_UNRECOGNIZEDSRCFORMAT = 0x361i32
	CopyTextureRegionUnrecognizedSrcFormat = 0x361i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCFORMAT = 0x362i32
	CopyTextureRegionInvalidSrcFormat = 0x362i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDIMENSIONS = 0x363i32
	CopyTextureRegionInvalidSrcDimensions = 0x363i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCROWPITCH = 0x364i32
	CopyTextureRegionInvalidSrcRowPitch = 0x364i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCPLACEMENT = 0x365i32
	CopyTextureRegionInvalidSrcPlacement = 0x365i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCDSPLACEDFOOTPRINTFORMAT = 0x366i32
	CopyTextureRegionInvalidSrcDsPlacedFootprintFormat = 0x366i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_SRCREGIONOUTOFBOUNDS = 0x367i32
	CopyTextureRegionSrcRegionOutOfBounds = 0x367i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDDSTCOORDINATES = 0x368i32
	CopyTextureRegionInvalidDstCoordinates = 0x368i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDSRCBOX = 0x369i32
	CopyTextureRegionInvalidSrcBox = 0x369i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_FORMATMISMATCH = 0x36Ai32
	CopyTextureRegionFormatMismatch = 0x36Ai32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_EMPTYBOX = 0x36Bi32
	CopyTextureRegionEmptyBox = 0x36Bi32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_INVALIDCOPYFLAGS = 0x36Ci32
	CopyTextureRegionInvalidCopyFlags = 0x36Ci32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SUBRESOURCE_INDEX = 0x36Di32
	ResolveSubresourceInvalidSubresourceIndex = 0x36Di32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_FORMAT = 0x36Ei32
	ResolveSubresourceInvalidFormat = 0x36Ei32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_MISMATCH = 0x36Fi32
	ResolveSubresourceResourceMismatch = 0x36Fi32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALID_SAMPLE_COUNT = 0x370i32
	ResolveSubresourceInvalidSampleCount = 0x370i32,
	/// D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_INVALID_SHADER = 0x371i32
	CreateComputePipelineStateInvalidShader = 0x371i32,
	/// D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_CS_ROOT_SIGNATURE_MISMATCH = 0x372i32
	CreateComputePipelineStateCsRootSignatureMismatch = 0x372i32,
	/// D3D12_MESSAGE_ID_CREATECOMPUTEPIPELINESTATE_MISSING_ROOT_SIGNATURE = 0x373i32
	CreateComputePipelineStateMissingRootSignature = 0x373i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALIDCACHEDBLOB = 0x374i32
	CreatePipelineStateInvalidCachedBlob = 0x374i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBADAPTERMISMATCH = 0x375i32
	CreatePipelineStateCachedBlobAdapterMismatch = 0x375i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDRIVERVERSIONMISMATCH = 0x376i32
	CreatePipelineStateCachedBlobDriverVersionMismatch = 0x376i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBDESCMISMATCH = 0x377i32
	CreatePipelineStateCachedBlobDescMismatch = 0x377i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CACHEDBLOBIGNORED = 0x378i32
	CreatePipelineStateCachedBlobIgnored = 0x378i32,
	/// D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDHEAP = 0x379i32
	WriteToSubresourceInvalidHeap = 0x379i32,
	/// D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDRESOURCE = 0x37Ai32
	WriteToSubresourceInvalidResource = 0x37Ai32,
	/// D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDBOX = 0x37Bi32
	WriteToSubresourceInvalidBox = 0x37Bi32,
	/// D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_INVALIDSUBRESOURCE = 0x37Ci32
	WriteToSubresourceInvalidSubresource = 0x37Ci32,
	/// D3D12_MESSAGE_ID_WRITETOSUBRESOURCE_EMPTYBOX = 0x37Di32
	WriteToSubresourceEmptyBox = 0x37Di32,
	/// D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDHEAP = 0x37Ei32
	ReadFromSubresourceInvalidHeap = 0x37Ei32,
	/// D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDRESOURCE = 0x37Fi32
	ReadFromSubresourceInvalidResource = 0x37Fi32,
	/// D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDBOX = 0x380i32
	ReadFromSubresourceInvalidBox = 0x380i32,
	/// D3D12_MESSAGE_ID_READFROMSUBRESOURCE_INVALIDSUBRESOURCE = 0x381i32
	ReadFromSubresourceInvalidSubresource = 0x381i32,
	/// D3D12_MESSAGE_ID_READFROMSUBRESOURCE_EMPTYBOX = 0x382i32
	ReadFromSubresourceEmptyBox = 0x382i32,
	/// D3D12_MESSAGE_ID_TOO_MANY_NODES_SPECIFIED = 0x383i32
	TooManyNodesSpecified = 0x383i32,
	/// D3D12_MESSAGE_ID_INVALID_NODE_INDEX = 0x384i32
	InvalidNodeIndex     = 0x384i32,
	/// D3D12_MESSAGE_ID_GETHEAPPROPERTIES_INVALIDRESOURCE = 0x385i32
	GetHeapPropertiesInvalidResource = 0x385i32,
	/// D3D12_MESSAGE_ID_NODE_MASK_MISMATCH = 0x386i32
	NodeMaskMismatch     = 0x386i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_OUTOFMEMORY = 0x387i32
	CommandListOutOfMemory = 0x387i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_MULTIPLE_SWAPCHAIN_BUFFER_REFERENCES = 0x388i32
	CommandListMultipleSwapChainBufferReferences = 0x388i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_TOO_MANY_SWAPCHAIN_REFERENCES = 0x389i32
	CommandListTooManySwapChainReferences = 0x389i32,
	/// D3D12_MESSAGE_ID_COMMAND_QUEUE_TOO_MANY_SWAPCHAIN_REFERENCES = 0x38Ai32
	CommandQueueTooManySwapChainReferences = 0x38Ai32,
	/// D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_WRONGSWAPCHAINBUFFERREFERENCE = 0x38Bi32
	ExecuteCommandListsWrongSwapChainBufferReference = 0x38Bi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_SETRENDERTARGETS_INVALIDNUMRENDERTARGETS = 0x38Ci32
	CommandListSetRenderTargetsInvalidNumRenderTargets = 0x38Ci32,
	/// D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_TYPE = 0x38Di32
	CreateQueueInvalidType = 0x38Di32,
	/// D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_FLAGS = 0x38Ei32
	CreateQueueInvalidFlags = 0x38Ei32,
	/// D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFLAGS = 0x38Fi32
	CreateSharedResourceInvalidFlags = 0x38Fi32,
	/// D3D12_MESSAGE_ID_CREATESHAREDRESOURCE_INVALIDFORMAT = 0x390i32
	CreateSharedResourceInvalidFormat = 0x390i32,
	/// D3D12_MESSAGE_ID_CREATESHAREDHEAP_INVALIDFLAGS = 0x391i32
	CreateSharedHeapInvalidFlags = 0x391i32,
	/// D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_UNRECOGNIZEDPROPERTIES = 0x392i32
	ReflectSharedPropertiesUnrecognizedProperties = 0x392i32,
	/// D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDSIZE = 0x393i32
	ReflectSharedPropertiesInvalidSize = 0x393i32,
	/// D3D12_MESSAGE_ID_REFLECTSHAREDPROPERTIES_INVALIDOBJECT = 0x394i32
	ReflectSharedPropertiesInvalidObject = 0x394i32,
	/// D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDOBJECT = 0x395i32
	KeyedMutexInvalidObject = 0x395i32,
	/// D3D12_MESSAGE_ID_KEYEDMUTEX_INVALIDKEY = 0x396i32
	KeyedMutexInvalidKey = 0x396i32,
	/// D3D12_MESSAGE_ID_KEYEDMUTEX_WRONGSTATE = 0x397i32
	KeyedMutexWrongState = 0x397i32,
	/// D3D12_MESSAGE_ID_CREATE_QUEUE_INVALID_PRIORITY = 0x398i32
	CreateQueueInvalidPriority = 0x398i32,
	/// D3D12_MESSAGE_ID_OBJECT_DELETED_WHILE_STILL_IN_USE = 0x399i32
	ObjectDeletedWhileStillInUse = 0x399i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_FLAGS = 0x39Ai32
	CreatePipelineStateInvalidFlags = 0x39Ai32,
	/// D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_HAS_NO_RESOURCE = 0x39Bi32
	HeapAddressRangeHasNoResource = 0x39Bi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DRAW_RENDER_TARGET_DELETED = 0x39Ci32
	CommandListDrawRenderTargetDeleted = 0x39Ci32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_ALL_RENDER_TARGETS_HAVE_UNKNOWN_FORMAT = 0x39Di32
	CreateGraphicsPipelineStateAllRenderTargetsHaveUnknownFormat = 0x39Di32,
	/// D3D12_MESSAGE_ID_HEAP_ADDRESS_RANGE_INTERSECTS_MULTIPLE_BUFFERS = 0x39Ei32
	HeapAddressRangeIntersectsMultipleBuffers = 0x39Ei32,
	/// D3D12_MESSAGE_ID_EXECUTECOMMANDLISTS_GPU_WRITTEN_READBACK_RESOURCE_MAPPED = 0x39Fi32
	ExecuteCommandListsGpuWrittenReadBackResourceMapped = 0x39Fi32,
	/// D3D12_MESSAGE_ID_UNMAP_RANGE_NOT_EMPTY = 0x3A1i32
	UnmapRangeNotEmpty   = 0x3A1i32,
	/// D3D12_MESSAGE_ID_MAP_INVALID_NULLRANGE = 0x3A2i32
	MapInvalidNullRange  = 0x3A2i32,
	/// D3D12_MESSAGE_ID_UNMAP_INVALID_NULLRANGE = 0x3A3i32
	UnmapInvalidNullRange = 0x3A3i32,
	/// D3D12_MESSAGE_ID_NO_GRAPHICS_API_SUPPORT = 0x3A4i32
	NoGraphicsApiSupport = 0x3A4i32,
	/// D3D12_MESSAGE_ID_NO_COMPUTE_API_SUPPORT = 0x3A5i32
	NoComputeApiSupport  = 0x3A5i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_RESOURCE_FLAGS_NOT_SUPPORTED = 0x3A6i32
	ResolveSubresourceResourceFlagsNotSupported = 0x3A6i32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_ARGUMENT_UNINITIALIZED = 0x3A7i32
	GpuBasedValidationRootArgumentUninitialized = 0x3A7i32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_HEAP_INDEX_OUT_OF_BOUNDS = 0x3A8i32
	GpuBasedValidationDescriptorHeapIndexOutOfBounds = 0x3A8i32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TABLE_REGISTER_INDEX_OUT_OF_BOUNDS = 0x3A9i32
	GpuBasedValidationDescriptorTableRegisterIndexOutOfBounds = 0x3A9i32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_UNINITIALIZED = 0x3AAi32
	GpuBasedValidationDescriptorUninitialized = 0x3AAi32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_DESCRIPTOR_TYPE_MISMATCH = 0x3ABi32
	GpuBasedValidationDescriptorTypeMismatch = 0x3ABi32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SRV_RESOURCE_DIMENSION_MISMATCH = 0x3ACi32
	GpuBasedValidationSrvResourceDimensionMismatch = 0x3ACi32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UAV_RESOURCE_DIMENSION_MISMATCH = 0x3ADi32
	GpuBasedValidationUavResourceDimensionMismatch = 0x3ADi32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INCOMPATIBLE_RESOURCE_STATE = 0x3AEi32
	GpuBasedValidationIncompatibleResourceState = 0x3AEi32,
	/// D3D12_MESSAGE_ID_COPYRESOURCE_NULLDST = 0x3AFi32
	CopyResourceNullDst  = 0x3AFi32,
	/// D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDDSTRESOURCE = 0x3B0i32
	CopyResourceInvalidDstResource = 0x3B0i32,
	/// D3D12_MESSAGE_ID_COPYRESOURCE_NULLSRC = 0x3B1i32
	CopyResourceNullSrc  = 0x3B1i32,
	/// D3D12_MESSAGE_ID_COPYRESOURCE_INVALIDSRCRESOURCE = 0x3B2i32
	CopyResourceInvalidSrcResource = 0x3B2i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLDST = 0x3B3i32
	ResolveSubresourceNullDst = 0x3B3i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDDSTRESOURCE = 0x3B4i32
	ResolveSubresourceInvalidDstResource = 0x3B4i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_NULLSRC = 0x3B5i32
	ResolveSubresourceNullSrc = 0x3B5i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_INVALIDSRCRESOURCE = 0x3B6i32
	ResolveSubresourceInvalidSrcResource = 0x3B6i32,
	/// D3D12_MESSAGE_ID_PIPELINE_STATE_TYPE_MISMATCH = 0x3B7i32
	PipelineStateTypeMismatch = 0x3B7i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_NOT_SET = 0x3B8i32
	CommandListDispatchRootSignatureNotSet = 0x3B8i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DISPATCH_ROOT_SIGNATURE_MISMATCH = 0x3B9i32
	CommandListDispatchRootSignatureMismatch = 0x3B9i32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_ZERO_BARRIERS = 0x3BAi32
	ResourceBarrierZeroBarriers = 0x3BAi32,
	/// D3D12_MESSAGE_ID_BEGIN_END_EVENT_MISMATCH = 0x3BBi32
	BeginEndEventMismatch = 0x3BBi32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_POSSIBLE_BEFORE_AFTER_MISMATCH = 0x3BCi32
	ResourceBarrierPossibleBeforeAfterMismatch = 0x3BCi32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_BEGIN_END = 0x3BDi32
	ResourceBarrierMismatchingBeginEnd = 0x3BDi32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_INVALID_RESOURCE = 0x3BEi32
	GpuBasedValidationInvalidResource = 0x3BEi32,
	/// D3D12_MESSAGE_ID_USE_OF_ZERO_REFCOUNT_OBJECT = 0x3BFi32
	UseOfZeroRefCountObject = 0x3BFi32,
	/// D3D12_MESSAGE_ID_OBJECT_EVICTED_WHILE_STILL_IN_USE = 0x3C0i32
	ObjectEvictedWhileStillInUse = 0x3C0i32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_ROOT_DESCRIPTOR_ACCESS_OUT_OF_BOUNDS = 0x3C1i32
	GpuBasedValidationRootDescriptorAccessOutOfBounds = 0x3C1i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_INVALIDLIBRARYBLOB = 0x3C2i32
	CreatePipelineLibraryInvalidLibraryBlob = 0x3C2i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_DRIVERVERSIONMISMATCH = 0x3C3i32
	CreatePipelineLibraryDriverVersionMismatch = 0x3C3i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_ADAPTERVERSIONMISMATCH = 0x3C4i32
	CreatePipelineLibraryAdapterVersionMismatch = 0x3C4i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINELIBRARY_UNSUPPORTED = 0x3C5i32
	CreatePipelineLibraryUnsupported = 0x3C5i32,
	/// D3D12_MESSAGE_ID_CREATE_PIPELINELIBRARY = 0x3C6i32
	CreatePipelineLibrary = 0x3C6i32,
	/// D3D12_MESSAGE_ID_LIVE_PIPELINELIBRARY = 0x3C7i32
	LivePipelineLibrary  = 0x3C7i32,
	/// D3D12_MESSAGE_ID_DESTROY_PIPELINELIBRARY = 0x3C8i32
	DestroyPipelineLibrary = 0x3C8i32,
	/// D3D12_MESSAGE_ID_STOREPIPELINE_NONAME = 0x3C9i32
	StorePipelineNoName  = 0x3C9i32,
	/// D3D12_MESSAGE_ID_STOREPIPELINE_DUPLICATENAME = 0x3CAi32
	StorePipelineDuplicateName = 0x3CAi32,
	/// D3D12_MESSAGE_ID_LOADPIPELINE_NAMENOTFOUND = 0x3CBi32
	LoadPipelineNameNotFound = 0x3CBi32,
	/// D3D12_MESSAGE_ID_LOADPIPELINE_INVALIDDESC = 0x3CCi32
	LoadPipelineInvalidDesc = 0x3CCi32,
	/// D3D12_MESSAGE_ID_PIPELINELIBRARY_SERIALIZE_NOTENOUGHMEMORY = 0x3CDi32
	PipelineLibrarySerializeNotEnoughMemory = 0x3CDi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_PS_OUTPUT_RT_OUTPUT_MISMATCH = 0x3CEi32
	CreateGraphicsPipelineStatePsOutputRtOutputMismatch = 0x3CEi32,
	/// D3D12_MESSAGE_ID_SETEVENTONMULTIPLEFENCECOMPLETION_INVALIDFLAGS = 0x3CFi32
	SetEvenTonMultipleFenceCompletionInvalidFlags = 0x3CFi32,
	/// D3D12_MESSAGE_ID_CREATE_QUEUE_VIDEO_NOT_SUPPORTED = 0x3D0i32
	CreateQueueVideoNotSupported = 0x3D0i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_ALLOCATOR_VIDEO_NOT_SUPPORTED = 0x3D1i32
	CreateCommandAllocatorVideoNotSupported = 0x3D1i32,
	/// D3D12_MESSAGE_ID_CREATEQUERY_HEAP_VIDEO_DECODE_STATISTICS_NOT_SUPPORTED = 0x3D2i32
	CreateQueryHeapVideoDecodeStatisticsNotSupported = 0x3D2i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDLIST = 0x3D3i32
	CreateVideoDecodeCommandList = 0x3D3i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEODECODER = 0x3D4i32
	CreateVideoDecoder   = 0x3D4i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEODECODESTREAM = 0x3D5i32
	CreateVideoDecodeStream = 0x3D5i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDLIST = 0x3D6i32
	LiveVideoDecodeCommandList = 0x3D6i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEODECODER = 0x3D7i32
	LiveVideoDecoder     = 0x3D7i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEODECODESTREAM = 0x3D8i32
	LiveVideoDecodeStream = 0x3D8i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDLIST = 0x3D9i32
	DestroyVideoDecodeCommandList = 0x3D9i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEODECODER = 0x3DAi32
	DestroyVideoDecoder  = 0x3DAi32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEODECODESTREAM = 0x3DBi32
	DestroyVideoDecodeStream = 0x3DBi32,
	/// D3D12_MESSAGE_ID_DECODE_FRAME_INVALID_PARAMETERS = 0x3DCi32
	DecodeFrameInvalidParameters = 0x3DCi32,
	/// D3D12_MESSAGE_ID_DEPRECATED_API = 0x3DDi32
	DeprecatedApi        = 0x3DDi32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_MISMATCHING_COMMAND_LIST_TYPE = 0x3DEi32
	ResourceBarrierMismatchingCommandListType = 0x3DEi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_DESCRIPTOR_TABLE_NOT_SET = 0x3DFi32
	CommandListDescriptorTableNotSet = 0x3DFi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_CONSTANT_BUFFER_VIEW_NOT_SET = 0x3E0i32
	CommandListRootConstantBufferViewNotSet = 0x3E0i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_SHADER_RESOURCE_VIEW_NOT_SET = 0x3E1i32
	CommandListRootShaderResourceViewNotSet = 0x3E1i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_UNORDERED_ACCESS_VIEW_NOT_SET = 0x3E2i32
	CommandListRootUnorderedAccessViewNotSet = 0x3E2i32,
	/// D3D12_MESSAGE_ID_DISCARD_INVALID_SUBRESOURCE_RANGE = 0x3E3i32
	DiscardInvalidSubresourceRange = 0x3E3i32,
	/// D3D12_MESSAGE_ID_DISCARD_ONE_SUBRESOURCE_FOR_MIPS_WITH_RECTS = 0x3E4i32
	DiscardOneSubresourceForMipsWithRects = 0x3E4i32,
	/// D3D12_MESSAGE_ID_DISCARD_NO_RECTS_FOR_NON_TEXTURE2D = 0x3E5i32
	DiscardNoRectsForNonTexture2d = 0x3E5i32,
	/// D3D12_MESSAGE_ID_COPY_ON_SAME_SUBRESOURCE = 0x3E6i32
	CopyOnSameSubresource = 0x3E6i32,
	/// D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PAGEABLE = 0x3E7i32
	SetResidencyPriorityInvalidPageable = 0x3E7i32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_UNSUPPORTED = 0x3E8i32
	GpuBasedValidationUnsupported = 0x3E8i32,
	/// D3D12_MESSAGE_ID_STATIC_DESCRIPTOR_INVALID_DESCRIPTOR_CHANGE = 0x3E9i32
	StaticDescriptorInvalidDescriptorChange = 0x3E9i32,
	/// D3D12_MESSAGE_ID_DATA_STATIC_DESCRIPTOR_INVALID_DATA_CHANGE = 0x3EAi32
	DataStaticDescriptorInvalidDataChange = 0x3EAi32,
	/// D3D12_MESSAGE_ID_DATA_STATIC_WHILE_SET_AT_EXECUTE_DESCRIPTOR_INVALID_DATA_CHANGE = 0x3EBi32
	DataStaticWhileSetAtExecuteDescriptorInvalidDataChange = 0x3EBi32,
	/// D3D12_MESSAGE_ID_EXECUTE_BUNDLE_STATIC_DESCRIPTOR_DATA_STATIC_NOT_SET = 0x3ECi32
	ExecuteBundleStaticDescriptorDataStaticNotSet = 0x3ECi32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_ACCESS_OUT_OF_BOUNDS = 0x3EDi32
	GpuBasedValidationResourceAccessOutOfBounds = 0x3EDi32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_SAMPLER_MODE_MISMATCH = 0x3EEi32
	GpuBasedValidationSamplerModeMismatch = 0x3EEi32,
	/// D3D12_MESSAGE_ID_CREATE_FENCE_INVALID_FLAGS = 0x3EFi32
	CreateFenceInvalidFlags = 0x3EFi32,
	/// D3D12_MESSAGE_ID_RESOURCE_BARRIER_DUPLICATE_SUBRESOURCE_TRANSITIONS = 0x3F0i32
	ResourceBarrierDuplicateSubresourceTransitions = 0x3F0i32,
	/// D3D12_MESSAGE_ID_SETRESIDENCYPRIORITY_INVALID_PRIORITY = 0x3F1i32
	SetResidencyPriorityInvalidPriority = 0x3F1i32,
	/// D3D12_MESSAGE_ID_CREATE_DESCRIPTOR_HEAP_LARGE_NUM_DESCRIPTORS = 0x3F5i32
	CreateDescriptorHeapLargeNumDescriptors = 0x3F5i32,
	/// D3D12_MESSAGE_ID_BEGIN_EVENT = 0x3F6i32
	BeginEvent           = 0x3F6i32,
	/// D3D12_MESSAGE_ID_END_EVENT = 0x3F7i32
	EndEvent             = 0x3F7i32,
	/// D3D12_MESSAGE_ID_CREATEDEVICE_DEBUG_LAYER_STARTUP_OPTIONS = 0x3F8i32
	CreateDeviceDebugLayerStartupOptions = 0x3F8i32,
	/// D3D12_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_DEPTHBOUNDSTEST_UNSUPPORTED = 0x3F9i32
	CreateDepthStencilStateDepthBoundsTestUnsupported = 0x3F9i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_DUPLICATE_SUBOBJECT = 0x3FAi32
	CreatePipelineStateDuplicateSubobject = 0x3FAi32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_UNKNOWN_SUBOBJECT = 0x3FBi32
	CreatePipelineStateUnknownSubobject = 0x3FBi32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_ZERO_SIZE_STREAM = 0x3FCi32
	CreatePipelineStateZeroSizeStream = 0x3FCi32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_INVALID_STREAM = 0x3FDi32
	CreatePipelineStateInvalidStream = 0x3FDi32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_CANNOT_DEDUCE_TYPE = 0x3FEi32
	CreatePipelineStateCannotDeduceType = 0x3FEi32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_RESOURCE_DIMENSION_MISMATCH = 0x3FFi32
	CommandListStaticDescriptorResourceDimensionMismatch = 0x3FFi32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_PRIVILEGE_FOR_GLOBAL_REALTIME = 0x400i32
	CreateCommandQueueInsufficientPrivilegeForGlobalRealTime = 0x400i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_QUEUE_INSUFFICIENT_HARDWARE_SUPPORT_FOR_GLOBAL_REALTIME = 0x401i32
	CreateCommandQueueInsufficientHardwareSupportForGlobalRealTime = 0x401i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_ARCHITECTURE = 0x402i32
	AtomicCopyBufferInvalidArchitecture = 0x402i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DST = 0x403i32
	AtomicCopyBufferNullDst = 0x403i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE_DIMENSION = 0x404i32
	AtomicCopyBufferInvalidDstResourceDimension = 0x404i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DST_RANGE_OUT_OF_BOUNDS = 0x405i32
	AtomicCopyBufferDstRangeOutOfBounds = 0x405i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_SRC = 0x406i32
	AtomicCopyBufferNullSrc = 0x406i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE_DIMENSION = 0x407i32
	AtomicCopyBufferInvalidSrcResourceDimension = 0x407i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_SRC_RANGE_OUT_OF_BOUNDS = 0x408i32
	AtomicCopyBufferSrcRangeOutOfBounds = 0x408i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_OFFSET_ALIGNMENT = 0x409i32
	AtomicCopyBufferInvalidOffsetAlignment = 0x409i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_RESOURCES = 0x40Ai32
	AtomicCopyBufferNullDependentResources = 0x40Ai32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_NULL_DEPENDENT_SUBRESOURCE_RANGES = 0x40Bi32
	AtomicCopyBufferNullDependentSubresourceRanges = 0x40Bi32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_RESOURCE = 0x40Ci32
	AtomicCopyBufferInvalidDependentResource = 0x40Ci32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DEPENDENT_SUBRESOURCE_RANGE = 0x40Di32
	AtomicCopyBufferInvalidDependentSubresourceRange = 0x40Di32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_SUBRESOURCE_OUT_OF_BOUNDS = 0x40Ei32
	AtomicCopyBufferDependentSubresourceOutOfBounds = 0x40Ei32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_DEPENDENT_RANGE_OUT_OF_BOUNDS = 0x40Fi32
	AtomicCopyBufferDependentRangeOutOfBounds = 0x40Fi32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_ZERO_DEPENDENCIES = 0x410i32
	AtomicCopyBufferZeroDependencies = 0x410i32,
	/// D3D12_MESSAGE_ID_DEVICE_CREATE_SHARED_HANDLE_INVALIDARG = 0x411i32
	DeviceCreateSharedHandleInvalidArg = 0x411i32,
	/// D3D12_MESSAGE_ID_DESCRIPTOR_HANDLE_WITH_INVALID_RESOURCE = 0x412i32
	DescriptorHandleWithInvalidResource = 0x412i32,
	/// D3D12_MESSAGE_ID_SETDEPTHBOUNDS_INVALIDARGS = 0x413i32
	SetDepthBoundsInvalidArgs = 0x413i32,
	/// D3D12_MESSAGE_ID_GPU_BASED_VALIDATION_RESOURCE_STATE_IMPRECISE = 0x414i32
	GpuBasedValidationResourceStateImprecise = 0x414i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_PIPELINE_STATE_NOT_SET = 0x415i32
	CommandListPipelineStateNotSet = 0x415i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_SHADER_MODEL_MISMATCH = 0x416i32
	CreateGraphicsPipelineStateShaderModelMismatch = 0x416i32,
	/// D3D12_MESSAGE_ID_OBJECT_ACCESSED_WHILE_STILL_IN_USE = 0x417i32
	ObjectAccessedWhileStillInUse = 0x417i32,
	/// D3D12_MESSAGE_ID_PROGRAMMABLE_MSAA_UNSUPPORTED = 0x418i32
	ProgrammableMSaaUnsupported = 0x418i32,
	/// D3D12_MESSAGE_ID_SETSAMPLEPOSITIONS_INVALIDARGS = 0x419i32
	SetSamplePositionsInvalidArgs = 0x419i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCEREGION_INVALID_RECT = 0x41Ai32
	ResolveSubresourceRegionInvalidRect = 0x41Ai32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEODECODECOMMANDQUEUE = 0x41Bi32
	CreateVideoDecodeCommandQueue = 0x41Bi32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDLIST = 0x41Ci32
	CreateVideoProcessCommandList = 0x41Ci32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSCOMMANDQUEUE = 0x41Di32
	CreateVideoProcessCommandQueue = 0x41Di32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEODECODECOMMANDQUEUE = 0x41Ei32
	LiveVideoDecodeCommandQueue = 0x41Ei32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDLIST = 0x41Fi32
	LiveVideoProcessCommandList = 0x41Fi32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSCOMMANDQUEUE = 0x420i32
	LiveVideoProcessCommandQueue = 0x420i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEODECODECOMMANDQUEUE = 0x421i32
	DestroyVideoDecodeCommandQueue = 0x421i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDLIST = 0x422i32
	DestroyVideoProcessCommandList = 0x422i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSCOMMANDQUEUE = 0x423i32
	DestroyVideoProcessCommandQueue = 0x423i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSOR = 0x424i32
	CreateVideoProcessor = 0x424i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOPROCESSSTREAM = 0x425i32
	CreateVideoProcessStream = 0x425i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSOR = 0x426i32
	LiveVideoProcessor   = 0x426i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOPROCESSSTREAM = 0x427i32
	LiveVideoProcessStream = 0x427i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSOR = 0x428i32
	DestroyVideoProcessor = 0x428i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOPROCESSSTREAM = 0x429i32
	DestroyVideoProcessStream = 0x429i32,
	/// D3D12_MESSAGE_ID_PROCESS_FRAME_INVALID_PARAMETERS = 0x42Ai32
	ProcessFrameInvalidParameters = 0x42Ai32,
	/// D3D12_MESSAGE_ID_COPY_INVALIDLAYOUT = 0x42Bi32
	CopyInvalidLayout    = 0x42Bi32,
	/// D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION = 0x42Ci32
	CreateCryptoSession  = 0x42Ci32,
	/// D3D12_MESSAGE_ID_CREATE_CRYPTO_SESSION_POLICY = 0x42Di32
	CreateCryptoSessionPolicy = 0x42Di32,
	/// D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION = 0x42Ei32
	CreateProtectedResourceSession = 0x42Ei32,
	/// D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION = 0x42Fi32
	LiveCryptoSession    = 0x42Fi32,
	/// D3D12_MESSAGE_ID_LIVE_CRYPTO_SESSION_POLICY = 0x430i32
	LiveCryptoSessionPolicy = 0x430i32,
	/// D3D12_MESSAGE_ID_LIVE_PROTECTED_RESOURCE_SESSION = 0x431i32
	LiveProtectedResourceSession = 0x431i32,
	/// D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION = 0x432i32
	DestroyCryptoSession = 0x432i32,
	/// D3D12_MESSAGE_ID_DESTROY_CRYPTO_SESSION_POLICY = 0x433i32
	DestroyCryptoSessionPolicy = 0x433i32,
	/// D3D12_MESSAGE_ID_DESTROY_PROTECTED_RESOURCE_SESSION = 0x434i32
	DestroyProtectedResourceSession = 0x434i32,
	/// D3D12_MESSAGE_ID_PROTECTED_RESOURCE_SESSION_UNSUPPORTED = 0x435i32
	ProtectedResourceSessionUnsupported = 0x435i32,
	/// D3D12_MESSAGE_ID_FENCE_INVALIDOPERATION = 0x436i32
	FenceInvalidOperation = 0x436i32,
	/// D3D12_MESSAGE_ID_CREATEQUERY_HEAP_COPY_QUEUE_TIMESTAMPS_NOT_SUPPORTED = 0x437i32
	CreateQueryHeapCopyQueueTimestampsNotSupported = 0x437i32,
	/// D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_DEFERRED = 0x438i32
	SamplePositionsMismatchDeferred = 0x438i32,
	/// D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMFIRSTUSE = 0x439i32
	SamplePositionsMismatchRecordTimeAssumedFromFirstUse = 0x439i32,
	/// D3D12_MESSAGE_ID_SAMPLEPOSITIONS_MISMATCH_RECORDTIME_ASSUMEDFROMCLEAR = 0x43Ai32
	SamplePositionsMismatchRecordTimeAssumedFromClear = 0x43Ai32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEODECODERHEAP = 0x43Bi32
	CreateVideoDecoderHeap = 0x43Bi32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEODECODERHEAP = 0x43Ci32
	LiveVideoDecoderHeap = 0x43Ci32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEODECODERHEAP = 0x43Di32
	DestroyVideoDecoderHeap = 0x43Di32,
	/// D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDARG_RETURN = 0x43Ei32
	OpenExistingHeapInvalidArgReturn = 0x43Ei32,
	/// D3D12_MESSAGE_ID_OPENEXISTINGHEAP_OUTOFMEMORY_RETURN = 0x43Fi32
	OpenExistingHeapOutOfMemoryReturn = 0x43Fi32,
	/// D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDADDRESS = 0x440i32
	OpenExistingHeapInvalidAddress = 0x440i32,
	/// D3D12_MESSAGE_ID_OPENEXISTINGHEAP_INVALIDHANDLE = 0x441i32
	OpenExistingHeapInvalidHandle = 0x441i32,
	/// D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_DEST = 0x442i32
	WriteBufferImmediateInvalidDest = 0x442i32,
	/// D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_MODE = 0x443i32
	WriteBufferImmediateInvalidMode = 0x443i32,
	/// D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_INVALID_ALIGNMENT = 0x444i32
	WriteBufferImmediateInvalidAlignment = 0x444i32,
	/// D3D12_MESSAGE_ID_WRITEBUFFERIMMEDIATE_NOT_SUPPORTED = 0x445i32
	WriteBufferImmediateNotSupported = 0x445i32,
	/// D3D12_MESSAGE_ID_SETVIEWINSTANCEMASK_INVALIDARGS = 0x446i32
	SetViewInstanceMaskInvalidArgs = 0x446i32,
	/// D3D12_MESSAGE_ID_VIEW_INSTANCING_UNSUPPORTED = 0x447i32
	ViewInstancingUnsupported = 0x447i32,
	/// D3D12_MESSAGE_ID_VIEW_INSTANCING_INVALIDARGS = 0x448i32
	ViewInstancingInvalidArgs = 0x448i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_DECODE_REFERENCE_ONLY_FLAG = 0x449i32
	CopyTextureRegionMismatchDecodeReferenceOnlyFlag = 0x449i32,
	/// D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_DECODE_REFERENCE_ONLY_FLAG = 0x44Ai32
	CopyResourceMismatchDecodeReferenceOnlyFlag = 0x44Ai32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_FAILURE = 0x44Bi32
	CreateVideoDecodeHeapCapsFailure = 0x44Bi32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEO_DECODE_HEAP_CAPS_UNSUPPORTED = 0x44Ci32
	CreateVideoDecodeHeapCapsUnsupported = 0x44Ci32,
	/// D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_INVALID_INPUT = 0x44Di32
	VideoDecodeSupportInvalidInput = 0x44Di32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEO_DECODER_UNSUPPORTED = 0x44Ei32
	CreateVideoDecoderUnsupported = 0x44Ei32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_METADATA_ERROR = 0x44Fi32
	CreateGraphicsPipelineStateMetaDataError = 0x44Fi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_VIEW_INSTANCING_VERTEX_SIZE_EXCEEDED = 0x450i32
	CreateGraphicsPipelineStateViewInstancingVertexSizeExceeded = 0x450i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_RUNTIME_INTERNAL_ERROR = 0x451i32
	CreateGraphicsPipelineStateRuntimeInternalError = 0x451i32,
	/// D3D12_MESSAGE_ID_NO_VIDEO_API_SUPPORT = 0x452i32
	NoVideoApiSupport    = 0x452i32,
	/// D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_INVALID_INPUT = 0x453i32
	VideoProcessSupportInvalidInput = 0x453i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEO_PROCESSOR_CAPS_FAILURE = 0x454i32
	CreateVideoProcessorCapsFailure = 0x454i32,
	/// D3D12_MESSAGE_ID_VIDEO_PROCESS_SUPPORT_UNSUPPORTED_FORMAT = 0x455i32
	VideoProcessSupportUnsupportedFormat = 0x455i32,
	/// D3D12_MESSAGE_ID_VIDEO_DECODE_FRAME_INVALID_ARGUMENT = 0x456i32
	VideoDecodeFrameInvalidArgument = 0x456i32,
	/// D3D12_MESSAGE_ID_ENQUEUE_MAKE_RESIDENT_INVALID_FLAGS = 0x457i32
	EnqueueMakeResidentInvalidFlags = 0x457i32,
	/// D3D12_MESSAGE_ID_OPENEXISTINGHEAP_UNSUPPORTED = 0x458i32
	OpenExistingHeapUnsupported = 0x458i32,
	/// D3D12_MESSAGE_ID_VIDEO_PROCESS_FRAMES_INVALID_ARGUMENT = 0x459i32
	VideoProcessFramesInvalidArgument = 0x459i32,
	/// D3D12_MESSAGE_ID_VIDEO_DECODE_SUPPORT_UNSUPPORTED = 0x45Ai32
	VideoDecodeSupportUnsupported = 0x45Ai32,
	/// D3D12_MESSAGE_ID_CREATE_COMMANDRECORDER = 0x45Bi32
	CreateCommandRecorder = 0x45Bi32,
	/// D3D12_MESSAGE_ID_LIVE_COMMANDRECORDER = 0x45Ci32
	LiveCommandRecorder  = 0x45Ci32,
	/// D3D12_MESSAGE_ID_DESTROY_COMMANDRECORDER = 0x45Di32
	DestroyCommandRecorder = 0x45Di32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_VIDEO_NOT_SUPPORTED = 0x45Ei32
	CreateCommandRecorderVideoNotSupported = 0x45Ei32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_SUPPORT_FLAGS = 0x45Fi32
	CreateCommandRecorderInvalidSupportFlags = 0x45Fi32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_INVALID_FLAGS = 0x460i32
	CreateCommandRecorderInvalidFlags = 0x460i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_RECORDER_MORE_RECORDERS_THAN_LOGICAL_PROCESSORS = 0x461i32
	CreateCommandRecorderMoreRecordersThanLogicalProcessors = 0x461i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMANDPOOL = 0x462i32
	CreateCommandPool    = 0x462i32,
	/// D3D12_MESSAGE_ID_LIVE_COMMANDPOOL = 0x463i32
	LiveCommandPool      = 0x463i32,
	/// D3D12_MESSAGE_ID_DESTROY_COMMANDPOOL = 0x464i32
	DestroyCommandPool   = 0x464i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_POOL_INVALID_FLAGS = 0x465i32
	CreateCommandPoolInvalidFlags = 0x465i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_VIDEO_NOT_SUPPORTED = 0x466i32
	CreateCommandListVideoNotSupported = 0x466i32,
	/// D3D12_MESSAGE_ID_COMMAND_RECORDER_SUPPORT_FLAGS_MISMATCH = 0x467i32
	CommandRecorderSupportFlagsMismatch = 0x467i32,
	/// D3D12_MESSAGE_ID_COMMAND_RECORDER_CONTENTION = 0x468i32
	CommandRecorderContention = 0x468i32,
	/// D3D12_MESSAGE_ID_COMMAND_RECORDER_USAGE_WITH_CREATECOMMANDLIST_COMMAND_LIST = 0x469i32
	CommandRecorderUsageWithCreateCommandListCommandList = 0x469i32,
	/// D3D12_MESSAGE_ID_COMMAND_ALLOCATOR_USAGE_WITH_CREATECOMMANDLIST1_COMMAND_LIST = 0x46Ai32
	CommandAllocatorUsageWithCreateCommandList1CommandList = 0x46Ai32,
	/// D3D12_MESSAGE_ID_CANNOT_EXECUTE_EMPTY_COMMAND_LIST = 0x46Bi32
	CannotExecuteEmptyCommandList = 0x46Bi32,
	/// D3D12_MESSAGE_ID_CANNOT_RESET_COMMAND_POOL_WITH_OPEN_COMMAND_LISTS = 0x46Ci32
	CannotResetCommandPoolWithOpenCommandLists = 0x46Ci32,
	/// D3D12_MESSAGE_ID_CANNOT_USE_COMMAND_RECORDER_WITHOUT_CURRENT_TARGET = 0x46Di32
	CannotUseCommandRecorderWithoutCurrentTarget = 0x46Di32,
	/// D3D12_MESSAGE_ID_CANNOT_CHANGE_COMMAND_RECORDER_TARGET_WHILE_RECORDING = 0x46Ei32
	CannotChangeCommandRecorderTargetWhileRecording = 0x46Ei32,
	/// D3D12_MESSAGE_ID_COMMAND_POOL_SYNC = 0x46Fi32
	CommandPoolSync      = 0x46Fi32,
	/// D3D12_MESSAGE_ID_EVICT_UNDERFLOW = 0x470i32
	EvictUnderflow       = 0x470i32,
	/// D3D12_MESSAGE_ID_CREATE_META_COMMAND = 0x471i32
	CreateMetaCommand    = 0x471i32,
	/// D3D12_MESSAGE_ID_LIVE_META_COMMAND = 0x472i32
	LiveMetaCommand      = 0x472i32,
	/// D3D12_MESSAGE_ID_DESTROY_META_COMMAND = 0x473i32
	DestroyMetaCommand   = 0x473i32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_DST_RESOURCE = 0x474i32
	CopyBufferRegionInvalidDstResource = 0x474i32,
	/// D3D12_MESSAGE_ID_COPYBUFFERREGION_INVALID_SRC_RESOURCE = 0x475i32
	CopyBufferRegionInvalidSrcResource = 0x475i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_DST_RESOURCE = 0x476i32
	AtomicCopyBufferInvalidDstResource = 0x476i32,
	/// D3D12_MESSAGE_ID_ATOMICCOPYBUFFER_INVALID_SRC_RESOURCE = 0x477i32
	AtomicCopyBufferInvalidSrcResource = 0x477i32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_BUFFER = 0x478i32
	CreatePlacedResourceOnBufferNullBuffer = 0x478i32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_NULL_RESOURCE_DESC = 0x479i32
	CreatePlacedResourceOnBufferNullResourceDesc = 0x479i32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_UNSUPPORTED = 0x47Ai32
	CreatePlacedResourceOnBufferUnsupported = 0x47Ai32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_DIMENSION = 0x47Bi32
	CreatePlacedResourceOnBufferInvalidBufferDimension = 0x47Bi32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_FLAGS = 0x47Ci32
	CreatePlacedResourceOnBufferInvalidBufferFlags = 0x47Ci32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_BUFFER_OFFSET = 0x47Di32
	CreatePlacedResourceOnBufferInvalidBufferOffset = 0x47Di32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_DIMENSION = 0x47Ei32
	CreatePlacedResourceOnBufferInvalidResourceDimension = 0x47Ei32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_INVALID_RESOURCE_FLAGS = 0x47Fi32
	CreatePlacedResourceOnBufferInvalidResourceFlags = 0x47Fi32,
	/// D3D12_MESSAGE_ID_CREATEPLACEDRESOURCEONBUFFER_OUTOFMEMORY_RETURN = 0x480i32
	CreatePlacedResourceOnBufferOutOfMemoryReturn = 0x480i32,
	/// D3D12_MESSAGE_ID_CANNOT_CREATE_GRAPHICS_AND_VIDEO_COMMAND_RECORDER = 0x481i32
	CannotCreateGraphicsAndVideoCommandRecorder = 0x481i32,
	/// D3D12_MESSAGE_ID_UPDATETILEMAPPINGS_POSSIBLY_MISMATCHING_PROPERTIES = 0x482i32
	UpdateTileMappingsPossiblyMismatchingProperties = 0x482i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE = 0x483i32
	CreateCommandListInvalidCommandListType = 0x483i32,
	/// D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INCOMPATIBLE_WITH_STRUCTURED_BUFFERS = 0x484i32
	ClearUnorderedAccessViewIncompatibleWithStructuredBuffers = 0x484i32,
	/// D3D12_MESSAGE_ID_COMPUTE_ONLY_DEVICE_OPERATION_UNSUPPORTED = 0x485i32
	ComputeOnlyDeviceOperationUnsupported = 0x485i32,
	/// D3D12_MESSAGE_ID_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INVALID = 0x486i32
	BuildRaytracingAccelerationStructureInvalid = 0x486i32,
	/// D3D12_MESSAGE_ID_EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_INVALID = 0x487i32
	EmitRaytracingAccelerationStructurePostBuildInfoInvalid = 0x487i32,
	/// D3D12_MESSAGE_ID_COPY_RAYTRACING_ACCELERATION_STRUCTURE_INVALID = 0x488i32
	CopyRaytracingAccelerationStructureInvalid = 0x488i32,
	/// D3D12_MESSAGE_ID_DISPATCH_RAYS_INVALID = 0x489i32
	DispatchRaysInvalid  = 0x489i32,
	/// D3D12_MESSAGE_ID_GET_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO_INVALID = 0x48Ai32
	GetRaytracingAccelerationStructurePrebuildInfoInvalid = 0x48Ai32,
	/// D3D12_MESSAGE_ID_CREATE_LIFETIMETRACKER = 0x48Bi32
	CreateLifetimeTracker = 0x48Bi32,
	/// D3D12_MESSAGE_ID_LIVE_LIFETIMETRACKER = 0x48Ci32
	LiveLifetimeTracker  = 0x48Ci32,
	/// D3D12_MESSAGE_ID_DESTROY_LIFETIMETRACKER = 0x48Di32
	DestroyLifetimeTracker = 0x48Di32,
	/// D3D12_MESSAGE_ID_DESTROYOWNEDOBJECT_OBJECTNOTOWNED = 0x48Ei32
	DestroyOwnedObjectObjectNotOwned = 0x48Ei32,
	/// D3D12_MESSAGE_ID_CREATE_TRACKEDWORKLOAD = 0x48Fi32
	CreateTrackedWorkload = 0x48Fi32,
	/// D3D12_MESSAGE_ID_LIVE_TRACKEDWORKLOAD = 0x490i32
	LiveTrackedWorkload  = 0x490i32,
	/// D3D12_MESSAGE_ID_DESTROY_TRACKEDWORKLOAD = 0x491i32
	DestroyTrackedWorkload = 0x491i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_ERROR = 0x492i32
	RenderPassError      = 0x492i32,
	/// D3D12_MESSAGE_ID_META_COMMAND_ID_INVALID = 0x493i32
	MetaCommandIdInvalid = 0x493i32,
	/// D3D12_MESSAGE_ID_META_COMMAND_UNSUPPORTED_PARAMS = 0x494i32
	MetaCommandUnsupportedParams = 0x494i32,
	/// D3D12_MESSAGE_ID_META_COMMAND_FAILED_ENUMERATION = 0x495i32
	MetaCommandFailedEnumeration = 0x495i32,
	/// D3D12_MESSAGE_ID_META_COMMAND_PARAMETER_SIZE_MISMATCH = 0x496i32
	MetaCommandParameterSizeMismatch = 0x496i32,
	/// D3D12_MESSAGE_ID_UNINITIALIZED_META_COMMAND = 0x497i32
	UninitializedMetaCommand = 0x497i32,
	/// D3D12_MESSAGE_ID_META_COMMAND_INVALID_GPU_VIRTUAL_ADDRESS = 0x498i32
	MetaCommandInvalidGpuVirtualAddress = 0x498i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDLIST = 0x499i32
	CreateVideoEncodeCommandList = 0x499i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDLIST = 0x49Ai32
	LiveVideoEncodeCommandList = 0x49Ai32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDLIST = 0x49Bi32
	DestroyVideoEncodeCommandList = 0x49Bi32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOENCODECOMMANDQUEUE = 0x49Ci32
	CreateVideoEncodeCommandQueue = 0x49Ci32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOENCODECOMMANDQUEUE = 0x49Di32
	LiveVideoEncodeCommandQueue = 0x49Di32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOENCODECOMMANDQUEUE = 0x49Ei32
	DestroyVideoEncodeCommandQueue = 0x49Ei32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONESTIMATOR = 0x49Fi32
	CreateVideoMotionEstimator = 0x49Fi32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONESTIMATOR = 0x4A0i32
	LiveVideoMotionEstimator = 0x4A0i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONESTIMATOR = 0x4A1i32
	DestroyVideoMotionEstimator = 0x4A1i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOMOTIONVECTORHEAP = 0x4A2i32
	CreateVideoMotionVectorHeap = 0x4A2i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOMOTIONVECTORHEAP = 0x4A3i32
	LiveVideoMotionVectorHeap = 0x4A3i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOMOTIONVECTORHEAP = 0x4A4i32
	DestroyVideoMotionVectorHeap = 0x4A4i32,
	/// D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOADS = 0x4A5i32
	MultipleTrackedWorkloads = 0x4A5i32,
	/// D3D12_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOAD_PAIRS = 0x4A6i32
	MultipleTrackedWorkloadPairs = 0x4A6i32,
	/// D3D12_MESSAGE_ID_OUT_OF_ORDER_TRACKED_WORKLOAD_PAIR = 0x4A7i32
	OutOfOrderTrackedWorkloadPair = 0x4A7i32,
	/// D3D12_MESSAGE_ID_CANNOT_ADD_TRACKED_WORKLOAD = 0x4A8i32
	CannotAddTrackedWorkload = 0x4A8i32,
	/// D3D12_MESSAGE_ID_INCOMPLETE_TRACKED_WORKLOAD_PAIR = 0x4A9i32
	IncompleteTrackedWorkloadPair = 0x4A9i32,
	/// D3D12_MESSAGE_ID_CREATE_STATE_OBJECT_ERROR = 0x4AAi32
	CreateStateObjectError = 0x4AAi32,
	/// D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_ERROR = 0x4ABi32
	GetShaderIdentifierError = 0x4ABi32,
	/// D3D12_MESSAGE_ID_GET_SHADER_STACK_SIZE_ERROR = 0x4ACi32
	GetShaderStackSizeError = 0x4ACi32,
	/// D3D12_MESSAGE_ID_GET_PIPELINE_STACK_SIZE_ERROR = 0x4ADi32
	GetPipelineStackSizeError = 0x4ADi32,
	/// D3D12_MESSAGE_ID_SET_PIPELINE_STACK_SIZE_ERROR = 0x4AEi32
	SetPipelineStackSizeError = 0x4AEi32,
	/// D3D12_MESSAGE_ID_GET_SHADER_IDENTIFIER_SIZE_INVALID = 0x4AFi32
	GetShaderIdentifierSizeInvalid = 0x4AFi32,
	/// D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_INVALID = 0x4B0i32
	CheckDriverMatchingIdentifierInvalid = 0x4B0i32,
	/// D3D12_MESSAGE_ID_CHECK_DRIVER_MATCHING_IDENTIFIER_DRIVER_REPORTED_ISSUE = 0x4B1i32
	CheckDriverMatchingIdentifierDriverReportedIssue = 0x4B1i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_INVALID_RESOURCE_BARRIER = 0x4B2i32
	RenderPassInvalidResourceBarrier = 0x4B2i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_DISALLOWED_API_CALLED = 0x4B3i32
	RenderPassDisallowedApiCalled = 0x4B3i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_NEST_RENDER_PASSES = 0x4B4i32
	RenderPassCannotNestRenderPasses = 0x4B4i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_END_WITHOUT_BEGIN = 0x4B5i32
	RenderPassCannotEndWithoutBegin = 0x4B5i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_CANNOT_CLOSE_COMMAND_LIST = 0x4B6i32
	RenderPassCannotCloseCommandList = 0x4B6i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_GPU_WORK_WHILE_SUSPENDED = 0x4B7i32
	RenderPassGpuWorkWhileSuspended = 0x4B7i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_SUSPEND_RESUME = 0x4B8i32
	RenderPassMismatchingSuspendResume = 0x4B8i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_NO_PRIOR_SUSPEND_WITHIN_EXECUTECOMMANDLISTS = 0x4B9i32
	RenderPassNoPriorSuspendWithinExecuteCommandLists = 0x4B9i32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_NO_SUBSEQUENT_RESUME_WITHIN_EXECUTECOMMANDLISTS = 0x4BAi32
	RenderPassNoSubsequentResumeWithinExecuteCommandLists = 0x4BAi32,
	/// D3D12_MESSAGE_ID_TRACKED_WORKLOAD_COMMAND_QUEUE_MISMATCH = 0x4BBi32
	TrackedWorkloadCommandQueueMismatch = 0x4BBi32,
	/// D3D12_MESSAGE_ID_TRACKED_WORKLOAD_NOT_SUPPORTED = 0x4BCi32
	TrackedWorkloadNotSupported = 0x4BCi32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_MISMATCHING_NO_ACCESS = 0x4BDi32
	RenderPassMismatchingNoAccess = 0x4BDi32,
	/// D3D12_MESSAGE_ID_RENDER_PASS_UNSUPPORTED_RESOLVE = 0x4BEi32
	RenderPassUnsupportedResolve = 0x4BEi32,
	/// D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INVALID_RESOURCE_PTR = 0x4BFi32
	ClearUnorderedAccessViewInvalidResourcePtr = 0x4BFi32,
	/// D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_SIGNAL = 0x4C0i32
	Windows7FenceOutOfOrderSignal = 0x4C0i32,
	/// D3D12_MESSAGE_ID_WINDOWS7_FENCE_OUTOFORDER_WAIT = 0x4C1i32
	Windows7FenceOutOfOrderWait = 0x4C1i32,
	/// D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_ESTIMATOR_INVALID_ARGUMENT = 0x4C2i32
	VideoCreateMotionEstimatorInvalidArgument = 0x4C2i32,
	/// D3D12_MESSAGE_ID_VIDEO_CREATE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT = 0x4C3i32
	VideoCreateMotionVectorHeapInvalidArgument = 0x4C3i32,
	/// D3D12_MESSAGE_ID_ESTIMATE_MOTION_INVALID_ARGUMENT = 0x4C4i32
	EstimateMotionInvalidArgument = 0x4C4i32,
	/// D3D12_MESSAGE_ID_RESOLVE_MOTION_VECTOR_HEAP_INVALID_ARGUMENT = 0x4C5i32
	ResolveMotionVectorHeapInvalidArgument = 0x4C5i32,
	/// D3D12_MESSAGE_ID_GETGPUVIRTUALADDRESS_INVALID_HEAP_TYPE = 0x4C6i32
	GetGpuVirtualAddressInvalidHeapType = 0x4C6i32,
	/// D3D12_MESSAGE_ID_SET_BACKGROUND_PROCESSING_MODE_INVALID_ARGUMENT = 0x4C7i32
	SetBackgroundProcessingModeInvalidArgument = 0x4C7i32,
	/// D3D12_MESSAGE_ID_CREATE_COMMAND_LIST_INVALID_COMMAND_LIST_TYPE_FOR_FEATURE_LEVEL = 0x4C8i32
	CreateCommandListInvalidCommandListTypeForFeatureLevel = 0x4C8i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOEXTENSIONCOMMAND = 0x4C9i32
	CreateVideoExtensionCommand = 0x4C9i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOEXTENSIONCOMMAND = 0x4CAi32
	LiveVideoExtensionCommand = 0x4CAi32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOEXTENSIONCOMMAND = 0x4CBi32
	DestroyVideoExtensionCommand = 0x4CBi32,
	/// D3D12_MESSAGE_ID_INVALID_VIDEO_EXTENSION_COMMAND_ID = 0x4CCi32
	InvalidVideoExtensionCommandId = 0x4CCi32,
	/// D3D12_MESSAGE_ID_VIDEO_EXTENSION_COMMAND_INVALID_ARGUMENT = 0x4CDi32
	VideoExtensionCommandInvalidArgument = 0x4CDi32,
	/// D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_NOT_UNIQUE_IN_DXIL_LIBRARY = 0x4CEi32
	CreateRootSignatureNotUniqueInDxilLibrary = 0x4CEi32,
	/// D3D12_MESSAGE_ID_VARIABLE_SHADING_RATE_NOT_ALLOWED_WITH_TIR = 0x4CFi32
	VariableShadingRateNotAllowedWithTir = 0x4CFi32,
	/// D3D12_MESSAGE_ID_GEOMETRY_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE = 0x4D0i32
	GeometryShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = 0x4D0i32,
	/// D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_SHADING_RATE = 0x4D1i32
	RsSetShadingRateInvalidShadingRate = 0x4D1i32,
	/// D3D12_MESSAGE_ID_RSSETSHADING_RATE_SHADING_RATE_NOT_PERMITTED_BY_CAP = 0x4D2i32
	RsSetShadingRateShadingRateNotPermittedByCap = 0x4D2i32,
	/// D3D12_MESSAGE_ID_RSSETSHADING_RATE_INVALID_COMBINER = 0x4D3i32
	RsSetShadingRateInvalidCombiner = 0x4D3i32,
	/// D3D12_MESSAGE_ID_RSSETSHADINGRATEIMAGE_REQUIRES_TIER_2 = 0x4D4i32
	RsSetShadIngrateImageRequiresTier2 = 0x4D4i32,
	/// D3D12_MESSAGE_ID_RSSETSHADINGRATE_REQUIRES_TIER_1 = 0x4D5i32
	RsSetShadIngrateRequiresTier1 = 0x4D5i32,
	/// D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_FORMAT = 0x4D6i32
	ShadingRateImageIncorrectFormat = 0x4D6i32,
	/// D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_ARRAY_SIZE = 0x4D7i32
	ShadingRateImageIncorrectArraySize = 0x4D7i32,
	/// D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_MIP_LEVEL = 0x4D8i32
	ShadingRateImageIncorrectMipLevel = 0x4D8i32,
	/// D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_COUNT = 0x4D9i32
	ShadingRateImageIncorrectSampleCount = 0x4D9i32,
	/// D3D12_MESSAGE_ID_SHADING_RATE_IMAGE_INCORRECT_SAMPLE_QUALITY = 0x4DAi32
	ShadingRateImageIncorrectSampleQuality = 0x4DAi32,
	/// D3D12_MESSAGE_ID_NON_RETAIL_SHADER_MODEL_WONT_VALIDATE = 0x4DBi32
	NonRetailShaderModelWontValidate = 0x4DBi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_ROOT_SIGNATURE_MISMATCH = 0x4DCi32
	CreateGraphicsPipelineStateAsRootSignatureMismatch = 0x4DCi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_ROOT_SIGNATURE_MISMATCH = 0x4DDi32
	CreateGraphicsPipelineStateMsRootSignatureMismatch = 0x4DDi32,
	/// D3D12_MESSAGE_ID_ADD_TO_STATE_OBJECT_ERROR = 0x4DEi32
	AddToStateObjectError = 0x4DEi32,
	/// D3D12_MESSAGE_ID_CREATE_PROTECTED_RESOURCE_SESSION_INVALID_ARGUMENT = 0x4DFi32
	CreateProtectedResourceSessionInvalidArgument = 0x4DFi32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_PSO_DESC_MISMATCH = 0x4E0i32
	CreateGraphicsPipelineStateMsPsoDescMismatch = 0x4E0i32,
	/// D3D12_MESSAGE_ID_CREATEPIPELINESTATE_MS_INCOMPLETE_TYPE = 0x4E1i32
	CreatePipelineStateMsIncompleteType = 0x4E1i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_AS_NOT_MS_MISMATCH = 0x4E2i32
	CreateGraphicsPipelineStateAsNotMsMismatch = 0x4E2i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_MS_NOT_PS_MISMATCH = 0x4E3i32
	CreateGraphicsPipelineStateMsNotPsMismatch = 0x4E3i32,
	/// D3D12_MESSAGE_ID_NONZERO_SAMPLER_FEEDBACK_MIP_REGION_WITH_INCOMPATIBLE_FORMAT = 0x4E4i32
	NonzeroSamplerFeedbackMipRegionWithIncompatibleFormat = 0x4E4i32,
	/// D3D12_MESSAGE_ID_CREATEGRAPHICSPIPELINESTATE_INPUTLAYOUT_SHADER_MISMATCH = 0x4E5i32
	CreateGraphicsPipelineStateInputLayoutShaderMismatch = 0x4E5i32,
	/// D3D12_MESSAGE_ID_EMPTY_DISPATCH = 0x4E6i32
	EmptyDispatch        = 0x4E6i32,
	/// D3D12_MESSAGE_ID_RESOURCE_FORMAT_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY = 0x4E7i32
	ResourceFormatRequiresSamplerFeedbackCapability = 0x4E7i32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_MIP_REGION = 0x4E8i32
	SamplerFeedbackMapInvalidMipRegion = 0x4E8i32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_DIMENSION = 0x4E9i32
	SamplerFeedbackMapInvalidDimension = 0x4E9i32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_COUNT = 0x4EAi32
	SamplerFeedbackMapInvalidSampleCount = 0x4EAi32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_SAMPLE_QUALITY = 0x4EBi32
	SamplerFeedbackMapInvalidSampleQuality = 0x4EBi32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_INVALID_LAYOUT = 0x4ECi32
	SamplerFeedbackMapInvalidLayout = 0x4ECi32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_MAP_REQUIRES_UNORDERED_ACCESS_FLAG = 0x4EDi32
	SamplerFeedbackMapRequiresUnorderedAccessFlag = 0x4EDi32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_NULL_ARGUMENTS = 0x4EEi32
	SamplerFeedbackCreateUavNullArguments = 0x4EEi32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_UAV_REQUIRES_SAMPLER_FEEDBACK_CAPABILITY = 0x4EFi32
	SamplerFeedbackUavRequiresSamplerFeedbackCapability = 0x4EFi32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_REQUIRES_FEEDBACK_MAP_FORMAT = 0x4F0i32
	SamplerFeedbackCreateUavRequiresFeedbackMapFormat = 0x4F0i32,
	/// D3D12_MESSAGE_ID_CREATEMESHSHADER_INVALIDSHADERBYTECODE = 0x4F1i32
	CreateMeshShaderInvalidShaderByteCode = 0x4F1i32,
	/// D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTOFMEMORY = 0x4F2i32
	CreateMeshShaderOutOfMemory = 0x4F2i32,
	/// D3D12_MESSAGE_ID_CREATEMESHSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE = 0x4F3i32
	CreateMeshShaderWithStreamOutputInvalidShaderType = 0x4F3i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_INVALID_FORMAT = 0x4F4i32
	ResolveSubresourceSamplerFeedbackTransCodeInvalidFormat = 0x4F4i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_INVALID_MIP_LEVEL_COUNT = 0x4F5i32
	ResolveSubresourceSamplerFeedbackInvalidMipLevelCount = 0x4F5i32,
	/// D3D12_MESSAGE_ID_RESOLVESUBRESOURCE_SAMPLER_FEEDBACK_TRANSCODE_ARRAY_SIZE_MISMATCH = 0x4F6i32
	ResolveSubresourceSamplerFeedbackTransCodeArraySizeMismatch = 0x4F6i32,
	/// D3D12_MESSAGE_ID_SAMPLER_FEEDBACK_CREATE_UAV_MISMATCHING_TARGETED_RESOURCE = 0x4F7i32
	SamplerFeedbackCreateUavMismatchingTargetedResource = 0x4F7i32,
	/// D3D12_MESSAGE_ID_CREATEMESHSHADER_OUTPUTEXCEEDSMAXSIZE = 0x4F8i32
	CreateMeshShaderOutputExceedsMaxSize = 0x4F8i32,
	/// D3D12_MESSAGE_ID_CREATEMESHSHADER_GROUPSHAREDEXCEEDSMAXSIZE = 0x4F9i32
	CreateMeshShaderGroupSharedExceedsMaxSize = 0x4F9i32,
	/// D3D12_MESSAGE_ID_VERTEX_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE = 0x4FAi32
	VertexShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = 0x4FAi32,
	/// D3D12_MESSAGE_ID_MESH_SHADER_OUTPUTTING_BOTH_VIEWPORT_ARRAY_INDEX_AND_SHADING_RATE_NOT_SUPPORTED_ON_DEVICE = 0x4FBi32
	MeshShaderOutputtingBothViewportArrayIndexAndShadingRateNotSupportedOnDevice = 0x4FBi32,
	/// D3D12_MESSAGE_ID_CREATEMESHSHADER_MISMATCHEDASMSPAYLOADSIZE = 0x4FCi32
	CreateMeshShaderMismatchedAsMsPayloadSize = 0x4FCi32,
	/// D3D12_MESSAGE_ID_CREATE_ROOT_SIGNATURE_UNBOUNDED_STATIC_DESCRIPTORS = 0x4FDi32
	CreateRootSignatureUnboundedStaticDescriptors = 0x4FDi32,
	/// D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_INVALIDSHADERBYTECODE = 0x4FEi32
	CreateAmplificationShaderInvalidShaderByteCode = 0x4FEi32,
	/// D3D12_MESSAGE_ID_CREATEAMPLIFICATIONSHADER_OUTOFMEMORY = 0x4FFi32
	CreateAmplificationShaderOutOfMemory = 0x4FFi32,
	/// D3D12_MESSAGE_ID_CREATE_SHADERCACHESESSION = 0x500i32
	CreateShaderCacheSession = 0x500i32,
	/// D3D12_MESSAGE_ID_LIVE_SHADERCACHESESSION = 0x501i32
	LiveShaderCacheSession = 0x501i32,
	/// D3D12_MESSAGE_ID_DESTROY_SHADERCACHESESSION = 0x502i32
	DestroyShaderCacheSession = 0x502i32,
	/// D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_INVALIDARGS = 0x503i32
	CreateShaderCacheSessionInvalidArgs = 0x503i32,
	/// D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_DISABLED = 0x504i32
	CreateShaderCacheSessionDisabled = 0x504i32,
	/// D3D12_MESSAGE_ID_CREATESHADERCACHESESSION_ALREADYOPEN = 0x505i32
	CreateShaderCacheSessionAlreadyOpen = 0x505i32,
	/// D3D12_MESSAGE_ID_SHADERCACHECONTROL_DEVELOPERMODE = 0x506i32
	ShaderCacheControlDeveloperMode = 0x506i32,
	/// D3D12_MESSAGE_ID_SHADERCACHECONTROL_INVALIDFLAGS = 0x507i32
	ShaderCacheControlInvalidFlags = 0x507i32,
	/// D3D12_MESSAGE_ID_SHADERCACHECONTROL_STATEALREADYSET = 0x508i32
	ShaderCacheControlStateAlreadySet = 0x508i32,
	/// D3D12_MESSAGE_ID_SHADERCACHECONTROL_IGNOREDFLAG = 0x509i32
	ShaderCacheControlIgnoredFlag = 0x509i32,
	/// D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_ALREADYPRESENT = 0x50Ai32
	ShaderCacheSessionSToRevalueAlreadyPresent = 0x50Ai32,
	/// D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_HASHCOLLISION = 0x50Bi32
	ShaderCacheSessionSToRevalueHashCollision = 0x50Bi32,
	/// D3D12_MESSAGE_ID_SHADERCACHESESSION_STOREVALUE_CACHEFULL = 0x50Ci32
	ShaderCacheSessionSToRevalueCacheFull = 0x50Ci32,
	/// D3D12_MESSAGE_ID_SHADERCACHESESSION_FINDVALUE_NOTFOUND = 0x50Di32
	ShaderCacheSessionFindValueNotFound = 0x50Di32,
	/// D3D12_MESSAGE_ID_SHADERCACHESESSION_CORRUPT = 0x50Ei32
	ShaderCacheSessionCorrupt = 0x50Ei32,
	/// D3D12_MESSAGE_ID_SHADERCACHESESSION_DISABLED = 0x50Fi32
	ShaderCacheSessionDisabled = 0x50Fi32,
	/// D3D12_MESSAGE_ID_OVERSIZED_DISPATCH = 0x510i32
	OversizedDispatch    = 0x510i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOENCODER = 0x511i32
	CreateVideoEncoder   = 0x511i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOENCODER = 0x512i32
	LiveVideoEncoder     = 0x512i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOENCODER = 0x513i32
	DestroyVideoEncoder  = 0x513i32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEOENCODERHEAP = 0x514i32
	CreateVideoEncoderHeap = 0x514i32,
	/// D3D12_MESSAGE_ID_LIVE_VIDEOENCODERHEAP = 0x515i32
	LiveVideoEncoderHeap = 0x515i32,
	/// D3D12_MESSAGE_ID_DESTROY_VIDEOENCODERHEAP = 0x516i32
	DestroyVideoEncoderHeap = 0x516i32,
	/// D3D12_MESSAGE_ID_COPYTEXTUREREGION_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG = 0x517i32
	CopyTextureRegionMismatchEncodeReferenceOnlyFlag = 0x517i32,
	/// D3D12_MESSAGE_ID_COPYRESOURCE_MISMATCH_ENCODE_REFERENCE_ONLY_FLAG = 0x518i32
	CopyResourceMismatchEncodeReferenceOnlyFlag = 0x518i32,
	/// D3D12_MESSAGE_ID_ENCODE_FRAME_INVALID_PARAMETERS = 0x519i32
	EncodeFrameInvalidParameters = 0x519i32,
	/// D3D12_MESSAGE_ID_ENCODE_FRAME_UNSUPPORTED_PARAMETERS = 0x51Ai32
	EncodeFrameUnsupportedParameters = 0x51Ai32,
	/// D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_INVALID_PARAMETERS = 0x51Bi32
	ResolveEncoderOutputMetaDataInvalidParameters = 0x51Bi32,
	/// D3D12_MESSAGE_ID_RESOLVE_ENCODER_OUTPUT_METADATA_UNSUPPORTED_PARAMETERS = 0x51Ci32
	ResolveEncoderOutputMetaDataUnsupportedParameters = 0x51Ci32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_INVALID_PARAMETERS = 0x51Di32
	CreateVideoEncoderInvalidParameters = 0x51Di32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_UNSUPPORTED_PARAMETERS = 0x51Ei32
	CreateVideoEncoderUnsupportedParameters = 0x51Ei32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_INVALID_PARAMETERS = 0x51Fi32
	CreateVideoEncoderHeapInvalidParameters = 0x51Fi32,
	/// D3D12_MESSAGE_ID_CREATE_VIDEO_ENCODER_HEAP_UNSUPPORTED_PARAMETERS = 0x520i32
	CreateVideoEncoderHeapUnsupportedParameters = 0x520i32,
	/// D3D12_MESSAGE_ID_CREATECOMMANDLIST_NULL_COMMANDALLOCATOR = 0x521i32
	CreateCommandListNullCommandAllocator = 0x521i32,
	/// D3D12_MESSAGE_ID_CLEAR_UNORDERED_ACCESS_VIEW_INVALID_DESCRIPTOR_HANDLE = 0x522i32
	ClearUnorderedAccessViewInvalidDescriptorHandle = 0x522i32,
	/// D3D12_MESSAGE_ID_DESCRIPTOR_HEAP_NOT_SHADER_VISIBLE = 0x523i32
	DescriptorHeapNotShaderVisible = 0x523i32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOP_WARNING = 0x524i32
	CreateBlendStateBlendOpWarning = 0x524i32,
	/// D3D12_MESSAGE_ID_CREATEBLENDSTATE_BLENDOPALPHA_WARNING = 0x525i32
	CreateBlendStateBlendOpAlphaWarning = 0x525i32,
	/// D3D12_MESSAGE_ID_WRITE_COMBINE_PERFORMANCE_WARNING = 0x526i32
	WriteCombinePerformanceWarning = 0x526i32,
	/// D3D12_MESSAGE_ID_RESOLVE_QUERY_INVALID_QUERY_STATE = 0x527i32
	ResolveQueryInvalidQueryState = 0x527i32,
	/// D3D12_MESSAGE_ID_SETPRIVATEDATA_NO_ACCESS = 0x528i32
	SetPrivateDataNoAccess = 0x528i32,
	/// D3D12_MESSAGE_ID_COMMAND_LIST_STATIC_DESCRIPTOR_SAMPLER_MODE_MISMATCH = 0x529i32
	CommandListStaticDescriptorSamplerModeMismatch = 0x529i32,
	/// D3D12_MESSAGE_ID_GETCOPYABLEFOOTPRINTS_UNSUPPORTED_BUFFER_WIDTH = 0x52Ai32
	GetCopyAbleFootprintsUnsupportedBufferWidth = 0x52Ai32,
	/// D3D12_MESSAGE_ID_CREATEMESHSHADER_TOPOLOGY_MISMATCH = 0x52Bi32
	CreateMeshShaderTopologyMismatch = 0x52Bi32,
	/// D3D12_MESSAGE_ID_VRS_SUM_COMBINER_REQUIRES_CAPABILITY = 0x52Ci32
	VrsSumCombinerRequiresCapability = 0x52Ci32,
	/// D3D12_MESSAGE_ID_SETTING_SHADING_RATE_FROM_MS_REQUIRES_CAPABILITY = 0x52Di32
	SettingShadingRateFromMsRequiresCapability = 0x52Di32,
	/// D3D12_MESSAGE_ID_SHADERCACHESESSION_SHADERCACHEDELETE_NOTSUPPORTED = 0x52Ei32
	ShaderCacheSessionShaderCacheDeleteNotSupported = 0x52Ei32,
	/// D3D12_MESSAGE_ID_SHADERCACHECONTROL_SHADERCACHECLEAR_NOTSUPPORTED = 0x52Fi32
	ShaderCacheControlShaderCacheClearNotSupported = 0x52Fi32,
	/// D3D12_MESSAGE_ID_D3D12_MESSAGES_END = 0x530i32
	D3D12MessagesEnd     = 0x530i32,
}

impl D3D12MessageId {
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT
	pub const CreateSrvUnrecognizedFormat: Self = Self::CreateShaderResourceViewUnrecognizedFormat;
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC
	pub const CreateSrvInvalidDesc: Self = Self::CreateShaderResourceViewInvalidDesc;
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT
	pub const CreateSrvInvalidFormat: Self = Self::CreateShaderResourceViewInvalidFormat;
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANESLICE
	pub const CreateSrvInvalidVideoPlaneSlice: Self = Self::CreateShaderResourceViewInvalidVideoPlaneSlice;
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANESLICE
	pub const CreateSrvInvalidPlaneSlice: Self = Self::CreateShaderResourceViewInvalidPlaneSlice;
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS
	pub const CreateSrvInvalidDimensions: Self = Self::CreateShaderResourceViewInvalidDimensions;
	/// D3D12_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE
	pub const CreateSrvInvalidResource: Self = Self::CreateShaderResourceViewInvalidResource;
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE
	pub const CreateUavInvalidResource: Self = Self::CreateUnorderedAccessViewInvalidResource;
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC
	pub const CreateUavInvalidDesc: Self = Self::CreateUnorderedAccessViewInvalidDesc;
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT
	pub const CreateUavInvalidFormat: Self = Self::CreateUnorderedAccessViewInvalidFormat;
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANESLICE
	pub const CreateUavInvalidVideoPlaneSlice: Self = Self::CreateUnorderedAccessViewInvalidVideoPlaneSlice;
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANESLICE
	pub const CreateUavInvalidPlaneSlice: Self = Self::CreateUnorderedAccessViewInvalidPlaneSlice;
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS
	pub const CreateUavInvalidDimensions: Self = Self::CreateUnorderedAccessViewInvalidDimensions;
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT
	pub const CreateUavUnrecognizedFormat: Self = Self::CreateUnorderedAccessViewUnrecognizedFormat;
	/// D3D12_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS
	pub const CreateUavInvalidFlags: Self = Self::CreateUnorderedAccessViewInvalidFlags;
	/// D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_RESOURCE
	pub const CreateCbvInvalidResource: Self = Self::CreateConstantBufferViewInvalidResource;
	/// D3D12_MESSAGE_ID_CREATE_CONSTANT_BUFFER_VIEW_INVALID_DESC
	pub const CreateCbvInvalidDesc: Self = Self::CreateConstantBufferViewInvalidDesc;
	/// D3D12_MESSAGE_ID_CREATE_UNORDEREDACCESS_VIEW_INVALID_COUNTER_USAGE
	pub const CreateUavInvalidCounterUsage: Self = Self::CreateUnorderedAccessViewInvalidCounterUsage;
	/// D3D12_MESSAGE_ID_SET_ROOT_CONSTANT_BUFFER_VIEW_INVALID
	pub const SetRootCbvInvalid   : Self = Self::SetRootConstantBufferViewInvalid;
	/// D3D12_MESSAGE_ID_SET_ROOT_SHADER_RESOURCE_VIEW_INVALID
	pub const SetRootSrvInvalid   : Self = Self::SetRootShaderResourceViewInvalid;
	/// D3D12_MESSAGE_ID_SET_ROOT_UNORDERED_ACCESS_VIEW_INVALID
	pub const SetRootUavInvalid   : Self = Self::SetRootUnorderedAccessViewInvalid;
	/// D3D12_MESSAGE_ID_GETCOPYABLELAYOUT_INVALIDSUBRESOURCERANGE = 0x2E3i32
	pub const GetCopyAbleLayoutInvalidSubresourceRange: Self = unsafe { transmute(0x2E3i32) };
	/// D3D12_MESSAGE_ID_GETCOPYABLELAYOUT_INVALIDBASEOFFSET = 0x2E4i32
	pub const GetCopyAbleLayoutInvalidBaseOffset: Self = unsafe { transmute(0x2E4i32) };
	/// D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_CONSTANT_BUFFER_VIEW_NOT_SET
	pub const CommandListRootCbvNotSet: Self = Self::CommandListRootConstantBufferViewNotSet;
	/// D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_SHADER_RESOURCE_VIEW_NOT_SET
	pub const CommandListRootSrvNotSet: Self = Self::CommandListRootShaderResourceViewNotSet;
	/// D3D12_MESSAGE_ID_COMMAND_LIST_ROOT_UNORDERED_ACCESS_VIEW_NOT_SET
	pub const CommandListRootUavNotSet: Self = Self::CommandListRootUnorderedAccessViewNotSet;
	/// D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INCOMPATIBLE_WITH_STRUCTURED_BUFFERS
	pub const ClearUavIncompatibleWithStructuredBuffers: Self = Self::ClearUnorderedAccessViewIncompatibleWithStructuredBuffers;
	/// D3D12_MESSAGE_ID_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INVALID
	pub const BuildRtAsInvalid    : Self = Self::BuildRaytracingAccelerationStructureInvalid;
	/// D3D12_MESSAGE_ID_EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_INVALID
	pub const EmitRtAsPostBuildInfoInvalid: Self = Self::EmitRaytracingAccelerationStructurePostBuildInfoInvalid;
	/// D3D12_MESSAGE_ID_COPY_RAYTRACING_ACCELERATION_STRUCTURE_INVALID
	pub const CopyRtAsInvalid     : Self = Self::CopyRaytracingAccelerationStructureInvalid;
	/// D3D12_MESSAGE_ID_GET_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO_INVALID
	pub const GetRtAsPrebuildInfoInvalid: Self = Self::GetRaytracingAccelerationStructurePrebuildInfoInvalid;
	/// D3D12_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_INVALID_RESOURCE_PTR
	pub const ClearUavInvalidResourcePtr: Self = Self::ClearUnorderedAccessViewInvalidResourcePtr;
	/// D3D12_MESSAGE_ID_CLEAR_UNORDERED_ACCESS_VIEW_INVALID_DESCRIPTOR_HANDLE
	pub const ClearUavInvalidDescriptorHandle: Self = Self::ClearUnorderedAccessViewInvalidDescriptorHandle;
}

/// D3D12_MESSAGE_CALLBACK_FLAGS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12MessageCallbackFlags
{
	/// D3D12_MESSAGE_CALLBACK_FLAG_NONE = 0x0i32
	None                 = 0x0i32,
	/// D3D12_MESSAGE_CALLBACK_IGNORE_FILTERS = 0x1i32
	IgnoreFilters        = 0x1i32,
}

/// D3D12_AXIS_SHADING_RATE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12AxisShadingRate
{
	/// D3D12_AXIS_SHADING_RATE_1X = 0x0i32
	_1X                  = 0x0i32,
	/// D3D12_AXIS_SHADING_RATE_2X = 0x1i32
	_2X                  = 0x1i32,
	/// D3D12_AXIS_SHADING_RATE_4X = 0x2i32
	_4X                  = 0x2i32,
}

/// D3D12_SHADING_RATE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ShadingRate
{
	/// D3D12_SHADING_RATE_1X1 = 0x0i32
	_1x1                 = 0x0i32,
	/// D3D12_SHADING_RATE_1X2 = 0x1i32
	_1x2                 = 0x1i32,
	/// D3D12_SHADING_RATE_2X1 = 0x4i32
	_2x1                 = 0x4i32,
	/// D3D12_SHADING_RATE_2X2 = 0x5i32
	_2x2                 = 0x5i32,
	/// D3D12_SHADING_RATE_2X4 = 0x6i32
	_2x4                 = 0x6i32,
	/// D3D12_SHADING_RATE_4X2 = 0x9i32
	_4x2                 = 0x9i32,
	/// D3D12_SHADING_RATE_4X4 = 0xAi32
	_4x4                 = 0xAi32,
}

/// D3D12_SHADING_RATE_COMBINER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ShadingRateCombiner
{
	/// D3D12_SHADING_RATE_COMBINER_PASSTHROUGH = 0x0i32
	PassThrough          = 0x0i32,
	/// D3D12_SHADING_RATE_COMBINER_OVERRIDE = 0x1i32
	Override             = 0x1i32,
	/// D3D12_SHADING_RATE_COMBINER_MIN = 0x2i32
	Min                  = 0x2i32,
	/// D3D12_SHADING_RATE_COMBINER_MAX = 0x3i32
	Max                  = 0x3i32,
	/// D3D12_SHADING_RATE_COMBINER_SUM = 0x4i32
	Sum                  = 0x4i32,
}

/// D3D12_SHADER_VERSION_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3D12ShaderVersionType
{
	/// D3D12_SHVER_PIXEL_SHADER = 0x0i32
	PixelShader          = 0x0i32,
	/// D3D12_SHVER_VERTEX_SHADER = 0x1i32
	VertexShader         = 0x1i32,
	/// D3D12_SHVER_GEOMETRY_SHADER = 0x2i32
	GeometryShader       = 0x2i32,
	/// D3D12_SHVER_HULL_SHADER = 0x3i32
	HullShader           = 0x3i32,
	/// D3D12_SHVER_DOMAIN_SHADER = 0x4i32
	DomainShader         = 0x4i32,
	/// D3D12_SHVER_COMPUTE_SHADER = 0x5i32
	ComputeShader        = 0x5i32,
	/// D3D12_SHVER_RESERVED0 = 0xFFF0i32
	Reserved0            = 0xFFF0i32,
}

