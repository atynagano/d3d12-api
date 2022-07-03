#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiResourcePriority
{
	Minimum              = 0x28000000u32,
	Low                  = 0x50000000u32,
	Normal               = 0x78000000u32,
	High                 = 0xA0000000u32,
	Maximum              = 0xC8000000u32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiResidency
{
	FullyResident        = 0x1i32,
	ResidentInSharedMemory = 0x2i32,
	EvictedToDisk        = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiSwapEffect
{
	Discard              = 0x0i32,
	Sequential           = 0x1i32,
	FlipSequential       = 0x3i32,
	FlipDiscard          = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiSwapChainFlag
{
	NonPreRotated        = 0x1i32,
	AllowModeSwitch      = 0x2i32,
	GdiCompatible        = 0x4i32,
	RestrictedContent    = 0x8i32,
	RestrictSharedResourceDriver = 0x10i32,
	DisplayOnly          = 0x20i32,
	FrameLatencyWAiTableObject = 0x40i32,
	ForegroundLayer      = 0x80i32,
	FullScreenVideo      = 0x100i32,
	YUVVideo             = 0x200i32,
	HwProtected          = 0x400i32,
	AllowTearing         = 0x800i32,
	RestrictedToAllHolographicDisplays = 0x1000i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiAdapterFlag
{
	None                 = 0x0u32,
	Remote               = 0x1u32,
	Software             = 0x2u32,
}

impl BitOr for DxgiAdapterFlag {
	type Output = DxgiAdapterFlag;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for DxgiAdapterFlag {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl DxgiAdapterFlag {
    pub fn contains(self, other: DxgiAdapterFlag) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiOutDuplPointerShapeType
{
	Monochrome           = 0x1i32,
	Color                = 0x2i32,
	MaskedColor          = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiOfferResourcePriority
{
	Low                  = 0x1i32,
	Normal               = 0x2i32,
	High                 = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiScaling
{
	Stretch              = 0x0i32,
	None                 = 0x1i32,
	AspectRatioStretch   = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiGraphicsPreemptionGranularity
{
	GraphicsPreemptionDMaBufferBoundary = 0x0i32,
	GraphicsPreemptionPrimitiveBoundary = 0x1i32,
	GraphicsPreemptionTriangleBoundary = 0x2i32,
	GraphicsPreemptionPixelBoundary = 0x3i32,
	GraphicsPreemptionInstructionBoundary = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiComputePreemptionGranularity
{
	ComputePreemptionDMaBufferBoundary = 0x0i32,
	ComputePreemptionDispatchBoundary = 0x1i32,
	ComputePreemptionThreadGroupBoundary = 0x2i32,
	ComputePreemptionThreadBoundary = 0x3i32,
	ComputePreemptionInstructionBoundary = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiMultiplaneOverlayYcbcrFlags
{
	NominalRange         = 0x1i32,
	Bt709                = 0x2i32,
	XVYCc                = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiFramePresentationMode
{
	Composed             = 0x0i32,
	Overlay              = 0x1i32,
	None                 = 0x2i32,
	CompositionFailure   = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiOverlaySupportFlag
{
	Direct               = 0x1i32,
	Scaling              = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiSwapChainColorSpaceSupportFlag
{
	Present              = 0x1i32,
	OverlayPresent       = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiOverlayColorSpaceSupportFlag
{
	Present              = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiMemorySegmentGroup
{
	Local                = 0x0i32,
	NonLocal             = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiOutDuplFlag
{
	OutDuplCompositedUiCaptureOnly = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiHdrMetaDataType
{
	None                 = 0x0i32,
	Hdr10                = 0x1i32,
	Hdr10Plus            = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiOfferResourceFlags
{
	AllowDeCommit        = 0x1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiReclaimResourceResults
{
	Ok                   = 0x0i32,
	Discarded            = 0x1i32,
	NotCommitted         = 0x2i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiFeature
{
	PresentAllowTearing  = 0x0i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiAdapterFlag3
{
	None                 = 0x0u32,
	Remote               = 0x1u32,
	Software             = 0x2u32,
	ACgCompatible        = 0x4u32,
	SupportMonitoredFences = 0x8u32,
	SupportNonMonitoredFences = 0x10u32,
	KeyedMuTexConformance = 0x20u32,
	ForceDWord           = 0xFFFFFFFFu32,
}

impl BitOr for DxgiAdapterFlag3 {
	type Output = DxgiAdapterFlag3;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for DxgiAdapterFlag3 {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl DxgiAdapterFlag3 {
    pub fn contains(self, other: DxgiAdapterFlag3) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiHardwareCompositionSupportFlags
{
	FullScreen           = 0x1u32,
	Windowed             = 0x2u32,
	CursorStretched      = 0x4u32,
}

impl BitOr for DxgiHardwareCompositionSupportFlags {
	type Output = DxgiHardwareCompositionSupportFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for DxgiHardwareCompositionSupportFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl DxgiHardwareCompositionSupportFlags {
    pub fn contains(self, other: DxgiHardwareCompositionSupportFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiGpuPreference
{
	Unspecified          = 0x0i32,
	MinimumPower         = 0x1i32,
	HighPerformance      = 0x2i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiDebugRLoFlags
{
	DebugRLoSummary      = 0x1u32,
	DebugRLoDetail       = 0x2u32,
	DebugRLoIgnoreInternal = 0x4u32,
	DebugRLoAll          = 0x7u32,
}

impl BitOr for DxgiDebugRLoFlags {
	type Output = DxgiDebugRLoFlags;
	fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
		unsafe { transmute(self as u32 | rhs as u32) }
	}
}

impl BitOrAssign for DxgiDebugRLoFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl DxgiDebugRLoFlags {
    pub fn contains(self, other: DxgiDebugRLoFlags) -> bool {
        (self as u32) & (other as u32) == (other as u32)
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiInfoQueueMessageCategory
{
	Unknown              = 0x0i32,
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
pub enum DxgiInfoQueueMessageSeverity
{
	Corruption           = 0x0i32,
	Error                = 0x1i32,
	Warning              = 0x2i32,
	Info                 = 0x3i32,
	Message              = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiMessageId
{
	MsgIDxgiSwapChainCreationOrResizeBuffersInvalidOutputWindow = 0x0i32,
	MsgIDxgiSwapChainCreationOrResizeBuffersBufferWidthInferred = 0x1i32,
	MsgIDxgiSwapChainCreationOrResizeBuffersBufferHeightInferred = 0x2i32,
	MsgIDxgiSwapChainCreationOrResizeBuffersNoScanoutFlagChanged = 0x3i32,
	MsgIDxgiSwapChainCreationMaxBufferCountExceeded = 0x4i32,
	MsgIDxgiSwapChainCreationTooFewBuffers = 0x5i32,
	MsgIDxgiSwapChainCreationNoOutputWindow = 0x6i32,
	MsgIDxgiSwapChainDestructionOtherMethodsCalled = 0x7i32,
	MsgIDxgiSwapChainGetDescPDesCisNull = 0x8i32,
	MsgIDxgiSwapChainGetBufferPpSurfaceIsNull = 0x9i32,
	MsgIDxgiSwapChainGetBufferNoAllocatedBuffers = 0xAi32,
	MsgIDxgiSwapChainGetBufferIBufferMustBeZero = 0xBi32,
	MsgIDxgiSwapChainGetBufferIBuffeRoOb = 0xCi32,
	MsgIDxgiSwapChainGetContainingOutputPPoutPuTisNull = 0xDi32,
	MsgIDxgiSwapChainPresentSynCinterVaLoob = 0xEi32,
	MsgIDxgiSwapChainPresentInvalidNonPreRotatedFlag = 0xFi32,
	MsgIDxgiSwapChainPresentNoAllocatedBuffers = 0x10i32,
	MsgIDxgiSwapChainPresentGetDXGIAdapterFailed = 0x11i32,
	MsgIDxgiSwapChainResizeBuffersBufferCoUntoOb = 0x12i32,
	MsgIDxgiSwapChainResizeBuffersUnreleasedReferences = 0x13i32,
	MsgIDxgiSwapChainResizeBuffersInvalidSwapChainFlag = 0x14i32,
	MsgIDxgiSwapChainResizeBuffersInvalidNonPreRotatedFlag = 0x15i32,
	MsgIDxgiSwapChainResizeTargetRefreshRateDivideByZero = 0x16i32,
	MsgIDxgiSwapChainSetFullscreenStateInvalidTarget = 0x17i32,
	MsgIDxgiSwapChainGetFrameStatisticsPStatSisNull = 0x18i32,
	MsgIDxgiSwapChainGetLastPresentCountPLastPresentCoUnTisNull = 0x19i32,
	MsgIDxgiSwapChainSetFullscreenStateRemoteNotSupported = 0x1Ai32,
	MsgIDxgiOutputTakeOwnershipFailedToAcquireFullscreenMutex = 0x1Bi32,
	MsgIDxgiFactoryCreateSoftwareAdapterPpAdapterInterfaceIsNull = 0x1Ci32,
	MsgIDxgiFactoryEnumAdaptersPpAdapterInterfaceIsNull = 0x1Di32,
	MsgIDxgiFactoryCreateSwapChainPpSwapChaiNisNull = 0x1Ei32,
	MsgIDxgiFactoryCreateSwapChainPDesCisNull = 0x1Fi32,
	MsgIDxgiFactoryCreateSwapChainUnknownSwapEffect = 0x20i32,
	MsgIDxgiFactoryCreateSwapChainInvalidFlags = 0x21i32,
	MsgIDxgiFactoryCreateSwapChainNonPreRotatedFlagAndWindowed = 0x22i32,
	MsgIDxgiFactoryCreateSwapChainNullDeviceInterface = 0x23i32,
	MsgIDxgiFactoryGetWindowAssociationPHwNDisNull = 0x24i32,
	MsgIDxgiFactoryMakeWindowAssociationInvalidFlags = 0x25i32,
	MsgIDxgiSurfaceMapInvalidSurface = 0x26i32,
	MsgIDxgiSurfaceMapFlagsSetToZero = 0x27i32,
	MsgIDxgiSurfaceMapDiscardAndReadFlagSet = 0x28i32,
	MsgIDxgiSurfaceMapDiscardButNotWriteFlagSet = 0x29i32,
	MsgIDxgiSurfaceMapNoCPUAccess = 0x2Ai32,
	MsgIDxgiSurfaceMapReadFlagSetButCPUAccessIsDynamic = 0x2Bi32,
	MsgIDxgiSurfaceMapDiscardFlagSetButCPUAccessIsNotDynamic = 0x2Ci32,
	MsgIDxgiOutputGetDisplayModeListPNumModeSisNull = 0x2Di32,
	MsgIDxgiOutputFindClosestMatchingModeModeHasInvalidWidthOrHeight = 0x2Ei32,
	MsgIDxgiOutputGetCammaControlCapabilitiesNoOwnerDevice = 0x2Fi32,
	MsgIDxgiOutputTakeOwnershipPDeviceIsNull = 0x30i32,
	MsgIDxgiOutputGetDisplaySurfaceDataNoOwnerDevice = 0x31i32,
	MsgIDxgiOutputGetDisplaySurfaceDataPDesTiNatIoNisNull = 0x32i32,
	MsgIDxgiOutputGetDisplaySurfaceDataMapOfDestinationFailed = 0x33i32,
	MsgIDxgiOutputGetFrameStatisticsNoOwnerDevice = 0x34i32,
	MsgIDxgiOutputGetFrameStatisticsPStatSisNull = 0x35i32,
	MsgIDxgiOutputSetGammaControlNoOwnerDevice = 0x36i32,
	MsgIDxgiOutputGetGammaControlNoOwnerDevice = 0x37i32,
	MsgIDxgiOutputGetGammaControlNoGammaControls = 0x38i32,
	MsgIDxgiOutputSetDisplaySurfaceIDxgiResourceNotSupportedBypPrimary = 0x39i32,
	MsgIDxgiOutputSetDisplaySurfacePPrimarYisInvalid = 0x3Ai32,
	MsgIDxgiOutputSetDisplaySurfaceNoOwnerDevice = 0x3Bi32,
	MsgIDxgiOutputTakeOwnershipRemoteDeviceNotSupported = 0x3Ci32,
	MsgIDxgiOutputGetDisplayModeListRemoteDeviceNotSupported = 0x3Di32,
	MsgIDxgiOutputFindClosestMatchingModeRemoteDeviceNotSupported = 0x3Ei32,
	MsgIDxgiDeviceCreateSurfaceInvalidParametersWithpSharedResource = 0x3Fi32,
	MsgIDxgiObjectGetPrivateDataPUiDataSizeIsNull = 0x40i32,
	MsgIDxgiSwapChainCreationInvalidOutputWindow = 0x41i32,
	MsgIDxgiSwapChainReleaseSwapChainIsFullscreen = 0x42i32,
	MsgIDxgiOutputGetDisplaySurfaceDataInvalidTargetSurfaceFormat = 0x43i32,
	MsgIDxgiFactoryCreateSoftwareAdapterMoDuLeisNull = 0x44i32,
	MsgIDxgiOutputFindClosestMatchingModeIDxgiDeviceNotSupportedBypConcernedDevice = 0x45i32,
	MsgIDxgiOutputFindClosestMatchingModePModeTomaTcHorPClosestMatChisNull = 0x46i32,
	MsgIDxgiOutputFindClosestMatchingModeModeHasRefreshRateDenominatorZero = 0x47i32,
	MsgIDxgiOutputFindClosestMatchingModeUnknownFormatIsInvalidForConfiguration = 0x48i32,
	MsgIDxgiOutputFindClosestMatchingModeInvalidDisplayModeScanlineOrdering = 0x49i32,
	MsgIDxgiOutputFindClosestMatchingModeInvalidDisplayModeScaling = 0x4Ai32,
	MsgIDxgiOutputFindClosestMatchingModeInvalidDisplayModeFormatAndDeviceCombination = 0x4Bi32,
	MsgIDxgiFactoryCreationCalledFromDllMain = 0x4Ci32,
	MsgIDxgiSwapChainSetFullscreenStateOutputNotOwnedBySwapChainDevice = 0x4Di32,
	MsgIDxgiSwapChainCreationInvalidWindowStyle = 0x4Ei32,
	MsgIDxgiSwapChainGetFrameStatisticsUnsupportedStatistics = 0x4Fi32,
	MsgIDxgiSwapChainGetContainingOutputSwapchainAdapterDoesNotControlOutput = 0x50i32,
	MsgIDxgiOutputSetOrGetGammaControlParraYisNull = 0x51i32,
	MsgIDxgiSwapChainSetFullscreenStateFullscreenInvalidForChildWindows = 0x52i32,
	MsgIDxgiFactoryReleaseCalledFromDllMain = 0x53i32,
	MsgIDxgiSwapChainPresentUnReLeasEdhDc = 0x54i32,
	MsgIDxgiSwapChainResizeBuffersNonPreRotatedAndGDICompatibleFlags = 0x55i32,
	MsgIDxgiFactoryCreateSwapChainNonPreRotatedAndGDICompatibleFlags = 0x56i32,
	MsgIDxgiSurface1GetDcPHdCisNull = 0x57i32,
	MsgIDxgiSurface1GetDcSurfaceNotTexture2D = 0x58i32,
	MsgIDxgiSurface1GetDcGdiCompatibleFlagNotSet = 0x59i32,
	MsgIDxgiSurface1GetDcUnReLeasEdhDc = 0x5Ai32,
	MsgIDxgiSurfaceMapNoCPUAccess2 = 0x5Bi32,
	MsgIDxgiSurface1ReleaseDcGetDCNotCalled = 0x5Ci32,
	MsgIDxgiSurface1ReleaseDcInvalidRectangleDimensions = 0x5Di32,
	MsgIDxgiOutputTakeOwnershipRemoteOutputNotSupported = 0x5Ei32,
	MsgIDxgiOutputFindClosestMatchingModeRemoteOutputNotSupported = 0x5Fi32,
	MsgIDxgiOutputGetDisplayModeListRemoteOutputNotSupported = 0x60i32,
	MsgIDxgiFactoryCreateSwapChainPDeviceHasMismatchedDxgiFactory = 0x61i32,
	MsgIDxgiSwapChainPresentNonOptimalFSConfiguration = 0x62i32,
	MsgIDxgiFactoryCreateSwapChainFlipSequentialNotSupPorteDonD3D10 = 0x63i32,
	MsgIDxgiFactoryCreateSwapChainBufferCountOOBForFlipSequential = 0x64i32,
	MsgIDxgiFactoryCreateSwapChainInvalidFormatForFlipSequential = 0x65i32,
	MsgIDxgiFactoryCreateSwapChainMultiSamplingNotSupportedForFlipSequential = 0x66i32,
	MsgIDxgiSwapChainResizeBuffersBufferCountOOBForFlipSequential = 0x67i32,
	MsgIDxgiSwapChainResizeBuffersInvalidFormatForFlipSequential = 0x68i32,
	MsgIDxgiSwapChainPresentPartialPresentationBeforeStandardPresentation = 0x69i32,
	MsgIDxgiSwapChainPresentFullscreenPartialPresentIsInvalid = 0x6Ai32,
	MsgIDxgiSwapChainPresentInvalidPresentTestOrDoNotSequenceFlag = 0x6Bi32,
	MsgIDxgiSwapChainPresentScrollInfoWithNoDirtyRectsSpecified = 0x6Ci32,
	MsgIDxgiSwapChainPresentEmptyScrollRect = 0x6Di32,
	MsgIDxgiSwapChainPresentScrollRectOutOfBackbufferBounds = 0x6Ei32,
	MsgIDxgiSwapChainPresentScrollRectOutOfBackbufferBoundsWithOffset = 0x6Fi32,
	MsgIDxgiSwapChainPresentEmptyDirtyRect = 0x70i32,
	MsgIDxgiSwapChainPresentDirtyRectOutOfBackbufferBounds = 0x71i32,
	MsgIDxgiFactoryCreateSwapChainUnsupportedBufferUsageFlags = 0x72i32,
	MsgIDxgiSwapChainPresentDoNotSequenceFlagSetButPreviousBufferIsUndefined = 0x73i32,
	MsgIDxgiSwapChainPresentUnsupportedFlags = 0x74i32,
	MsgIDxgiSwapChainPresentFlipModelChainMustResizeOrCreateOnFSTransition = 0x75i32,
	MsgIDxgiFactoryCreateSwapChainPRestrictToOutputFroMotherIDxgiFactory = 0x76i32,
	MsgIDxgiFactoryCreateSwapChainRestrictOutputNotSupportedOnAdapter = 0x77i32,
	MsgIDxgiSwapChainPresentRestrictToOutputFlagSetButInvalidpRestrictToOutput = 0x78i32,
	MsgIDxgiSwapChainPresentRestrictToOutputFlagdWithFullscreen = 0x79i32,
	MsgIDxgiSwapChainPresentRestrictOutputFlagWithStaleSwapChain = 0x7Ai32,
	MsgIDxgiSwapChainPresentOtherFlagsCausingInvalidPresentTestFlag = 0x7Bi32,
	MsgIDxgiFactoryCreateSwapChainUnavailableInSession0 = 0x7Ci32,
	MsgIDxgiFactoryMakeWindowAssociationUnavailableInSession0 = 0x7Di32,
	MsgIDxgiFactoryGetWindowAssociationUnavailableInSession0 = 0x7Ei32,
	MsgIDxgiAdapterEnumOutputsUnavailableInSession0 = 0x7Fi32,
	MsgIDxgiSwapChainCreationOrSetFullscreenStateStereoDisabled = 0x80i32,
	MsgIDxgiFactory2UnregisterStatusCookieNotFound = 0x81i32,
	MsgIDxgiSwapChainPresentProtectedContentInWindowedModeWithoutFSOrOverlay = 0x82i32,
	MsgIDxgiSwapChainPresentProtectedContentInWindowedModeWithoutFlipSequential = 0x83i32,
	MsgIDxgiSwapChainPresentProtectedContentWithRDPDriver = 0x84i32,
	MsgIDxgiSwapChainPresentProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity = 0x85i32,
	MsgIDxgiFactoryCreateSwapChainForCompositionWidthOrHeightIsZero = 0x86i32,
	MsgIDxgiFactoryCreateSwapChainForCompositionOnlyFlipSequentialSupported = 0x87i32,
	MsgIDxgiFactoryCreateSwapChainForCompositionUnsupportedOnAdapter = 0x88i32,
	MsgIDxgiFactoryCreateSwapChainForCompositionUnsupportedOnWindows7 = 0x89i32,
	MsgIDxgiSwapChainSetFullscreenStateFsTransitionWithCompositionSwapChain = 0x8Ai32,
	MsgIDxgiSwapChainResizeTargetInvalidWithCompositionSwapChain = 0x8Bi32,
	MsgIDxgiSwapChainResizeBuffersWidthOrHeightIsZero = 0x8Ci32,
	MsgIDxgiFactoryCreateSwapChainScalingNoneIsFlipModelOnly = 0x8Di32,
	MsgIDxgiFactoryCreateSwapChainScalingUnrecognized = 0x8Ei32,
	MsgIDxgiFactoryCreateSwapChainDisplayOnlyFullscreenUnsupported = 0x8Fi32,
	MsgIDxgiFactoryCreateSwapChainDisplayOnlyUnsupported = 0x90i32,
	MsgIDxgiSwapChainPresentRestartIsFullscreenOnly = 0x91i32,
	MsgIDxgiSwapChainPresentProtectedWindowlessPresentationRequiresDisplayOnly = 0x92i32,
	MsgIDxgiSwapChainSetFullscreenStateDisplayOnlyUnsupported = 0x93i32,
	MsgIDxgiSwapChain1SetBackgroundColorOutOfRange = 0x94i32,
	MsgIDxgiSwapChainResizeBuffersDisplayOnlyFullscreenUnsupported = 0x95i32,
	MsgIDxgiSwapChainResizeBuffersDisplayOnlyUnsupported = 0x96i32,
	MsgIDxgiSwapChainPresentScrollUnsupported = 0x97i32,
	MsgIDxgiSwapChain1SetRotationUnSupPorteDos = 0x98i32,
	MsgIDxgiSwapChain1GetRotationUnSupPorteDos = 0x99i32,
	MsgIDxgiSwapChainPresentFullscreenRotation = 0x9Ai32,
	MsgIDxgiSwapChainPresentPartialPresentationWithMSAABuffers = 0x9Bi32,
	MsgIDxgiSwapChain1SetRotationFlipSequentialRequired = 0x9Ci32,
	MsgIDxgiSwapChain1SetRotationInvalidRotation = 0x9Di32,
	MsgIDxgiSwapChain1GetRotationFlipSequentialRequired = 0x9Ei32,
	MsgIDxgiSwapChainGetHwndWrongType = 0x9Fi32,
	MsgIDxgiSwapChainGetCompositionSurfaceWrongType = 0xA0i32,
	MsgIDxgiSwapChainGetCoreWindowWrongType = 0xA1i32,
	MsgIDxgiSwapChainGetFullscreenDescNonHwnd = 0xA2i32,
	MsgIDxgiSwapChainSetFullscreenStateCoreWindow = 0xA3i32,
	MsgIDxgiFactory2CreateSwapChainForCoreWindowUnsupportedOnWindows7 = 0xA4i32,
	MsgIDxgiFactory2CreateSwapChainForCoreWindowPWinDoWisNull = 0xA5i32,
	MsgIDxgiFactoryCreateSwapChainFsUnsupportedForModerNaPps = 0xA6i32,
	MsgIDxgiFactoryMakeWindowAssociationModernApp = 0xA7i32,
	MsgIDxgiSwapChainResizeTargetModernApp = 0xA8i32,
	MsgIDxgiSwapChainResizeTargetPNewTargetParameterSisNull = 0xA9i32,
	MsgIDxgiOutputSetDisplaySurfaceModernApp = 0xAAi32,
	MsgIDxgiOutputTakeOwnershipModernApp = 0xABi32,
	MsgIDxgiFactory2CreateSwapChainForCoreWindowPWinDoWisInvalid = 0xACi32,
	MsgIDxgiFactory2CreateSwapChainForCompositionSurfaceInvalidHandle = 0xADi32,
	MsgIDxgiSurface1GetDcModernApp = 0xAEi32,
	MsgIDxgiFactoryCreateSwapChainScalingNoneRequiresWindows8OrNewer = 0xAFi32,
	MsgIDxgiSwapChainPresentTemporaryMonoAndPreferRight = 0xB0i32,
	MsgIDxgiSwapChainPresentTemporaryMonoOrPreferRightWithDoNotSequence = 0xB1i32,
	MsgIDxgiSwapChainPresentTemporaryMonoOrPreferRightWithoutStereo = 0xB2i32,
	MsgIDxgiSwapChainPresentTemporaryMonoUnsupported = 0xB3i32,
	MsgIDxgiOutputGetDisplaySurfaceDataArraySizeMismatch = 0xB4i32,
	MsgIDxgiSwapChainPresentPartialPresentationWithSwapEffectDiscard = 0xB5i32,
	MsgIDxgiFactoryCreateSwapChainAlphaUnrecognized = 0xB6i32,
	MsgIDxgiFactoryCreateSwapChainAlphaIsWindowlessOnly = 0xB7i32,
	MsgIDxgiFactoryCreateSwapChainAlphaIsFlipModelOnly = 0xB8i32,
	MsgIDxgiFactoryCreateSwapChainRestrictToOutputAdapterMismatch = 0xB9i32,
	MsgIDxgiFactoryCreateSwapChainDisplayOnlyOnLegacy = 0xBAi32,
	MsgIDxgiSwapChainResizeBuffersDisplayOnlyOnLegacy = 0xBBi32,
	MsgIDxgiResource1CreateSubresourceSurfaceInvalidIndex = 0xBCi32,
	MsgIDxgiFactoryCreateSwapChainForCompositionInvalidScaling = 0xBDi32,
	MsgIDxgiFactoryCreateSwapChainForCoreWindowInvalidSwapEffect = 0xBEi32,
	MsgIDxgiResource1CreateSharedHandleUnSupPorteDos = 0xBFi32,
	MsgIDxgiFactory2RegisterOcclusionStatusWindowUnSupPorteDos = 0xC0i32,
	MsgIDxgiFactory2RegisterOcclusionStatusEventUnSupPorteDos = 0xC1i32,
	MsgIDxgiOutput1DuplicateOutputUnSupPorteDos = 0xC2i32,
	MsgIDxgiDisplayControlIsStereoEnabledUnSupPorteDos = 0xC3i32,
	MsgIDxgiFactoryCreateSwapChainForCompositionInvalidAlphaMode = 0xC4i32,
	MsgIDxgiFactoryGetSharedResourceAdapterLuidInvalidResource = 0xC5i32,
	MsgIDxgiFactoryGetSharedResourceAdapterLuidInvalidLuid = 0xC6i32,
	MsgIDxgiFactoryGetSharedResourceAdapterLuidUnSupPorteDos = 0xC7i32,
	MsgIDxgiOutput1GetDisplaySurfaceData1_2DOnly = 0xC8i32,
	MsgIDxgiOutput1GetDisplaySurfaceData1StagingOnly = 0xC9i32,
	MsgIDxgiOutput1GetDisplaySurfaceData1NeedCPUAccessWrite = 0xCAi32,
	MsgIDxgiOutput1GetDisplaySurfaceData1NoShared = 0xCBi32,
	MsgIDxgiOutput1GetDisplaySurfaceData1OnlyMipLevels1 = 0xCCi32,
	MsgIDxgiOutput1GetDisplaySurfaceData1MappedOrOfferedResource = 0xCDi32,
	MsgIDxgiSwapChainSetFullscreenStateFsUnsupportedForModerNaPps = 0xCEi32,
	MsgIDxgiFactoryCreateSwapChainFailedToGoFSButNonPreRotated = 0xCFi32,
	MsgIDxgiFactoryCreateSwapChainOrRegisterOcclusionStatusBlitModelUsedWhileRegisteredForOcclusionStatusEvents = 0xD0i32,
	MsgIDxgiSwapChainPresentBlitModelUsedWhileRegisteredForOcclusionStatusEvents = 0xD1i32,
	MsgIDxgiFactoryCreateSwapChainWaitableSwapChainsAreFlipModelOnly = 0xD2i32,
	MsgIDxgiFactoryCreateSwapChainWaitableSwapChainsAreNotFullscreen = 0xD3i32,
	MsgIDxgiSwapChainSetFullscreenStateWaitable = 0xD4i32,
	MsgIDxgiSwapChainResizeBuffersCannotAddOrRemoveWaitableFlag = 0xD5i32,
	MsgIDxgiSwapChainGetFrameLatencyWaitableObjectOnlyWaitable = 0xD6i32,
	MsgIDxgiSwapChainGetMaximumFrameLatencyOnlyWaitable = 0xD7i32,
	MsgIDxgiSwapChainGetMaximumFrameLatencyPMaxLatEncYisNull = 0xD8i32,
	MsgIDxgiSwapChainSetMaximumFrameLatencyOnlyWaitable = 0xD9i32,
	MsgIDxgiSwapChainSetMaximumFrameLatencyMaxLatencyIsOutOfBounds = 0xDAi32,
	MsgIDxgiFactoryCreateSwapChainForegroundIsCoreWindowOnly = 0xDBi32,
	MsgIDxgiFactory2CreateSwapChainForCoreWindowForegroundUnsupportedOnAdapter = 0xDCi32,
	MsgIDxgiFactory2CreateSwapChainForCoreWindowInvalidScaling = 0xDDi32,
	MsgIDxgiFactory2CreateSwapChainForCoreWindowInvalidAlphaMode = 0xDEi32,
	MsgIDxgiSwapChainResizeBuffersCannotAddOrRemoveForegroundFlag = 0xDFi32,
	MsgIDxgiSwapChainSetMatrixTransformMatrixPointerCannotBeNull = 0xE0i32,
	MsgIDxgiSwapChainSetMatrixTransformRequiresCompositionSwapChain = 0xE1i32,
	MsgIDxgiSwapChainSetMatrixTransformMatrixMustBeFinite = 0xE2i32,
	MsgIDxgiSwapChainSetMatrixTransformMatrixMustBeTranslateAndOrScale = 0xE3i32,
	MsgIDxgiSwapChainGetMatrixTransformMatrixPointerCannotBeNull = 0xE4i32,
	MsgIDxgiSwapChainGetMatrixTransformRequiresCompositionSwapChain = 0xE5i32,
	MsgDxgiGetDebugInterface1NullPpDebug = 0xE6i32,
	MsgDxgiGetDebugInterface1InvalidFlags = 0xE7i32,
	MsgIDxgiSwapChainPresentDecode = 0xE8i32,
	MsgIDxgiSwapChainResizeBuffersDecode = 0xE9i32,
	MsgIDxgiSwapChainSetSourceSizeFlipModel = 0xEAi32,
	MsgIDxgiSwapChainSetSourceSizeDecode = 0xEBi32,
	MsgIDxgiSwapChainSetSourceSizeWidthHeight = 0xECi32,
	MsgIDxgiSwapChainGetSourceSizeNullPointers = 0xEDi32,
	MsgIDxgiSwapChainGetSourceSizeDecode = 0xEEi32,
	MsgIDxgiDecodeSwapChainSetColorSpaceInvalidFlags = 0xEFi32,
	MsgIDxgiDecodeSwapChainSetSourceRectInvalidRect = 0xF0i32,
	MsgIDxgiDecodeSwapChainSetTargetRectInvalidRect = 0xF1i32,
	MsgIDxgiDecodeSwapChainSetDestSizeInvalidSize = 0xF2i32,
	MsgIDxgiDecodeSwapChainGetSourceRectInvalidPointer = 0xF3i32,
	MsgIDxgiDecodeSwapChainGetTargetRectInvalidPointer = 0xF4i32,
	MsgIDxgiDecodeSwapChainGetDestSizeInvalidPointer = 0xF5i32,
	MsgIDxgiSwapChainPresentBufferYUV = 0xF6i32,
	MsgIDxgiSwapChainSetSourceSizeYUV = 0xF7i32,
	MsgIDxgiSwapChainGetSourceSizeYUV = 0xF8i32,
	MsgIDxgiSwapChainSetMatrixTransformYUV = 0xF9i32,
	MsgIDxgiSwapChainGetMatrixTransformYUV = 0xFAi32,
	MsgIDxgiSwapChainPresentPartialPresentationYUV = 0xFBi32,
	MsgIDxgiSwapChainResizeBuffersCannotAddOrRemoveFlagYUV = 0xFCi32,
	MsgIDxgiSwapChainResizeBuffersAlignmentYUV = 0xFDi32,
	MsgIDxgiFactoryCreateSwapChainShaderInputUnsupportedYUV = 0xFEi32,
	MsgIDxgiOutput3CheckOverlaySupportNullPointers = 0xFFi32,
	MsgIDxgiOutput3CheckOverlaySupportIDxgiDeviceNotSupportedBypConcernedDevice = 0x100i32,
	MsgIDxgiAdapterEnumOutputs2InvalidEnumOutputs2Flag = 0x101i32,
	MsgIDxgiSwapChainCreationOrSetFullscreenStateFsUnsupportedForFlipDiscard = 0x102i32,
	MsgIDxgiOutput4CheckOverlayColorSpaceSupportNullPointers = 0x103i32,
	MsgIDxgiOutput4CheckOverlayColorSpaceSupportIDxgiDeviceNotSupportedBypConcernedDevice = 0x104i32,
	MsgIDxgiSwapChain3CheckColorSpaceSupportNullPointers = 0x105i32,
	MsgIDxgiSwapChain3SetColorSpace1InvalidColorSpace = 0x106i32,
	MsgIDxgiFactoryCreateSwapChainInvalidHwProtect = 0x107i32,
	MsgIDxgiFactoryCreateSwapChainHwProtectUnsupported = 0x108i32,
	MsgIDxgiSwapChainResizeBuffersInvalidHwProtect = 0x109i32,
	MsgIDxgiSwapChainResizeBuffersHwProtectUnsupported = 0x10Ai32,
	MsgIDxgiSwapChainResizeBuffers1D3D12Only = 0x10Bi32,
	MsgIDxgiSwapChainResizeBuffers1FlipModel = 0x10Ci32,
	MsgIDxgiSwapChainResizeBuffers1NodeMaskAndQueueRequired = 0x10Di32,
	MsgIDxgiSwapChainCreateSwapChainInvalidHwProtectGdiFlag = 0x10Ei32,
	MsgIDxgiSwapChainResizeBuffersInvalidHwProtectGdiFlag = 0x10Fi32,
	MsgIDxgiFactoryCreateSwapChain10BitFormatNotSupported = 0x110i32,
	MsgIDxgiFactoryCreateSwapChainFlipSwapEffectRequired = 0x111i32,
	MsgIDxgiFactoryCreateSwapChainInvalidDevice = 0x112i32,
	MsgIDxgiOutputTakeOwnershipUnsupported = 0x113i32,
	MsgIDxgiFactoryCreateSwapChainInvalidQueue = 0x114i32,
	MsgIDxgiSwapChain3ResizeBuffers1InvalidQueue = 0x115i32,
	MsgIDxgiFactoryCreateSwapChainForHwndInvalidScaling = 0x116i32,
	MsgIDxgiSwapChain3SetHDRMetaDataInvalidSize = 0x117i32,
	MsgIDxgiSwapChain3SetHDRMetaDataInvalidPointer = 0x118i32,
	MsgIDxgiSwapChain3SetHDRMetaDataInvalidType = 0x119i32,
	MsgIDxgiSwapChainPresentFullscreenAllowTearingIsInvalid = 0x11Ai32,
	MsgIDxgiSwapChainPresentAllowTearingRequiresPresentIntervalZero = 0x11Bi32,
	MsgIDxgiSwapChainPresentAllowTearingRequiresCreationFlag = 0x11Ci32,
	MsgIDxgiSwapChainResizeBuffersCannotAddOrRemoveAllowTearingFlag = 0x11Di32,
	MsgIDxgiFactoryCreateSwapChainAllowTearingFlagIsFlipModelOnly = 0x11Ei32,
	MsgIDxgiFactoryCheckFeatureSupportInvalidFeature = 0x11Fi32,
	MsgIDxgiFactoryCheckFeatureSupportInvalidSize = 0x120i32,
	MsgIDxgiOutput6CheckHardwareCompositionSupportNullPointer = 0x121i32,
	MsgIDxgiSwapChainSetFullscreenStatePerMonitorDpiShimApplied = 0x122i32,
	MsgIDxgiOutputDuplicateOutputPerMonitorDpiShimApplied = 0x123i32,
	MsgIDxgiOutputDuplicateOutput1PerMonitorDpiRequired = 0x124i32,
	MsgIDxgiFactory7UnregisterAdaptersChangedEventCookieNotFound = 0x125i32,
	MsgIDxgiFactoryCreateSwapChainLegacyBltModelSwapEffect = 0x126i32,
	MsgIDxgiSwapChain4SetHDRMetaDataMetadataUnchanged = 0x127i32,
	MsgIDxgiSwapChainPresent11On12ReleasedResource = 0x128i32,
	MsgIDxgiFactoryCreateSwapChainMultipleSwapchainRefToSurfaceDeferredDtr = 0x129i32,
	MsgIDxgiFactoryMakeWindowAssociationNoOpBehavior = 0x12Ai32,
	MsgPhoneIDxgiFactoryCreateSwapChainNotForegroundWindow = 0x3E8i32,
	MsgPhoneIDxgiFactoryCreateSwapChainDiscardBufferCount = 0x3E9i32,
	MsgPhoneIDxgiSwapChainSetFullscreenStateNotAvailable = 0x3EAi32,
	MsgPhoneIDxgiSwapChainResizeBuffersNotAvailable = 0x3EBi32,
	MsgPhoneIDxgiSwapChainResizeTargetNotAvailable = 0x3ECi32,
	MsgPhoneIDxgiSwapChainPresentInvalidLayerIndex = 0x3EDi32,
	MsgPhoneIDxgiSwapChainPresentMultipleLayerIndex = 0x3EEi32,
	MsgPhoneIDxgiSwapChainPresentInvalidLayerFlag = 0x3EFi32,
	MsgPhoneIDxgiSwapChainPresentInvalidRotation = 0x3F0i32,
	MsgPhoneIDxgiSwapChainPresentInvalidBlend = 0x3F1i32,
	MsgPhoneIDxgiSwapChainPresentInvalidResource = 0x3F2i32,
	MsgPhoneIDxgiSwapChainPresentInvalidMultiPlaneOverlayResource = 0x3F3i32,
	MsgPhoneIDxgiSwapChainPresentInvalidIndexForPrimary = 0x3F4i32,
	MsgPhoneIDxgiSwapChainPresentInvalidIndexForOverlay = 0x3F5i32,
	MsgPhoneIDxgiSwapChainPresentInvalidSubResourceIndex = 0x3F6i32,
	MsgPhoneIDxgiSwapChainPresentInvalidSourceRect = 0x3F7i32,
	MsgPhoneIDxgiSwapChainPresentInvalidDestinationRect = 0x3F8i32,
	MsgPhoneIDxgiSwapChainPresentMultipleResource = 0x3F9i32,
	MsgPhoneIDxgiSwapChainPresentNotSharedResource = 0x3FAi32,
	MsgPhoneIDxgiSwapChainPresentInvalidFlag = 0x3FBi32,
	MsgPhoneIDxgiSwapChainPresentInvalidInterval = 0x3FCi32,
	MsgPhoneIDxgiFactoryCreateSwapChainMSaaNotSupported = 0x3FDi32,
	MsgPhoneIDxgiFactoryCreateSwapChainScalingAspectRatioStretchSupportedModernApp = 0x3FEi32,
	MsgPhoneIDxgiSwapChainGetFrameStatisticsNotAvailableModernApp = 0x3FFi32,
	MsgPhoneIDxgiSwapChainPresentReplaceInterval0With1 = 0x400i32,
	MsgPhoneIDxgiFactoryCreateSwapChainFailedRegisterWithCompositor = 0x401i32,
	MsgPhoneIDxgiFactoryCreateSwapChainNotForegroundWindowAtRendering = 0x402i32,
	MsgPhoneIDxgiFactoryCreateSwapChainFlipSequentialBufferCount = 0x403i32,
	MsgPhoneIDxgiFactoryCreateSwapChainFlipModernCoreWindowOnly = 0x404i32,
	MsgPhoneIDxgiSwapChainPresent1RequiresOverlays = 0x405i32,
	MsgPhoneIDxgiSwapChainSetBackgroundColorFlipSequentialRequired = 0x406i32,
	MsgPhoneIDxgiSwapChainGetBackgroundColorFlipSequentialRequired = 0x407i32,
}

