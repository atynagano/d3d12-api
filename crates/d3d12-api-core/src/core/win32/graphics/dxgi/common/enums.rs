#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiFormat
{
	Unknown              = 0x0u32,
	R32G32B32A32Typeless = 0x1u32,
	R32G32B32A32Float    = 0x2u32,
	R32G32B32A32UInt     = 0x3u32,
	R32G32B32A32SInt     = 0x4u32,
	R32G32B32Typeless    = 0x5u32,
	R32G32B32Float       = 0x6u32,
	R32G32B32UInt        = 0x7u32,
	R32G32B32SInt        = 0x8u32,
	R16G16B16A16Typeless = 0x9u32,
	R16G16B16A16Float    = 0xAu32,
	R16G16B16A16UNorm    = 0xBu32,
	R16G16B16A16UInt     = 0xCu32,
	R16G16B16A16SNorm    = 0xDu32,
	R16G16B16A16SInt     = 0xEu32,
	R32G32Typeless       = 0xFu32,
	R32G32Float          = 0x10u32,
	R32G32UInt           = 0x11u32,
	R32G32SInt           = 0x12u32,
	R32G8X24Typeless     = 0x13u32,
	D32FloatS8X24UInt    = 0x14u32,
	R32FloatX8X24Typeless = 0x15u32,
	X32TypelessG8X24UInt = 0x16u32,
	R10G10B10A2Typeless  = 0x17u32,
	R10G10B10A2UNorm     = 0x18u32,
	R10G10B10A2UInt      = 0x19u32,
	R11G11B10Float       = 0x1Au32,
	R8G8B8A8Typeless     = 0x1Bu32,
	R8G8B8A8UNorm        = 0x1Cu32,
	R8G8B8A8UNormSRgb    = 0x1Du32,
	R8G8B8A8UInt         = 0x1Eu32,
	R8G8B8A8SNorm        = 0x1Fu32,
	R8G8B8A8SInt         = 0x20u32,
	R16G16Typeless       = 0x21u32,
	R16G16Float          = 0x22u32,
	R16G16UNorm          = 0x23u32,
	R16G16UInt           = 0x24u32,
	R16G16SNorm          = 0x25u32,
	R16G16SInt           = 0x26u32,
	R32Typeless          = 0x27u32,
	D32Float             = 0x28u32,
	R32Float             = 0x29u32,
	R32UInt              = 0x2Au32,
	R32SInt              = 0x2Bu32,
	R24G8Typeless        = 0x2Cu32,
	D24UNormS8UInt       = 0x2Du32,
	R24UNormX8Typeless   = 0x2Eu32,
	X24TypelessG8UInt    = 0x2Fu32,
	R8G8Typeless         = 0x30u32,
	R8G8UNorm            = 0x31u32,
	R8G8UInt             = 0x32u32,
	R8G8SNorm            = 0x33u32,
	R8G8SInt             = 0x34u32,
	R16Typeless          = 0x35u32,
	R16Float             = 0x36u32,
	D16UNorm             = 0x37u32,
	R16UNorm             = 0x38u32,
	R16UInt              = 0x39u32,
	R16SNorm             = 0x3Au32,
	R16SInt              = 0x3Bu32,
	R8Typeless           = 0x3Cu32,
	R8UNorm              = 0x3Du32,
	R8UInt               = 0x3Eu32,
	R8SNorm              = 0x3Fu32,
	R8SInt               = 0x40u32,
	A8UNorm              = 0x41u32,
	R1UNorm              = 0x42u32,
	R9G9B9E5SharedExp    = 0x43u32,
	R8G8B8G8UNorm        = 0x44u32,
	G8R8G8B8UNorm        = 0x45u32,
	BC1Typeless          = 0x46u32,
	BC1UNorm             = 0x47u32,
	BC1UNormSRgb         = 0x48u32,
	BC2Typeless          = 0x49u32,
	BC2UNorm             = 0x4Au32,
	BC2UNormSRgb         = 0x4Bu32,
	BC3Typeless          = 0x4Cu32,
	BC3UNorm             = 0x4Du32,
	BC3UNormSRgb         = 0x4Eu32,
	BC4Typeless          = 0x4Fu32,
	BC4UNorm             = 0x50u32,
	BC4SNorm             = 0x51u32,
	BC5Typeless          = 0x52u32,
	BC5UNorm             = 0x53u32,
	BC5SNorm             = 0x54u32,
	B5G6R5UNorm          = 0x55u32,
	B5G5R5A1UNorm        = 0x56u32,
	B8G8R8A8UNorm        = 0x57u32,
	B8G8R8X8UNorm        = 0x58u32,
	R10G10B10XrBiasA2UNorm = 0x59u32,
	B8G8R8A8Typeless     = 0x5Au32,
	B8G8R8A8UNormSRgb    = 0x5Bu32,
	B8G8R8X8Typeless     = 0x5Cu32,
	B8G8R8X8UNormSRgb    = 0x5Du32,
	BC6HTypeless         = 0x5Eu32,
	BC6HUF16             = 0x5Fu32,
	BC6HSf16             = 0x60u32,
	BC7Typeless          = 0x61u32,
	BC7UNorm             = 0x62u32,
	BC7UNormSRgb         = 0x63u32,
	AyuV                 = 0x64u32,
	Y410                 = 0x65u32,
	Y416                 = 0x66u32,
	Nv12                 = 0x67u32,
	P010                 = 0x68u32,
	P016                 = 0x69u32,
	_420Opaque           = 0x6Au32,
	YUY2                 = 0x6Bu32,
	Y210                 = 0x6Cu32,
	Y216                 = 0x6Du32,
	Nv11                 = 0x6Eu32,
	Ai44                 = 0x6Fu32,
	Ia44                 = 0x70u32,
	P8                   = 0x71u32,
	A8P8                 = 0x72u32,
	B4G4R4A4UNorm        = 0x73u32,
	P208                 = 0x82u32,
	V208                 = 0x83u32,
	V408                 = 0x84u32,
	SamplerFeedbackMinMipOpaque = 0xBDu32,
	SamplerFeedbackMipRegionUsedOpaque = 0xBEu32,
	ForceUInt            = 0xFFFFFFFFu32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiModeRotation
{
	Unspecified          = 0x0i32,
	Identity             = 0x1i32,
	Rotate90             = 0x2i32,
	Rotate180            = 0x3i32,
	Rotate270            = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiModeScanLineOrder
{
	Unspecified          = 0x0i32,
	Progressive          = 0x1i32,
	UpperFieldFirst      = 0x2i32,
	LowerFieldFirst      = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiModeScaling
{
	Unspecified          = 0x0i32,
	Centered             = 0x1i32,
	Stretched            = 0x2i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiAlphaMode
{
	Unspecified          = 0x0u32,
	PRemultiplied        = 0x1u32,
	Straight             = 0x2u32,
	Ignore               = 0x3u32,
	ForceDWord           = 0xFFFFFFFFu32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxgiColorSpaceType
{
	ColorSpaceRgbFullG22NoneP709 = 0x0i32,
	ColorSpaceRgbFullG10NoneP709 = 0x1i32,
	ColorSpaceRgbStudioG22NoneP709 = 0x2i32,
	ColorSpaceRgbStudioG22NoneP2020 = 0x3i32,
	ColorSpaceReserved   = 0x4i32,
	ColorSpaceYcbcrFullG22NoneP709X601 = 0x5i32,
	ColorSpaceYcbcrStudioG22LeftP601 = 0x6i32,
	ColorSpaceYcbcrFullG22LeftP601 = 0x7i32,
	ColorSpaceYcbcrStudioG22LeftP709 = 0x8i32,
	ColorSpaceYcbcrFullG22LeftP709 = 0x9i32,
	ColorSpaceYcbcrStudioG22LeftP2020 = 0xAi32,
	ColorSpaceYcbcrFullG22LeftP2020 = 0xBi32,
	ColorSpaceRgbFullG2084NoneP2020 = 0xCi32,
	ColorSpaceYcbcrStudioG2084LeftP2020 = 0xDi32,
	ColorSpaceRgbStudioG2084NoneP2020 = 0xEi32,
	ColorSpaceYcbcrStudioG22TopLeftP2020 = 0xFi32,
	ColorSpaceYcbcrStudioG2084TopLeftP2020 = 0x10i32,
	ColorSpaceRgbFullG22NoneP2020 = 0x11i32,
	ColorSpaceYcbcrStudioGHLgTopLeftP2020 = 0x12i32,
	ColorSpaceYcbcrFullGHLgTopLeftP2020 = 0x13i32,
	ColorSpaceRgbStudioG24NoneP709 = 0x14i32,
	ColorSpaceRgbStudioG24NoneP2020 = 0x15i32,
	ColorSpaceYcbcrStudioG24LeftP709 = 0x16i32,
	ColorSpaceYcbcrStudioG24LeftP2020 = 0x17i32,
	ColorSpaceYcbcrStudioG24TopLeftP2020 = 0x18i32,
	ColorSpaceCustom     = -0x1i32,
}

