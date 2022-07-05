#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DFeatureLevel
{
	_1_0Core             = 0x1000i32,
	_9_1                 = 0x9100i32,
	_9_2                 = 0x9200i32,
	_9_3                 = 0x9300i32,
	_10_0                = 0xA000i32,
	_10_1                = 0xA100i32,
	_11_0                = 0xB000i32,
	_11_1                = 0xB100i32,
	_12_0                = 0xC000i32,
	_12_1                = 0xC100i32,
	_12_2                = 0xC200i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DName
{
	Undefined            = 0x0i32,
	Position             = 0x1i32,
	ClipDistance         = 0x2i32,
	CullDistance         = 0x3i32,
	RenderTargetArrayIndex = 0x4i32,
	ViewportArrayIndex   = 0x5i32,
	VertexId             = 0x6i32,
	PrimitiveId          = 0x7i32,
	InstanceId           = 0x8i32,
	IsFrontFace          = 0x9i32,
	SampleIndex          = 0xAi32,
	FinalQuadEdgeTessFactor = 0xBi32,
	FinalQuadInsideTessFactor = 0xCi32,
	FinalTriEdgeTessFactor = 0xDi32,
	FinalTriInsideTessFactor = 0xEi32,
	FinalLineDetailTessFactor = 0xFi32,
	FinalLineDensityTessFactor = 0x10i32,
	BAryCenTriCs         = 0x17i32,
	ShadIngrate          = 0x18i32,
	CullPrimitive        = 0x19i32,
	Target               = 0x40i32,
	Depth                = 0x41i32,
	Coverage             = 0x42i32,
	DepthGreaterEqual    = 0x43i32,
	DepthLessEqual       = 0x44i32,
	StencilRef           = 0x45i32,
	InnerCoverage        = 0x46i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DRegisterComponentType
{
	Unknown              = 0x0i32,
	UInt32               = 0x1i32,
	SInt32               = 0x2i32,
	Float32              = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DMinPrecision
{
	Default              = 0x0i32,
	Float16              = 0x1i32,
	Float2_8             = 0x2i32,
	Reserved             = 0x3i32,
	SInt16               = 0x4i32,
	UInt16               = 0x5i32,
	Any16                = 0xF0i32,
	Any10                = 0xF1i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DCBufferType
{
	CBuffer              = 0x0i32,
	TBuffer              = 0x1i32,
	InterfacePointers    = 0x2i32,
	ResourceBindInfo     = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DShaderVariableClass
{
	Scalar               = 0x0i32,
	Vector               = 0x1i32,
	MatrixRows           = 0x2i32,
	MatrixColumns        = 0x3i32,
	Object               = 0x4i32,
	Struct               = 0x5i32,
	InterfaceClass       = 0x6i32,
	InterfacePointer     = 0x7i32,
	ForceDWord           = 0x7FFFFFFFi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DShaderVariableType
{
	Void                 = 0x0i32,
	Bool                 = 0x1i32,
	Int                  = 0x2i32,
	Float                = 0x3i32,
	String               = 0x4i32,
	Texture              = 0x5i32,
	Texture1D            = 0x6i32,
	Texture2D            = 0x7i32,
	Texture3D            = 0x8i32,
	TextureCube          = 0x9i32,
	Sampler              = 0xAi32,
	Sampler1D            = 0xBi32,
	Sampler2D            = 0xCi32,
	Sampler3D            = 0xDi32,
	SamplerCube          = 0xEi32,
	PixelShader          = 0xFi32,
	VertexShader         = 0x10i32,
	PixelFragment        = 0x11i32,
	VertexFragment       = 0x12i32,
	UInt                 = 0x13i32,
	UInt8                = 0x14i32,
	GeometryShader       = 0x15i32,
	Rasterizer           = 0x16i32,
	DepthStencil         = 0x17i32,
	Blend                = 0x18i32,
	Buffer               = 0x19i32,
	CBuffer              = 0x1Ai32,
	TBuffer              = 0x1Bi32,
	Texture1DArray       = 0x1Ci32,
	Texture2DArray       = 0x1Di32,
	RenderTargetView     = 0x1Ei32,
	DepthStencilView     = 0x1Fi32,
	Texture2DMs          = 0x20i32,
	Texture2DMsArray     = 0x21i32,
	TextureCubeArray     = 0x22i32,
	HullShader           = 0x23i32,
	DomainShader         = 0x24i32,
	InterfacePointer     = 0x25i32,
	ComputeShader        = 0x26i32,
	Double               = 0x27i32,
	RwTexture1D          = 0x28i32,
	RwTexture1DArray     = 0x29i32,
	RwTexture2D          = 0x2Ai32,
	RwTexture2DArray     = 0x2Bi32,
	RwTexture3D          = 0x2Ci32,
	RwBuffer             = 0x2Di32,
	ByteAddressBuffer    = 0x2Ei32,
	RwByteAddressBuffer  = 0x2Fi32,
	StructuredBuffer     = 0x30i32,
	RwStructuredBuffer   = 0x31i32,
	AppendStructuredBuffer = 0x32i32,
	ConsumeStructuredBuffer = 0x33i32,
	Min8Float            = 0x34i32,
	Min10Float           = 0x35i32,
	Min16Float           = 0x36i32,
	Min12Int             = 0x37i32,
	Min16Int             = 0x38i32,
	Min16UInt            = 0x39i32,
	Int16                = 0x3Ai32,
	UInt16               = 0x3Bi32,
	Float16              = 0x3Ci32,
	Int64                = 0x3Di32,
	UInt64               = 0x3Ei32,
	ForceDWord           = 0x7FFFFFFFi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DPrimitiveTopology
{
	Undefined            = 0x0i32,
	PointList            = 0x1i32,
	LineList             = 0x2i32,
	LineStrip            = 0x3i32,
	TriangleList         = 0x4i32,
	TriangleStrip        = 0x5i32,
	LineListAdj          = 0xAi32,
	LineStripAdj         = 0xBi32,
	TriangleListAdj      = 0xCi32,
	TriangleStripAdj     = 0xDi32,
	_1ControlPointPatchList = 0x21i32,
	_2ControlPointPatchList = 0x22i32,
	_3ControlPointPatchList = 0x23i32,
	_4ControlPointPatchList = 0x24i32,
	_5ControlPointPatchList = 0x25i32,
	_6ControlPointPatchList = 0x26i32,
	_7ControlPointPatchList = 0x27i32,
	_8ControlPointPatchList = 0x28i32,
	_9ControlPointPatchList = 0x29i32,
	_10ControlPointPatchList = 0x2Ai32,
	_11ControlPointPatchList = 0x2Bi32,
	_12ControlPointPatchList = 0x2Ci32,
	_13ControlPointPatchList = 0x2Di32,
	_14ControlPointPatchList = 0x2Ei32,
	_15ControlPointPatchList = 0x2Fi32,
	_16ControlPointPatchList = 0x30i32,
	_17ControlPointPatchList = 0x31i32,
	_18ControlPointPatchList = 0x32i32,
	_19ControlPointPatchList = 0x33i32,
	_20ControlPointPatchList = 0x34i32,
	_21ControlPointPatchList = 0x35i32,
	_22ControlPointPatchList = 0x36i32,
	_23ControlPointPatchList = 0x37i32,
	_24ControlPointPatchList = 0x38i32,
	_25ControlPointPatchList = 0x39i32,
	_26ControlPointPatchList = 0x3Ai32,
	_27ControlPointPatchList = 0x3Bi32,
	_28ControlPointPatchList = 0x3Ci32,
	_29ControlPointPatchList = 0x3Di32,
	_30ControlPointPatchList = 0x3Ei32,
	_31ControlPointPatchList = 0x3Fi32,
	_32ControlPointPatchList = 0x40i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DPrimitive
{
	Undefined            = 0x0i32,
	Point                = 0x1i32,
	Line                 = 0x2i32,
	Triangle             = 0x3i32,
	LineAdj              = 0x6i32,
	TriangleAdj          = 0x7i32,
	_1ControlPointPatch  = 0x8i32,
	_2ControlPointPatch  = 0x9i32,
	_3ControlPointPatch  = 0xAi32,
	_4ControlPointPatch  = 0xBi32,
	_5ControlPointPatch  = 0xCi32,
	_6ControlPointPatch  = 0xDi32,
	_7ControlPointPatch  = 0xEi32,
	_8ControlPointPatch  = 0xFi32,
	_9ControlPointPatch  = 0x10i32,
	_10ControlPointPatch = 0x11i32,
	_11ControlPointPatch = 0x12i32,
	_12ControlPointPatch = 0x13i32,
	_13ControlPointPatch = 0x14i32,
	_14ControlPointPatch = 0x15i32,
	_15ControlPointPatch = 0x16i32,
	_16ControlPointPatch = 0x17i32,
	_17ControlPointPatch = 0x18i32,
	_18ControlPointPatch = 0x19i32,
	_19ControlPointPatch = 0x1Ai32,
	_20ControlPointPatch = 0x1Bi32,
	_21ControlPointPatch = 0x1Ci32,
	_22ControlPointPatch = 0x1Di32,
	_23ControlPointPatch = 0x1Ei32,
	_24ControlPointPatch = 0x1Fi32,
	_25ControlPointPatch = 0x20i32,
	_26ControlPointPatch = 0x21i32,
	_27ControlPointPatch = 0x22i32,
	_28ControlPointPatch = 0x23i32,
	_29ControlPointPatch = 0x24i32,
	_30ControlPointPatch = 0x25i32,
	_31ControlPointPatch = 0x26i32,
	_32ControlPointPatch = 0x27i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DTessellatorOutputPrimitive
{
	Undefined            = 0x0i32,
	Point                = 0x1i32,
	Line                 = 0x2i32,
	TriangleCW           = 0x3i32,
	TriangleCcw          = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DTessellatorPartitioning
{
	Undefined            = 0x0i32,
	Integer              = 0x1i32,
	Pow2                 = 0x2i32,
	FractionalOdd        = 0x3i32,
	FractionalEven       = 0x4i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DTessellatorDomain
{
	Undefined            = 0x0i32,
	Isoline              = 0x1i32,
	Tri                  = 0x2i32,
	Quad                 = 0x3i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DShaderInputType
{
	CBuffer              = 0x0i32,
	TBuffer              = 0x1i32,
	Texture              = 0x2i32,
	Sampler              = 0x3i32,
	UavRwTyped           = 0x4i32,
	Structured           = 0x5i32,
	UavRwStructured      = 0x6i32,
	ByteAddress          = 0x7i32,
	UavRwByteAddress     = 0x8i32,
	UavAppendStructured  = 0x9i32,
	UavConsumeStructured = 0xAi32,
	UavRwStructuredWithCounter = 0xBi32,
	RtAccelerationStructure = 0xCi32,
	UavFeedbackTexture   = 0xDi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DResourceReturnType
{
	UNorm                = 0x1i32,
	SNorm                = 0x2i32,
	SInt                 = 0x3i32,
	UInt                 = 0x4i32,
	Float                = 0x5i32,
	Mixed                = 0x6i32,
	Double               = 0x7i32,
	Continued            = 0x8i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DSrvDimension
{
	DimensionUnknown     = 0x0i32,
	DimensionBuffer      = 0x1i32,
	DimensionTexture1D   = 0x2i32,
	DimensionTexture1DArray = 0x3i32,
	DimensionTexture2D   = 0x4i32,
	DimensionTexture2DArray = 0x5i32,
	DimensionTexture2DMs = 0x6i32,
	DimensionTexture2DMsArray = 0x7i32,
	DimensionTexture3D   = 0x8i32,
	DimensionTextureCube = 0x9i32,
	DimensionTextureCubeArray = 0xAi32,
	DimensionBufferEx    = 0xBi32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DInterpolationMode
{
	InterpolationUndefined = 0x0i32,
	InterpolationConstant = 0x1i32,
	InterpolationLinear  = 0x2i32,
	InterpolationLinearCentroid = 0x3i32,
	InterpolationLinearNoPerspective = 0x4i32,
	InterpolationLinearNoPerspectiveCentroid = 0x5i32,
	InterpolationLinearSample = 0x6i32,
	InterpolationLinearNoPerspectiveSample = 0x7i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum D3DParameterFlags
{
	None                 = 0x0i32,
	In                   = 0x1i32,
	Out                  = 0x2i32,
	ForceDWord           = 0x7FFFFFFFi32,
}

