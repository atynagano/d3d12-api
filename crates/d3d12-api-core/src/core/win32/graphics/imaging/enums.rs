#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// WICColorContextType
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicColorContextType
{
	/// WICColorContextUninitialized = 0x0i32
	WICColorContextUninitialized = 0x0i32,
	/// WICColorContextProfile = 0x1i32
	WICColorContextProfile = 0x1i32,
	/// WICColorContextExifColorSpace = 0x2i32
	WICColorContextExifColorSpace = 0x2i32,
}

/// WICDecodeOptions
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicDecodeOptions
{
	/// WICDecodeMetadataCacheOnDemand = 0x0i32
	WICDecodeMetadataCacheOnDemand = 0x0i32,
	/// WICDecodeMetadataCacheOnLoad = 0x1i32
	WICDecodeMetadataCacheOnLoad = 0x1i32,
	/// WICMETADATACACHEOPTION_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

/// WICBitmapCreateCacheOption
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicBitmapCreateCacheOption
{
	/// WICBitmapNoCache = 0x0i32
	WICBitmapNoCache     = 0x0i32,
	/// WICBitmapCacheOnDemand = 0x1i32
	WICBitmapCacheOnDemand = 0x1i32,
	/// WICBitmapCacheOnLoad = 0x2i32
	WICBitmapCacheOnLoad = 0x2i32,
	/// WICBITMAPCREATECACHEOPTION_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

/// WICBitmapAlphaChannelOption
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicBitmapAlphaChannelOption
{
	/// WICBitmapUseAlpha = 0x0i32
	WICBitmapUseAlpha    = 0x0i32,
	/// WICBitmapUsePremultipliedAlpha = 0x1i32
	WICBitmapUsePremultipliedAlpha = 0x1i32,
	/// WICBitmapIgnoreAlpha = 0x2i32
	WICBitmapIgnoreAlpha = 0x2i32,
	/// WICBITMAPALPHACHANNELOPTIONS_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

/// WICBitmapPaletteType
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicBitmapPaletteType
{
	/// WICBitmapPaletteTypeCustom = 0x0i32
	Custom               = 0x0i32,
	/// WICBitmapPaletteTypeMedianCut = 0x1i32
	MedianCut            = 0x1i32,
	/// WICBitmapPaletteTypeFixedBW = 0x2i32
	FixedBW              = 0x2i32,
	/// WICBitmapPaletteTypeFixedHalftone8 = 0x3i32
	FixedHalftone8       = 0x3i32,
	/// WICBitmapPaletteTypeFixedHalftone27 = 0x4i32
	FixedHalftone27      = 0x4i32,
	/// WICBitmapPaletteTypeFixedHalftone64 = 0x5i32
	FixedHalftone64      = 0x5i32,
	/// WICBitmapPaletteTypeFixedHalftone125 = 0x6i32
	FixedHalftone125     = 0x6i32,
	/// WICBitmapPaletteTypeFixedHalftone216 = 0x7i32
	FixedHalftone216     = 0x7i32,
	/// WICBitmapPaletteTypeFixedHalftone252 = 0x8i32
	FixedHalftone252     = 0x8i32,
	/// WICBitmapPaletteTypeFixedHalftone256 = 0x9i32
	FixedHalftone256     = 0x9i32,
	/// WICBitmapPaletteTypeFixedGray4 = 0xAi32
	FixedGray4           = 0xAi32,
	/// WICBitmapPaletteTypeFixedGray16 = 0xBi32
	FixedGray16          = 0xBi32,
	/// WICBitmapPaletteTypeFixedGray256 = 0xCi32
	FixedGray256         = 0xCi32,
	/// WICBITMAPPALETTETYPE_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

impl WicBitmapPaletteType {
	/// WICBitmapPaletteTypeFixedWebPalette = 0x7i32
	pub const FixedWebPalette     : Self = unsafe { transmute(0x7i32) };
}

/// WICComponentType
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicComponentType
{
	/// WICDecoder = 0x1i32
	WICDecoder           = 0x1i32,
	/// WICEncoder = 0x2i32
	WICEncoder           = 0x2i32,
	/// WICPixelFormatConverter = 0x4i32
	WICPixelFormatConverter = 0x4i32,
	/// WICMetadataReader = 0x8i32
	WICMetadataReader    = 0x8i32,
	/// WICMetadataWriter = 0x10i32
	WICMetadataWriter    = 0x10i32,
	/// WICPixelFormat = 0x20i32
	WICPixelFormat       = 0x20i32,
	/// WICAllComponents = 0x3Fi32
	WICAllComponents     = 0x3Fi32,
	/// WICCOMPONENTTYPE_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

/// WICBitmapEncoderCacheOption
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicBitmapEncoderCacheOption
{
	/// WICBitmapEncoderCacheInMemory = 0x0i32
	WICBitmapEncoderCacheInMemory = 0x0i32,
	/// WICBitmapEncoderCacheTempFile = 0x1i32
	WICBitmapEncoderCacheTempFile = 0x1i32,
	/// WICBitmapEncoderNoCache = 0x2i32
	WICBitmapEncoderNoCache = 0x2i32,
	/// WICBITMAPENCODERCACHEOPTION_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

/// WICBitmapDitherType
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicBitmapDitherType
{
	/// WICBitmapDitherTypeNone = 0x0i32
	None                 = 0x0i32,
	/// WICBitmapDitherTypeOrdered4x4 = 0x1i32
	Ordered4x4           = 0x1i32,
	/// WICBitmapDitherTypeOrdered8x8 = 0x2i32
	Ordered8x8           = 0x2i32,
	/// WICBitmapDitherTypeOrdered16x16 = 0x3i32
	Ordered16x16         = 0x3i32,
	/// WICBitmapDitherTypeSpiral4x4 = 0x4i32
	Spiral4x4            = 0x4i32,
	/// WICBitmapDitherTypeSpiral8x8 = 0x5i32
	Spiral8x8            = 0x5i32,
	/// WICBitmapDitherTypeDualSpiral4x4 = 0x6i32
	DualSpiral4x4        = 0x6i32,
	/// WICBitmapDitherTypeDualSpiral8x8 = 0x7i32
	DualSpiral8x8        = 0x7i32,
	/// WICBitmapDitherTypeErrorDiffusion = 0x8i32
	ErrorDiffusion       = 0x8i32,
	/// WICBITMAPDITHERTYPE_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

impl WicBitmapDitherType {
	/// WICBitmapDitherTypeSolid = 0x0i32
	pub const Solid               : Self = unsafe { transmute(0x0i32) };
}

/// WICBitmapInterpolationMode
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicBitmapInterpolationMode
{
	/// WICBitmapInterpolationModeNearestNeighbor = 0x0i32
	NearestNeighbor      = 0x0i32,
	/// WICBitmapInterpolationModeLinear = 0x1i32
	Linear               = 0x1i32,
	/// WICBitmapInterpolationModeCubic = 0x2i32
	Cubic                = 0x2i32,
	/// WICBitmapInterpolationModeFant = 0x3i32
	Fant                 = 0x3i32,
	/// WICBitmapInterpolationModeHighQualityCubic = 0x4i32
	HighQualityCubic     = 0x4i32,
	/// WICBITMAPINTERPOLATIONMODE_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

/// WICBitmapTransformOptions
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WicBitmapTransformOptions
{
	/// WICBitmapTransformRotate0 = 0x0i32
	WICBitmapTransformRotate0 = 0x0i32,
	/// WICBitmapTransformRotate90 = 0x1i32
	WICBitmapTransformRotate90 = 0x1i32,
	/// WICBitmapTransformRotate180 = 0x2i32
	WICBitmapTransformRotate180 = 0x2i32,
	/// WICBitmapTransformRotate270 = 0x3i32
	WICBitmapTransformRotate270 = 0x3i32,
	/// WICBitmapTransformFlipHorizontal = 0x8i32
	WICBitmapTransformFlipHorizontal = 0x8i32,
	/// WICBitmapTransformFlipVertical = 0x10i32
	WICBitmapTransformFlipVertical = 0x10i32,
	/// WICBITMAPTRANSFORMOPTIONS_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

