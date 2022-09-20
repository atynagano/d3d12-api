#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// DXGI_RESOURCE_PRIORITY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiResourcePriority
{
	/// DXGI_RESOURCE_PRIORITY_MINIMUM = 0x28000000u32
	Minimum              = 0x28000000u32,
	/// DXGI_RESOURCE_PRIORITY_LOW = 0x50000000u32
	Low                  = 0x50000000u32,
	/// DXGI_RESOURCE_PRIORITY_NORMAL = 0x78000000u32
	Normal               = 0x78000000u32,
	/// DXGI_RESOURCE_PRIORITY_HIGH = 0xA0000000u32
	High                 = 0xA0000000u32,
	/// DXGI_RESOURCE_PRIORITY_MAXIMUM = 0xC8000000u32
	Maximum              = 0xC8000000u32,
}

bitflags::bitflags! {
	/// 
	#[repr(transparent)]
	pub struct DxgiPresent: u32 {
		///  = 0x0u32
		const None                 = 0x0u32;
		/// DXGI_PRESENT_TEST = 0x1u32
		const Test                 = 0x1u32;
		/// DXGI_PRESENT_DO_NOT_SEQUENCE = 0x2u32
		const DoNotSequence        = 0x2u32;
		/// DXGI_PRESENT_RESTART = 0x4u32
		const Restart              = 0x4u32;
		/// DXGI_PRESENT_DO_NOT_WAIT = 0x8u32
		const DoNotWait            = 0x8u32;
		/// DXGI_PRESENT_STEREO_PREFER_RIGHT = 0x10u32
		const StereoPreferRight    = 0x10u32;
		/// DXGI_PRESENT_STEREO_TEMPORARY_MONO = 0x20u32
		const StereoTemporaryMono  = 0x20u32;
		/// DXGI_PRESENT_RESTRICT_TO_OUTPUT = 0x40u32
		const RestrictToOutput     = 0x40u32;
		/// DXGI_PRESENT_USE_DURATION = 0x100u32
		const UseDuration          = 0x100u32;
		/// DXGI_PRESENT_ALLOW_TEARING = 0x200u32
		const AllowTearing         = 0x200u32;
	}
}

bitflags::bitflags! {
	/// 
	#[repr(transparent)]
	pub struct DxgiUsage: u32 {
		/// DXGI_USAGE_SHADER_INPUT = 0x10u32
		const ShaderInput          = 0x10u32;
		/// DXGI_USAGE_RENDER_TARGET_OUTPUT = 0x20u32
		const RenderTargetOutput   = 0x20u32;
		/// DXGI_USAGE_BACK_BUFFER = 0x40u32
		const BackBuffer           = 0x40u32;
		/// DXGI_USAGE_SHARED = 0x80u32
		const Shared               = 0x80u32;
		/// DXGI_USAGE_READ_ONLY = 0x100u32
		const ReadOnly             = 0x100u32;
		/// DXGI_USAGE_DISCARD_ON_PRESENT = 0x200u32
		const DiscardOnPresent     = 0x200u32;
		/// DXGI_USAGE_UNORDERED_ACCESS = 0x400u32
		const UnorderedAccess      = 0x400u32;
	}
}

bitflags::bitflags! {
	/// 
	#[repr(transparent)]
	pub struct DxgiMwa: u32 {
		/// DXGI_MWA_NO_WINDOW_CHANGES = 0x1u32
		const NoWindowChanges      = 0x1u32;
		/// DXGI_MWA_NO_ALT_ENTER = 0x2u32
		const NoAltEnter           = 0x2u32;
		/// DXGI_MWA_NO_PRINT_SCREEN = 0x4u32
		const NoPrintScreen        = 0x4u32;
		/// DXGI_MWA_VALID = 0x7u32
		const Valid                = 0x7u32;
	}
}

/// DXGI_RESIDENCY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiResidency
{
	/// DXGI_RESIDENCY_FULLY_RESIDENT = 0x1i32
	FullyResident        = 0x1i32,
	/// DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY = 0x2i32
	ResidentInSharedMemory = 0x2i32,
	/// DXGI_RESIDENCY_EVICTED_TO_DISK = 0x3i32
	EvictedToDisk        = 0x3i32,
}

/// DXGI_SWAP_EFFECT
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiSwapEffect
{
	/// DXGI_SWAP_EFFECT_DISCARD = 0x0i32
	Discard              = 0x0i32,
	/// DXGI_SWAP_EFFECT_SEQUENTIAL = 0x1i32
	Sequential           = 0x1i32,
	/// DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL = 0x3i32
	FlipSequential       = 0x3i32,
	/// DXGI_SWAP_EFFECT_FLIP_DISCARD = 0x4i32
	FlipDiscard          = 0x4i32,
}

bitflags::bitflags! {
	/// DXGI_SWAP_CHAIN_FLAG
	#[repr(transparent)]
	pub struct DxgiSwapChainFlag: i32 {
		/// NONE = 0x0i32
		const None                 = 0x0i32;
		/// DXGI_SWAP_CHAIN_FLAG_NONPREROTATED = 0x1i32
		const NonPreRotated        = 0x1i32;
		/// DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH = 0x2i32
		const AllowModeSwitch      = 0x2i32;
		/// DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE = 0x4i32
		const GdiCompatible        = 0x4i32;
		/// DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT = 0x8i32
		const RestrictedContent    = 0x8i32;
		/// DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER = 0x10i32
		const RestrictSharedResourceDriver = 0x10i32;
		/// DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY = 0x20i32
		const DisplayOnly          = 0x20i32;
		/// DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT = 0x40i32
		const FrameLatencyWaitableObject = 0x40i32;
		/// DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER = 0x80i32
		const ForegroundLayer      = 0x80i32;
		/// DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO = 0x100i32
		const FullScreenVideo      = 0x100i32;
		/// DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO = 0x200i32
		const YuvVideo             = 0x200i32;
		/// DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED = 0x400i32
		const HwProtected          = 0x400i32;
		/// DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING = 0x800i32
		const AllowTearing         = 0x800i32;
		/// DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS = 0x1000i32
		const RestrictedToAllHolographicDisplays = 0x1000i32;
	}
}

bitflags::bitflags! {
	/// DXGI_ADAPTER_FLAG
	#[repr(transparent)]
	pub struct DxgiAdapterFlag: u32 {
		/// DXGI_ADAPTER_FLAG_NONE = 0x0u32
		const None                 = 0x0u32;
		/// DXGI_ADAPTER_FLAG_REMOTE = 0x1u32
		const Remote               = 0x1u32;
		/// DXGI_ADAPTER_FLAG_SOFTWARE = 0x2u32
		const Software             = 0x2u32;
	}
}

/// DXGI_OUTDUPL_POINTER_SHAPE_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiOutDuplPointerShapeType
{
	/// DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME = 0x1i32
	Monochrome           = 0x1i32,
	/// DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR = 0x2i32
	Color                = 0x2i32,
	/// DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR = 0x4i32
	MaskedColor          = 0x4i32,
}

/// DXGI_OFFER_RESOURCE_PRIORITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiOfferResourcePriority
{
	/// DXGI_OFFER_RESOURCE_PRIORITY_LOW = 0x1i32
	Low                  = 0x1i32,
	/// DXGI_OFFER_RESOURCE_PRIORITY_NORMAL = 0x2i32
	Normal               = 0x2i32,
	/// DXGI_OFFER_RESOURCE_PRIORITY_HIGH = 0x3i32
	High                 = 0x3i32,
}

/// DXGI_SCALING
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiScaling
{
	/// DXGI_SCALING_STRETCH = 0x0i32
	Stretch              = 0x0i32,
	/// DXGI_SCALING_NONE = 0x1i32
	None                 = 0x1i32,
	/// DXGI_SCALING_ASPECT_RATIO_STRETCH = 0x2i32
	AspectRatioStretch   = 0x2i32,
}

/// DXGI_GRAPHICS_PREEMPTION_GRANULARITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiGraphicsPreemptionGranularity
{
	/// DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY = 0x0i32
	DmaBufferBoundary    = 0x0i32,
	/// DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY = 0x1i32
	PrimitiveBoundary    = 0x1i32,
	/// DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY = 0x2i32
	TriangleBoundary     = 0x2i32,
	/// DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY = 0x3i32
	PixelBoundary        = 0x3i32,
	/// DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY = 0x4i32
	InstructionBoundary  = 0x4i32,
}

/// DXGI_COMPUTE_PREEMPTION_GRANULARITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiComputePreemptionGranularity
{
	/// DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY = 0x0i32
	DmaBufferBoundary    = 0x0i32,
	/// DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY = 0x1i32
	DispatchBoundary     = 0x1i32,
	/// DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY = 0x2i32
	ThreadGroupBoundary  = 0x2i32,
	/// DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY = 0x3i32
	ThreadBoundary       = 0x3i32,
	/// DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY = 0x4i32
	InstructionBoundary  = 0x4i32,
}

/// DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiMultiplaneOverlayYCbCrFlags
{
	/// DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE = 0x1i32
	NominalRange         = 0x1i32,
	/// DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709 = 0x2i32
	BT709                = 0x2i32,
	/// DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC = 0x4i32
	xvYCC                = 0x4i32,
}

/// DXGI_FRAME_PRESENTATION_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiFramePresentationMode
{
	/// DXGI_FRAME_PRESENTATION_MODE_COMPOSED = 0x0i32
	Composed             = 0x0i32,
	/// DXGI_FRAME_PRESENTATION_MODE_OVERLAY = 0x1i32
	Overlay              = 0x1i32,
	/// DXGI_FRAME_PRESENTATION_MODE_NONE = 0x2i32
	None                 = 0x2i32,
	/// DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE = 0x3i32
	CompositionFailure   = 0x3i32,
}

/// DXGI_OVERLAY_SUPPORT_FLAG
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiOverlaySupportFlag
{
	/// DXGI_OVERLAY_SUPPORT_FLAG_DIRECT = 0x1i32
	Direct               = 0x1i32,
	/// DXGI_OVERLAY_SUPPORT_FLAG_SCALING = 0x2i32
	Scaling              = 0x2i32,
}

/// DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiSwapChainColorSpaceSupportFlag
{
	/// DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x1i32
	Present              = 0x1i32,
	/// DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT = 0x2i32
	OverlayPresent       = 0x2i32,
}

/// DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiOverlayColorSpaceSupportFlag
{
	/// DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x1i32
	Present              = 0x1i32,
}

/// DXGI_MEMORY_SEGMENT_GROUP
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiMemorySegmentGroup
{
	/// DXGI_MEMORY_SEGMENT_GROUP_LOCAL = 0x0i32
	Local                = 0x0i32,
	/// DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL = 0x1i32
	NonLocal             = 0x1i32,
}

/// DXGI_OUTDUPL_FLAG
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiOutDuplFlag
{
	/// DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY = 0x1i32
	CompositedUiCaptureOnly = 0x1i32,
}

/// DXGI_HDR_METADATA_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiHdrMetaDataType
{
	/// DXGI_HDR_METADATA_TYPE_NONE = 0x0i32
	None                 = 0x0i32,
	/// DXGI_HDR_METADATA_TYPE_HDR10 = 0x1i32
	Hdr10                = 0x1i32,
	/// DXGI_HDR_METADATA_TYPE_HDR10PLUS = 0x2i32
	Hdr10Plus            = 0x2i32,
}

/// DXGI_OFFER_RESOURCE_FLAGS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiOfferResourceFlags
{
	/// DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT = 0x1i32
	AllowDecommit        = 0x1i32,
}

/// DXGI_RECLAIM_RESOURCE_RESULTS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiReclaimResourceResults
{
	/// DXGI_RECLAIM_RESOURCE_RESULT_OK = 0x0i32
	Ok                   = 0x0i32,
	/// DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED = 0x1i32
	Discarded            = 0x1i32,
	/// DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED = 0x2i32
	NotCommitted         = 0x2i32,
}

/// DXGI_FEATURE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiFeature
{
	/// DXGI_FEATURE_PRESENT_ALLOW_TEARING = 0x0i32
	PresentAllowTearing  = 0x0i32,
}

bitflags::bitflags! {
	/// DXGI_ADAPTER_FLAG3
	#[repr(transparent)]
	pub struct DxgiAdapterFlag3: u32 {
		/// DXGI_ADAPTER_FLAG3_NONE = 0x0u32
		const None                 = 0x0u32;
		/// DXGI_ADAPTER_FLAG3_REMOTE = 0x1u32
		const Remote               = 0x1u32;
		/// DXGI_ADAPTER_FLAG3_SOFTWARE = 0x2u32
		const Software             = 0x2u32;
		/// DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE = 0x4u32
		const AcgCompatible        = 0x4u32;
		/// DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES = 0x8u32
		const SupportMonitoredFences = 0x8u32;
		/// DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES = 0x10u32
		const SupportNonMonitoredFences = 0x10u32;
		/// DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE = 0x20u32
		const KeyedMutexConformance = 0x20u32;
		/// DXGI_ADAPTER_FLAG3_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS
	#[repr(transparent)]
	pub struct DxgiHardwareCompositionSupportFlags: u32 {
		/// DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN = 0x1u32
		const FullScreen           = 0x1u32;
		/// DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED = 0x2u32
		const Windowed             = 0x2u32;
		/// DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED = 0x4u32
		const CursorStretched      = 0x4u32;
	}
}

/// DXGI_GPU_PREFERENCE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiGpuPreference
{
	/// DXGI_GPU_PREFERENCE_UNSPECIFIED = 0x0i32
	Unspecified          = 0x0i32,
	/// DXGI_GPU_PREFERENCE_MINIMUM_POWER = 0x1i32
	MinimumPower         = 0x1i32,
	/// DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE = 0x2i32
	HighPerformance      = 0x2i32,
}

bitflags::bitflags! {
	/// DXGI_DEBUG_RLO_FLAGS
	#[repr(transparent)]
	pub struct DxgiDebugRloFlags: u32 {
		/// DXGI_DEBUG_RLO_SUMMARY = 0x1u32
		const Summary              = 0x1u32;
		/// DXGI_DEBUG_RLO_DETAIL = 0x2u32
		const Detail               = 0x2u32;
		/// DXGI_DEBUG_RLO_IGNORE_INTERNAL = 0x4u32
		const IgnoreInternal       = 0x4u32;
		/// DXGI_DEBUG_RLO_ALL = 0x7u32
		const All                  = 0x7u32;
	}
}

/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiInfoQueueMessageCategory
{
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS = 0x1i32
	Miscellaneous        = 0x1i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION = 0x2i32
	Initialization       = 0x2i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP = 0x3i32
	Cleanup              = 0x3i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION = 0x4i32
	Compilation          = 0x4i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION = 0x5i32
	StateCreation        = 0x5i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING = 0x6i32
	StateSetting         = 0x6i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING = 0x7i32
	StateGetting         = 0x7i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION = 0x8i32
	ResourceManipulation = 0x8i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION = 0x9i32
	Execution            = 0x9i32,
	/// DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER = 0xAi32
	Shader               = 0xAi32,
}

/// DXGI_INFO_QUEUE_MESSAGE_SEVERITY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiInfoQueueMessageSeverity
{
	/// DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION = 0x0i32
	Corruption           = 0x0i32,
	/// DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR = 0x1i32
	Error                = 0x1i32,
	/// DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING = 0x2i32
	Warning              = 0x2i32,
	/// DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO = 0x3i32
	Info                 = 0x3i32,
	/// DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE = 0x4i32
	Message              = 0x4i32,
}

/// DXGI_Message_Id
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiMessageId
{
	/// DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_InvalidOutputWindow = 0x0i32
	IDXGISwapChainCreationOrResizeBuffersInvalidOutputWindow = 0x0i32,
	/// DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferWidthInferred = 0x1i32
	IDXGISwapChainCreationOrResizeBuffersBufferWidthInferred = 0x1i32,
	/// DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferHeightInferred = 0x2i32
	IDXGISwapChainCreationOrResizeBuffersBufferHeightInferred = 0x2i32,
	/// DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_NoScanoutFlagChanged = 0x3i32
	IDXGISwapChainCreationOrResizeBuffersNoScanoutFlagChanged = 0x3i32,
	/// DXGI_MSG_IDXGISwapChain_Creation_MaxBufferCountExceeded = 0x4i32
	IDXGISwapChainCreationMaxBufferCountExceeded = 0x4i32,
	/// DXGI_MSG_IDXGISwapChain_Creation_TooFewBuffers = 0x5i32
	IDXGISwapChainCreationTooFewBuffers = 0x5i32,
	/// DXGI_MSG_IDXGISwapChain_Creation_NoOutputWindow = 0x6i32
	IDXGISwapChainCreationNoOutputWindow = 0x6i32,
	/// DXGI_MSG_IDXGISwapChain_Destruction_OtherMethodsCalled = 0x7i32
	IDXGISwapChainDestructionOtherMethodsCalled = 0x7i32,
	/// DXGI_MSG_IDXGISwapChain_GetDesc_pDescIsNULL = 0x8i32
	IDXGISwapChainGetDescpDescIsNULL = 0x8i32,
	/// DXGI_MSG_IDXGISwapChain_GetBuffer_ppSurfaceIsNULL = 0x9i32
	IDXGISwapChainGetBufferppSurfaceIsNULL = 0x9i32,
	/// DXGI_MSG_IDXGISwapChain_GetBuffer_NoAllocatedBuffers = 0xAi32
	IDXGISwapChainGetBufferNoAllocatedBuffers = 0xAi32,
	/// DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferMustBeZero = 0xBi32
	IDXGISwapChainGetBufferiBufferMustBeZero = 0xBi32,
	/// DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferOOB = 0xCi32
	IDXGISwapChainGetBufferiBufferOOB = 0xCi32,
	/// DXGI_MSG_IDXGISwapChain_GetContainingOutput_ppOutputIsNULL = 0xDi32
	IDXGISwapChainGetContainingOutputppOutputIsNULL = 0xDi32,
	/// DXGI_MSG_IDXGISwapChain_Present_SyncIntervalOOB = 0xEi32
	IDXGISwapChainPresentSyncIntervalOOB = 0xEi32,
	/// DXGI_MSG_IDXGISwapChain_Present_InvalidNonPreRotatedFlag = 0xFi32
	IDXGISwapChainPresentInvalidNonPreRotatedFlag = 0xFi32,
	/// DXGI_MSG_IDXGISwapChain_Present_NoAllocatedBuffers = 0x10i32
	IDXGISwapChainPresentNoAllocatedBuffers = 0x10i32,
	/// DXGI_MSG_IDXGISwapChain_Present_GetDXGIAdapterFailed = 0x11i32
	IDXGISwapChainPresentGetDXGIAdapterFailed = 0x11i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOB = 0x12i32
	IDXGISwapChainResizeBuffersBufferCountOOB = 0x12i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_UnreleasedReferences = 0x13i32
	IDXGISwapChainResizeBuffersUnreleasedReferences = 0x13i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidSwapChainFlag = 0x14i32
	IDXGISwapChainResizeBuffersInvalidSwapChainFlag = 0x14i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidNonPreRotatedFlag = 0x15i32
	IDXGISwapChainResizeBuffersInvalidNonPreRotatedFlag = 0x15i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeTarget_RefreshRateDivideByZero = 0x16i32
	IDXGISwapChainResizeTargetRefreshRateDivideByZero = 0x16i32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_InvalidTarget = 0x17i32
	IDXGISwapChainSetFullscreenStateInvalidTarget = 0x17i32,
	/// DXGI_MSG_IDXGISwapChain_GetFrameStatistics_pStatsIsNULL = 0x18i32
	IDXGISwapChainGetFrameStatisticspStatsIsNULL = 0x18i32,
	/// DXGI_MSG_IDXGISwapChain_GetLastPresentCount_pLastPresentCountIsNULL = 0x19i32
	IDXGISwapChainGetLastPresentCountpLastPresentCountIsNULL = 0x19i32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_RemoteNotSupported = 0x1Ai32
	IDXGISwapChainSetFullscreenStateRemoteNotSupported = 0x1Ai32,
	/// DXGI_MSG_IDXGIOutput_TakeOwnership_FailedToAcquireFullscreenMutex = 0x1Bi32
	IDXGIOutputTakeOwnershipFailedToAcquireFullscreenMutex = 0x1Bi32,
	/// DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ppAdapterInterfaceIsNULL = 0x1Ci32
	IDXGIFactoryCreateSoftwareAdapterppAdapterInterfaceIsNULL = 0x1Ci32,
	/// DXGI_MSG_IDXGIFactory_EnumAdapters_ppAdapterInterfaceIsNULL = 0x1Di32
	IDXGIFactoryEnumAdaptersppAdapterInterfaceIsNULL = 0x1Di32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_ppSwapChainIsNULL = 0x1Ei32
	IDXGIFactoryCreateSwapChainppSwapChainIsNULL = 0x1Ei32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_pDescIsNULL = 0x1Fi32
	IDXGIFactoryCreateSwapChainpDescIsNULL = 0x1Fi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_UnknownSwapEffect = 0x20i32
	IDXGIFactoryCreateSwapChainUnknownSwapEffect = 0x20i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFlags = 0x21i32
	IDXGIFactoryCreateSwapChainInvalidFlags = 0x21i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedFlagAndWindowed = 0x22i32
	IDXGIFactoryCreateSwapChainNonPreRotatedFlagAndWindowed = 0x22i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_NullDeviceInterface = 0x23i32
	IDXGIFactoryCreateSwapChainNullDeviceInterface = 0x23i32,
	/// DXGI_MSG_IDXGIFactory_GetWindowAssociation_phWndIsNULL = 0x24i32
	IDXGIFactoryGetWindowAssociationphWndIsNULL = 0x24i32,
	/// DXGI_MSG_IDXGIFactory_MakeWindowAssociation_InvalidFlags = 0x25i32
	IDXGIFactoryMakeWindowAssociationInvalidFlags = 0x25i32,
	/// DXGI_MSG_IDXGISurface_Map_InvalidSurface = 0x26i32
	IDXGISurfaceMapInvalidSurface = 0x26i32,
	/// DXGI_MSG_IDXGISurface_Map_FlagsSetToZero = 0x27i32
	IDXGISurfaceMapFlagsSetToZero = 0x27i32,
	/// DXGI_MSG_IDXGISurface_Map_DiscardAndReadFlagSet = 0x28i32
	IDXGISurfaceMapDiscardAndReadFlagSet = 0x28i32,
	/// DXGI_MSG_IDXGISurface_Map_DiscardButNotWriteFlagSet = 0x29i32
	IDXGISurfaceMapDiscardButNotWriteFlagSet = 0x29i32,
	/// DXGI_MSG_IDXGISurface_Map_NoCPUAccess = 0x2Ai32
	IDXGISurfaceMapNoCPUAccess = 0x2Ai32,
	/// DXGI_MSG_IDXGISurface_Map_ReadFlagSetButCPUAccessIsDynamic = 0x2Bi32
	IDXGISurfaceMapReadFlagSetButCPUAccessIsDynamic = 0x2Bi32,
	/// DXGI_MSG_IDXGISurface_Map_DiscardFlagSetButCPUAccessIsNotDynamic = 0x2Ci32
	IDXGISurfaceMapDiscardFlagSetButCPUAccessIsNotDynamic = 0x2Ci32,
	/// DXGI_MSG_IDXGIOutput_GetDisplayModeList_pNumModesIsNULL = 0x2Di32
	IDXGIOutputGetDisplayModeListpNumModesIsNULL = 0x2Di32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasInvalidWidthOrHeight = 0x2Ei32
	IDXGIOutputFindClosestMatchingModeModeHasInvalidWidthOrHeight = 0x2Ei32,
	/// DXGI_MSG_IDXGIOutput_GetCammaControlCapabilities_NoOwnerDevice = 0x2Fi32
	IDXGIOutputGetCammaControlCapabilitiesNoOwnerDevice = 0x2Fi32,
	/// DXGI_MSG_IDXGIOutput_TakeOwnership_pDeviceIsNULL = 0x30i32
	IDXGIOutputTakeOwnershippDeviceIsNULL = 0x30i32,
	/// DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_NoOwnerDevice = 0x31i32
	IDXGIOutputGetDisplaySurfaceDataNoOwnerDevice = 0x31i32,
	/// DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_pDestinationIsNULL = 0x32i32
	IDXGIOutputGetDisplaySurfaceDatapDestinationIsNULL = 0x32i32,
	/// DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_MapOfDestinationFailed = 0x33i32
	IDXGIOutputGetDisplaySurfaceDataMapOfDestinationFailed = 0x33i32,
	/// DXGI_MSG_IDXGIOutput_GetFrameStatistics_NoOwnerDevice = 0x34i32
	IDXGIOutputGetFrameStatisticsNoOwnerDevice = 0x34i32,
	/// DXGI_MSG_IDXGIOutput_GetFrameStatistics_pStatsIsNULL = 0x35i32
	IDXGIOutputGetFrameStatisticspStatsIsNULL = 0x35i32,
	/// DXGI_MSG_IDXGIOutput_SetGammaControl_NoOwnerDevice = 0x36i32
	IDXGIOutputSetGammaControlNoOwnerDevice = 0x36i32,
	/// DXGI_MSG_IDXGIOutput_GetGammaControl_NoOwnerDevice = 0x37i32
	IDXGIOutputGetGammaControlNoOwnerDevice = 0x37i32,
	/// DXGI_MSG_IDXGIOutput_GetGammaControl_NoGammaControls = 0x38i32
	IDXGIOutputGetGammaControlNoGammaControls = 0x38i32,
	/// DXGI_MSG_IDXGIOutput_SetDisplaySurface_IDXGIResourceNotSupportedBypPrimary = 0x39i32
	IDXGIOutputSetDisplaySurfaceIDXGIResourceNotSupportedBypPrimary = 0x39i32,
	/// DXGI_MSG_IDXGIOutput_SetDisplaySurface_pPrimaryIsInvalid = 0x3Ai32
	IDXGIOutputSetDisplaySurfacepPrimaryIsInvalid = 0x3Ai32,
	/// DXGI_MSG_IDXGIOutput_SetDisplaySurface_NoOwnerDevice = 0x3Bi32
	IDXGIOutputSetDisplaySurfaceNoOwnerDevice = 0x3Bi32,
	/// DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteDeviceNotSupported = 0x3Ci32
	IDXGIOutputTakeOwnershipRemoteDeviceNotSupported = 0x3Ci32,
	/// DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteDeviceNotSupported = 0x3Di32
	IDXGIOutputGetDisplayModeListRemoteDeviceNotSupported = 0x3Di32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteDeviceNotSupported = 0x3Ei32
	IDXGIOutputFindClosestMatchingModeRemoteDeviceNotSupported = 0x3Ei32,
	/// DXGI_MSG_IDXGIDevice_CreateSurface_InvalidParametersWithpSharedResource = 0x3Fi32
	IDXGIDeviceCreateSurfaceInvalidParametersWithpSharedResource = 0x3Fi32,
	/// DXGI_MSG_IDXGIObject_GetPrivateData_puiDataSizeIsNULL = 0x40i32
	IDXGIObjectGetPrivateDatapuiDataSizeIsNULL = 0x40i32,
	/// DXGI_MSG_IDXGISwapChain_Creation_InvalidOutputWindow = 0x41i32
	IDXGISwapChainCreationInvalidOutputWindow = 0x41i32,
	/// DXGI_MSG_IDXGISwapChain_Release_SwapChainIsFullscreen = 0x42i32
	IDXGISwapChainReleaseSwapChainIsFullscreen = 0x42i32,
	/// DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_InvalidTargetSurfaceFormat = 0x43i32
	IDXGIOutputGetDisplaySurfaceDataInvalidTargetSurfaceFormat = 0x43i32,
	/// DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ModuleIsNULL = 0x44i32
	IDXGIFactoryCreateSoftwareAdapterModuleIsNULL = 0x44i32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_IDXGIDeviceNotSupportedBypConcernedDevice = 0x45i32
	IDXGIOutputFindClosestMatchingModeIDXGIDeviceNotSupportedBypConcernedDevice = 0x45i32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_pModeToMatchOrpClosestMatchIsNULL = 0x46i32
	IDXGIOutputFindClosestMatchingModepModeToMatchOrpClosestMatchIsNULL = 0x46i32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasRefreshRateDenominatorZero = 0x47i32
	IDXGIOutputFindClosestMatchingModeModeHasRefreshRateDenominatorZero = 0x47i32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_UnknownFormatIsInvalidForConfiguration = 0x48i32
	IDXGIOutputFindClosestMatchingModeUnknownFormatIsInvalidForConfiguration = 0x48i32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScanlineOrdering = 0x49i32
	IDXGIOutputFindClosestMatchingModeInvalidDisplayModeScanlineOrdering = 0x49i32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScaling = 0x4Ai32
	IDXGIOutputFindClosestMatchingModeInvalidDisplayModeScaling = 0x4Ai32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeFormatAndDeviceCombination = 0x4Bi32
	IDXGIOutputFindClosestMatchingModeInvalidDisplayModeFormatAndDeviceCombination = 0x4Bi32,
	/// DXGI_MSG_IDXGIFactory_Creation_CalledFromDllMain = 0x4Ci32
	IDXGIFactoryCreationCalledFromDllMain = 0x4Ci32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_OutputNotOwnedBySwapChainDevice = 0x4Di32
	IDXGISwapChainSetFullscreenStateOutputNotOwnedBySwapChainDevice = 0x4Di32,
	/// DXGI_MSG_IDXGISwapChain_Creation_InvalidWindowStyle = 0x4Ei32
	IDXGISwapChainCreationInvalidWindowStyle = 0x4Ei32,
	/// DXGI_MSG_IDXGISwapChain_GetFrameStatistics_UnsupportedStatistics = 0x4Fi32
	IDXGISwapChainGetFrameStatisticsUnsupportedStatistics = 0x4Fi32,
	/// DXGI_MSG_IDXGISwapChain_GetContainingOutput_SwapchainAdapterDoesNotControlOutput = 0x50i32
	IDXGISwapChainGetContainingOutputSwapchainAdapterDoesNotControlOutput = 0x50i32,
	/// DXGI_MSG_IDXGIOutput_SetOrGetGammaControl_pArrayIsNULL = 0x51i32
	IDXGIOutputSetOrGetGammaControlpArrayIsNULL = 0x51i32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_FullscreenInvalidForChildWindows = 0x52i32
	IDXGISwapChainSetFullscreenStateFullscreenInvalidForChildWindows = 0x52i32,
	/// DXGI_MSG_IDXGIFactory_Release_CalledFromDllMain = 0x53i32
	IDXGIFactoryReleaseCalledFromDllMain = 0x53i32,
	/// DXGI_MSG_IDXGISwapChain_Present_UnreleasedHDC = 0x54i32
	IDXGISwapChainPresentUnreleasedHDC = 0x54i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_NonPreRotatedAndGDICompatibleFlags = 0x55i32
	IDXGISwapChainResizeBuffersNonPreRotatedAndGDICompatibleFlags = 0x55i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedAndGDICompatibleFlags = 0x56i32
	IDXGIFactoryCreateSwapChainNonPreRotatedAndGDICompatibleFlags = 0x56i32,
	/// DXGI_MSG_IDXGISurface1_GetDC_pHdcIsNULL = 0x57i32
	IDXGISurface1GetDCpHdcIsNULL = 0x57i32,
	/// DXGI_MSG_IDXGISurface1_GetDC_SurfaceNotTexture2D = 0x58i32
	IDXGISurface1GetDCSurfaceNotTexture2D = 0x58i32,
	/// DXGI_MSG_IDXGISurface1_GetDC_GDICompatibleFlagNotSet = 0x59i32
	IDXGISurface1GetDCGDICompatibleFlagNotSet = 0x59i32,
	/// DXGI_MSG_IDXGISurface1_GetDC_UnreleasedHDC = 0x5Ai32
	IDXGISurface1GetDCUnreleasedHDC = 0x5Ai32,
	/// DXGI_MSG_IDXGISurface_Map_NoCPUAccess2 = 0x5Bi32
	IDXGISurfaceMapNoCPUAccess2 = 0x5Bi32,
	/// DXGI_MSG_IDXGISurface1_ReleaseDC_GetDCNotCalled = 0x5Ci32
	IDXGISurface1ReleaseDCGetDCNotCalled = 0x5Ci32,
	/// DXGI_MSG_IDXGISurface1_ReleaseDC_InvalidRectangleDimensions = 0x5Di32
	IDXGISurface1ReleaseDCInvalidRectangleDimensions = 0x5Di32,
	/// DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteOutputNotSupported = 0x5Ei32
	IDXGIOutputTakeOwnershipRemoteOutputNotSupported = 0x5Ei32,
	/// DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteOutputNotSupported = 0x5Fi32
	IDXGIOutputFindClosestMatchingModeRemoteOutputNotSupported = 0x5Fi32,
	/// DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteOutputNotSupported = 0x60i32
	IDXGIOutputGetDisplayModeListRemoteOutputNotSupported = 0x60i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_pDeviceHasMismatchedDXGIFactory = 0x61i32
	IDXGIFactoryCreateSwapChainpDeviceHasMismatchedDXGIFactory = 0x61i32,
	/// DXGI_MSG_IDXGISwapChain_Present_NonOptimalFSConfiguration = 0x62i32
	IDXGISwapChainPresentNonOptimalFSConfiguration = 0x62i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSequentialNotSupportedOnD3D10 = 0x63i32
	IDXGIFactoryCreateSwapChainFlipSequentialNotSupportedOnD3D10 = 0x63i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_BufferCountOOBForFlipSequential = 0x64i32
	IDXGIFactoryCreateSwapChainBufferCountOOBForFlipSequential = 0x64i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFormatForFlipSequential = 0x65i32
	IDXGIFactoryCreateSwapChainInvalidFormatForFlipSequential = 0x65i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_MultiSamplingNotSupportedForFlipSequential = 0x66i32
	IDXGIFactoryCreateSwapChainMultiSamplingNotSupportedForFlipSequential = 0x66i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOBForFlipSequential = 0x67i32
	IDXGISwapChainResizeBuffersBufferCountOOBForFlipSequential = 0x67i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidFormatForFlipSequential = 0x68i32
	IDXGISwapChainResizeBuffersInvalidFormatForFlipSequential = 0x68i32,
	/// DXGI_MSG_IDXGISwapChain_Present_PartialPresentationBeforeStandardPresentation = 0x69i32
	IDXGISwapChainPresentPartialPresentationBeforeStandardPresentation = 0x69i32,
	/// DXGI_MSG_IDXGISwapChain_Present_FullscreenPartialPresentIsInvalid = 0x6Ai32
	IDXGISwapChainPresentFullscreenPartialPresentIsInvalid = 0x6Ai32,
	/// DXGI_MSG_IDXGISwapChain_Present_InvalidPresentTestOrDoNotSequenceFlag = 0x6Bi32
	IDXGISwapChainPresentInvalidPresentTestOrDoNotSequenceFlag = 0x6Bi32,
	/// DXGI_MSG_IDXGISwapChain_Present_ScrollInfoWithNoDirtyRectsSpecified = 0x6Ci32
	IDXGISwapChainPresentScrollInfoWithNoDirtyRectsSpecified = 0x6Ci32,
	/// DXGI_MSG_IDXGISwapChain_Present_EmptyScrollRect = 0x6Di32
	IDXGISwapChainPresentEmptyScrollRect = 0x6Di32,
	/// DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBounds = 0x6Ei32
	IDXGISwapChainPresentScrollRectOutOfBackbufferBounds = 0x6Ei32,
	/// DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBoundsWithOffset = 0x6Fi32
	IDXGISwapChainPresentScrollRectOutOfBackbufferBoundsWithOffset = 0x6Fi32,
	/// DXGI_MSG_IDXGISwapChain_Present_EmptyDirtyRect = 0x70i32
	IDXGISwapChainPresentEmptyDirtyRect = 0x70i32,
	/// DXGI_MSG_IDXGISwapChain_Present_DirtyRectOutOfBackbufferBounds = 0x71i32
	IDXGISwapChainPresentDirtyRectOutOfBackbufferBounds = 0x71i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_UnsupportedBufferUsageFlags = 0x72i32
	IDXGIFactoryCreateSwapChainUnsupportedBufferUsageFlags = 0x72i32,
	/// DXGI_MSG_IDXGISwapChain_Present_DoNotSequenceFlagSetButPreviousBufferIsUndefined = 0x73i32
	IDXGISwapChainPresentDoNotSequenceFlagSetButPreviousBufferIsUndefined = 0x73i32,
	/// DXGI_MSG_IDXGISwapChain_Present_UnsupportedFlags = 0x74i32
	IDXGISwapChainPresentUnsupportedFlags = 0x74i32,
	/// DXGI_MSG_IDXGISwapChain_Present_FlipModelChainMustResizeOrCreateOnFSTransition = 0x75i32
	IDXGISwapChainPresentFlipModelChainMustResizeOrCreateOnFSTransition = 0x75i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_pRestrictToOutputFromOtherIDXGIFactory = 0x76i32
	IDXGIFactoryCreateSwapChainpRestrictToOutputFromOtherIDXGIFactory = 0x76i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictOutputNotSupportedOnAdapter = 0x77i32
	IDXGIFactoryCreateSwapChainRestrictOutputNotSupportedOnAdapter = 0x77i32,
	/// DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagSetButInvalidpRestrictToOutput = 0x78i32
	IDXGISwapChainPresentRestrictToOutputFlagSetButInvalidpRestrictToOutput = 0x78i32,
	/// DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagdWithFullscreen = 0x79i32
	IDXGISwapChainPresentRestrictToOutputFlagdWithFullscreen = 0x79i32,
	/// DXGI_MSG_IDXGISwapChain_Present_RestrictOutputFlagWithStaleSwapChain = 0x7Ai32
	IDXGISwapChainPresentRestrictOutputFlagWithStaleSwapChain = 0x7Ai32,
	/// DXGI_MSG_IDXGISwapChain_Present_OtherFlagsCausingInvalidPresentTestFlag = 0x7Bi32
	IDXGISwapChainPresentOtherFlagsCausingInvalidPresentTestFlag = 0x7Bi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_UnavailableInSession0 = 0x7Ci32
	IDXGIFactoryCreateSwapChainUnavailableInSession0 = 0x7Ci32,
	/// DXGI_MSG_IDXGIFactory_MakeWindowAssociation_UnavailableInSession0 = 0x7Di32
	IDXGIFactoryMakeWindowAssociationUnavailableInSession0 = 0x7Di32,
	/// DXGI_MSG_IDXGIFactory_GetWindowAssociation_UnavailableInSession0 = 0x7Ei32
	IDXGIFactoryGetWindowAssociationUnavailableInSession0 = 0x7Ei32,
	/// DXGI_MSG_IDXGIAdapter_EnumOutputs_UnavailableInSession0 = 0x7Fi32
	IDXGIAdapterEnumOutputsUnavailableInSession0 = 0x7Fi32,
	/// DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_StereoDisabled = 0x80i32
	IDXGISwapChainCreationOrSetFullscreenStateStereoDisabled = 0x80i32,
	/// DXGI_MSG_IDXGIFactory2_UnregisterStatus_CookieNotFound = 0x81i32
	IDXGIFactory2UnregisterStatusCookieNotFound = 0x81i32,
	/// DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFSOrOverlay = 0x82i32
	IDXGISwapChainPresentProtectedContentInWindowedModeWithoutFSOrOverlay = 0x82i32,
	/// DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFlipSequential = 0x83i32
	IDXGISwapChainPresentProtectedContentInWindowedModeWithoutFlipSequential = 0x83i32,
	/// DXGI_MSG_IDXGISwapChain_Present_ProtectedContentWithRDPDriver = 0x84i32
	IDXGISwapChainPresentProtectedContentWithRDPDriver = 0x84i32,
	/// DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity = 0x85i32
	IDXGISwapChainPresentProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity = 0x85i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_WidthOrHeightIsZero = 0x86i32
	IDXGIFactoryCreateSwapChainForCompositionWidthOrHeightIsZero = 0x86i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_OnlyFlipSequentialSupported = 0x87i32
	IDXGIFactoryCreateSwapChainForCompositionOnlyFlipSequentialSupported = 0x87i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnAdapter = 0x88i32
	IDXGIFactoryCreateSwapChainForCompositionUnsupportedOnAdapter = 0x88i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnWindows7 = 0x89i32
	IDXGIFactoryCreateSwapChainForCompositionUnsupportedOnWindows7 = 0x89i32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSTransitionWithCompositionSwapChain = 0x8Ai32
	IDXGISwapChainSetFullscreenStateFSTransitionWithCompositionSwapChain = 0x8Ai32,
	/// DXGI_MSG_IDXGISwapChain_ResizeTarget_InvalidWithCompositionSwapChain = 0x8Bi32
	IDXGISwapChainResizeTargetInvalidWithCompositionSwapChain = 0x8Bi32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_WidthOrHeightIsZero = 0x8Ci32
	IDXGISwapChainResizeBuffersWidthOrHeightIsZero = 0x8Ci32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneIsFlipModelOnly = 0x8Di32
	IDXGIFactoryCreateSwapChainScalingNoneIsFlipModelOnly = 0x8Di32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingUnrecognized = 0x8Ei32
	IDXGIFactoryCreateSwapChainScalingUnrecognized = 0x8Ei32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyFullscreenUnsupported = 0x8Fi32
	IDXGIFactoryCreateSwapChainDisplayOnlyFullscreenUnsupported = 0x8Fi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyUnsupported = 0x90i32
	IDXGIFactoryCreateSwapChainDisplayOnlyUnsupported = 0x90i32,
	/// DXGI_MSG_IDXGISwapChain_Present_RestartIsFullscreenOnly = 0x91i32
	IDXGISwapChainPresentRestartIsFullscreenOnly = 0x91i32,
	/// DXGI_MSG_IDXGISwapChain_Present_ProtectedWindowlessPresentationRequiresDisplayOnly = 0x92i32
	IDXGISwapChainPresentProtectedWindowlessPresentationRequiresDisplayOnly = 0x92i32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_DisplayOnlyUnsupported = 0x93i32
	IDXGISwapChainSetFullscreenStateDisplayOnlyUnsupported = 0x93i32,
	/// DXGI_MSG_IDXGISwapChain1_SetBackgroundColor_OutOfRange = 0x94i32
	IDXGISwapChain1SetBackgroundColorOutOfRange = 0x94i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyFullscreenUnsupported = 0x95i32
	IDXGISwapChainResizeBuffersDisplayOnlyFullscreenUnsupported = 0x95i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyUnsupported = 0x96i32
	IDXGISwapChainResizeBuffersDisplayOnlyUnsupported = 0x96i32,
	/// DXGI_MSG_IDXGISwapchain_Present_ScrollUnsupported = 0x97i32
	IDXGISwapchainPresentScrollUnsupported = 0x97i32,
	/// DXGI_MSG_IDXGISwapChain1_SetRotation_UnsupportedOS = 0x98i32
	IDXGISwapChain1SetRotationUnsupportedOS = 0x98i32,
	/// DXGI_MSG_IDXGISwapChain1_GetRotation_UnsupportedOS = 0x99i32
	IDXGISwapChain1GetRotationUnsupportedOS = 0x99i32,
	/// DXGI_MSG_IDXGISwapchain_Present_FullscreenRotation = 0x9Ai32
	IDXGISwapchainPresentFullscreenRotation = 0x9Ai32,
	/// DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithMSAABuffers = 0x9Bi32
	IDXGISwapChainPresentPartialPresentationWithMSAABuffers = 0x9Bi32,
	/// DXGI_MSG_IDXGISwapChain1_SetRotation_FlipSequentialRequired = 0x9Ci32
	IDXGISwapChain1SetRotationFlipSequentialRequired = 0x9Ci32,
	/// DXGI_MSG_IDXGISwapChain1_SetRotation_InvalidRotation = 0x9Di32
	IDXGISwapChain1SetRotationInvalidRotation = 0x9Di32,
	/// DXGI_MSG_IDXGISwapChain1_GetRotation_FlipSequentialRequired = 0x9Ei32
	IDXGISwapChain1GetRotationFlipSequentialRequired = 0x9Ei32,
	/// DXGI_MSG_IDXGISwapChain_GetHwnd_WrongType = 0x9Fi32
	IDXGISwapChainGetHwndWrongType = 0x9Fi32,
	/// DXGI_MSG_IDXGISwapChain_GetCompositionSurface_WrongType = 0xA0i32
	IDXGISwapChainGetCompositionSurfaceWrongType = 0xA0i32,
	/// DXGI_MSG_IDXGISwapChain_GetCoreWindow_WrongType = 0xA1i32
	IDXGISwapChainGetCoreWindowWrongType = 0xA1i32,
	/// DXGI_MSG_IDXGISwapChain_GetFullscreenDesc_NonHwnd = 0xA2i32
	IDXGISwapChainGetFullscreenDescNonHwnd = 0xA2i32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_CoreWindow = 0xA3i32
	IDXGISwapChainSetFullscreenStateCoreWindow = 0xA3i32,
	/// DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_UnsupportedOnWindows7 = 0xA4i32
	IDXGIFactory2CreateSwapChainForCoreWindowUnsupportedOnWindows7 = 0xA4i32,
	/// DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsNULL = 0xA5i32
	IDXGIFactory2CreateSwapChainForCoreWindowpWindowIsNULL = 0xA5i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_FSUnsupportedForModernApps = 0xA6i32
	IDXGIFactoryCreateSwapChainFSUnsupportedForModernApps = 0xA6i32,
	/// DXGI_MSG_IDXGIFactory_MakeWindowAssociation_ModernApp = 0xA7i32
	IDXGIFactoryMakeWindowAssociationModernApp = 0xA7i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeTarget_ModernApp = 0xA8i32
	IDXGISwapChainResizeTargetModernApp = 0xA8i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeTarget_pNewTargetParametersIsNULL = 0xA9i32
	IDXGISwapChainResizeTargetpNewTargetParametersIsNULL = 0xA9i32,
	/// DXGI_MSG_IDXGIOutput_SetDisplaySurface_ModernApp = 0xAAi32
	IDXGIOutputSetDisplaySurfaceModernApp = 0xAAi32,
	/// DXGI_MSG_IDXGIOutput_TakeOwnership_ModernApp = 0xABi32
	IDXGIOutputTakeOwnershipModernApp = 0xABi32,
	/// DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsInvalid = 0xACi32
	IDXGIFactory2CreateSwapChainForCoreWindowpWindowIsInvalid = 0xACi32,
	/// DXGI_MSG_IDXGIFactory2_CreateSwapChainForCompositionSurface_InvalidHandle = 0xADi32
	IDXGIFactory2CreateSwapChainForCompositionSurfaceInvalidHandle = 0xADi32,
	/// DXGI_MSG_IDXGISurface1_GetDC_ModernApp = 0xAEi32
	IDXGISurface1GetDCModernApp = 0xAEi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneRequiresWindows8OrNewer = 0xAFi32
	IDXGIFactoryCreateSwapChainScalingNoneRequiresWindows8OrNewer = 0xAFi32,
	/// DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoAndPreferRight = 0xB0i32
	IDXGISwapChainPresentTemporaryMonoAndPreferRight = 0xB0i32,
	/// DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithDoNotSequence = 0xB1i32
	IDXGISwapChainPresentTemporaryMonoOrPreferRightWithDoNotSequence = 0xB1i32,
	/// DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithoutStereo = 0xB2i32
	IDXGISwapChainPresentTemporaryMonoOrPreferRightWithoutStereo = 0xB2i32,
	/// DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoUnsupported = 0xB3i32
	IDXGISwapChainPresentTemporaryMonoUnsupported = 0xB3i32,
	/// DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_ArraySizeMismatch = 0xB4i32
	IDXGIOutputGetDisplaySurfaceDataArraySizeMismatch = 0xB4i32,
	/// DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithSwapEffectDiscard = 0xB5i32
	IDXGISwapChainPresentPartialPresentationWithSwapEffectDiscard = 0xB5i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaUnrecognized = 0xB6i32
	IDXGIFactoryCreateSwapChainAlphaUnrecognized = 0xB6i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsWindowlessOnly = 0xB7i32
	IDXGIFactoryCreateSwapChainAlphaIsWindowlessOnly = 0xB7i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsFlipModelOnly = 0xB8i32
	IDXGIFactoryCreateSwapChainAlphaIsFlipModelOnly = 0xB8i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictToOutputAdapterMismatch = 0xB9i32
	IDXGIFactoryCreateSwapChainRestrictToOutputAdapterMismatch = 0xB9i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyOnLegacy = 0xBAi32
	IDXGIFactoryCreateSwapChainDisplayOnlyOnLegacy = 0xBAi32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyOnLegacy = 0xBBi32
	IDXGISwapChainResizeBuffersDisplayOnlyOnLegacy = 0xBBi32,
	/// DXGI_MSG_IDXGIResource1_CreateSubresourceSurface_InvalidIndex = 0xBCi32
	IDXGIResource1CreateSubresourceSurfaceInvalidIndex = 0xBCi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidScaling = 0xBDi32
	IDXGIFactoryCreateSwapChainForCompositionInvalidScaling = 0xBDi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainForCoreWindow_InvalidSwapEffect = 0xBEi32
	IDXGIFactoryCreateSwapChainForCoreWindowInvalidSwapEffect = 0xBEi32,
	/// DXGI_MSG_IDXGIResource1_CreateSharedHandle_UnsupportedOS = 0xBFi32
	IDXGIResource1CreateSharedHandleUnsupportedOS = 0xBFi32,
	/// DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusWindow_UnsupportedOS = 0xC0i32
	IDXGIFactory2RegisterOcclusionStatusWindowUnsupportedOS = 0xC0i32,
	/// DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusEvent_UnsupportedOS = 0xC1i32
	IDXGIFactory2RegisterOcclusionStatusEventUnsupportedOS = 0xC1i32,
	/// DXGI_MSG_IDXGIOutput1_DuplicateOutput_UnsupportedOS = 0xC2i32
	IDXGIOutput1DuplicateOutputUnsupportedOS = 0xC2i32,
	/// DXGI_MSG_IDXGIDisplayControl_IsStereoEnabled_UnsupportedOS = 0xC3i32
	IDXGIDisplayControlIsStereoEnabledUnsupportedOS = 0xC3i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidAlphaMode = 0xC4i32
	IDXGIFactoryCreateSwapChainForCompositionInvalidAlphaMode = 0xC4i32,
	/// DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidResource = 0xC5i32
	IDXGIFactoryGetSharedResourceAdapterLuidInvalidResource = 0xC5i32,
	/// DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidLUID = 0xC6i32
	IDXGIFactoryGetSharedResourceAdapterLuidInvalidLUID = 0xC6i32,
	/// DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_UnsupportedOS = 0xC7i32
	IDXGIFactoryGetSharedResourceAdapterLuidUnsupportedOS = 0xC7i32,
	/// DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_2DOnly = 0xC8i32
	IDXGIOutput1GetDisplaySurfaceData1_2DOnly = 0xC8i32,
	/// DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_StagingOnly = 0xC9i32
	IDXGIOutput1GetDisplaySurfaceData1StagingOnly = 0xC9i32,
	/// DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NeedCPUAccessWrite = 0xCAi32
	IDXGIOutput1GetDisplaySurfaceData1NeedCPUAccessWrite = 0xCAi32,
	/// DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NoShared = 0xCBi32
	IDXGIOutput1GetDisplaySurfaceData1NoShared = 0xCBi32,
	/// DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_OnlyMipLevels1 = 0xCCi32
	IDXGIOutput1GetDisplaySurfaceData1OnlyMipLevels1 = 0xCCi32,
	/// DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_MappedOrOfferedResource = 0xCDi32
	IDXGIOutput1GetDisplaySurfaceData1MappedOrOfferedResource = 0xCDi32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSUnsupportedForModernApps = 0xCEi32
	IDXGISwapChainSetFullscreenStateFSUnsupportedForModernApps = 0xCEi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_FailedToGoFSButNonPreRotated = 0xCFi32
	IDXGIFactoryCreateSwapChainFailedToGoFSButNonPreRotated = 0xCFi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainOrRegisterOcclusionStatus_BlitModelUsedWhileRegisteredForOcclusionStatusEvents = 0xD0i32
	IDXGIFactoryCreateSwapChainOrRegisterOcclusionStatusBlitModelUsedWhileRegisteredForOcclusionStatusEvents = 0xD0i32,
	/// DXGI_MSG_IDXGISwapChain_Present_BlitModelUsedWhileRegisteredForOcclusionStatusEvents = 0xD1i32
	IDXGISwapChainPresentBlitModelUsedWhileRegisteredForOcclusionStatusEvents = 0xD1i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreFlipModelOnly = 0xD2i32
	IDXGIFactoryCreateSwapChainWaitableSwapChainsAreFlipModelOnly = 0xD2i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreNotFullscreen = 0xD3i32
	IDXGIFactoryCreateSwapChainWaitableSwapChainsAreNotFullscreen = 0xD3i32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_Waitable = 0xD4i32
	IDXGISwapChainSetFullscreenStateWaitable = 0xD4i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveWaitableFlag = 0xD5i32
	IDXGISwapChainResizeBuffersCannotAddOrRemoveWaitableFlag = 0xD5i32,
	/// DXGI_MSG_IDXGISwapChain_GetFrameLatencyWaitableObject_OnlyWaitable = 0xD6i32
	IDXGISwapChainGetFrameLatencyWaitableObjectOnlyWaitable = 0xD6i32,
	/// DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_OnlyWaitable = 0xD7i32
	IDXGISwapChainGetMaximumFrameLatencyOnlyWaitable = 0xD7i32,
	/// DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_pMaxLatencyIsNULL = 0xD8i32
	IDXGISwapChainGetMaximumFrameLatencypMaxLatencyIsNULL = 0xD8i32,
	/// DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_OnlyWaitable = 0xD9i32
	IDXGISwapChainSetMaximumFrameLatencyOnlyWaitable = 0xD9i32,
	/// DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_MaxLatencyIsOutOfBounds = 0xDAi32
	IDXGISwapChainSetMaximumFrameLatencyMaxLatencyIsOutOfBounds = 0xDAi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_ForegroundIsCoreWindowOnly = 0xDBi32
	IDXGIFactoryCreateSwapChainForegroundIsCoreWindowOnly = 0xDBi32,
	/// DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_ForegroundUnsupportedOnAdapter = 0xDCi32
	IDXGIFactory2CreateSwapChainForCoreWindowForegroundUnsupportedOnAdapter = 0xDCi32,
	/// DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidScaling = 0xDDi32
	IDXGIFactory2CreateSwapChainForCoreWindowInvalidScaling = 0xDDi32,
	/// DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidAlphaMode = 0xDEi32
	IDXGIFactory2CreateSwapChainForCoreWindowInvalidAlphaMode = 0xDEi32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveForegroundFlag = 0xDFi32
	IDXGISwapChainResizeBuffersCannotAddOrRemoveForegroundFlag = 0xDFi32,
	/// DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixPointerCannotBeNull = 0xE0i32
	IDXGISwapChainSetMatrixTransformMatrixPointerCannotBeNull = 0xE0i32,
	/// DXGI_MSG_IDXGISwapChain_SetMatrixTransform_RequiresCompositionSwapChain = 0xE1i32
	IDXGISwapChainSetMatrixTransformRequiresCompositionSwapChain = 0xE1i32,
	/// DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeFinite = 0xE2i32
	IDXGISwapChainSetMatrixTransformMatrixMustBeFinite = 0xE2i32,
	/// DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeTranslateAndOrScale = 0xE3i32
	IDXGISwapChainSetMatrixTransformMatrixMustBeTranslateAndOrScale = 0xE3i32,
	/// DXGI_MSG_IDXGISwapChain_GetMatrixTransform_MatrixPointerCannotBeNull = 0xE4i32
	IDXGISwapChainGetMatrixTransformMatrixPointerCannotBeNull = 0xE4i32,
	/// DXGI_MSG_IDXGISwapChain_GetMatrixTransform_RequiresCompositionSwapChain = 0xE5i32
	IDXGISwapChainGetMatrixTransformRequiresCompositionSwapChain = 0xE5i32,
	/// DXGI_MSG_DXGIGetDebugInterface1_NULL_ppDebug = 0xE6i32
	DXGIGetDebugInterface1NullppDebug = 0xE6i32,
	/// DXGI_MSG_DXGIGetDebugInterface1_InvalidFlags = 0xE7i32
	DXGIGetDebugInterface1InvalidFlags = 0xE7i32,
	/// DXGI_MSG_IDXGISwapChain_Present_Decode = 0xE8i32
	IDXGISwapChainPresentDecode = 0xE8i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_Decode = 0xE9i32
	IDXGISwapChainResizeBuffersDecode = 0xE9i32,
	/// DXGI_MSG_IDXGISwapChain_SetSourceSize_FlipModel = 0xEAi32
	IDXGISwapChainSetSourceSizeFlipModel = 0xEAi32,
	/// DXGI_MSG_IDXGISwapChain_SetSourceSize_Decode = 0xEBi32
	IDXGISwapChainSetSourceSizeDecode = 0xEBi32,
	/// DXGI_MSG_IDXGISwapChain_SetSourceSize_WidthHeight = 0xECi32
	IDXGISwapChainSetSourceSizeWidthHeight = 0xECi32,
	/// DXGI_MSG_IDXGISwapChain_GetSourceSize_NullPointers = 0xEDi32
	IDXGISwapChainGetSourceSizeNullPointers = 0xEDi32,
	/// DXGI_MSG_IDXGISwapChain_GetSourceSize_Decode = 0xEEi32
	IDXGISwapChainGetSourceSizeDecode = 0xEEi32,
	/// DXGI_MSG_IDXGIDecodeSwapChain_SetColorSpace_InvalidFlags = 0xEFi32
	IDXGIDecodeSwapChainSetColorSpaceInvalidFlags = 0xEFi32,
	/// DXGI_MSG_IDXGIDecodeSwapChain_SetSourceRect_InvalidRect = 0xF0i32
	IDXGIDecodeSwapChainSetSourceRectInvalidRect = 0xF0i32,
	/// DXGI_MSG_IDXGIDecodeSwapChain_SetTargetRect_InvalidRect = 0xF1i32
	IDXGIDecodeSwapChainSetTargetRectInvalidRect = 0xF1i32,
	/// DXGI_MSG_IDXGIDecodeSwapChain_SetDestSize_InvalidSize = 0xF2i32
	IDXGIDecodeSwapChainSetDestSizeInvalidSize = 0xF2i32,
	/// DXGI_MSG_IDXGIDecodeSwapChain_GetSourceRect_InvalidPointer = 0xF3i32
	IDXGIDecodeSwapChainGetSourceRectInvalidPointer = 0xF3i32,
	/// DXGI_MSG_IDXGIDecodeSwapChain_GetTargetRect_InvalidPointer = 0xF4i32
	IDXGIDecodeSwapChainGetTargetRectInvalidPointer = 0xF4i32,
	/// DXGI_MSG_IDXGIDecodeSwapChain_GetDestSize_InvalidPointer = 0xF5i32
	IDXGIDecodeSwapChainGetDestSizeInvalidPointer = 0xF5i32,
	/// DXGI_MSG_IDXGISwapChain_PresentBuffer_YUV = 0xF6i32
	IDXGISwapChainPresentBufferYuv = 0xF6i32,
	/// DXGI_MSG_IDXGISwapChain_SetSourceSize_YUV = 0xF7i32
	IDXGISwapChainSetSourceSizeYuv = 0xF7i32,
	/// DXGI_MSG_IDXGISwapChain_GetSourceSize_YUV = 0xF8i32
	IDXGISwapChainGetSourceSizeYuv = 0xF8i32,
	/// DXGI_MSG_IDXGISwapChain_SetMatrixTransform_YUV = 0xF9i32
	IDXGISwapChainSetMatrixTransformYuv = 0xF9i32,
	/// DXGI_MSG_IDXGISwapChain_GetMatrixTransform_YUV = 0xFAi32
	IDXGISwapChainGetMatrixTransformYuv = 0xFAi32,
	/// DXGI_MSG_IDXGISwapChain_Present_PartialPresentation_YUV = 0xFBi32
	IDXGISwapChainPresentPartialPresentationYuv = 0xFBi32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveFlag_YUV = 0xFCi32
	IDXGISwapChainResizeBuffersCannotAddOrRemoveFlagYuv = 0xFCi32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_Alignment_YUV = 0xFDi32
	IDXGISwapChainResizeBuffersAlignmentYuv = 0xFDi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_ShaderInputUnsupported_YUV = 0xFEi32
	IDXGIFactoryCreateSwapChainShaderInputUnsupportedYuv = 0xFEi32,
	/// DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_NullPointers = 0xFFi32
	IDXGIOutput3CheckOverlaySupportNullPointers = 0xFFi32,
	/// DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_IDXGIDeviceNotSupportedBypConcernedDevice = 0x100i32
	IDXGIOutput3CheckOverlaySupportIDXGIDeviceNotSupportedBypConcernedDevice = 0x100i32,
	/// DXGI_MSG_IDXGIAdapter_EnumOutputs2_InvalidEnumOutputs2Flag = 0x101i32
	IDXGIAdapterEnumOutputs2InvalidEnumOutputs2Flag = 0x101i32,
	/// DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_FSUnsupportedForFlipDiscard = 0x102i32
	IDXGISwapChainCreationOrSetFullscreenStateFSUnsupportedForFlipDiscard = 0x102i32,
	/// DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_NullPointers = 0x103i32
	IDXGIOutput4CheckOverlayColorSpaceSupportNullPointers = 0x103i32,
	/// DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_IDXGIDeviceNotSupportedBypConcernedDevice = 0x104i32
	IDXGIOutput4CheckOverlayColorSpaceSupportIDXGIDeviceNotSupportedBypConcernedDevice = 0x104i32,
	/// DXGI_MSG_IDXGISwapChain3_CheckColorSpaceSupport_NullPointers = 0x105i32
	IDXGISwapChain3CheckColorSpaceSupportNullPointers = 0x105i32,
	/// DXGI_MSG_IDXGISwapChain3_SetColorSpace1_InvalidColorSpace = 0x106i32
	IDXGISwapChain3SetColorSpace1InvalidColorSpace = 0x106i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidHwProtect = 0x107i32
	IDXGIFactoryCreateSwapChainInvalidHwProtect = 0x107i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_HwProtectUnsupported = 0x108i32
	IDXGIFactoryCreateSwapChainHwProtectUnsupported = 0x108i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtect = 0x109i32
	IDXGISwapChainResizeBuffersInvalidHwProtect = 0x109i32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_HwProtectUnsupported = 0x10Ai32
	IDXGISwapChainResizeBuffersHwProtectUnsupported = 0x10Ai32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers1_D3D12Only = 0x10Bi32
	IDXGISwapChainResizeBuffers1D3D12Only = 0x10Bi32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers1_FlipModel = 0x10Ci32
	IDXGISwapChainResizeBuffers1FlipModel = 0x10Ci32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers1_NodeMaskAndQueueRequired = 0x10Di32
	IDXGISwapChainResizeBuffers1NodeMaskAndQueueRequired = 0x10Di32,
	/// DXGI_MSG_IDXGISwapChain_CreateSwapChain_InvalidHwProtectGdiFlag = 0x10Ei32
	IDXGISwapChainCreateSwapChainInvalidHwProtectGdiFlag = 0x10Ei32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtectGdiFlag = 0x10Fi32
	IDXGISwapChainResizeBuffersInvalidHwProtectGdiFlag = 0x10Fi32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_10BitFormatNotSupported = 0x110i32
	IDXGIFactoryCreateSwapChain10BitFormatNotSupported = 0x110i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSwapEffectRequired = 0x111i32
	IDXGIFactoryCreateSwapChainFlipSwapEffectRequired = 0x111i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidDevice = 0x112i32
	IDXGIFactoryCreateSwapChainInvalidDevice = 0x112i32,
	/// DXGI_MSG_IDXGIOutput_TakeOwnership_Unsupported = 0x113i32
	IDXGIOutputTakeOwnershipUnsupported = 0x113i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidQueue = 0x114i32
	IDXGIFactoryCreateSwapChainInvalidQueue = 0x114i32,
	/// DXGI_MSG_IDXGISwapChain3_ResizeBuffers1_InvalidQueue = 0x115i32
	IDXGISwapChain3ResizeBuffers1InvalidQueue = 0x115i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChainForHwnd_InvalidScaling = 0x116i32
	IDXGIFactoryCreateSwapChainForHwndInvalidScaling = 0x116i32,
	/// DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidSize = 0x117i32
	IDXGISwapChain3SetHDRMetaDataInvalidSize = 0x117i32,
	/// DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidPointer = 0x118i32
	IDXGISwapChain3SetHDRMetaDataInvalidPointer = 0x118i32,
	/// DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidType = 0x119i32
	IDXGISwapChain3SetHDRMetaDataInvalidType = 0x119i32,
	/// DXGI_MSG_IDXGISwapChain_Present_FullscreenAllowTearingIsInvalid = 0x11Ai32
	IDXGISwapChainPresentFullscreenAllowTearingIsInvalid = 0x11Ai32,
	/// DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresPresentIntervalZero = 0x11Bi32
	IDXGISwapChainPresentAllowTearingRequiresPresentIntervalZero = 0x11Bi32,
	/// DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresCreationFlag = 0x11Ci32
	IDXGISwapChainPresentAllowTearingRequiresCreationFlag = 0x11Ci32,
	/// DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveAllowTearingFlag = 0x11Di32
	IDXGISwapChainResizeBuffersCannotAddOrRemoveAllowTearingFlag = 0x11Di32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_AllowTearingFlagIsFlipModelOnly = 0x11Ei32
	IDXGIFactoryCreateSwapChainAllowTearingFlagIsFlipModelOnly = 0x11Ei32,
	/// DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidFeature = 0x11Fi32
	IDXGIFactoryCheckFeatureSupportInvalidFeature = 0x11Fi32,
	/// DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidSize = 0x120i32
	IDXGIFactoryCheckFeatureSupportInvalidSize = 0x120i32,
	/// DXGI_MSG_IDXGIOutput6_CheckHardwareCompositionSupport_NullPointer = 0x121i32
	IDXGIOutput6CheckHardwareCompositionSupportNullPointer = 0x121i32,
	/// DXGI_MSG_IDXGISwapChain_SetFullscreenState_PerMonitorDpiShimApplied = 0x122i32
	IDXGISwapChainSetFullscreenStatePerMonitorDpiShimApplied = 0x122i32,
	/// DXGI_MSG_IDXGIOutput_DuplicateOutput_PerMonitorDpiShimApplied = 0x123i32
	IDXGIOutputDuplicateOutputPerMonitorDpiShimApplied = 0x123i32,
	/// DXGI_MSG_IDXGIOutput_DuplicateOutput1_PerMonitorDpiRequired = 0x124i32
	IDXGIOutputDuplicateOutput1PerMonitorDpiRequired = 0x124i32,
	/// DXGI_MSG_IDXGIFactory7_UnregisterAdaptersChangedEvent_CookieNotFound = 0x125i32
	IDXGIFactory7UnregisterAdaptersChangedEventCookieNotFound = 0x125i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_LegacyBltModelSwapEffect = 0x126i32
	IDXGIFactoryCreateSwapChainLegacyBltModelSwapEffect = 0x126i32,
	/// DXGI_MSG_IDXGISwapChain4_SetHDRMetaData_MetadataUnchanged = 0x127i32
	IDXGISwapChain4SetHDRMetaDataMetadataUnchanged = 0x127i32,
	/// DXGI_MSG_IDXGISwapChain_Present_11On12_Released_Resource = 0x128i32
	IDXGISwapChainPresent11On12ReleasedResource = 0x128i32,
	/// DXGI_MSG_IDXGIFactory_CreateSwapChain_MultipleSwapchainRefToSurface_DeferredDtr = 0x129i32
	IDXGIFactoryCreateSwapChainMultipleSwapchainRefToSurfaceDeferredDtr = 0x129i32,
	/// DXGI_MSG_IDXGIFactory_MakeWindowAssociation_NoOpBehavior = 0x12Ai32
	IDXGIFactoryMakeWindowAssociationNoOpBehavior = 0x12Ai32,
	/// DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow = 0x3E8i32
	PhoneIDXGIFactoryCreateSwapChainNotForegroundWindow = 0x3E8i32,
	/// DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_DISCARD_BufferCount = 0x3E9i32
	PhoneIDXGIFactoryCreateSwapChainDiscardBufferCount = 0x3E9i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_SetFullscreenState_NotAvailable = 0x3EAi32
	PhoneIDXGISwapChainSetFullscreenStateNotAvailable = 0x3EAi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_ResizeBuffers_NotAvailable = 0x3EBi32
	PhoneIDXGISwapChainResizeBuffersNotAvailable = 0x3EBi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_ResizeTarget_NotAvailable = 0x3ECi32
	PhoneIDXGISwapChainResizeTargetNotAvailable = 0x3ECi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerIndex = 0x3EDi32
	PhoneIDXGISwapChainPresentInvalidLayerIndex = 0x3EDi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleLayerIndex = 0x3EEi32
	PhoneIDXGISwapChainPresentMultipleLayerIndex = 0x3EEi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerFlag = 0x3EFi32
	PhoneIDXGISwapChainPresentInvalidLayerFlag = 0x3EFi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidRotation = 0x3F0i32
	PhoneIDXGISwapChainPresentInvalidRotation = 0x3F0i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidBlend = 0x3F1i32
	PhoneIDXGISwapChainPresentInvalidBlend = 0x3F1i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidResource = 0x3F2i32
	PhoneIDXGISwapChainPresentInvalidResource = 0x3F2i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidMultiPlaneOverlayResource = 0x3F3i32
	PhoneIDXGISwapChainPresentInvalidMultiPlaneOverlayResource = 0x3F3i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForPrimary = 0x3F4i32
	PhoneIDXGISwapChainPresentInvalidIndexForPrimary = 0x3F4i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForOverlay = 0x3F5i32
	PhoneIDXGISwapChainPresentInvalidIndexForOverlay = 0x3F5i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSubResourceIndex = 0x3F6i32
	PhoneIDXGISwapChainPresentInvalidSubResourceIndex = 0x3F6i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSourceRect = 0x3F7i32
	PhoneIDXGISwapChainPresentInvalidSourceRect = 0x3F7i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidDestinationRect = 0x3F8i32
	PhoneIDXGISwapChainPresentInvalidDestinationRect = 0x3F8i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleResource = 0x3F9i32
	PhoneIDXGISwapChainPresentMultipleResource = 0x3F9i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_NotSharedResource = 0x3FAi32
	PhoneIDXGISwapChainPresentNotSharedResource = 0x3FAi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidFlag = 0x3FBi32
	PhoneIDXGISwapChainPresentInvalidFlag = 0x3FBi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidInterval = 0x3FCi32
	PhoneIDXGISwapChainPresentInvalidInterval = 0x3FCi32,
	/// DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_MSAA_NotSupported = 0x3FDi32
	PhoneIDXGIFactoryCreateSwapChainMSaaNotSupported = 0x3FDi32,
	/// DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_ScalingAspectRatioStretch_Supported_ModernApp = 0x3FEi32
	PhoneIDXGIFactoryCreateSwapChainScalingAspectRatioStretchSupportedModernApp = 0x3FEi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_GetFrameStatistics_NotAvailable_ModernApp = 0x3FFi32
	PhoneIDXGISwapChainGetFrameStatisticsNotAvailableModernApp = 0x3FFi32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present_ReplaceInterval0With1 = 0x400i32
	PhoneIDXGISwapChainPresentReplaceInterval0With1 = 0x400i32,
	/// DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FailedRegisterWithCompositor = 0x401i32
	PhoneIDXGIFactoryCreateSwapChainFailedRegisterWithCompositor = 0x401i32,
	/// DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow_AtRendering = 0x402i32
	PhoneIDXGIFactoryCreateSwapChainNotForegroundWindowAtRendering = 0x402i32,
	/// DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_SEQUENTIAL_BufferCount = 0x403i32
	PhoneIDXGIFactoryCreateSwapChainFlipSequentialBufferCount = 0x403i32,
	/// DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_Modern_CoreWindow_Only = 0x404i32
	PhoneIDXGIFactoryCreateSwapChainFlipModernCoreWindowOnly = 0x404i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_Present1_RequiresOverlays = 0x405i32
	PhoneIDXGISwapChainPresent1RequiresOverlays = 0x405i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_SetBackgroundColor_FlipSequentialRequired = 0x406i32
	PhoneIDXGISwapChainSetBackgroundColorFlipSequentialRequired = 0x406i32,
	/// DXGI_MSG_Phone_IDXGISwapChain_GetBackgroundColor_FlipSequentialRequired = 0x407i32
	PhoneIDXGISwapChainGetBackgroundColorFlipSequentialRequired = 0x407i32,
}

