#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// D2D1_INTERPOLATION_MODE_DEFINITION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1InterpolationModeDefinition
{
	/// D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR = 0x0i32
	NearestNeighbor      = 0x0i32,
	/// D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR = 0x1i32
	Linear               = 0x1i32,
	/// D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC = 0x2i32
	Cubic                = 0x2i32,
	/// D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR = 0x3i32
	MultiSampleLinear    = 0x3i32,
	/// D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC = 0x4i32
	Anisotropic          = 0x4i32,
	/// D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC = 0x5i32
	HighQualityCubic     = 0x5i32,
	/// D2D1_INTERPOLATION_MODE_DEFINITION_FANT = 0x6i32
	Fant                 = 0x6i32,
	/// D2D1_INTERPOLATION_MODE_DEFINITION_MIPMAP_LINEAR = 0x7i32
	MipMapLinear         = 0x7i32,
}

/// D2D1_GAMMA
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1Gamma
{
	/// D2D1_GAMMA_2_2 = 0x0u32
	_2_2                 = 0x0u32,
	/// D2D1_GAMMA_1_0 = 0x1u32
	_1_0                 = 0x1u32,
	/// D2D1_GAMMA_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_OPACITY_MASK_CONTENT
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1OpacityMaskContent
{
	/// D2D1_OPACITY_MASK_CONTENT_GRAPHICS = 0x0u32
	Graphics             = 0x0u32,
	/// D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL = 0x1u32
	TextNatural          = 0x1u32,
	/// D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE = 0x2u32
	TextGdiCompatible    = 0x2u32,
	/// D2D1_OPACITY_MASK_CONTENT_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_EXTEND_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ExtendMode
{
	/// D2D1_EXTEND_MODE_CLAMP = 0x0u32
	Clamp                = 0x0u32,
	/// D2D1_EXTEND_MODE_WRAP = 0x1u32
	Wrap                 = 0x1u32,
	/// D2D1_EXTEND_MODE_MIRROR = 0x2u32
	Mirror               = 0x2u32,
	/// D2D1_EXTEND_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_ANTIALIAS_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1AntiAliasMode
{
	/// D2D1_ANTIALIAS_MODE_PER_PRIMITIVE = 0x0u32
	PerPrimitive         = 0x0u32,
	/// D2D1_ANTIALIAS_MODE_ALIASED = 0x1u32
	Aliased              = 0x1u32,
	/// D2D1_ANTIALIAS_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_TEXT_ANTIALIAS_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1TextAntiAliasMode
{
	/// D2D1_TEXT_ANTIALIAS_MODE_DEFAULT = 0x0u32
	Default              = 0x0u32,
	/// D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE = 0x1u32
	ClearType            = 0x1u32,
	/// D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE = 0x2u32
	GrayScale            = 0x2u32,
	/// D2D1_TEXT_ANTIALIAS_MODE_ALIASED = 0x3u32
	Aliased              = 0x3u32,
	/// D2D1_TEXT_ANTIALIAS_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BITMAP_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BitmapInterpolationMode
{
	/// D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_BITMAP_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_BITMAP_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_DRAW_TEXT_OPTIONS
	#[repr(transparent)]
	pub struct D2D1DrawTextOptions: u32 {
		/// D2D1_DRAW_TEXT_OPTIONS_NO_SNAP = 0x1u32
		const NoSnap               = 0x1u32;
		/// D2D1_DRAW_TEXT_OPTIONS_CLIP = 0x2u32
		const Clip                 = 0x2u32;
		/// D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT = 0x4u32
		const EnableColorFont      = 0x4u32;
		/// D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING = 0x8u32
		const DisableColorBitmapSnapping = 0x8u32;
		/// D2D1_DRAW_TEXT_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_DRAW_TEXT_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_ARC_SIZE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ArcSize
{
	/// D2D1_ARC_SIZE_SMALL = 0x0u32
	Small                = 0x0u32,
	/// D2D1_ARC_SIZE_LARGE = 0x1u32
	Large                = 0x1u32,
	/// D2D1_ARC_SIZE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CAP_STYLE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1CapStyle
{
	/// D2D1_CAP_STYLE_FLAT = 0x0u32
	Flat                 = 0x0u32,
	/// D2D1_CAP_STYLE_SQUARE = 0x1u32
	Square               = 0x1u32,
	/// D2D1_CAP_STYLE_ROUND = 0x2u32
	Round                = 0x2u32,
	/// D2D1_CAP_STYLE_TRIANGLE = 0x3u32
	Triangle             = 0x3u32,
	/// D2D1_CAP_STYLE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DASH_STYLE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DashStyle
{
	/// D2D1_DASH_STYLE_SOLID = 0x0u32
	Solid                = 0x0u32,
	/// D2D1_DASH_STYLE_DASH = 0x1u32
	Dash                 = 0x1u32,
	/// D2D1_DASH_STYLE_DOT = 0x2u32
	Dot                  = 0x2u32,
	/// D2D1_DASH_STYLE_DASH_DOT = 0x3u32
	DashDot              = 0x3u32,
	/// D2D1_DASH_STYLE_DASH_DOT_DOT = 0x4u32
	DashDotDot           = 0x4u32,
	/// D2D1_DASH_STYLE_CUSTOM = 0x5u32
	Custom               = 0x5u32,
	/// D2D1_DASH_STYLE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_LINE_JOIN
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1LineJoin
{
	/// D2D1_LINE_JOIN_MITER = 0x0u32
	Miter                = 0x0u32,
	/// D2D1_LINE_JOIN_BEVEL = 0x1u32
	Bevel                = 0x1u32,
	/// D2D1_LINE_JOIN_ROUND = 0x2u32
	Round                = 0x2u32,
	/// D2D1_LINE_JOIN_MITER_OR_BEVEL = 0x3u32
	MiterOrBevel         = 0x3u32,
	/// D2D1_LINE_JOIN_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COMBINE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1CombineMode
{
	/// D2D1_COMBINE_MODE_UNION = 0x0u32
	Union                = 0x0u32,
	/// D2D1_COMBINE_MODE_INTERSECT = 0x1u32
	Intersect            = 0x1u32,
	/// D2D1_COMBINE_MODE_XOR = 0x2u32
	Xor                  = 0x2u32,
	/// D2D1_COMBINE_MODE_EXCLUDE = 0x3u32
	Exclude              = 0x3u32,
	/// D2D1_COMBINE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_GEOMETRY_RELATION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1GeometryRelation
{
	/// D2D1_GEOMETRY_RELATION_UNKNOWN = 0x0u32
	Unknown              = 0x0u32,
	/// D2D1_GEOMETRY_RELATION_DISJOINT = 0x1u32
	Disjoint             = 0x1u32,
	/// D2D1_GEOMETRY_RELATION_IS_CONTAINED = 0x2u32
	IsContained          = 0x2u32,
	/// D2D1_GEOMETRY_RELATION_CONTAINS = 0x3u32
	Contains             = 0x3u32,
	/// D2D1_GEOMETRY_RELATION_OVERLAP = 0x4u32
	Overlap              = 0x4u32,
	/// D2D1_GEOMETRY_RELATION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_GEOMETRY_SIMPLIFICATION_OPTION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1GeometrySimplificationOption
{
	/// D2D1_GEOMETRY_SIMPLIFICATION_OPTION_CUBICS_AND_LINES = 0x0u32
	CubicsAndLines       = 0x0u32,
	/// D2D1_GEOMETRY_SIMPLIFICATION_OPTION_LINES = 0x1u32
	Lines                = 0x1u32,
	/// D2D1_GEOMETRY_SIMPLIFICATION_OPTION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SWEEP_DIRECTION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SweepDirection
{
	/// D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE = 0x0u32
	CounterClockwise     = 0x0u32,
	/// D2D1_SWEEP_DIRECTION_CLOCKWISE = 0x1u32
	Clockwise            = 0x1u32,
	/// D2D1_SWEEP_DIRECTION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_LAYER_OPTIONS
	#[repr(transparent)]
	pub struct D2D1LayerOptions: u32 {
		/// D2D1_LAYER_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE = 0x1u32
		const InitializeForClearType = 0x1u32;
		/// D2D1_LAYER_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// D2D1_WINDOW_STATE
	#[repr(transparent)]
	pub struct D2D1WindowState: u32 {
		/// D2D1_WINDOW_STATE_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_WINDOW_STATE_OCCLUDED = 0x1u32
		const Occluded             = 0x1u32;
		/// D2D1_WINDOW_STATE_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_RENDER_TARGET_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1RenderTargetType
{
	/// D2D1_RENDER_TARGET_TYPE_DEFAULT = 0x0u32
	Default              = 0x0u32,
	/// D2D1_RENDER_TARGET_TYPE_SOFTWARE = 0x1u32
	Software             = 0x1u32,
	/// D2D1_RENDER_TARGET_TYPE_HARDWARE = 0x2u32
	Hardware             = 0x2u32,
	/// D2D1_RENDER_TARGET_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_FEATURE_LEVEL
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1FeatureLevel
{
	/// D2D1_FEATURE_LEVEL_DEFAULT = 0x0u32
	Default              = 0x0u32,
	/// D2D1_FEATURE_LEVEL_9 = 0x9100u32
	_9                   = 0x9100u32,
	/// D2D1_FEATURE_LEVEL_10 = 0xA000u32
	_10                  = 0xA000u32,
	/// D2D1_FEATURE_LEVEL_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_RENDER_TARGET_USAGE
	#[repr(transparent)]
	pub struct D2D1RenderTargetUsage: u32 {
		/// D2D1_RENDER_TARGET_USAGE_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING = 0x1u32
		const ForceBitmapREmoting  = 0x1u32;
		/// D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE = 0x2u32
		const GdiCompatible        = 0x2u32;
		/// D2D1_RENDER_TARGET_USAGE_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// D2D1_PRESENT_OPTIONS
	#[repr(transparent)]
	pub struct D2D1PresentOptions: u32 {
		/// D2D1_PRESENT_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS = 0x1u32
		const RetainContents       = 0x1u32;
		/// D2D1_PRESENT_OPTIONS_IMMEDIATELY = 0x2u32
		const Immediately          = 0x2u32;
		/// D2D1_PRESENT_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS
	#[repr(transparent)]
	pub struct D2D1CompatibleRenderTargetOptions: u32 {
		/// D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE = 0x1u32
		const GdiCompatible        = 0x1u32;
		/// D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_DC_INITIALIZE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DcInitializeMode
{
	/// D2D1_DC_INITIALIZE_MODE_COPY = 0x0u32
	Copy                 = 0x0u32,
	/// D2D1_DC_INITIALIZE_MODE_CLEAR = 0x1u32
	Clear                = 0x1u32,
	/// D2D1_DC_INITIALIZE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DEBUG_LEVEL
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DebugLevel
{
	/// D2D1_DEBUG_LEVEL_NONE = 0x0u32
	None                 = 0x0u32,
	/// D2D1_DEBUG_LEVEL_ERROR = 0x1u32
	Error                = 0x1u32,
	/// D2D1_DEBUG_LEVEL_WARNING = 0x2u32
	Warning              = 0x2u32,
	/// D2D1_DEBUG_LEVEL_INFORMATION = 0x3u32
	Information          = 0x3u32,
	/// D2D1_DEBUG_LEVEL_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_FACTORY_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1FactoryType
{
	/// D2D1_FACTORY_TYPE_SINGLE_THREADED = 0x0u32
	SingleThreaded       = 0x0u32,
	/// D2D1_FACTORY_TYPE_MULTI_THREADED = 0x1u32
	MultiThreaded        = 0x1u32,
	/// D2D1_FACTORY_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CHANNEL_SELECTOR
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ChannelSelector
{
	/// D2D1_CHANNEL_SELECTOR_R = 0x0u32
	R                    = 0x0u32,
	/// D2D1_CHANNEL_SELECTOR_G = 0x1u32
	G                    = 0x1u32,
	/// D2D1_CHANNEL_SELECTOR_B = 0x2u32
	B                    = 0x2u32,
	/// D2D1_CHANNEL_SELECTOR_A = 0x3u32
	A                    = 0x3u32,
	/// D2D1_CHANNEL_SELECTOR_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BITMAPSOURCE_ORIENTATION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BitmapSourceOrientation
{
	/// D2D1_BITMAPSOURCE_ORIENTATION_DEFAULT = 0x1u32
	Default              = 0x1u32,
	/// D2D1_BITMAPSOURCE_ORIENTATION_FLIP_HORIZONTAL = 0x2u32
	FlipHorizontal       = 0x2u32,
	/// D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180 = 0x3u32
	RotateClockwise180   = 0x3u32,
	/// D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL = 0x4u32
	RotateClockwise180FlipHorizontal = 0x4u32,
	/// D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL = 0x5u32
	RotateClockwise270FlipHorizontal = 0x5u32,
	/// D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90 = 0x6u32
	RotateClockwise90    = 0x6u32,
	/// D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL = 0x7u32
	RotateClockwise90FlipHorizontal = 0x7u32,
	/// D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270 = 0x8u32
	RotateClockwise270   = 0x8u32,
	/// D2D1_BITMAPSOURCE_ORIENTATION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_GAUSSIANBLUR_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1GaussianBlurProp
{
	/// D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION = 0x0u32
	StandardDeviation    = 0x0u32,
	/// D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION = 0x1u32
	Optimization         = 0x1u32,
	/// D2D1_GAUSSIANBLUR_PROP_BORDER_MODE = 0x2u32
	BorderMode           = 0x2u32,
	/// D2D1_GAUSSIANBLUR_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_GAUSSIANBLUR_OPTIMIZATION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1GaussianBlurOptimization
{
	/// D2D1_GAUSSIANBLUR_OPTIMIZATION_SPEED = 0x0u32
	Speed                = 0x0u32,
	/// D2D1_GAUSSIANBLUR_OPTIMIZATION_BALANCED = 0x1u32
	Balanced             = 0x1u32,
	/// D2D1_GAUSSIANBLUR_OPTIMIZATION_QUALITY = 0x2u32
	Quality              = 0x2u32,
	/// D2D1_GAUSSIANBLUR_OPTIMIZATION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DIRECTIONALBLUR_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DirectionalBlurProp
{
	/// D2D1_DIRECTIONALBLUR_PROP_STANDARD_DEVIATION = 0x0u32
	StandardDeviation    = 0x0u32,
	/// D2D1_DIRECTIONALBLUR_PROP_ANGLE = 0x1u32
	Angle                = 0x1u32,
	/// D2D1_DIRECTIONALBLUR_PROP_OPTIMIZATION = 0x2u32
	Optimization         = 0x2u32,
	/// D2D1_DIRECTIONALBLUR_PROP_BORDER_MODE = 0x3u32
	BorderMode           = 0x3u32,
	/// D2D1_DIRECTIONALBLUR_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DIRECTIONALBLUR_OPTIMIZATION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DirectionalBlurOptimization
{
	/// D2D1_DIRECTIONALBLUR_OPTIMIZATION_SPEED = 0x0u32
	Speed                = 0x0u32,
	/// D2D1_DIRECTIONALBLUR_OPTIMIZATION_BALANCED = 0x1u32
	Balanced             = 0x1u32,
	/// D2D1_DIRECTIONALBLUR_OPTIMIZATION_QUALITY = 0x2u32
	Quality              = 0x2u32,
	/// D2D1_DIRECTIONALBLUR_OPTIMIZATION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SHADOW_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ShadowProp
{
	/// D2D1_SHADOW_PROP_BLUR_STANDARD_DEVIATION = 0x0u32
	BlurStandardDeviation = 0x0u32,
	/// D2D1_SHADOW_PROP_COLOR = 0x1u32
	Color                = 0x1u32,
	/// D2D1_SHADOW_PROP_OPTIMIZATION = 0x2u32
	Optimization         = 0x2u32,
	/// D2D1_SHADOW_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SHADOW_OPTIMIZATION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ShadowOptimization
{
	/// D2D1_SHADOW_OPTIMIZATION_SPEED = 0x0u32
	Speed                = 0x0u32,
	/// D2D1_SHADOW_OPTIMIZATION_BALANCED = 0x1u32
	Balanced             = 0x1u32,
	/// D2D1_SHADOW_OPTIMIZATION_QUALITY = 0x2u32
	Quality              = 0x2u32,
	/// D2D1_SHADOW_OPTIMIZATION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BLEND_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BlendProp
{
	/// D2D1_BLEND_PROP_MODE = 0x0u32
	Mode                 = 0x0u32,
	/// D2D1_BLEND_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SATURATION_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SaturationProp
{
	/// D2D1_SATURATION_PROP_SATURATION = 0x0u32
	Saturation           = 0x0u32,
	/// D2D1_SATURATION_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_HUEROTATION_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1HueRotationProp
{
	/// D2D1_HUEROTATION_PROP_ANGLE = 0x0u32
	Angle                = 0x0u32,
	/// D2D1_HUEROTATION_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLORMATRIX_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorMatrixProp
{
	/// D2D1_COLORMATRIX_PROP_COLOR_MATRIX = 0x0u32
	ColorMatrix          = 0x0u32,
	/// D2D1_COLORMATRIX_PROP_ALPHA_MODE = 0x1u32
	AlphaMode            = 0x1u32,
	/// D2D1_COLORMATRIX_PROP_CLAMP_OUTPUT = 0x2u32
	ClampOutput          = 0x2u32,
	/// D2D1_COLORMATRIX_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BITMAPSOURCE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BitmapSourceProp
{
	/// D2D1_BITMAPSOURCE_PROP_WIC_BITMAP_SOURCE = 0x0u32
	WicBitmapSource      = 0x0u32,
	/// D2D1_BITMAPSOURCE_PROP_SCALE = 0x1u32
	Scale                = 0x1u32,
	/// D2D1_BITMAPSOURCE_PROP_INTERPOLATION_MODE = 0x2u32
	InterpolationMode    = 0x2u32,
	/// D2D1_BITMAPSOURCE_PROP_ENABLE_DPI_CORRECTION = 0x3u32
	EnableDpiCorrection  = 0x3u32,
	/// D2D1_BITMAPSOURCE_PROP_ALPHA_MODE = 0x4u32
	AlphaMode            = 0x4u32,
	/// D2D1_BITMAPSOURCE_PROP_ORIENTATION = 0x5u32
	Orientation          = 0x5u32,
	/// D2D1_BITMAPSOURCE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BITMAPSOURCE_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BitmapSourceInterpolationMode
{
	/// D2D1_BITMAPSOURCE_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_BITMAPSOURCE_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_BITMAPSOURCE_INTERPOLATION_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FANT = 0x6u32
	Fant                 = 0x6u32,
	/// D2D1_BITMAPSOURCE_INTERPOLATION_MODE_MIPMAP_LINEAR = 0x7u32
	MipMapLinear         = 0x7u32,
	/// D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BITMAPSOURCE_ALPHA_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BitmapSourceAlphaMode
{
	/// D2D1_BITMAPSOURCE_ALPHA_MODE_PREMULTIPLIED = 0x1u32
	Premultiplied        = 0x1u32,
	/// D2D1_BITMAPSOURCE_ALPHA_MODE_STRAIGHT = 0x2u32
	Straight             = 0x2u32,
	/// D2D1_BITMAPSOURCE_ALPHA_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COMPOSITE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1CompositeProp
{
	/// D2D1_COMPOSITE_PROP_MODE = 0x0u32
	Mode                 = 0x0u32,
	/// D2D1_COMPOSITE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_3DTRANSFORM_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1_3dTransformProp
{
	/// D2D1_3DTRANSFORM_PROP_INTERPOLATION_MODE = 0x0u32
	InterpolationMode    = 0x0u32,
	/// D2D1_3DTRANSFORM_PROP_BORDER_MODE = 0x1u32
	BorderMode           = 0x1u32,
	/// D2D1_3DTRANSFORM_PROP_TRANSFORM_MATRIX = 0x2u32
	TransformMatrix      = 0x2u32,
	/// D2D1_3DTRANSFORM_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_3DTRANSFORM_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1_3dTransformInterpolationMode
{
	/// D2D1_3DTRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_3DTRANSFORM_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_3DTRANSFORM_INTERPOLATION_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_3DTRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_3DTRANSFORM_INTERPOLATION_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_3DTRANSFORM_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_3DPERSPECTIVETRANSFORM_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1_3dPerspectiveTransformProp
{
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_INTERPOLATION_MODE = 0x0u32
	InterpolationMode    = 0x0u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_BORDER_MODE = 0x1u32
	BorderMode           = 0x1u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_DEPTH = 0x2u32
	Depth                = 0x2u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_PERSPECTIVE_ORIGIN = 0x3u32
	PerspectiveOrigin    = 0x3u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_LOCAL_OFFSET = 0x4u32
	LocalOffset          = 0x4u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_GLOBAL_OFFSET = 0x5u32
	GlobalOffset         = 0x5u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION_ORIGIN = 0x6u32
	RotationOrigin       = 0x6u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION = 0x7u32
	Rotation             = 0x7u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1_3dPerspectiveTransformInterpolationMode
{
	/// D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_2DAFFINETRANSFORM_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1_2dAffineTransformProp
{
	/// D2D1_2DAFFINETRANSFORM_PROP_INTERPOLATION_MODE = 0x0u32
	InterpolationMode    = 0x0u32,
	/// D2D1_2DAFFINETRANSFORM_PROP_BORDER_MODE = 0x1u32
	BorderMode           = 0x1u32,
	/// D2D1_2DAFFINETRANSFORM_PROP_TRANSFORM_MATRIX = 0x2u32
	TransformMatrix      = 0x2u32,
	/// D2D1_2DAFFINETRANSFORM_PROP_SHARPNESS = 0x3u32
	Sharpness            = 0x3u32,
	/// D2D1_2DAFFINETRANSFORM_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DPICOMPENSATION_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DpiCompensationProp
{
	/// D2D1_DPICOMPENSATION_PROP_INTERPOLATION_MODE = 0x0u32
	InterpolationMode    = 0x0u32,
	/// D2D1_DPICOMPENSATION_PROP_BORDER_MODE = 0x1u32
	BorderMode           = 0x1u32,
	/// D2D1_DPICOMPENSATION_PROP_INPUT_DPI = 0x2u32
	InputDpi             = 0x2u32,
	/// D2D1_DPICOMPENSATION_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DPICOMPENSATION_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DpiCompensationInterpolationMode
{
	/// D2D1_DPICOMPENSATION_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_DPICOMPENSATION_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_DPICOMPENSATION_INTERPOLATION_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_DPICOMPENSATION_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_DPICOMPENSATION_INTERPOLATION_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_DPICOMPENSATION_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_DPICOMPENSATION_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SCALE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ScaleProp
{
	/// D2D1_SCALE_PROP_SCALE = 0x0u32
	Scale                = 0x0u32,
	/// D2D1_SCALE_PROP_CENTER_POINT = 0x1u32
	CenterPoint          = 0x1u32,
	/// D2D1_SCALE_PROP_INTERPOLATION_MODE = 0x2u32
	InterpolationMode    = 0x2u32,
	/// D2D1_SCALE_PROP_BORDER_MODE = 0x3u32
	BorderMode           = 0x3u32,
	/// D2D1_SCALE_PROP_SHARPNESS = 0x4u32
	Sharpness            = 0x4u32,
	/// D2D1_SCALE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SCALE_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ScaleInterpolationMode
{
	/// D2D1_SCALE_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_SCALE_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_SCALE_INTERPOLATION_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_SCALE_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_SCALE_INTERPOLATION_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_SCALE_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_SCALE_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_TURBULENCE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1TurbulenceProp
{
	/// D2D1_TURBULENCE_PROP_OFFSET = 0x0u32
	Offset               = 0x0u32,
	/// D2D1_TURBULENCE_PROP_SIZE = 0x1u32
	Size                 = 0x1u32,
	/// D2D1_TURBULENCE_PROP_BASE_FREQUENCY = 0x2u32
	BaseFrequency        = 0x2u32,
	/// D2D1_TURBULENCE_PROP_NUM_OCTAVES = 0x3u32
	NumOctaves           = 0x3u32,
	/// D2D1_TURBULENCE_PROP_SEED = 0x4u32
	Seed                 = 0x4u32,
	/// D2D1_TURBULENCE_PROP_NOISE = 0x5u32
	Noise                = 0x5u32,
	/// D2D1_TURBULENCE_PROP_STITCHABLE = 0x6u32
	STITcHable           = 0x6u32,
	/// D2D1_TURBULENCE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DISPLACEMENTMAP_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DisplacementMapProp
{
	/// D2D1_DISPLACEMENTMAP_PROP_SCALE = 0x0u32
	Scale                = 0x0u32,
	/// D2D1_DISPLACEMENTMAP_PROP_X_CHANNEL_SELECT = 0x1u32
	XChannelSelect       = 0x1u32,
	/// D2D1_DISPLACEMENTMAP_PROP_Y_CHANNEL_SELECT = 0x2u32
	YChannelSelect       = 0x2u32,
	/// D2D1_DISPLACEMENTMAP_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLORMANAGEMENT_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorManagementProp
{
	/// D2D1_COLORMANAGEMENT_PROP_SOURCE_COLOR_CONTEXT = 0x0u32
	SourceColorContext   = 0x0u32,
	/// D2D1_COLORMANAGEMENT_PROP_SOURCE_RENDERING_INTENT = 0x1u32
	SourceRenderingIntent = 0x1u32,
	/// D2D1_COLORMANAGEMENT_PROP_DESTINATION_COLOR_CONTEXT = 0x2u32
	DestinationColorContext = 0x2u32,
	/// D2D1_COLORMANAGEMENT_PROP_DESTINATION_RENDERING_INTENT = 0x3u32
	DestinationRenderingIntent = 0x3u32,
	/// D2D1_COLORMANAGEMENT_PROP_ALPHA_MODE = 0x4u32
	AlphaMode            = 0x4u32,
	/// D2D1_COLORMANAGEMENT_PROP_QUALITY = 0x5u32
	Quality              = 0x5u32,
	/// D2D1_COLORMANAGEMENT_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLORMANAGEMENT_ALPHA_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorManagementAlphaMode
{
	/// D2D1_COLORMANAGEMENT_ALPHA_MODE_PREMULTIPLIED = 0x1u32
	Premultiplied        = 0x1u32,
	/// D2D1_COLORMANAGEMENT_ALPHA_MODE_STRAIGHT = 0x2u32
	Straight             = 0x2u32,
	/// D2D1_COLORMANAGEMENT_ALPHA_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLORMANAGEMENT_QUALITY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorManagementQuality
{
	/// D2D1_COLORMANAGEMENT_QUALITY_PROOF = 0x0u32
	Proof                = 0x0u32,
	/// D2D1_COLORMANAGEMENT_QUALITY_NORMAL = 0x1u32
	Normal               = 0x1u32,
	/// D2D1_COLORMANAGEMENT_QUALITY_BEST = 0x2u32
	Best                 = 0x2u32,
	/// D2D1_COLORMANAGEMENT_QUALITY_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLORMANAGEMENT_RENDERING_INTENT
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorManagementRenderingIntent
{
	/// D2D1_COLORMANAGEMENT_RENDERING_INTENT_PERCEPTUAL = 0x0u32
	Perceptual           = 0x0u32,
	/// D2D1_COLORMANAGEMENT_RENDERING_INTENT_RELATIVE_COLORIMETRIC = 0x1u32
	RelativeColorimetric = 0x1u32,
	/// D2D1_COLORMANAGEMENT_RENDERING_INTENT_SATURATION = 0x2u32
	Saturation           = 0x2u32,
	/// D2D1_COLORMANAGEMENT_RENDERING_INTENT_ABSOLUTE_COLORIMETRIC = 0x3u32
	AbsoluteColorimetric = 0x3u32,
	/// D2D1_COLORMANAGEMENT_RENDERING_INTENT_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_HISTOGRAM_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1HistogramProp
{
	/// D2D1_HISTOGRAM_PROP_NUM_BINS = 0x0u32
	NumBins              = 0x0u32,
	/// D2D1_HISTOGRAM_PROP_CHANNEL_SELECT = 0x1u32
	ChannelSelect        = 0x1u32,
	/// D2D1_HISTOGRAM_PROP_HISTOGRAM_OUTPUT = 0x2u32
	HistogramOutput      = 0x2u32,
	/// D2D1_HISTOGRAM_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_POINTSPECULAR_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PointSpecularProp
{
	/// D2D1_POINTSPECULAR_PROP_LIGHT_POSITION = 0x0u32
	LightPosition        = 0x0u32,
	/// D2D1_POINTSPECULAR_PROP_SPECULAR_EXPONENT = 0x1u32
	SpecularExponent     = 0x1u32,
	/// D2D1_POINTSPECULAR_PROP_SPECULAR_CONSTANT = 0x2u32
	SpecularConstant     = 0x2u32,
	/// D2D1_POINTSPECULAR_PROP_SURFACE_SCALE = 0x3u32
	SurfaceScale         = 0x3u32,
	/// D2D1_POINTSPECULAR_PROP_COLOR = 0x4u32
	Color                = 0x4u32,
	/// D2D1_POINTSPECULAR_PROP_KERNEL_UNIT_LENGTH = 0x5u32
	KernelUnitLength     = 0x5u32,
	/// D2D1_POINTSPECULAR_PROP_SCALE_MODE = 0x6u32
	ScaleMode            = 0x6u32,
	/// D2D1_POINTSPECULAR_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_POINTSPECULAR_SCALE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PointSpecularScaleMode
{
	/// D2D1_POINTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_POINTSPECULAR_SCALE_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_POINTSPECULAR_SCALE_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_POINTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_POINTSPECULAR_SCALE_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_POINTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_POINTSPECULAR_SCALE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SPOTSPECULAR_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SpotSpecularProp
{
	/// D2D1_SPOTSPECULAR_PROP_LIGHT_POSITION = 0x0u32
	LightPosition        = 0x0u32,
	/// D2D1_SPOTSPECULAR_PROP_POINTS_AT = 0x1u32
	PointsAt             = 0x1u32,
	/// D2D1_SPOTSPECULAR_PROP_FOCUS = 0x2u32
	Focus                = 0x2u32,
	/// D2D1_SPOTSPECULAR_PROP_LIMITING_CONE_ANGLE = 0x3u32
	LimitingConeAngle    = 0x3u32,
	/// D2D1_SPOTSPECULAR_PROP_SPECULAR_EXPONENT = 0x4u32
	SpecularExponent     = 0x4u32,
	/// D2D1_SPOTSPECULAR_PROP_SPECULAR_CONSTANT = 0x5u32
	SpecularConstant     = 0x5u32,
	/// D2D1_SPOTSPECULAR_PROP_SURFACE_SCALE = 0x6u32
	SurfaceScale         = 0x6u32,
	/// D2D1_SPOTSPECULAR_PROP_COLOR = 0x7u32
	Color                = 0x7u32,
	/// D2D1_SPOTSPECULAR_PROP_KERNEL_UNIT_LENGTH = 0x8u32
	KernelUnitLength     = 0x8u32,
	/// D2D1_SPOTSPECULAR_PROP_SCALE_MODE = 0x9u32
	ScaleMode            = 0x9u32,
	/// D2D1_SPOTSPECULAR_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SPOTSPECULAR_SCALE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SpotSpecularScaleMode
{
	/// D2D1_SPOTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_SPOTSPECULAR_SCALE_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_SPOTSPECULAR_SCALE_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_SPOTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_SPOTSPECULAR_SCALE_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_SPOTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_SPOTSPECULAR_SCALE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DISTANTSPECULAR_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DistantSpecularProp
{
	/// D2D1_DISTANTSPECULAR_PROP_AZIMUTH = 0x0u32
	Azimuth              = 0x0u32,
	/// D2D1_DISTANTSPECULAR_PROP_ELEVATION = 0x1u32
	Elevation            = 0x1u32,
	/// D2D1_DISTANTSPECULAR_PROP_SPECULAR_EXPONENT = 0x2u32
	SpecularExponent     = 0x2u32,
	/// D2D1_DISTANTSPECULAR_PROP_SPECULAR_CONSTANT = 0x3u32
	SpecularConstant     = 0x3u32,
	/// D2D1_DISTANTSPECULAR_PROP_SURFACE_SCALE = 0x4u32
	SurfaceScale         = 0x4u32,
	/// D2D1_DISTANTSPECULAR_PROP_COLOR = 0x5u32
	Color                = 0x5u32,
	/// D2D1_DISTANTSPECULAR_PROP_KERNEL_UNIT_LENGTH = 0x6u32
	KernelUnitLength     = 0x6u32,
	/// D2D1_DISTANTSPECULAR_PROP_SCALE_MODE = 0x7u32
	ScaleMode            = 0x7u32,
	/// D2D1_DISTANTSPECULAR_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DISTANTSPECULAR_SCALE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DistantSpecularScaleMode
{
	/// D2D1_DISTANTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_DISTANTSPECULAR_SCALE_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_DISTANTSPECULAR_SCALE_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_DISTANTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_DISTANTSPECULAR_SCALE_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_DISTANTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_DISTANTSPECULAR_SCALE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_POINTDIFFUSE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PointDiffuseProp
{
	/// D2D1_POINTDIFFUSE_PROP_LIGHT_POSITION = 0x0u32
	LightPosition        = 0x0u32,
	/// D2D1_POINTDIFFUSE_PROP_DIFFUSE_CONSTANT = 0x1u32
	DiffuseConstant      = 0x1u32,
	/// D2D1_POINTDIFFUSE_PROP_SURFACE_SCALE = 0x2u32
	SurfaceScale         = 0x2u32,
	/// D2D1_POINTDIFFUSE_PROP_COLOR = 0x3u32
	Color                = 0x3u32,
	/// D2D1_POINTDIFFUSE_PROP_KERNEL_UNIT_LENGTH = 0x4u32
	KernelUnitLength     = 0x4u32,
	/// D2D1_POINTDIFFUSE_PROP_SCALE_MODE = 0x5u32
	ScaleMode            = 0x5u32,
	/// D2D1_POINTDIFFUSE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_POINTDIFFUSE_SCALE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PointDiffuseScaleMode
{
	/// D2D1_POINTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_POINTDIFFUSE_SCALE_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_POINTDIFFUSE_SCALE_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_POINTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_POINTDIFFUSE_SCALE_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_POINTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_POINTDIFFUSE_SCALE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SPOTDIFFUSE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SpotDiffuseProp
{
	/// D2D1_SPOTDIFFUSE_PROP_LIGHT_POSITION = 0x0u32
	LightPosition        = 0x0u32,
	/// D2D1_SPOTDIFFUSE_PROP_POINTS_AT = 0x1u32
	PointsAt             = 0x1u32,
	/// D2D1_SPOTDIFFUSE_PROP_FOCUS = 0x2u32
	Focus                = 0x2u32,
	/// D2D1_SPOTDIFFUSE_PROP_LIMITING_CONE_ANGLE = 0x3u32
	LimitingConeAngle    = 0x3u32,
	/// D2D1_SPOTDIFFUSE_PROP_DIFFUSE_CONSTANT = 0x4u32
	DiffuseConstant      = 0x4u32,
	/// D2D1_SPOTDIFFUSE_PROP_SURFACE_SCALE = 0x5u32
	SurfaceScale         = 0x5u32,
	/// D2D1_SPOTDIFFUSE_PROP_COLOR = 0x6u32
	Color                = 0x6u32,
	/// D2D1_SPOTDIFFUSE_PROP_KERNEL_UNIT_LENGTH = 0x7u32
	KernelUnitLength     = 0x7u32,
	/// D2D1_SPOTDIFFUSE_PROP_SCALE_MODE = 0x8u32
	ScaleMode            = 0x8u32,
	/// D2D1_SPOTDIFFUSE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SPOTDIFFUSE_SCALE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SpotDiffuseScaleMode
{
	/// D2D1_SPOTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_SPOTDIFFUSE_SCALE_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_SPOTDIFFUSE_SCALE_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_SPOTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_SPOTDIFFUSE_SCALE_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_SPOTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_SPOTDIFFUSE_SCALE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DISTANTDIFFUSE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DistantDiffuseProp
{
	/// D2D1_DISTANTDIFFUSE_PROP_AZIMUTH = 0x0u32
	Azimuth              = 0x0u32,
	/// D2D1_DISTANTDIFFUSE_PROP_ELEVATION = 0x1u32
	Elevation            = 0x1u32,
	/// D2D1_DISTANTDIFFUSE_PROP_DIFFUSE_CONSTANT = 0x2u32
	DiffuseConstant      = 0x2u32,
	/// D2D1_DISTANTDIFFUSE_PROP_SURFACE_SCALE = 0x3u32
	SurfaceScale         = 0x3u32,
	/// D2D1_DISTANTDIFFUSE_PROP_COLOR = 0x4u32
	Color                = 0x4u32,
	/// D2D1_DISTANTDIFFUSE_PROP_KERNEL_UNIT_LENGTH = 0x5u32
	KernelUnitLength     = 0x5u32,
	/// D2D1_DISTANTDIFFUSE_PROP_SCALE_MODE = 0x6u32
	ScaleMode            = 0x6u32,
	/// D2D1_DISTANTDIFFUSE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DISTANTDIFFUSE_SCALE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DistantDiffuseScaleMode
{
	/// D2D1_DISTANTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_DISTANTDIFFUSE_SCALE_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_DISTANTDIFFUSE_SCALE_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_DISTANTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_DISTANTDIFFUSE_SCALE_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_DISTANTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_DISTANTDIFFUSE_SCALE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_FLOOD_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1FloodProp
{
	/// D2D1_FLOOD_PROP_COLOR = 0x0u32
	Color                = 0x0u32,
	/// D2D1_FLOOD_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_LINEARTRANSFER_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1LinearTransferProp
{
	/// D2D1_LINEARTRANSFER_PROP_RED_Y_INTERCEPT = 0x0u32
	RedYIntercept        = 0x0u32,
	/// D2D1_LINEARTRANSFER_PROP_RED_SLOPE = 0x1u32
	RedSlope             = 0x1u32,
	/// D2D1_LINEARTRANSFER_PROP_RED_DISABLE = 0x2u32
	RedDisable           = 0x2u32,
	/// D2D1_LINEARTRANSFER_PROP_GREEN_Y_INTERCEPT = 0x3u32
	GreenYIntercept      = 0x3u32,
	/// D2D1_LINEARTRANSFER_PROP_GREEN_SLOPE = 0x4u32
	GreenSlope           = 0x4u32,
	/// D2D1_LINEARTRANSFER_PROP_GREEN_DISABLE = 0x5u32
	GreenDisable         = 0x5u32,
	/// D2D1_LINEARTRANSFER_PROP_BLUE_Y_INTERCEPT = 0x6u32
	BlueYIntercept       = 0x6u32,
	/// D2D1_LINEARTRANSFER_PROP_BLUE_SLOPE = 0x7u32
	BlueSlope            = 0x7u32,
	/// D2D1_LINEARTRANSFER_PROP_BLUE_DISABLE = 0x8u32
	BlueDisable          = 0x8u32,
	/// D2D1_LINEARTRANSFER_PROP_ALPHA_Y_INTERCEPT = 0x9u32
	AlphaYIntercept      = 0x9u32,
	/// D2D1_LINEARTRANSFER_PROP_ALPHA_SLOPE = 0xAu32
	AlphaSlope           = 0xAu32,
	/// D2D1_LINEARTRANSFER_PROP_ALPHA_DISABLE = 0xBu32
	AlphaDisable         = 0xBu32,
	/// D2D1_LINEARTRANSFER_PROP_CLAMP_OUTPUT = 0xCu32
	ClampOutput          = 0xCu32,
	/// D2D1_LINEARTRANSFER_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_GAMMATRANSFER_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1GammaTransferProp
{
	/// D2D1_GAMMATRANSFER_PROP_RED_AMPLITUDE = 0x0u32
	RedAmplitude         = 0x0u32,
	/// D2D1_GAMMATRANSFER_PROP_RED_EXPONENT = 0x1u32
	RedExponent          = 0x1u32,
	/// D2D1_GAMMATRANSFER_PROP_RED_OFFSET = 0x2u32
	RedOffset            = 0x2u32,
	/// D2D1_GAMMATRANSFER_PROP_RED_DISABLE = 0x3u32
	RedDisable           = 0x3u32,
	/// D2D1_GAMMATRANSFER_PROP_GREEN_AMPLITUDE = 0x4u32
	GreenAmplitude       = 0x4u32,
	/// D2D1_GAMMATRANSFER_PROP_GREEN_EXPONENT = 0x5u32
	GreenExponent        = 0x5u32,
	/// D2D1_GAMMATRANSFER_PROP_GREEN_OFFSET = 0x6u32
	GreenOffset          = 0x6u32,
	/// D2D1_GAMMATRANSFER_PROP_GREEN_DISABLE = 0x7u32
	GreenDisable         = 0x7u32,
	/// D2D1_GAMMATRANSFER_PROP_BLUE_AMPLITUDE = 0x8u32
	BlueAmplitude        = 0x8u32,
	/// D2D1_GAMMATRANSFER_PROP_BLUE_EXPONENT = 0x9u32
	BlueExponent         = 0x9u32,
	/// D2D1_GAMMATRANSFER_PROP_BLUE_OFFSET = 0xAu32
	BlueOffset           = 0xAu32,
	/// D2D1_GAMMATRANSFER_PROP_BLUE_DISABLE = 0xBu32
	BlueDisable          = 0xBu32,
	/// D2D1_GAMMATRANSFER_PROP_ALPHA_AMPLITUDE = 0xCu32
	AlphaAmplitude       = 0xCu32,
	/// D2D1_GAMMATRANSFER_PROP_ALPHA_EXPONENT = 0xDu32
	AlphaExponent        = 0xDu32,
	/// D2D1_GAMMATRANSFER_PROP_ALPHA_OFFSET = 0xEu32
	AlphaOffset          = 0xEu32,
	/// D2D1_GAMMATRANSFER_PROP_ALPHA_DISABLE = 0xFu32
	AlphaDisable         = 0xFu32,
	/// D2D1_GAMMATRANSFER_PROP_CLAMP_OUTPUT = 0x10u32
	ClampOutput          = 0x10u32,
	/// D2D1_GAMMATRANSFER_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_TABLETRANSFER_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1TableTransferProp
{
	/// D2D1_TABLETRANSFER_PROP_RED_TABLE = 0x0u32
	RedTable             = 0x0u32,
	/// D2D1_TABLETRANSFER_PROP_RED_DISABLE = 0x1u32
	RedDisable           = 0x1u32,
	/// D2D1_TABLETRANSFER_PROP_GREEN_TABLE = 0x2u32
	GreenTable           = 0x2u32,
	/// D2D1_TABLETRANSFER_PROP_GREEN_DISABLE = 0x3u32
	GreenDisable         = 0x3u32,
	/// D2D1_TABLETRANSFER_PROP_BLUE_TABLE = 0x4u32
	BlueTable            = 0x4u32,
	/// D2D1_TABLETRANSFER_PROP_BLUE_DISABLE = 0x5u32
	BlueDisable          = 0x5u32,
	/// D2D1_TABLETRANSFER_PROP_ALPHA_TABLE = 0x6u32
	AlphaTable           = 0x6u32,
	/// D2D1_TABLETRANSFER_PROP_ALPHA_DISABLE = 0x7u32
	AlphaDisable         = 0x7u32,
	/// D2D1_TABLETRANSFER_PROP_CLAMP_OUTPUT = 0x8u32
	ClampOutput          = 0x8u32,
	/// D2D1_TABLETRANSFER_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_DISCRETETRANSFER_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1DiscreteTransferProp
{
	/// D2D1_DISCRETETRANSFER_PROP_RED_TABLE = 0x0u32
	RedTable             = 0x0u32,
	/// D2D1_DISCRETETRANSFER_PROP_RED_DISABLE = 0x1u32
	RedDisable           = 0x1u32,
	/// D2D1_DISCRETETRANSFER_PROP_GREEN_TABLE = 0x2u32
	GreenTable           = 0x2u32,
	/// D2D1_DISCRETETRANSFER_PROP_GREEN_DISABLE = 0x3u32
	GreenDisable         = 0x3u32,
	/// D2D1_DISCRETETRANSFER_PROP_BLUE_TABLE = 0x4u32
	BlueTable            = 0x4u32,
	/// D2D1_DISCRETETRANSFER_PROP_BLUE_DISABLE = 0x5u32
	BlueDisable          = 0x5u32,
	/// D2D1_DISCRETETRANSFER_PROP_ALPHA_TABLE = 0x6u32
	AlphaTable           = 0x6u32,
	/// D2D1_DISCRETETRANSFER_PROP_ALPHA_DISABLE = 0x7u32
	AlphaDisable         = 0x7u32,
	/// D2D1_DISCRETETRANSFER_PROP_CLAMP_OUTPUT = 0x8u32
	ClampOutput          = 0x8u32,
	/// D2D1_DISCRETETRANSFER_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CONVOLVEMATRIX_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ConvolveMatrixProp
{
	/// D2D1_CONVOLVEMATRIX_PROP_KERNEL_UNIT_LENGTH = 0x0u32
	KernelUnitLength     = 0x0u32,
	/// D2D1_CONVOLVEMATRIX_PROP_SCALE_MODE = 0x1u32
	ScaleMode            = 0x1u32,
	/// D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_X = 0x2u32
	KernelSizeX          = 0x2u32,
	/// D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_Y = 0x3u32
	KernelSizeY          = 0x3u32,
	/// D2D1_CONVOLVEMATRIX_PROP_KERNEL_MATRIX = 0x4u32
	KernelMatrix         = 0x4u32,
	/// D2D1_CONVOLVEMATRIX_PROP_DIVISOR = 0x5u32
	Divisor              = 0x5u32,
	/// D2D1_CONVOLVEMATRIX_PROP_BIAS = 0x6u32
	Bias                 = 0x6u32,
	/// D2D1_CONVOLVEMATRIX_PROP_KERNEL_OFFSET = 0x7u32
	KernelOffset         = 0x7u32,
	/// D2D1_CONVOLVEMATRIX_PROP_PRESERVE_ALPHA = 0x8u32
	PreserveAlpha        = 0x8u32,
	/// D2D1_CONVOLVEMATRIX_PROP_BORDER_MODE = 0x9u32
	BorderMode           = 0x9u32,
	/// D2D1_CONVOLVEMATRIX_PROP_CLAMP_OUTPUT = 0xAu32
	ClampOutput          = 0xAu32,
	/// D2D1_CONVOLVEMATRIX_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CONVOLVEMATRIX_SCALE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ConvolveMatrixScaleMode
{
	/// D2D1_CONVOLVEMATRIX_SCALE_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_CONVOLVEMATRIX_SCALE_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_CONVOLVEMATRIX_SCALE_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_CONVOLVEMATRIX_SCALE_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_CONVOLVEMATRIX_SCALE_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_CONVOLVEMATRIX_SCALE_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_CONVOLVEMATRIX_SCALE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BRIGHTNESS_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BrightnessProp
{
	/// D2D1_BRIGHTNESS_PROP_WHITE_POINT = 0x0u32
	WhitePoint           = 0x0u32,
	/// D2D1_BRIGHTNESS_PROP_BLACK_POINT = 0x1u32
	BlackPoint           = 0x1u32,
	/// D2D1_BRIGHTNESS_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_ARITHMETICCOMPOSITE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ArithmeticCompositeProp
{
	/// D2D1_ARITHMETICCOMPOSITE_PROP_COEFFICIENTS = 0x0u32
	Coefficients         = 0x0u32,
	/// D2D1_ARITHMETICCOMPOSITE_PROP_CLAMP_OUTPUT = 0x1u32
	ClampOutput          = 0x1u32,
	/// D2D1_ARITHMETICCOMPOSITE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CROP_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1CropProp
{
	/// D2D1_CROP_PROP_RECT = 0x0u32
	Rect                 = 0x0u32,
	/// D2D1_CROP_PROP_BORDER_MODE = 0x1u32
	BorderMode           = 0x1u32,
	/// D2D1_CROP_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BORDER_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BorderProp
{
	/// D2D1_BORDER_PROP_EDGE_MODE_X = 0x0u32
	EdgeModeX            = 0x0u32,
	/// D2D1_BORDER_PROP_EDGE_MODE_Y = 0x1u32
	EdgeModeY            = 0x1u32,
	/// D2D1_BORDER_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BORDER_EDGE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BorderEdgeMode
{
	/// D2D1_BORDER_EDGE_MODE_CLAMP = 0x0u32
	Clamp                = 0x0u32,
	/// D2D1_BORDER_EDGE_MODE_WRAP = 0x1u32
	Wrap                 = 0x1u32,
	/// D2D1_BORDER_EDGE_MODE_MIRROR = 0x2u32
	Mirror               = 0x2u32,
	/// D2D1_BORDER_EDGE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_MORPHOLOGY_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1MorphologyProp
{
	/// D2D1_MORPHOLOGY_PROP_MODE = 0x0u32
	Mode                 = 0x0u32,
	/// D2D1_MORPHOLOGY_PROP_WIDTH = 0x1u32
	Width                = 0x1u32,
	/// D2D1_MORPHOLOGY_PROP_HEIGHT = 0x2u32
	Height               = 0x2u32,
	/// D2D1_MORPHOLOGY_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_MORPHOLOGY_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1MorphologyMode
{
	/// D2D1_MORPHOLOGY_MODE_ERODE = 0x0u32
	Erode                = 0x0u32,
	/// D2D1_MORPHOLOGY_MODE_DILATE = 0x1u32
	Dilate               = 0x1u32,
	/// D2D1_MORPHOLOGY_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_TILE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1TileProp
{
	/// D2D1_TILE_PROP_RECT = 0x0u32
	Rect                 = 0x0u32,
	/// D2D1_TILE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_ATLAS_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1AtlasProp
{
	/// D2D1_ATLAS_PROP_INPUT_RECT = 0x0u32
	InputRect            = 0x0u32,
	/// D2D1_ATLAS_PROP_INPUT_PADDING_RECT = 0x1u32
	InputPaddingRect     = 0x1u32,
	/// D2D1_ATLAS_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_OPACITYMETADATA_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1OpacityMetaDataProp
{
	/// D2D1_OPACITYMETADATA_PROP_INPUT_OPAQUE_RECT = 0x0u32
	InputOpaqueRect      = 0x0u32,
	/// D2D1_OPACITYMETADATA_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_PROPERTY_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PropertyType
{
	/// D2D1_PROPERTY_TYPE_UNKNOWN = 0x0u32
	Unknown              = 0x0u32,
	/// D2D1_PROPERTY_TYPE_STRING = 0x1u32
	String               = 0x1u32,
	/// D2D1_PROPERTY_TYPE_BOOL = 0x2u32
	Bool                 = 0x2u32,
	/// D2D1_PROPERTY_TYPE_UINT32 = 0x3u32
	Uint32               = 0x3u32,
	/// D2D1_PROPERTY_TYPE_INT32 = 0x4u32
	Int32                = 0x4u32,
	/// D2D1_PROPERTY_TYPE_FLOAT = 0x5u32
	Float                = 0x5u32,
	/// D2D1_PROPERTY_TYPE_VECTOR2 = 0x6u32
	Vector2              = 0x6u32,
	/// D2D1_PROPERTY_TYPE_VECTOR3 = 0x7u32
	Vector3              = 0x7u32,
	/// D2D1_PROPERTY_TYPE_VECTOR4 = 0x8u32
	Vector4              = 0x8u32,
	/// D2D1_PROPERTY_TYPE_BLOB = 0x9u32
	Blob                 = 0x9u32,
	/// D2D1_PROPERTY_TYPE_IUNKNOWN = 0xAu32
	IUnknown             = 0xAu32,
	/// D2D1_PROPERTY_TYPE_ENUM = 0xBu32
	Enum                 = 0xBu32,
	/// D2D1_PROPERTY_TYPE_ARRAY = 0xCu32
	Array                = 0xCu32,
	/// D2D1_PROPERTY_TYPE_CLSID = 0xDu32
	CLSid                = 0xDu32,
	/// D2D1_PROPERTY_TYPE_MATRIX_3X2 = 0xEu32
	Matrix3x2            = 0xEu32,
	/// D2D1_PROPERTY_TYPE_MATRIX_4X3 = 0xFu32
	Matrix4x3            = 0xFu32,
	/// D2D1_PROPERTY_TYPE_MATRIX_4X4 = 0x10u32
	Matrix4x4            = 0x10u32,
	/// D2D1_PROPERTY_TYPE_MATRIX_5X4 = 0x11u32
	Matrix5x4            = 0x11u32,
	/// D2D1_PROPERTY_TYPE_COLOR_CONTEXT = 0x12u32
	ColorContext         = 0x12u32,
	/// D2D1_PROPERTY_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_PROPERTY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1Property
{
	/// D2D1_PROPERTY_CLSID = 0x80000000u32
	CLSid                = 0x80000000u32,
	/// D2D1_PROPERTY_DISPLAYNAME = 0x80000001u32
	DisplayName          = 0x80000001u32,
	/// D2D1_PROPERTY_AUTHOR = 0x80000002u32
	Author               = 0x80000002u32,
	/// D2D1_PROPERTY_CATEGORY = 0x80000003u32
	Category             = 0x80000003u32,
	/// D2D1_PROPERTY_DESCRIPTION = 0x80000004u32
	Description          = 0x80000004u32,
	/// D2D1_PROPERTY_INPUTS = 0x80000005u32
	Inputs               = 0x80000005u32,
	/// D2D1_PROPERTY_CACHED = 0x80000006u32
	Cached               = 0x80000006u32,
	/// D2D1_PROPERTY_PRECISION = 0x80000007u32
	Precision            = 0x80000007u32,
	/// D2D1_PROPERTY_MIN_INPUTS = 0x80000008u32
	MinInputs            = 0x80000008u32,
	/// D2D1_PROPERTY_MAX_INPUTS = 0x80000009u32
	MaxInputs            = 0x80000009u32,
	/// D2D1_PROPERTY_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SUBPROPERTY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SubProperty
{
	/// D2D1_SUBPROPERTY_DISPLAYNAME = 0x80000000u32
	DisplayName          = 0x80000000u32,
	/// D2D1_SUBPROPERTY_ISREADONLY = 0x80000001u32
	IsReadOnly           = 0x80000001u32,
	/// D2D1_SUBPROPERTY_MIN = 0x80000002u32
	Min                  = 0x80000002u32,
	/// D2D1_SUBPROPERTY_MAX = 0x80000003u32
	Max                  = 0x80000003u32,
	/// D2D1_SUBPROPERTY_DEFAULT = 0x80000004u32
	Default              = 0x80000004u32,
	/// D2D1_SUBPROPERTY_FIELDS = 0x80000005u32
	Fields               = 0x80000005u32,
	/// D2D1_SUBPROPERTY_INDEX = 0x80000006u32
	Index                = 0x80000006u32,
	/// D2D1_SUBPROPERTY_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_BITMAP_OPTIONS
	#[repr(transparent)]
	pub struct D2D1BitmapOptions: u32 {
		/// D2D1_BITMAP_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_BITMAP_OPTIONS_TARGET = 0x1u32
		const Target               = 0x1u32;
		/// D2D1_BITMAP_OPTIONS_CANNOT_DRAW = 0x2u32
		const CannotDraw           = 0x2u32;
		/// D2D1_BITMAP_OPTIONS_CPU_READ = 0x4u32
		const CpuRead              = 0x4u32;
		/// D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE = 0x8u32
		const GdiCompatible        = 0x8u32;
		/// D2D1_BITMAP_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_BUFFER_PRECISION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BufferPrecision
{
	/// D2D1_BUFFER_PRECISION_UNKNOWN = 0x0u32
	Unknown              = 0x0u32,
	/// D2D1_BUFFER_PRECISION_8BPC_UNORM = 0x1u32
	_8BPcUnorm           = 0x1u32,
	/// D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB = 0x2u32
	_8BPcUnormSrgb       = 0x2u32,
	/// D2D1_BUFFER_PRECISION_16BPC_UNORM = 0x3u32
	_16BPcUnorm          = 0x3u32,
	/// D2D1_BUFFER_PRECISION_16BPC_FLOAT = 0x4u32
	_16BPcFloat          = 0x4u32,
	/// D2D1_BUFFER_PRECISION_32BPC_FLOAT = 0x5u32
	_32BPcFloat          = 0x5u32,
	/// D2D1_BUFFER_PRECISION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_MAP_OPTIONS
	#[repr(transparent)]
	pub struct D2D1MapOptions: u32 {
		/// D2D1_MAP_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_MAP_OPTIONS_READ = 0x1u32
		const Read                 = 0x1u32;
		/// D2D1_MAP_OPTIONS_WRITE = 0x2u32
		const Write                = 0x2u32;
		/// D2D1_MAP_OPTIONS_DISCARD = 0x4u32
		const Discard              = 0x4u32;
		/// D2D1_MAP_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1InterpolationMode
{
	/// D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_INTERPOLATION_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_INTERPOLATION_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_UNIT_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1UnitMode
{
	/// D2D1_UNIT_MODE_DIPS = 0x0u32
	Dips                 = 0x0u32,
	/// D2D1_UNIT_MODE_PIXELS = 0x1u32
	Pixels               = 0x1u32,
	/// D2D1_UNIT_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLOR_SPACE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorSpace
{
	/// D2D1_COLOR_SPACE_CUSTOM = 0x0u32
	Custom               = 0x0u32,
	/// D2D1_COLOR_SPACE_SRGB = 0x1u32
	Srgb                 = 0x1u32,
	/// D2D1_COLOR_SPACE_SCRGB = 0x2u32
	SCRgb                = 0x2u32,
	/// D2D1_COLOR_SPACE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_DEVICE_CONTEXT_OPTIONS
	#[repr(transparent)]
	pub struct D2D1DeviceContextOptions: u32 {
		/// D2D1_DEVICE_CONTEXT_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS = 0x1u32
		const EnableMultithreadedOptimizations = 0x1u32;
		/// D2D1_DEVICE_CONTEXT_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_STROKE_TRANSFORM_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1StrokeTransformType
{
	/// D2D1_STROKE_TRANSFORM_TYPE_NORMAL = 0x0u32
	Normal               = 0x0u32,
	/// D2D1_STROKE_TRANSFORM_TYPE_FIXED = 0x1u32
	Fixed                = 0x1u32,
	/// D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE = 0x2u32
	Hairline             = 0x2u32,
	/// D2D1_STROKE_TRANSFORM_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_PRIMITIVE_BLEND
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PrimitiveBlend
{
	/// D2D1_PRIMITIVE_BLEND_SOURCE_OVER = 0x0u32
	SourceOver           = 0x0u32,
	/// D2D1_PRIMITIVE_BLEND_COPY = 0x1u32
	Copy                 = 0x1u32,
	/// D2D1_PRIMITIVE_BLEND_MIN = 0x2u32
	Min                  = 0x2u32,
	/// D2D1_PRIMITIVE_BLEND_ADD = 0x3u32
	Add                  = 0x3u32,
	/// D2D1_PRIMITIVE_BLEND_MAX = 0x4u32
	Max                  = 0x4u32,
	/// D2D1_PRIMITIVE_BLEND_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_THREADING_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ThreadingMode
{
	/// D2D1_THREADING_MODE_SINGLE_THREADED = 0x0u32
	SingleThreaded       = 0x0u32,
	/// D2D1_THREADING_MODE_MULTI_THREADED = 0x1u32
	MultiThreaded        = 0x1u32,
	/// D2D1_THREADING_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLOR_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorInterpolationMode
{
	/// D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT = 0x0u32
	Straight             = 0x0u32,
	/// D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED = 0x1u32
	Premultiplied        = 0x1u32,
	/// D2D1_COLOR_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_LAYER_OPTIONS1
	#[repr(transparent)]
	pub struct D2D1LayerOptions1: u32 {
		/// D2D1_LAYER_OPTIONS1_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND = 0x1u32
		const InitializeFromBackground = 0x1u32;
		/// D2D1_LAYER_OPTIONS1_IGNORE_ALPHA = 0x2u32
		const IgnoreAlpha          = 0x2u32;
		/// D2D1_LAYER_OPTIONS1_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_PRINT_FONT_SUBSET_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PrintFontSubsetMode
{
	/// D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT = 0x0u32
	Default              = 0x0u32,
	/// D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE = 0x1u32
	EachPage             = 0x1u32,
	/// D2D1_PRINT_FONT_SUBSET_MODE_NONE = 0x2u32
	None                 = 0x2u32,
	/// D2D1_PRINT_FONT_SUBSET_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_CHANGE_TYPE
	#[repr(transparent)]
	pub struct D2D1ChangeType: u32 {
		/// D2D1_CHANGE_TYPE_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_CHANGE_TYPE_PROPERTIES = 0x1u32
		const Properties           = 0x1u32;
		/// D2D1_CHANGE_TYPE_CONTEXT = 0x2u32
		const Context              = 0x2u32;
		/// D2D1_CHANGE_TYPE_GRAPH = 0x3u32
		const Graph                = 0x3u32;
		/// D2D1_CHANGE_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// D2D1_PIXEL_OPTIONS
	#[repr(transparent)]
	pub struct D2D1PixelOptions: u32 {
		/// D2D1_PIXEL_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING = 0x1u32
		const TrivialSampling      = 0x1u32;
		/// D2D1_PIXEL_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// D2D1_VERTEX_OPTIONS
	#[repr(transparent)]
	pub struct D2D1VertexOptions: u32 {
		/// D2D1_VERTEX_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR = 0x1u32
		const DoNotClear           = 0x1u32;
		/// D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER = 0x2u32
		const UseDepthBuffer       = 0x2u32;
		/// D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP = 0x4u32
		const AssumeNoOverlap      = 0x4u32;
		/// D2D1_VERTEX_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_VERTEX_USAGE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1VertexUsage
{
	/// D2D1_VERTEX_USAGE_STATIC = 0x0u32
	Static               = 0x0u32,
	/// D2D1_VERTEX_USAGE_DYNAMIC = 0x1u32
	Dynamic              = 0x1u32,
	/// D2D1_VERTEX_USAGE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BLEND_OPERATION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1BlendOperation
{
	/// D2D1_BLEND_OPERATION_ADD = 0x1u32
	Add                  = 0x1u32,
	/// D2D1_BLEND_OPERATION_SUBTRACT = 0x2u32
	Subtract             = 0x2u32,
	/// D2D1_BLEND_OPERATION_REV_SUBTRACT = 0x3u32
	RevSubtract          = 0x3u32,
	/// D2D1_BLEND_OPERATION_MIN = 0x4u32
	Min                  = 0x4u32,
	/// D2D1_BLEND_OPERATION_MAX = 0x5u32
	Max                  = 0x5u32,
	/// D2D1_BLEND_OPERATION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_BLEND
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1Blend
{
	/// D2D1_BLEND_ZERO = 0x1u32
	Zero                 = 0x1u32,
	/// D2D1_BLEND_ONE = 0x2u32
	One                  = 0x2u32,
	/// D2D1_BLEND_SRC_COLOR = 0x3u32
	SrcColor             = 0x3u32,
	/// D2D1_BLEND_INV_SRC_COLOR = 0x4u32
	InvSrcColor          = 0x4u32,
	/// D2D1_BLEND_SRC_ALPHA = 0x5u32
	SrcAlpha             = 0x5u32,
	/// D2D1_BLEND_INV_SRC_ALPHA = 0x6u32
	InvSrcAlpha          = 0x6u32,
	/// D2D1_BLEND_DEST_ALPHA = 0x7u32
	DestAlpha            = 0x7u32,
	/// D2D1_BLEND_INV_DEST_ALPHA = 0x8u32
	InvDestAlpha         = 0x8u32,
	/// D2D1_BLEND_DEST_COLOR = 0x9u32
	DestColor            = 0x9u32,
	/// D2D1_BLEND_INV_DEST_COLOR = 0xAu32
	InvDestColor         = 0xAu32,
	/// D2D1_BLEND_SRC_ALPHA_SAT = 0xBu32
	SrcAlphaSat          = 0xBu32,
	/// D2D1_BLEND_BLEND_FACTOR = 0xEu32
	BlendFactor          = 0xEu32,
	/// D2D1_BLEND_INV_BLEND_FACTOR = 0xFu32
	InvBlendFactor       = 0xFu32,
	/// D2D1_BLEND_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CHANNEL_DEPTH
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ChannelDepth
{
	/// D2D1_CHANNEL_DEPTH_DEFAULT = 0x0u32
	Default              = 0x0u32,
	/// D2D1_CHANNEL_DEPTH_1 = 0x1u32
	_1                   = 0x1u32,
	/// D2D1_CHANNEL_DEPTH_4 = 0x4u32
	_4                   = 0x4u32,
	/// D2D1_CHANNEL_DEPTH_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_FILTER
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1Filter
{
	/// D2D1_FILTER_MIN_MAG_MIP_POINT = 0x0u32
	MinMagMipPoint       = 0x0u32,
	/// D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR = 0x1u32
	MinMagPointMipLinear = 0x1u32,
	/// D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x4u32
	MinPointMagLinearMipPoint = 0x4u32,
	/// D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR = 0x5u32
	MinPointMagMipLinear = 0x5u32,
	/// D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT = 0x10u32
	MinLinearMagMipPoint = 0x10u32,
	/// D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x11u32
	MinLinearMagPointMipLinear = 0x11u32,
	/// D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT = 0x14u32
	MinMagLinearMipPoint = 0x14u32,
	/// D2D1_FILTER_MIN_MAG_MIP_LINEAR = 0x15u32
	MinMagMipLinear      = 0x15u32,
	/// D2D1_FILTER_ANISOTROPIC = 0x55u32
	Anisotropic          = 0x55u32,
	/// D2D1_FILTER_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_FEATURE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1Feature
{
	/// D2D1_FEATURE_DOUBLES = 0x0u32
	Doubles              = 0x0u32,
	/// D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS = 0x1u32
	D3D10XHardwareOptions = 0x1u32,
	/// D2D1_FEATURE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_YCBCR_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1YCbCrProp
{
	/// D2D1_YCBCR_PROP_CHROMA_SUBSAMPLING = 0x0u32
	ChromaSubsampling    = 0x0u32,
	/// D2D1_YCBCR_PROP_TRANSFORM_MATRIX = 0x1u32
	TransformMatrix      = 0x1u32,
	/// D2D1_YCBCR_PROP_INTERPOLATION_MODE = 0x2u32
	InterpolationMode    = 0x2u32,
	/// D2D1_YCBCR_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_YCBCR_CHROMA_SUBSAMPLING
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1YCbCrChromaSubsampling
{
	/// D2D1_YCBCR_CHROMA_SUBSAMPLING_AUTO = 0x0u32
	Auto                 = 0x0u32,
	/// D2D1_YCBCR_CHROMA_SUBSAMPLING_420 = 0x1u32
	_420                 = 0x1u32,
	/// D2D1_YCBCR_CHROMA_SUBSAMPLING_422 = 0x2u32
	_422                 = 0x2u32,
	/// D2D1_YCBCR_CHROMA_SUBSAMPLING_444 = 0x3u32
	_444                 = 0x3u32,
	/// D2D1_YCBCR_CHROMA_SUBSAMPLING_440 = 0x4u32
	_440                 = 0x4u32,
	/// D2D1_YCBCR_CHROMA_SUBSAMPLING_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_YCBCR_INTERPOLATION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1YCbCrInterpolationMode
{
	/// D2D1_YCBCR_INTERPOLATION_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_YCBCR_INTERPOLATION_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_YCBCR_INTERPOLATION_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_YCBCR_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_YCBCR_INTERPOLATION_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_YCBCR_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC = 0x5u32
	HighQualityCubic     = 0x5u32,
	/// D2D1_YCBCR_INTERPOLATION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CONTRAST_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ContrastProp
{
	/// D2D1_CONTRAST_PROP_CONTRAST = 0x0u32
	Contrast             = 0x0u32,
	/// D2D1_CONTRAST_PROP_CLAMP_INPUT = 0x1u32
	ClampInput           = 0x1u32,
	/// D2D1_CONTRAST_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_RGBTOHUE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1RgbToHueProp
{
	/// D2D1_RGBTOHUE_PROP_OUTPUT_COLOR_SPACE = 0x0u32
	OutputColorSpace     = 0x0u32,
	/// D2D1_RGBTOHUE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1RgbToHueOutputColorSpace
{
	/// D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_VALUE = 0x0u32
	HueSaturationValue   = 0x0u32,
	/// D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS = 0x1u32
	HueSaturationLightness = 0x1u32,
	/// D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_HUETORGB_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1HueToRgbProp
{
	/// D2D1_HUETORGB_PROP_INPUT_COLOR_SPACE = 0x0u32
	InputColorSpace      = 0x0u32,
	/// D2D1_HUETORGB_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_HUETORGB_INPUT_COLOR_SPACE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1HueToRgbInputColorSpace
{
	/// D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_VALUE = 0x0u32
	HueSaturationValue   = 0x0u32,
	/// D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS = 0x1u32
	HueSaturationLightness = 0x1u32,
	/// D2D1_HUETORGB_INPUT_COLOR_SPACE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CHROMAKEY_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1CHRomAkeyProp
{
	/// D2D1_CHROMAKEY_PROP_COLOR = 0x0u32
	Color                = 0x0u32,
	/// D2D1_CHROMAKEY_PROP_TOLERANCE = 0x1u32
	Tolerance            = 0x1u32,
	/// D2D1_CHROMAKEY_PROP_INVERT_ALPHA = 0x2u32
	InvertAlpha          = 0x2u32,
	/// D2D1_CHROMAKEY_PROP_FEATHER = 0x3u32
	Feather              = 0x3u32,
	/// D2D1_CHROMAKEY_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_EMBOSS_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1EmbossProp
{
	/// D2D1_EMBOSS_PROP_HEIGHT = 0x0u32
	Height               = 0x0u32,
	/// D2D1_EMBOSS_PROP_DIRECTION = 0x1u32
	Direction            = 0x1u32,
	/// D2D1_EMBOSS_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_EXPOSURE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ExposureProp
{
	/// D2D1_EXPOSURE_PROP_EXPOSURE_VALUE = 0x0u32
	ExposureValue        = 0x0u32,
	/// D2D1_EXPOSURE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_POSTERIZE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PosterizeProp
{
	/// D2D1_POSTERIZE_PROP_RED_VALUE_COUNT = 0x0u32
	RedValueCount        = 0x0u32,
	/// D2D1_POSTERIZE_PROP_GREEN_VALUE_COUNT = 0x1u32
	GreenValueCount      = 0x1u32,
	/// D2D1_POSTERIZE_PROP_BLUE_VALUE_COUNT = 0x2u32
	BlueValueCount       = 0x2u32,
	/// D2D1_POSTERIZE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SEPIA_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SepiaProp
{
	/// D2D1_SEPIA_PROP_INTENSITY = 0x0u32
	Intensity            = 0x0u32,
	/// D2D1_SEPIA_PROP_ALPHA_MODE = 0x1u32
	AlphaMode            = 0x1u32,
	/// D2D1_SEPIA_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SHARPEN_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SharpenProp
{
	/// D2D1_SHARPEN_PROP_SHARPNESS = 0x0u32
	Sharpness            = 0x0u32,
	/// D2D1_SHARPEN_PROP_THRESHOLD = 0x1u32
	Threshold            = 0x1u32,
	/// D2D1_SHARPEN_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_STRAIGHTEN_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1StraightenProp
{
	/// D2D1_STRAIGHTEN_PROP_ANGLE = 0x0u32
	Angle                = 0x0u32,
	/// D2D1_STRAIGHTEN_PROP_MAINTAIN_SIZE = 0x1u32
	MaintainSize         = 0x1u32,
	/// D2D1_STRAIGHTEN_PROP_SCALE_MODE = 0x2u32
	ScaleMode            = 0x2u32,
	/// D2D1_STRAIGHTEN_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_STRAIGHTEN_SCALE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1StraightenScaleMode
{
	/// D2D1_STRAIGHTEN_SCALE_MODE_NEAREST_NEIGHBOR = 0x0u32
	NearestNeighbor      = 0x0u32,
	/// D2D1_STRAIGHTEN_SCALE_MODE_LINEAR = 0x1u32
	Linear               = 0x1u32,
	/// D2D1_STRAIGHTEN_SCALE_MODE_CUBIC = 0x2u32
	Cubic                = 0x2u32,
	/// D2D1_STRAIGHTEN_SCALE_MODE_MULTI_SAMPLE_LINEAR = 0x3u32
	MultiSampleLinear    = 0x3u32,
	/// D2D1_STRAIGHTEN_SCALE_MODE_ANISOTROPIC = 0x4u32
	Anisotropic          = 0x4u32,
	/// D2D1_STRAIGHTEN_SCALE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_TEMPERATUREANDTINT_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1TemperatureAndTintProp
{
	/// D2D1_TEMPERATUREANDTINT_PROP_TEMPERATURE = 0x0u32
	Temperature          = 0x0u32,
	/// D2D1_TEMPERATUREANDTINT_PROP_TINT = 0x1u32
	Tint                 = 0x1u32,
	/// D2D1_TEMPERATUREANDTINT_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_VIGNETTE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1VignetteProp
{
	/// D2D1_VIGNETTE_PROP_COLOR = 0x0u32
	Color                = 0x0u32,
	/// D2D1_VIGNETTE_PROP_TRANSITION_SIZE = 0x1u32
	TransitionSize       = 0x1u32,
	/// D2D1_VIGNETTE_PROP_STRENGTH = 0x2u32
	Strength             = 0x2u32,
	/// D2D1_VIGNETTE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_EDGEDETECTION_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1EdgeDetectionProp
{
	/// D2D1_EDGEDETECTION_PROP_STRENGTH = 0x0u32
	Strength             = 0x0u32,
	/// D2D1_EDGEDETECTION_PROP_BLUR_RADIUS = 0x1u32
	BlurRadius           = 0x1u32,
	/// D2D1_EDGEDETECTION_PROP_MODE = 0x2u32
	Mode                 = 0x2u32,
	/// D2D1_EDGEDETECTION_PROP_OVERLAY_EDGES = 0x3u32
	OverlayEdges         = 0x3u32,
	/// D2D1_EDGEDETECTION_PROP_ALPHA_MODE = 0x4u32
	AlphaMode            = 0x4u32,
	/// D2D1_EDGEDETECTION_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_EDGEDETECTION_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1EdgeDetectionMode
{
	/// D2D1_EDGEDETECTION_MODE_SOBEL = 0x0u32
	SoBel                = 0x0u32,
	/// D2D1_EDGEDETECTION_MODE_PREWITT = 0x1u32
	PRewITt              = 0x1u32,
	/// D2D1_EDGEDETECTION_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_HIGHLIGHTSANDSHADOWS_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1HighlightSandShadowsProp
{
	/// D2D1_HIGHLIGHTSANDSHADOWS_PROP_HIGHLIGHTS = 0x0u32
	Highlights           = 0x0u32,
	/// D2D1_HIGHLIGHTSANDSHADOWS_PROP_SHADOWS = 0x1u32
	Shadows              = 0x1u32,
	/// D2D1_HIGHLIGHTSANDSHADOWS_PROP_CLARITY = 0x2u32
	Clarity              = 0x2u32,
	/// D2D1_HIGHLIGHTSANDSHADOWS_PROP_INPUT_GAMMA = 0x3u32
	InputGamma           = 0x3u32,
	/// D2D1_HIGHLIGHTSANDSHADOWS_PROP_MASK_BLUR_RADIUS = 0x4u32
	MaskBlurRadius       = 0x4u32,
	/// D2D1_HIGHLIGHTSANDSHADOWS_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1HighlightSandShadowsInputGamma
{
	/// D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_LINEAR = 0x0u32
	Linear               = 0x0u32,
	/// D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_SRGB = 0x1u32
	Srgb                 = 0x1u32,
	/// D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_LOOKUPTABLE3D_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1LookUptable3dProp
{
	/// D2D1_LOOKUPTABLE3D_PROP_LUT = 0x0u32
	Lut                  = 0x0u32,
	/// D2D1_LOOKUPTABLE3D_PROP_ALPHA_MODE = 0x1u32
	AlphaMode            = 0x1u32,
	/// D2D1_LOOKUPTABLE3D_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_OPACITY_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1OpacityProp
{
	/// D2D1_OPACITY_PROP_OPACITY = 0x0u32
	Opacity              = 0x0u32,
	/// D2D1_OPACITY_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_CROSSFADE_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1CrossFadeProp
{
	/// D2D1_CROSSFADE_PROP_WEIGHT = 0x0u32
	Weight               = 0x0u32,
	/// D2D1_CROSSFADE_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_TINT_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1TintProp
{
	/// D2D1_TINT_PROP_COLOR = 0x0u32
	Color                = 0x0u32,
	/// D2D1_TINT_PROP_CLAMP_OUTPUT = 0x1u32
	ClampOutput          = 0x1u32,
	/// D2D1_TINT_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_WHITELEVELADJUSTMENT_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1WhiteLevelAdjustmentProp
{
	/// D2D1_WHITELEVELADJUSTMENT_PROP_INPUT_WHITE_LEVEL = 0x0u32
	InputWhiteLevel      = 0x0u32,
	/// D2D1_WHITELEVELADJUSTMENT_PROP_OUTPUT_WHITE_LEVEL = 0x1u32
	OutputWhiteLevel     = 0x1u32,
	/// D2D1_WHITELEVELADJUSTMENT_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_HDRTONEMAP_PROP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1HdrToneMapProp
{
	/// D2D1_HDRTONEMAP_PROP_INPUT_MAX_LUMINANCE = 0x0u32
	InputMaxLuminance    = 0x0u32,
	/// D2D1_HDRTONEMAP_PROP_OUTPUT_MAX_LUMINANCE = 0x1u32
	OutputMaxLuminance   = 0x1u32,
	/// D2D1_HDRTONEMAP_PROP_DISPLAY_MODE = 0x2u32
	DisplayMode          = 0x2u32,
	/// D2D1_HDRTONEMAP_PROP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_HDRTONEMAP_DISPLAY_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1HdrToneMapDisplayMode
{
	/// D2D1_HDRTONEMAP_DISPLAY_MODE_SDR = 0x0u32
	Sdr                  = 0x0u32,
	/// D2D1_HDRTONEMAP_DISPLAY_MODE_HDR = 0x1u32
	Hdr                  = 0x1u32,
	/// D2D1_HDRTONEMAP_DISPLAY_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_RENDERING_PRIORITY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1RenderingPriority
{
	/// D2D1_RENDERING_PRIORITY_NORMAL = 0x0u32
	Normal               = 0x0u32,
	/// D2D1_RENDERING_PRIORITY_LOW = 0x1u32
	Low                  = 0x1u32,
	/// D2D1_RENDERING_PRIORITY_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_PAINT_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgPaintType
{
	/// D2D1_SVG_PAINT_TYPE_NONE = 0x0u32
	None                 = 0x0u32,
	/// D2D1_SVG_PAINT_TYPE_COLOR = 0x1u32
	Color                = 0x1u32,
	/// D2D1_SVG_PAINT_TYPE_CURRENT_COLOR = 0x2u32
	CurrentColor         = 0x2u32,
	/// D2D1_SVG_PAINT_TYPE_URI = 0x3u32
	Uri                  = 0x3u32,
	/// D2D1_SVG_PAINT_TYPE_URI_NONE = 0x4u32
	UriNone              = 0x4u32,
	/// D2D1_SVG_PAINT_TYPE_URI_COLOR = 0x5u32
	UriColor             = 0x5u32,
	/// D2D1_SVG_PAINT_TYPE_URI_CURRENT_COLOR = 0x6u32
	UriCurrentColor      = 0x6u32,
	/// D2D1_SVG_PAINT_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_LENGTH_UNITS
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgLengthUnits
{
	/// D2D1_SVG_LENGTH_UNITS_NUMBER = 0x0u32
	Number               = 0x0u32,
	/// D2D1_SVG_LENGTH_UNITS_PERCENTAGE = 0x1u32
	Percentage           = 0x1u32,
	/// D2D1_SVG_LENGTH_UNITS_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_DISPLAY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgDisplay
{
	/// D2D1_SVG_DISPLAY_INLINE = 0x0u32
	Inline               = 0x0u32,
	/// D2D1_SVG_DISPLAY_NONE = 0x1u32
	None                 = 0x1u32,
	/// D2D1_SVG_DISPLAY_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_VISIBILITY
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgVisibility
{
	/// D2D1_SVG_VISIBILITY_VISIBLE = 0x0u32
	Visible              = 0x0u32,
	/// D2D1_SVG_VISIBILITY_HIDDEN = 0x1u32
	Hidden               = 0x1u32,
	/// D2D1_SVG_VISIBILITY_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_OVERFLOW
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgOverflow
{
	/// D2D1_SVG_OVERFLOW_VISIBLE = 0x0u32
	Visible              = 0x0u32,
	/// D2D1_SVG_OVERFLOW_HIDDEN = 0x1u32
	Hidden               = 0x1u32,
	/// D2D1_SVG_OVERFLOW_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_LINE_CAP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgLineCap
{
	/// D2D1_SVG_LINE_CAP_BUTT = 0x0u32
	Butt                 = 0x0u32,
	/// D2D1_SVG_LINE_CAP_SQUARE = 0x1u32
	Square               = 0x1u32,
	/// D2D1_SVG_LINE_CAP_ROUND = 0x2u32
	Round                = 0x2u32,
	/// D2D1_SVG_LINE_CAP_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_LINE_JOIN
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgLineJoin
{
	/// D2D1_SVG_LINE_JOIN_BEVEL = 0x1u32
	Bevel                = 0x1u32,
	/// D2D1_SVG_LINE_JOIN_MITER = 0x3u32
	Miter                = 0x3u32,
	/// D2D1_SVG_LINE_JOIN_ROUND = 0x2u32
	Round                = 0x2u32,
	/// D2D1_SVG_LINE_JOIN_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_ASPECT_ALIGN
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgAspectAlign
{
	/// D2D1_SVG_ASPECT_ALIGN_NONE = 0x0u32
	None                 = 0x0u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MIN = 0x1u32
	XMinYMin             = 0x1u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MIN = 0x2u32
	XMidYMin             = 0x2u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MIN = 0x3u32
	XMaxYMin             = 0x3u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MID = 0x4u32
	XMinYMid             = 0x4u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MID = 0x5u32
	XMidYMid             = 0x5u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MID = 0x6u32
	XMaxYMid             = 0x6u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MAX = 0x7u32
	XMinYMax             = 0x7u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MAX = 0x8u32
	XMidYMax             = 0x8u32,
	/// D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MAX = 0x9u32
	XMaxYMax             = 0x9u32,
	/// D2D1_SVG_ASPECT_ALIGN_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_ASPECT_SCALING
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgAspectScaling
{
	/// D2D1_SVG_ASPECT_SCALING_MEET = 0x0u32
	Meet                 = 0x0u32,
	/// D2D1_SVG_ASPECT_SCALING_SLICE = 0x1u32
	Slice                = 0x1u32,
	/// D2D1_SVG_ASPECT_SCALING_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_PATH_COMMAND
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgPathCommand
{
	/// D2D1_SVG_PATH_COMMAND_CLOSE_PATH = 0x0u32
	ClosePath            = 0x0u32,
	/// D2D1_SVG_PATH_COMMAND_MOVE_ABSOLUTE = 0x1u32
	MoveAbsolute         = 0x1u32,
	/// D2D1_SVG_PATH_COMMAND_MOVE_RELATIVE = 0x2u32
	MoveRelative         = 0x2u32,
	/// D2D1_SVG_PATH_COMMAND_LINE_ABSOLUTE = 0x3u32
	LineAbsolute         = 0x3u32,
	/// D2D1_SVG_PATH_COMMAND_LINE_RELATIVE = 0x4u32
	LineRelative         = 0x4u32,
	/// D2D1_SVG_PATH_COMMAND_CUBIC_ABSOLUTE = 0x5u32
	CubicAbsolute        = 0x5u32,
	/// D2D1_SVG_PATH_COMMAND_CUBIC_RELATIVE = 0x6u32
	CubicRelative        = 0x6u32,
	/// D2D1_SVG_PATH_COMMAND_QUADRADIC_ABSOLUTE = 0x7u32
	QUAdradICAbsolute    = 0x7u32,
	/// D2D1_SVG_PATH_COMMAND_QUADRADIC_RELATIVE = 0x8u32
	QUAdradICRelative    = 0x8u32,
	/// D2D1_SVG_PATH_COMMAND_ARC_ABSOLUTE = 0x9u32
	ArcAbsolute          = 0x9u32,
	/// D2D1_SVG_PATH_COMMAND_ARC_RELATIVE = 0xAu32
	ArcRelative          = 0xAu32,
	/// D2D1_SVG_PATH_COMMAND_HORIZONTAL_ABSOLUTE = 0xBu32
	HorizontalAbsolute   = 0xBu32,
	/// D2D1_SVG_PATH_COMMAND_HORIZONTAL_RELATIVE = 0xCu32
	HorizontalRelative   = 0xCu32,
	/// D2D1_SVG_PATH_COMMAND_VERTICAL_ABSOLUTE = 0xDu32
	VerticalAbsolute     = 0xDu32,
	/// D2D1_SVG_PATH_COMMAND_VERTICAL_RELATIVE = 0xEu32
	VerticalRelative     = 0xEu32,
	/// D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_ABSOLUTE = 0xFu32
	CubicSmoothAbsolute  = 0xFu32,
	/// D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_RELATIVE = 0x10u32
	CubicSmoothRelative  = 0x10u32,
	/// D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_ABSOLUTE = 0x11u32
	QUAdradICSmoothAbsolute = 0x11u32,
	/// D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_RELATIVE = 0x12u32
	QUAdradICSmoothRelative = 0x12u32,
	/// D2D1_SVG_PATH_COMMAND_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_UNIT_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgUnitType
{
	/// D2D1_SVG_UNIT_TYPE_USER_SPACE_ON_USE = 0x0u32
	UserSpaceOnUse       = 0x0u32,
	/// D2D1_SVG_UNIT_TYPE_OBJECT_BOUNDING_BOX = 0x1u32
	ObjectBoundingBox    = 0x1u32,
	/// D2D1_SVG_UNIT_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_ATTRIBUTE_STRING_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgAttributeStringType
{
	/// D2D1_SVG_ATTRIBUTE_STRING_TYPE_SVG = 0x0u32
	Svg                  = 0x0u32,
	/// D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID = 0x1u32
	Id                   = 0x1u32,
	/// D2D1_SVG_ATTRIBUTE_STRING_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_SVG_ATTRIBUTE_POD_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1SvgAttributePodType
{
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT = 0x0u32
	Float                = 0x0u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR = 0x1u32
	Color                = 0x1u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_FILL_MODE = 0x2u32
	FillMode             = 0x2u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_DISPLAY = 0x3u32
	Display              = 0x3u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_OVERFLOW = 0x4u32
	Overflow             = 0x4u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_CAP = 0x5u32
	LineCap              = 0x5u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_JOIN = 0x6u32
	LineJoin             = 0x6u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_VISIBILITY = 0x7u32
	Visibility           = 0x7u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_MATRIX = 0x8u32
	Matrix               = 0x8u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_UNIT_TYPE = 0x9u32
	UnitType             = 0x9u32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_EXTEND_MODE = 0xAu32
	ExtendMode           = 0xAu32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_PRESERVE_ASPECT_RATIO = 0xBu32
	PreserveAspectRatio  = 0xBu32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX = 0xCu32
	ViewBox              = 0xCu32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_LENGTH = 0xDu32
	Length               = 0xDu32,
	/// D2D1_SVG_ATTRIBUTE_POD_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_INK_NIB_SHAPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1InkNibShape
{
	/// D2D1_INK_NIB_SHAPE_ROUND = 0x0u32
	Round                = 0x0u32,
	/// D2D1_INK_NIB_SHAPE_SQUARE = 0x1u32
	Square               = 0x1u32,
	/// D2D1_INK_NIB_SHAPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_ORIENTATION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1Orientation
{
	/// D2D1_ORIENTATION_DEFAULT = 0x1u32
	Default              = 0x1u32,
	/// D2D1_ORIENTATION_FLIP_HORIZONTAL = 0x2u32
	FlipHorizontal       = 0x2u32,
	/// D2D1_ORIENTATION_ROTATE_CLOCKWISE180 = 0x3u32
	RotateClockwise180   = 0x3u32,
	/// D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL = 0x4u32
	RotateClockwise180FlipHorizontal = 0x4u32,
	/// D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL = 0x5u32
	RotateClockwise90FlipHorizontal = 0x5u32,
	/// D2D1_ORIENTATION_ROTATE_CLOCKWISE270 = 0x6u32
	RotateClockwise270   = 0x6u32,
	/// D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL = 0x7u32
	RotateClockwise270FlipHorizontal = 0x7u32,
	/// D2D1_ORIENTATION_ROTATE_CLOCKWISE90 = 0x8u32
	RotateClockwise90    = 0x8u32,
	/// D2D1_ORIENTATION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_IMAGE_SOURCE_LOADING_OPTIONS
	#[repr(transparent)]
	pub struct D2D1ImageSourceLoadingOptions: u32 {
		/// D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE = 0x1u32
		const ReleaseSource        = 0x1u32;
		/// D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND = 0x2u32
		const CacheOnDemand        = 0x2u32;
		/// D2D1_IMAGE_SOURCE_LOADING_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS
	#[repr(transparent)]
	pub struct D2D1ImageSourceFromDxgiOptions: u32 {
		/// D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION = 0x1u32
		const LowQualityPrimaryConversion = 0x1u32;
		/// D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

bitflags::bitflags! {
	/// D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS
	#[repr(transparent)]
	pub struct D2D1TransformedImageSourceOptions: u32 {
		/// D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE = 0x1u32
		const DisableDpiScale      = 0x1u32;
		/// D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_PATCH_EDGE_MODE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1PatchEdgeMode
{
	/// D2D1_PATCH_EDGE_MODE_ALIASED = 0x0u32
	Aliased              = 0x0u32,
	/// D2D1_PATCH_EDGE_MODE_ANTIALIASED = 0x1u32
	Antialiased          = 0x1u32,
	/// D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED = 0x2u32
	AliasedInflated      = 0x2u32,
	/// D2D1_PATCH_EDGE_MODE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

bitflags::bitflags! {
	/// D2D1_SPRITE_OPTIONS
	#[repr(transparent)]
	pub struct D2D1SpriteOptions: u32 {
		/// D2D1_SPRITE_OPTIONS_NONE = 0x0u32
		const None                 = 0x0u32;
		/// D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE = 0x1u32
		const ClampToSourceRectangle = 0x1u32;
		/// D2D1_SPRITE_OPTIONS_FORCE_DWORD = 0xFFFFFFFFu32
		const ForceDword           = 0xFFFFFFFFu32;
	}
}

/// D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorBitmapGlyphSnapOption
{
	/// D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT = 0x0u32
	Default              = 0x0u32,
	/// D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE = 0x1u32
	Disable              = 0x1u32,
	/// D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_GAMMA1
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1Gamma1
{
	/// D2D1_GAMMA1_G22 = 0x0u32
	G22                  = 0x0u32,
	/// D2D1_GAMMA1_G10 = 0x1u32
	G10                  = 0x1u32,
	/// D2D1_GAMMA1_G2084 = 0x2u32
	G2084                = 0x2u32,
	/// D2D1_GAMMA1_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

/// D2D1_COLOR_CONTEXT_TYPE
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D2D1ColorContextType
{
	/// D2D1_COLOR_CONTEXT_TYPE_ICC = 0x0u32
	Icc                  = 0x0u32,
	/// D2D1_COLOR_CONTEXT_TYPE_SIMPLE = 0x1u32
	Simple               = 0x1u32,
	/// D2D1_COLOR_CONTEXT_TYPE_DXGI = 0x2u32
	Dxgi                 = 0x2u32,
	/// D2D1_COLOR_CONTEXT_TYPE_FORCE_DWORD = 0xFFFFFFFFu32
	ForceDword           = 0xFFFFFFFFu32,
}

