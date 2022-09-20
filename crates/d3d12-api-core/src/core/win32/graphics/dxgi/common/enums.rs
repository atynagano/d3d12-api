#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// DXGI_FORMAT
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiFormat
{
	/// DXGI_FORMAT_UNKNOWN = 0x0u32
	Unknown              = 0x0u32,
	/// DXGI_FORMAT_R32G32B32A32_TYPELESS = 0x1u32
	R32G32B32A32Typeless = 0x1u32,
	/// DXGI_FORMAT_R32G32B32A32_FLOAT = 0x2u32
	R32G32B32A32Float    = 0x2u32,
	/// DXGI_FORMAT_R32G32B32A32_UINT = 0x3u32
	R32G32B32A32Uint     = 0x3u32,
	/// DXGI_FORMAT_R32G32B32A32_SINT = 0x4u32
	R32G32B32A32Sint     = 0x4u32,
	/// DXGI_FORMAT_R32G32B32_TYPELESS = 0x5u32
	R32G32B32Typeless    = 0x5u32,
	/// DXGI_FORMAT_R32G32B32_FLOAT = 0x6u32
	R32G32B32Float       = 0x6u32,
	/// DXGI_FORMAT_R32G32B32_UINT = 0x7u32
	R32G32B32Uint        = 0x7u32,
	/// DXGI_FORMAT_R32G32B32_SINT = 0x8u32
	R32G32B32Sint        = 0x8u32,
	/// DXGI_FORMAT_R16G16B16A16_TYPELESS = 0x9u32
	R16G16B16A16Typeless = 0x9u32,
	/// DXGI_FORMAT_R16G16B16A16_FLOAT = 0xAu32
	R16G16B16A16Float    = 0xAu32,
	/// DXGI_FORMAT_R16G16B16A16_UNORM = 0xBu32
	R16G16B16A16Unorm    = 0xBu32,
	/// DXGI_FORMAT_R16G16B16A16_UINT = 0xCu32
	R16G16B16A16Uint     = 0xCu32,
	/// DXGI_FORMAT_R16G16B16A16_SNORM = 0xDu32
	R16G16B16A16Snorm    = 0xDu32,
	/// DXGI_FORMAT_R16G16B16A16_SINT = 0xEu32
	R16G16B16A16Sint     = 0xEu32,
	/// DXGI_FORMAT_R32G32_TYPELESS = 0xFu32
	R32G32Typeless       = 0xFu32,
	/// DXGI_FORMAT_R32G32_FLOAT = 0x10u32
	R32G32Float          = 0x10u32,
	/// DXGI_FORMAT_R32G32_UINT = 0x11u32
	R32G32Uint           = 0x11u32,
	/// DXGI_FORMAT_R32G32_SINT = 0x12u32
	R32G32Sint           = 0x12u32,
	/// DXGI_FORMAT_R32G8X24_TYPELESS = 0x13u32
	R32G8X24Typeless     = 0x13u32,
	/// DXGI_FORMAT_D32_FLOAT_S8X24_UINT = 0x14u32
	D32FloatS8X24Uint    = 0x14u32,
	/// DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS = 0x15u32
	R32FloatX8X24Typeless = 0x15u32,
	/// DXGI_FORMAT_X32_TYPELESS_G8X24_UINT = 0x16u32
	X32TypelessG8X24Uint = 0x16u32,
	/// DXGI_FORMAT_R10G10B10A2_TYPELESS = 0x17u32
	R10G10B10A2Typeless  = 0x17u32,
	/// DXGI_FORMAT_R10G10B10A2_UNORM = 0x18u32
	R10G10B10A2Unorm     = 0x18u32,
	/// DXGI_FORMAT_R10G10B10A2_UINT = 0x19u32
	R10G10B10A2Uint      = 0x19u32,
	/// DXGI_FORMAT_R11G11B10_FLOAT = 0x1Au32
	R11G11B10Float       = 0x1Au32,
	/// DXGI_FORMAT_R8G8B8A8_TYPELESS = 0x1Bu32
	R8G8B8A8Typeless     = 0x1Bu32,
	/// DXGI_FORMAT_R8G8B8A8_UNORM = 0x1Cu32
	R8G8B8A8Unorm        = 0x1Cu32,
	/// DXGI_FORMAT_R8G8B8A8_UNORM_SRGB = 0x1Du32
	R8G8B8A8UnormSrgb    = 0x1Du32,
	/// DXGI_FORMAT_R8G8B8A8_UINT = 0x1Eu32
	R8G8B8A8Uint         = 0x1Eu32,
	/// DXGI_FORMAT_R8G8B8A8_SNORM = 0x1Fu32
	R8G8B8A8Snorm        = 0x1Fu32,
	/// DXGI_FORMAT_R8G8B8A8_SINT = 0x20u32
	R8G8B8A8Sint         = 0x20u32,
	/// DXGI_FORMAT_R16G16_TYPELESS = 0x21u32
	R16G16Typeless       = 0x21u32,
	/// DXGI_FORMAT_R16G16_FLOAT = 0x22u32
	R16G16Float          = 0x22u32,
	/// DXGI_FORMAT_R16G16_UNORM = 0x23u32
	R16G16Unorm          = 0x23u32,
	/// DXGI_FORMAT_R16G16_UINT = 0x24u32
	R16G16Uint           = 0x24u32,
	/// DXGI_FORMAT_R16G16_SNORM = 0x25u32
	R16G16Snorm          = 0x25u32,
	/// DXGI_FORMAT_R16G16_SINT = 0x26u32
	R16G16Sint           = 0x26u32,
	/// DXGI_FORMAT_R32_TYPELESS = 0x27u32
	R32Typeless          = 0x27u32,
	/// DXGI_FORMAT_D32_FLOAT = 0x28u32
	D32Float             = 0x28u32,
	/// DXGI_FORMAT_R32_FLOAT = 0x29u32
	R32Float             = 0x29u32,
	/// DXGI_FORMAT_R32_UINT = 0x2Au32
	R32Uint              = 0x2Au32,
	/// DXGI_FORMAT_R32_SINT = 0x2Bu32
	R32Sint              = 0x2Bu32,
	/// DXGI_FORMAT_R24G8_TYPELESS = 0x2Cu32
	R24G8Typeless        = 0x2Cu32,
	/// DXGI_FORMAT_D24_UNORM_S8_UINT = 0x2Du32
	D24UnormS8Uint       = 0x2Du32,
	/// DXGI_FORMAT_R24_UNORM_X8_TYPELESS = 0x2Eu32
	R24UnormX8Typeless   = 0x2Eu32,
	/// DXGI_FORMAT_X24_TYPELESS_G8_UINT = 0x2Fu32
	X24TypelessG8Uint    = 0x2Fu32,
	/// DXGI_FORMAT_R8G8_TYPELESS = 0x30u32
	R8G8Typeless         = 0x30u32,
	/// DXGI_FORMAT_R8G8_UNORM = 0x31u32
	R8G8Unorm            = 0x31u32,
	/// DXGI_FORMAT_R8G8_UINT = 0x32u32
	R8G8Uint             = 0x32u32,
	/// DXGI_FORMAT_R8G8_SNORM = 0x33u32
	R8G8Snorm            = 0x33u32,
	/// DXGI_FORMAT_R8G8_SINT = 0x34u32
	R8G8Sint             = 0x34u32,
	/// DXGI_FORMAT_R16_TYPELESS = 0x35u32
	R16Typeless          = 0x35u32,
	/// DXGI_FORMAT_R16_FLOAT = 0x36u32
	R16Float             = 0x36u32,
	/// DXGI_FORMAT_D16_UNORM = 0x37u32
	D16Unorm             = 0x37u32,
	/// DXGI_FORMAT_R16_UNORM = 0x38u32
	R16Unorm             = 0x38u32,
	/// DXGI_FORMAT_R16_UINT = 0x39u32
	R16Uint              = 0x39u32,
	/// DXGI_FORMAT_R16_SNORM = 0x3Au32
	R16Snorm             = 0x3Au32,
	/// DXGI_FORMAT_R16_SINT = 0x3Bu32
	R16Sint              = 0x3Bu32,
	/// DXGI_FORMAT_R8_TYPELESS = 0x3Cu32
	R8Typeless           = 0x3Cu32,
	/// DXGI_FORMAT_R8_UNORM = 0x3Du32
	R8Unorm              = 0x3Du32,
	/// DXGI_FORMAT_R8_UINT = 0x3Eu32
	R8Uint               = 0x3Eu32,
	/// DXGI_FORMAT_R8_SNORM = 0x3Fu32
	R8Snorm              = 0x3Fu32,
	/// DXGI_FORMAT_R8_SINT = 0x40u32
	R8Sint               = 0x40u32,
	/// DXGI_FORMAT_A8_UNORM = 0x41u32
	A8Unorm              = 0x41u32,
	/// DXGI_FORMAT_R1_UNORM = 0x42u32
	R1Unorm              = 0x42u32,
	/// DXGI_FORMAT_R9G9B9E5_SHAREDEXP = 0x43u32
	R9G9B9E5SharedExp    = 0x43u32,
	/// DXGI_FORMAT_R8G8_B8G8_UNORM = 0x44u32
	R8G8B8G8Unorm        = 0x44u32,
	/// DXGI_FORMAT_G8R8_G8B8_UNORM = 0x45u32
	G8R8G8B8Unorm        = 0x45u32,
	/// DXGI_FORMAT_BC1_TYPELESS = 0x46u32
	Bc1Typeless          = 0x46u32,
	/// DXGI_FORMAT_BC1_UNORM = 0x47u32
	Bc1Unorm             = 0x47u32,
	/// DXGI_FORMAT_BC1_UNORM_SRGB = 0x48u32
	Bc1UnormSrgb         = 0x48u32,
	/// DXGI_FORMAT_BC2_TYPELESS = 0x49u32
	Bc2Typeless          = 0x49u32,
	/// DXGI_FORMAT_BC2_UNORM = 0x4Au32
	Bc2Unorm             = 0x4Au32,
	/// DXGI_FORMAT_BC2_UNORM_SRGB = 0x4Bu32
	Bc2UnormSrgb         = 0x4Bu32,
	/// DXGI_FORMAT_BC3_TYPELESS = 0x4Cu32
	Bc3Typeless          = 0x4Cu32,
	/// DXGI_FORMAT_BC3_UNORM = 0x4Du32
	Bc3Unorm             = 0x4Du32,
	/// DXGI_FORMAT_BC3_UNORM_SRGB = 0x4Eu32
	Bc3UnormSrgb         = 0x4Eu32,
	/// DXGI_FORMAT_BC4_TYPELESS = 0x4Fu32
	Bc4Typeless          = 0x4Fu32,
	/// DXGI_FORMAT_BC4_UNORM = 0x50u32
	Bc4Unorm             = 0x50u32,
	/// DXGI_FORMAT_BC4_SNORM = 0x51u32
	Bc4Snorm             = 0x51u32,
	/// DXGI_FORMAT_BC5_TYPELESS = 0x52u32
	Bc5Typeless          = 0x52u32,
	/// DXGI_FORMAT_BC5_UNORM = 0x53u32
	Bc5Unorm             = 0x53u32,
	/// DXGI_FORMAT_BC5_SNORM = 0x54u32
	Bc5Snorm             = 0x54u32,
	/// DXGI_FORMAT_B5G6R5_UNORM = 0x55u32
	B5G6R5Unorm          = 0x55u32,
	/// DXGI_FORMAT_B5G5R5A1_UNORM = 0x56u32
	B5G5R5A1Unorm        = 0x56u32,
	/// DXGI_FORMAT_B8G8R8A8_UNORM = 0x57u32
	B8G8R8A8Unorm        = 0x57u32,
	/// DXGI_FORMAT_B8G8R8X8_UNORM = 0x58u32
	B8G8R8X8Unorm        = 0x58u32,
	/// DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM = 0x59u32
	R10G10B10XrBiasA2Unorm = 0x59u32,
	/// DXGI_FORMAT_B8G8R8A8_TYPELESS = 0x5Au32
	B8G8R8A8Typeless     = 0x5Au32,
	/// DXGI_FORMAT_B8G8R8A8_UNORM_SRGB = 0x5Bu32
	B8G8R8A8UnormSrgb    = 0x5Bu32,
	/// DXGI_FORMAT_B8G8R8X8_TYPELESS = 0x5Cu32
	B8G8R8X8Typeless     = 0x5Cu32,
	/// DXGI_FORMAT_B8G8R8X8_UNORM_SRGB = 0x5Du32
	B8G8R8X8UnormSrgb    = 0x5Du32,
	/// DXGI_FORMAT_BC6H_TYPELESS = 0x5Eu32
	BC6HTypeless         = 0x5Eu32,
	/// DXGI_FORMAT_BC6H_UF16 = 0x5Fu32
	BC6HUF16             = 0x5Fu32,
	/// DXGI_FORMAT_BC6H_SF16 = 0x60u32
	BC6HSF16             = 0x60u32,
	/// DXGI_FORMAT_BC7_TYPELESS = 0x61u32
	Bc7Typeless          = 0x61u32,
	/// DXGI_FORMAT_BC7_UNORM = 0x62u32
	Bc7Unorm             = 0x62u32,
	/// DXGI_FORMAT_BC7_UNORM_SRGB = 0x63u32
	Bc7UnormSrgb         = 0x63u32,
	/// DXGI_FORMAT_AYUV = 0x64u32
	AYuv                 = 0x64u32,
	/// DXGI_FORMAT_Y410 = 0x65u32
	Y410                 = 0x65u32,
	/// DXGI_FORMAT_Y416 = 0x66u32
	Y416                 = 0x66u32,
	/// DXGI_FORMAT_NV12 = 0x67u32
	NV12                 = 0x67u32,
	/// DXGI_FORMAT_P010 = 0x68u32
	P010                 = 0x68u32,
	/// DXGI_FORMAT_P016 = 0x69u32
	P016                 = 0x69u32,
	/// DXGI_FORMAT_420_OPAQUE = 0x6Au32
	_420Opaque           = 0x6Au32,
	/// DXGI_FORMAT_YUY2 = 0x6Bu32
	YUY2                 = 0x6Bu32,
	/// DXGI_FORMAT_Y210 = 0x6Cu32
	Y210                 = 0x6Cu32,
	/// DXGI_FORMAT_Y216 = 0x6Du32
	Y216                 = 0x6Du32,
	/// DXGI_FORMAT_NV11 = 0x6Eu32
	NV11                 = 0x6Eu32,
	/// DXGI_FORMAT_AI44 = 0x6Fu32
	Ai44                 = 0x6Fu32,
	/// DXGI_FORMAT_IA44 = 0x70u32
	Ia44                 = 0x70u32,
	/// DXGI_FORMAT_P8 = 0x71u32
	P8                   = 0x71u32,
	/// DXGI_FORMAT_A8P8 = 0x72u32
	A8P8                 = 0x72u32,
	/// DXGI_FORMAT_B4G4R4A4_UNORM = 0x73u32
	B4G4R4A4Unorm        = 0x73u32,
	/// DXGI_FORMAT_P208 = 0x82u32
	P208                 = 0x82u32,
	/// DXGI_FORMAT_V208 = 0x83u32
	V208                 = 0x83u32,
	/// DXGI_FORMAT_V408 = 0x84u32
	V408                 = 0x84u32,
	/// DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE = 0xBDu32
	SamplerFeedbackMinMipOpaque = 0xBDu32,
	/// DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE = 0xBEu32
	SamplerFeedbackMipRegionUsedOpaque = 0xBEu32,
	/// DXGI_FORMAT_FORCE_UINT = 0xFFFFFFFFu32
	ForceUint            = 0xFFFFFFFFu32,
}

/// DXGI_MODE_ROTATION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiModeRotation
{
	/// DXGI_MODE_ROTATION_UNSPECIFIED = 0x0i32
	Unspecified          = 0x0i32,
	/// DXGI_MODE_ROTATION_IDENTITY = 0x1i32
	Identity             = 0x1i32,
	/// DXGI_MODE_ROTATION_ROTATE90 = 0x2i32
	Rotate90             = 0x2i32,
	/// DXGI_MODE_ROTATION_ROTATE180 = 0x3i32
	Rotate180            = 0x3i32,
	/// DXGI_MODE_ROTATION_ROTATE270 = 0x4i32
	Rotate270            = 0x4i32,
}

/// DXGI_MODE_SCANLINE_ORDER
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiModeScanLineOrder
{
	/// DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED = 0x0i32
	Unspecified          = 0x0i32,
	/// DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE = 0x1i32
	Progressive          = 0x1i32,
	/// DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST = 0x2i32
	UpperFieldFirst      = 0x2i32,
	/// DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST = 0x3i32
	LowerFieldFirst      = 0x3i32,
}

/// DXGI_MODE_SCALING
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiModeScaling
{
	/// DXGI_MODE_SCALING_UNSPECIFIED = 0x0i32
	Unspecified          = 0x0i32,
	/// DXGI_MODE_SCALING_CENTERED = 0x1i32
	Centered             = 0x1i32,
	/// DXGI_MODE_SCALING_STRETCHED = 0x2i32
	Stretched            = 0x2i32,
}

/// DXGI_ALPHA_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiAlphaMode
{
	/// DXGI_ALPHA_MODE_UNSPECIFIED = 0x0u32
	Unspecified          = 0x0u32,
	/// DXGI_ALPHA_MODE_PREMULTIPLIED = 0x1u32
	Premultiplied        = 0x1u32,
	/// DXGI_ALPHA_MODE_STRAIGHT = 0x2u32
	Straight             = 0x2u32,
	/// DXGI_ALPHA_MODE_IGNORE = 0x3u32
	Ignore               = 0x3u32,
	/// DXGI_ALPHA_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// DXGI_COLOR_SPACE_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxgiColorSpaceType
{
	/// DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709 = 0x0i32
	RgbFullG22NoneP709   = 0x0i32,
	/// DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709 = 0x1i32
	RgbFullG10NoneP709   = 0x1i32,
	/// DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709 = 0x2i32
	RgbStudioG22NoneP709 = 0x2i32,
	/// DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020 = 0x3i32
	RgbStudioG22NoneP2020 = 0x3i32,
	/// DXGI_COLOR_SPACE_RESERVED = 0x4i32
	Reserved             = 0x4i32,
	/// DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601 = 0x5i32
	YCbCrFullG22NoneP709X601 = 0x5i32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601 = 0x6i32
	YCbCrStudioG22LeftP601 = 0x6i32,
	/// DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601 = 0x7i32
	YCbCrFullG22LeftP601 = 0x7i32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709 = 0x8i32
	YCbCrStudioG22LeftP709 = 0x8i32,
	/// DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709 = 0x9i32
	YCbCrFullG22LeftP709 = 0x9i32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020 = 0xAi32
	YCbCrStudioG22LeftP2020 = 0xAi32,
	/// DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020 = 0xBi32
	YCbCrFullG22LeftP2020 = 0xBi32,
	/// DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020 = 0xCi32
	RgbFullG2084NoneP2020 = 0xCi32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020 = 0xDi32
	YCbCrStudioG2084LeftP2020 = 0xDi32,
	/// DXGI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020 = 0xEi32
	RgbStudioG2084NoneP2020 = 0xEi32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020 = 0xFi32
	YCbCrStudioG22TopLeftP2020 = 0xFi32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020 = 0x10i32
	YCbCrStudioG2084TopLeftP2020 = 0x10i32,
	/// DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020 = 0x11i32
	RgbFullG22NoneP2020  = 0x11i32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020 = 0x12i32
	YCbCrStudioGHLGTopLeftP2020 = 0x12i32,
	/// DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020 = 0x13i32
	YCbCrFullGHLGTopLeftP2020 = 0x13i32,
	/// DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709 = 0x14i32
	RgbStudioG24NoneP709 = 0x14i32,
	/// DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020 = 0x15i32
	RgbStudioG24NoneP2020 = 0x15i32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709 = 0x16i32
	YCbCrStudioG24LeftP709 = 0x16i32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020 = 0x17i32
	YCbCrStudioG24LeftP2020 = 0x17i32,
	/// DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020 = 0x18i32
	YCbCrStudioG24TopLeftP2020 = 0x18i32,
	/// DXGI_COLOR_SPACE_CUSTOM = -0x1i32
	Custom               = -0x1i32,
}

