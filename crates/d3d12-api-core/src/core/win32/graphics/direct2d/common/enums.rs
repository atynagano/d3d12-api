#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// D2D1_ALPHA_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1AlphaMode
{
	/// D2D1_ALPHA_MODE_UNKNOWN = 0x0u32
	Unknown              = 0x0u32,
	/// D2D1_ALPHA_MODE_PREMULTIPLIED = 0x1u32
	Premultiplied        = 0x1u32,
	/// D2D1_ALPHA_MODE_STRAIGHT = 0x2u32
	Straight             = 0x2u32,
	/// D2D1_ALPHA_MODE_IGNORE = 0x3u32
	Ignore               = 0x3u32,
	/// D2D1_ALPHA_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_FIGURE_BEGIN
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1FigureBegin
{
	/// D2D1_FIGURE_BEGIN_FILLED = 0x0u32
	Filled               = 0x0u32,
	/// D2D1_FIGURE_BEGIN_HOLLOW = 0x1u32
	Hollow               = 0x1u32,
	/// D2D1_FIGURE_BEGIN_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_FIGURE_END
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1FigureEnd
{
	/// D2D1_FIGURE_END_OPEN = 0x0u32
	Open                 = 0x0u32,
	/// D2D1_FIGURE_END_CLOSED = 0x1u32
	Closed               = 0x1u32,
	/// D2D1_FIGURE_END_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_PATH_SEGMENT
	#[repr(transparent)]
	pub struct D2D1PathSegment: u32 {
		/// D2D1_PATH_SEGMENT_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_PATH_SEGMENT_FORCE_UNSTROKED = 0x1u32
		const ForceUnstroked       = 0x1u32;
		/// D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN = 0x2u32
		const ForceRoundLineJoin   = 0x2u32;
		/// D2D1_PATH_SEGMENT_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_FILL_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1FillMode
{
	/// D2D1_FILL_MODE_ALTERNATE = 0x0u32
	Alternate            = 0x0u32,
	/// D2D1_FILL_MODE_WINDING = 0x1u32
	Winding              = 0x1u32,
	/// D2D1_FILL_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BORDER_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BorderMode
{
	/// D2D1_BORDER_MODE_SOFT = 0x0u32
	Soft                 = 0x0u32,
	/// D2D1_BORDER_MODE_HARD = 0x1u32
	Hard                 = 0x1u32,
	/// D2D1_BORDER_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BLEND_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BlendMode
{
	/// D2D1_BLEND_MODE_MULTIPLY = 0x0u32
	Multiply             = 0x0u32,
	/// D2D1_BLEND_MODE_SCREEN = 0x1u32
	Screen               = 0x1u32,
	/// D2D1_BLEND_MODE_DARKEN = 0x2u32
	Darken               = 0x2u32,
	/// D2D1_BLEND_MODE_LIGHTEN = 0x3u32
	Lighten              = 0x3u32,
	/// D2D1_BLEND_MODE_DISSOLVE = 0x4u32
	Dissolve             = 0x4u32,
	/// D2D1_BLEND_MODE_COLOR_BURN = 0x5u32
	ColorBurn            = 0x5u32,
	/// D2D1_BLEND_MODE_LINEAR_BURN = 0x6u32
	LinearBurn           = 0x6u32,
	/// D2D1_BLEND_MODE_DARKER_COLOR = 0x7u32
	DarkerColor          = 0x7u32,
	/// D2D1_BLEND_MODE_LIGHTER_COLOR = 0x8u32
	LighterColor         = 0x8u32,
	/// D2D1_BLEND_MODE_COLOR_DODGE = 0x9u32
	ColorDodge           = 0x9u32,
	/// D2D1_BLEND_MODE_LINEAR_DODGE = 0xAu32
	LinearDodge          = 0xAu32,
	/// D2D1_BLEND_MODE_OVERLAY = 0xBu32
	Overlay              = 0xBu32,
	/// D2D1_BLEND_MODE_SOFT_LIGHT = 0xCu32
	SoftLight            = 0xCu32,
	/// D2D1_BLEND_MODE_HARD_LIGHT = 0xDu32
	HardLight            = 0xDu32,
	/// D2D1_BLEND_MODE_VIVID_LIGHT = 0xEu32
	VividLight           = 0xEu32,
	/// D2D1_BLEND_MODE_LINEAR_LIGHT = 0xFu32
	LinearLight          = 0xFu32,
	/// D2D1_BLEND_MODE_PIN_LIGHT = 0x10u32
	PinLight             = 0x10u32,
	/// D2D1_BLEND_MODE_HARD_MIX = 0x11u32
	HardMix              = 0x11u32,
	/// D2D1_BLEND_MODE_DIFFERENCE = 0x12u32
	Difference           = 0x12u32,
	/// D2D1_BLEND_MODE_EXCLUSION = 0x13u32
	Exclusion            = 0x13u32,
	/// D2D1_BLEND_MODE_HUE = 0x14u32
	Hue                  = 0x14u32,
	/// D2D1_BLEND_MODE_SATURATION = 0x15u32
	Saturation           = 0x15u32,
	/// D2D1_BLEND_MODE_COLOR = 0x16u32
	Color                = 0x16u32,
	/// D2D1_BLEND_MODE_LUMINOSITY = 0x17u32
	Luminosity           = 0x17u32,
	/// D2D1_BLEND_MODE_SUBTRACT = 0x18u32
	Subtract             = 0x18u32,
	/// D2D1_BLEND_MODE_DIVISION = 0x19u32
	Division             = 0x19u32,
	/// D2D1_BLEND_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLORMATRIX_ALPHA_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorMatrixAlphaMode
{
	/// D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED = 0x1u32
	Premultiplied        = 0x1u32,
	/// D2D1_COLORMATRIX_ALPHA_MODE_STRAIGHT = 0x2u32
	Straight             = 0x2u32,
	/// D2D1_COLORMATRIX_ALPHA_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1_2dAffineTransformInterpolationMode
{
	/// D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_TURBULENCE_NOISE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1TurbulenceNoise
{
	/// D2D1_TURBULENCE_NOISE_FRACTAL_SUM = 0x0u32
	FractalSum           = 0x0u32,
	/// D2D1_TURBULENCE_NOISE_TURBULENCE = 0x1u32
	Turbulence           = 0x1u32,
	/// D2D1_TURBULENCE_NOISE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COMPOSITE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1CompositeMode
{
	/// D2D1_COMPOSITE_MODE_SOURCE_OVER = 0x0u32
	SourceOver           = 0x0u32,
	/// D2D1_COMPOSITE_MODE_DESTINATION_OVER = 0x1u32
	DestinationOver      = 0x1u32,
	/// D2D1_COMPOSITE_MODE_SOURCE_IN = 0x2u32
	SourceIn             = 0x2u32,
	/// D2D1_COMPOSITE_MODE_DESTINATION_IN = 0x3u32
	DestinationIn        = 0x3u32,
	/// D2D1_COMPOSITE_MODE_SOURCE_OUT = 0x4u32
	SourceOut            = 0x4u32,
	/// D2D1_COMPOSITE_MODE_DESTINATION_OUT = 0x5u32
	DestinationOut       = 0x5u32,
	/// D2D1_COMPOSITE_MODE_SOURCE_ATOP = 0x6u32
	SourceAtop           = 0x6u32,
	/// D2D1_COMPOSITE_MODE_DESTINATION_ATOP = 0x7u32
	DestinationAtop      = 0x7u32,
	/// D2D1_COMPOSITE_MODE_XOR = 0x8u32
	Xor                  = 0x8u32,
	/// D2D1_COMPOSITE_MODE_PLUS = 0x9u32
	Plus                 = 0x9u32,
	/// D2D1_COMPOSITE_MODE_SOURCE_COPY = 0xAu32
	SourceCopy           = 0xAu32,
	/// D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY = 0xBu32
	BoundedSourceCopy    = 0xBu32,
	/// D2D1_COMPOSITE_MODE_MASK_INVERT = 0xCu32
	MaskInvert           = 0xCu32,
	/// D2D1_COMPOSITE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

