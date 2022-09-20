#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// D3D_FEATURE_LEVEL
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DFeatureLevel
{
	/// D3D_FEATURE_LEVEL_1_0_CORE = 0x1000i32
	_1_0Core             = 0x1000i32,
	/// D3D_FEATURE_LEVEL_9_1 = 0x9100i32
	_9_1                 = 0x9100i32,
	/// D3D_FEATURE_LEVEL_9_2 = 0x9200i32
	_9_2                 = 0x9200i32,
	/// D3D_FEATURE_LEVEL_9_3 = 0x9300i32
	_9_3                 = 0x9300i32,
	/// D3D_FEATURE_LEVEL_10_0 = 0xA000i32
	_10_0                = 0xA000i32,
	/// D3D_FEATURE_LEVEL_10_1 = 0xA100i32
	_10_1                = 0xA100i32,
	/// D3D_FEATURE_LEVEL_11_0 = 0xB000i32
	_11_0                = 0xB000i32,
	/// D3D_FEATURE_LEVEL_11_1 = 0xB100i32
	_11_1                = 0xB100i32,
	/// D3D_FEATURE_LEVEL_12_0 = 0xC000i32
	_12_0                = 0xC000i32,
	/// D3D_FEATURE_LEVEL_12_1 = 0xC100i32
	_12_1                = 0xC100i32,
	/// D3D_FEATURE_LEVEL_12_2 = 0xC200i32
	_12_2                = 0xC200i32,
}

/// D3D_NAME
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DName
{
	/// D3D_NAME_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// D3D_NAME_POSITION = 0x1i32
	Position             = 0x1i32,
	/// D3D_NAME_CLIP_DISTANCE = 0x2i32
	ClipDistance         = 0x2i32,
	/// D3D_NAME_CULL_DISTANCE = 0x3i32
	CullDistance         = 0x3i32,
	/// D3D_NAME_RENDER_TARGET_ARRAY_INDEX = 0x4i32
	RenderTargetArrayIndex = 0x4i32,
	/// D3D_NAME_VIEWPORT_ARRAY_INDEX = 0x5i32
	ViewportArrayIndex   = 0x5i32,
	/// D3D_NAME_VERTEX_ID = 0x6i32
	VertexId             = 0x6i32,
	/// D3D_NAME_PRIMITIVE_ID = 0x7i32
	PrimitiveId          = 0x7i32,
	/// D3D_NAME_INSTANCE_ID = 0x8i32
	InstanceId           = 0x8i32,
	/// D3D_NAME_IS_FRONT_FACE = 0x9i32
	IsFrontFace          = 0x9i32,
	/// D3D_NAME_SAMPLE_INDEX = 0xAi32
	SampleIndex          = 0xAi32,
	/// D3D_NAME_FINAL_QUAD_EDGE_TESSFACTOR = 0xBi32
	FinalQuadEdgeTessFactor = 0xBi32,
	/// D3D_NAME_FINAL_QUAD_INSIDE_TESSFACTOR = 0xCi32
	FinalQuadInsideTessFactor = 0xCi32,
	/// D3D_NAME_FINAL_TRI_EDGE_TESSFACTOR = 0xDi32
	FinalTriEdgeTessFactor = 0xDi32,
	/// D3D_NAME_FINAL_TRI_INSIDE_TESSFACTOR = 0xEi32
	FinalTriInsideTessFactor = 0xEi32,
	/// D3D_NAME_FINAL_LINE_DETAIL_TESSFACTOR = 0xFi32
	FinalLineDetailTessFactor = 0xFi32,
	/// D3D_NAME_FINAL_LINE_DENSITY_TESSFACTOR = 0x10i32
	FinalLineDensityTessFactor = 0x10i32,
	/// D3D_NAME_BARYCENTRICS = 0x17i32
	BAryCenTriCs         = 0x17i32,
	/// D3D_NAME_SHADINGRATE = 0x18i32
	ShadIngrate          = 0x18i32,
	/// D3D_NAME_CULLPRIMITIVE = 0x19i32
	CullPrimitive        = 0x19i32,
	/// D3D_NAME_TARGET = 0x40i32
	Target               = 0x40i32,
	/// D3D_NAME_DEPTH = 0x41i32
	Depth                = 0x41i32,
	/// D3D_NAME_COVERAGE = 0x42i32
	Coverage             = 0x42i32,
	/// D3D_NAME_DEPTH_GREATER_EQUAL = 0x43i32
	DepthGreaterEqual    = 0x43i32,
	/// D3D_NAME_DEPTH_LESS_EQUAL = 0x44i32
	DepthLessEqual       = 0x44i32,
	/// D3D_NAME_STENCIL_REF = 0x45i32
	StencilRef           = 0x45i32,
	/// D3D_NAME_INNER_COVERAGE = 0x46i32
	InnerCoverage        = 0x46i32,
}

/// D3D_REGISTER_COMPONENT_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DRegisterComponentType
{
	/// D3D_REGISTER_COMPONENT_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D_REGISTER_COMPONENT_UINT32 = 0x1i32
	Uint32               = 0x1i32,
	/// D3D_REGISTER_COMPONENT_SINT32 = 0x2i32
	Sint32               = 0x2i32,
	/// D3D_REGISTER_COMPONENT_FLOAT32 = 0x3i32
	Float32              = 0x3i32,
}

/// D3D_MIN_PRECISION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DMinPrecision
{
	/// D3D_MIN_PRECISION_DEFAULT = 0x0i32
	Default              = 0x0i32,
	/// D3D_MIN_PRECISION_FLOAT_16 = 0x1i32
	Float16              = 0x1i32,
	/// D3D_MIN_PRECISION_FLOAT_2_8 = 0x2i32
	Float2_8             = 0x2i32,
	/// D3D_MIN_PRECISION_RESERVED = 0x3i32
	Reserved             = 0x3i32,
	/// D3D_MIN_PRECISION_SINT_16 = 0x4i32
	Sint16               = 0x4i32,
	/// D3D_MIN_PRECISION_UINT_16 = 0x5i32
	Uint16               = 0x5i32,
	/// D3D_MIN_PRECISION_ANY_16 = 0xF0i32
	Any16                = 0xF0i32,
	/// D3D_MIN_PRECISION_ANY_10 = 0xF1i32
	Any10                = 0xF1i32,
}

/// D3D_CBUFFER_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DCBufferType
{
	/// D3D_CT_CBUFFER = 0x0i32
	CBuffer              = 0x0i32,
	/// D3D_CT_TBUFFER = 0x1i32
	TBuffer              = 0x1i32,
	/// D3D_CT_INTERFACE_POINTERS = 0x2i32
	InterfacePointers    = 0x2i32,
	/// D3D_CT_RESOURCE_BIND_INFO = 0x3i32
	ResourceBindInfo     = 0x3i32,
}

/// D3D_SHADER_VARIABLE_CLASS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DShaderVariableClass
{
	/// D3D_SVC_SCALAR = 0x0i32
	Scalar               = 0x0i32,
	/// D3D_SVC_VECTOR = 0x1i32
	Vector               = 0x1i32,
	/// D3D_SVC_MATRIX_ROWS = 0x2i32
	MatrixRows           = 0x2i32,
	/// D3D_SVC_MATRIX_COLUMNS = 0x3i32
	MatrixColumns        = 0x3i32,
	/// D3D_SVC_OBJECT = 0x4i32
	Object               = 0x4i32,
	/// D3D_SVC_STRUCT = 0x5i32
	Struct               = 0x5i32,
	/// D3D_SVC_INTERFACE_CLASS = 0x6i32
	InterfaceClass       = 0x6i32,
	/// D3D_SVC_INTERFACE_POINTER = 0x7i32
	InterfacePointer     = 0x7i32,
	/// D3D_SVC_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

/// D3D_SHADER_VARIABLE_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DShaderVariableType
{
	/// D3D_SVT_VOID = 0x0i32
	Void                 = 0x0i32,
	/// D3D_SVT_BOOL = 0x1i32
	Bool                 = 0x1i32,
	/// D3D_SVT_INT = 0x2i32
	Int                  = 0x2i32,
	/// D3D_SVT_FLOAT = 0x3i32
	Float                = 0x3i32,
	/// D3D_SVT_STRING = 0x4i32
	String               = 0x4i32,
	/// D3D_SVT_TEXTURE = 0x5i32
	Texture              = 0x5i32,
	/// D3D_SVT_TEXTURE1D = 0x6i32
	Texture1d            = 0x6i32,
	/// D3D_SVT_TEXTURE2D = 0x7i32
	Texture2d            = 0x7i32,
	/// D3D_SVT_TEXTURE3D = 0x8i32
	Texture3d            = 0x8i32,
	/// D3D_SVT_TEXTURECUBE = 0x9i32
	TextureCube          = 0x9i32,
	/// D3D_SVT_SAMPLER = 0xAi32
	Sampler              = 0xAi32,
	/// D3D_SVT_SAMPLER1D = 0xBi32
	Sampler1d            = 0xBi32,
	/// D3D_SVT_SAMPLER2D = 0xCi32
	Sampler2d            = 0xCi32,
	/// D3D_SVT_SAMPLER3D = 0xDi32
	Sampler3d            = 0xDi32,
	/// D3D_SVT_SAMPLERCUBE = 0xEi32
	SamplerCube          = 0xEi32,
	/// D3D_SVT_PIXELSHADER = 0xFi32
	PixelShader          = 0xFi32,
	/// D3D_SVT_VERTEXSHADER = 0x10i32
	VertexShader         = 0x10i32,
	/// D3D_SVT_PIXELFRAGMENT = 0x11i32
	PixelFragment        = 0x11i32,
	/// D3D_SVT_VERTEXFRAGMENT = 0x12i32
	VertexFragment       = 0x12i32,
	/// D3D_SVT_UINT = 0x13i32
	Uint                 = 0x13i32,
	/// D3D_SVT_UINT8 = 0x14i32
	Uint8                = 0x14i32,
	/// D3D_SVT_GEOMETRYSHADER = 0x15i32
	GeometryShader       = 0x15i32,
	/// D3D_SVT_RASTERIZER = 0x16i32
	Rasterizer           = 0x16i32,
	/// D3D_SVT_DEPTHSTENCIL = 0x17i32
	DepthStencil         = 0x17i32,
	/// D3D_SVT_BLEND = 0x18i32
	Blend                = 0x18i32,
	/// D3D_SVT_BUFFER = 0x19i32
	Buffer               = 0x19i32,
	/// D3D_SVT_CBUFFER = 0x1Ai32
	CBuffer              = 0x1Ai32,
	/// D3D_SVT_TBUFFER = 0x1Bi32
	TBuffer              = 0x1Bi32,
	/// D3D_SVT_TEXTURE1DARRAY = 0x1Ci32
	Texture1dArray       = 0x1Ci32,
	/// D3D_SVT_TEXTURE2DARRAY = 0x1Di32
	Texture2dArray       = 0x1Di32,
	/// D3D_SVT_RENDERTARGETVIEW = 0x1Ei32
	RenderTargetView     = 0x1Ei32,
	/// D3D_SVT_DEPTHSTENCILVIEW = 0x1Fi32
	DepthStencilView     = 0x1Fi32,
	/// D3D_SVT_TEXTURE2DMS = 0x20i32
	Texture2dMs          = 0x20i32,
	/// D3D_SVT_TEXTURE2DMSARRAY = 0x21i32
	Texture2dMsArray     = 0x21i32,
	/// D3D_SVT_TEXTURECUBEARRAY = 0x22i32
	TextureCubeArray     = 0x22i32,
	/// D3D_SVT_HULLSHADER = 0x23i32
	HullShader           = 0x23i32,
	/// D3D_SVT_DOMAINSHADER = 0x24i32
	DomainShader         = 0x24i32,
	/// D3D_SVT_INTERFACE_POINTER = 0x25i32
	InterfacePointer     = 0x25i32,
	/// D3D_SVT_COMPUTESHADER = 0x26i32
	ComputeShader        = 0x26i32,
	/// D3D_SVT_DOUBLE = 0x27i32
	Double               = 0x27i32,
	/// D3D_SVT_RWTEXTURE1D = 0x28i32
	RwTexture1d          = 0x28i32,
	/// D3D_SVT_RWTEXTURE1DARRAY = 0x29i32
	RwTexture1dArray     = 0x29i32,
	/// D3D_SVT_RWTEXTURE2D = 0x2Ai32
	RwTexture2d          = 0x2Ai32,
	/// D3D_SVT_RWTEXTURE2DARRAY = 0x2Bi32
	RwTexture2dArray     = 0x2Bi32,
	/// D3D_SVT_RWTEXTURE3D = 0x2Ci32
	RwTexture3d          = 0x2Ci32,
	/// D3D_SVT_RWBUFFER = 0x2Di32
	RwBuffer             = 0x2Di32,
	/// D3D_SVT_BYTEADDRESS_BUFFER = 0x2Ei32
	ByteAddressBuffer    = 0x2Ei32,
	/// D3D_SVT_RWBYTEADDRESS_BUFFER = 0x2Fi32
	RwByteAddressBuffer  = 0x2Fi32,
	/// D3D_SVT_STRUCTURED_BUFFER = 0x30i32
	StructuredBuffer     = 0x30i32,
	/// D3D_SVT_RWSTRUCTURED_BUFFER = 0x31i32
	RwStructuredBuffer   = 0x31i32,
	/// D3D_SVT_APPEND_STRUCTURED_BUFFER = 0x32i32
	AppendStructuredBuffer = 0x32i32,
	/// D3D_SVT_CONSUME_STRUCTURED_BUFFER = 0x33i32
	ConsumeStructuredBuffer = 0x33i32,
	/// D3D_SVT_MIN8FLOAT = 0x34i32
	Min8Float            = 0x34i32,
	/// D3D_SVT_MIN10FLOAT = 0x35i32
	Min10Float           = 0x35i32,
	/// D3D_SVT_MIN16FLOAT = 0x36i32
	Min16Float           = 0x36i32,
	/// D3D_SVT_MIN12INT = 0x37i32
	Min12Int             = 0x37i32,
	/// D3D_SVT_MIN16INT = 0x38i32
	Min16Int             = 0x38i32,
	/// D3D_SVT_MIN16UINT = 0x39i32
	Min16Uint            = 0x39i32,
	/// D3D_SVT_INT16 = 0x3Ai32
	Int16                = 0x3Ai32,
	/// D3D_SVT_UINT16 = 0x3Bi32
	Uint16               = 0x3Bi32,
	/// D3D_SVT_FLOAT16 = 0x3Ci32
	Float16              = 0x3Ci32,
	/// D3D_SVT_INT64 = 0x3Di32
	Int64                = 0x3Di32,
	/// D3D_SVT_UINT64 = 0x3Ei32
	Uint64               = 0x3Ei32,
	/// D3D_SVT_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

/// D3D_PRIMITIVE_TOPOLOGY
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DPrimitiveTopology
{
	/// D3D_PRIMITIVE_TOPOLOGY_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// D3D_PRIMITIVE_TOPOLOGY_POINTLIST = 0x1i32
	PointList            = 0x1i32,
	/// D3D_PRIMITIVE_TOPOLOGY_LINELIST = 0x2i32
	LineList             = 0x2i32,
	/// D3D_PRIMITIVE_TOPOLOGY_LINESTRIP = 0x3i32
	LineStrip            = 0x3i32,
	/// D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST = 0x4i32
	TriangleList         = 0x4i32,
	/// D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP = 0x5i32
	TriangleStrip        = 0x5i32,
	/// D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ = 0xAi32
	LineListAdj          = 0xAi32,
	/// D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ = 0xBi32
	LineStripAdj         = 0xBi32,
	/// D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ = 0xCi32
	TriangleListAdj      = 0xCi32,
	/// D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ = 0xDi32
	TriangleStripAdj     = 0xDi32,
	/// D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST = 0x21i32
	_1ControlPointPatchList = 0x21i32,
	/// D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST = 0x22i32
	_2ControlPointPatchList = 0x22i32,
	/// D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST = 0x23i32
	_3ControlPointPatchList = 0x23i32,
	/// D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST = 0x24i32
	_4ControlPointPatchList = 0x24i32,
	/// D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST = 0x25i32
	_5ControlPointPatchList = 0x25i32,
	/// D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST = 0x26i32
	_6ControlPointPatchList = 0x26i32,
	/// D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST = 0x27i32
	_7ControlPointPatchList = 0x27i32,
	/// D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST = 0x28i32
	_8ControlPointPatchList = 0x28i32,
	/// D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST = 0x29i32
	_9ControlPointPatchList = 0x29i32,
	/// D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST = 0x2Ai32
	_10ControlPointPatchList = 0x2Ai32,
	/// D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST = 0x2Bi32
	_11ControlPointPatchList = 0x2Bi32,
	/// D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST = 0x2Ci32
	_12ControlPointPatchList = 0x2Ci32,
	/// D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST = 0x2Di32
	_13ControlPointPatchList = 0x2Di32,
	/// D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST = 0x2Ei32
	_14ControlPointPatchList = 0x2Ei32,
	/// D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST = 0x2Fi32
	_15ControlPointPatchList = 0x2Fi32,
	/// D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST = 0x30i32
	_16ControlPointPatchList = 0x30i32,
	/// D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST = 0x31i32
	_17ControlPointPatchList = 0x31i32,
	/// D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST = 0x32i32
	_18ControlPointPatchList = 0x32i32,
	/// D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST = 0x33i32
	_19ControlPointPatchList = 0x33i32,
	/// D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST = 0x34i32
	_20ControlPointPatchList = 0x34i32,
	/// D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST = 0x35i32
	_21ControlPointPatchList = 0x35i32,
	/// D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST = 0x36i32
	_22ControlPointPatchList = 0x36i32,
	/// D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST = 0x37i32
	_23ControlPointPatchList = 0x37i32,
	/// D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST = 0x38i32
	_24ControlPointPatchList = 0x38i32,
	/// D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST = 0x39i32
	_25ControlPointPatchList = 0x39i32,
	/// D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST = 0x3Ai32
	_26ControlPointPatchList = 0x3Ai32,
	/// D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST = 0x3Bi32
	_27ControlPointPatchList = 0x3Bi32,
	/// D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST = 0x3Ci32
	_28ControlPointPatchList = 0x3Ci32,
	/// D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST = 0x3Di32
	_29ControlPointPatchList = 0x3Di32,
	/// D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST = 0x3Ei32
	_30ControlPointPatchList = 0x3Ei32,
	/// D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST = 0x3Fi32
	_31ControlPointPatchList = 0x3Fi32,
	/// D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST = 0x40i32
	_32ControlPointPatchList = 0x40i32,
}

/// D3D_PRIMITIVE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DPrimitive
{
	/// D3D_PRIMITIVE_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// D3D_PRIMITIVE_POINT = 0x1i32
	Point                = 0x1i32,
	/// D3D_PRIMITIVE_LINE = 0x2i32
	Line                 = 0x2i32,
	/// D3D_PRIMITIVE_TRIANGLE = 0x3i32
	Triangle             = 0x3i32,
	/// D3D_PRIMITIVE_LINE_ADJ = 0x6i32
	LineAdj              = 0x6i32,
	/// D3D_PRIMITIVE_TRIANGLE_ADJ = 0x7i32
	TriangleAdj          = 0x7i32,
	/// D3D_PRIMITIVE_1_CONTROL_POINT_PATCH = 0x8i32
	_1ControlPointPatch  = 0x8i32,
	/// D3D_PRIMITIVE_2_CONTROL_POINT_PATCH = 0x9i32
	_2ControlPointPatch  = 0x9i32,
	/// D3D_PRIMITIVE_3_CONTROL_POINT_PATCH = 0xAi32
	_3ControlPointPatch  = 0xAi32,
	/// D3D_PRIMITIVE_4_CONTROL_POINT_PATCH = 0xBi32
	_4ControlPointPatch  = 0xBi32,
	/// D3D_PRIMITIVE_5_CONTROL_POINT_PATCH = 0xCi32
	_5ControlPointPatch  = 0xCi32,
	/// D3D_PRIMITIVE_6_CONTROL_POINT_PATCH = 0xDi32
	_6ControlPointPatch  = 0xDi32,
	/// D3D_PRIMITIVE_7_CONTROL_POINT_PATCH = 0xEi32
	_7ControlPointPatch  = 0xEi32,
	/// D3D_PRIMITIVE_8_CONTROL_POINT_PATCH = 0xFi32
	_8ControlPointPatch  = 0xFi32,
	/// D3D_PRIMITIVE_9_CONTROL_POINT_PATCH = 0x10i32
	_9ControlPointPatch  = 0x10i32,
	/// D3D_PRIMITIVE_10_CONTROL_POINT_PATCH = 0x11i32
	_10ControlPointPatch = 0x11i32,
	/// D3D_PRIMITIVE_11_CONTROL_POINT_PATCH = 0x12i32
	_11ControlPointPatch = 0x12i32,
	/// D3D_PRIMITIVE_12_CONTROL_POINT_PATCH = 0x13i32
	_12ControlPointPatch = 0x13i32,
	/// D3D_PRIMITIVE_13_CONTROL_POINT_PATCH = 0x14i32
	_13ControlPointPatch = 0x14i32,
	/// D3D_PRIMITIVE_14_CONTROL_POINT_PATCH = 0x15i32
	_14ControlPointPatch = 0x15i32,
	/// D3D_PRIMITIVE_15_CONTROL_POINT_PATCH = 0x16i32
	_15ControlPointPatch = 0x16i32,
	/// D3D_PRIMITIVE_16_CONTROL_POINT_PATCH = 0x17i32
	_16ControlPointPatch = 0x17i32,
	/// D3D_PRIMITIVE_17_CONTROL_POINT_PATCH = 0x18i32
	_17ControlPointPatch = 0x18i32,
	/// D3D_PRIMITIVE_18_CONTROL_POINT_PATCH = 0x19i32
	_18ControlPointPatch = 0x19i32,
	/// D3D_PRIMITIVE_19_CONTROL_POINT_PATCH = 0x1Ai32
	_19ControlPointPatch = 0x1Ai32,
	/// D3D_PRIMITIVE_20_CONTROL_POINT_PATCH = 0x1Bi32
	_20ControlPointPatch = 0x1Bi32,
	/// D3D_PRIMITIVE_21_CONTROL_POINT_PATCH = 0x1Ci32
	_21ControlPointPatch = 0x1Ci32,
	/// D3D_PRIMITIVE_22_CONTROL_POINT_PATCH = 0x1Di32
	_22ControlPointPatch = 0x1Di32,
	/// D3D_PRIMITIVE_23_CONTROL_POINT_PATCH = 0x1Ei32
	_23ControlPointPatch = 0x1Ei32,
	/// D3D_PRIMITIVE_24_CONTROL_POINT_PATCH = 0x1Fi32
	_24ControlPointPatch = 0x1Fi32,
	/// D3D_PRIMITIVE_25_CONTROL_POINT_PATCH = 0x20i32
	_25ControlPointPatch = 0x20i32,
	/// D3D_PRIMITIVE_26_CONTROL_POINT_PATCH = 0x21i32
	_26ControlPointPatch = 0x21i32,
	/// D3D_PRIMITIVE_27_CONTROL_POINT_PATCH = 0x22i32
	_27ControlPointPatch = 0x22i32,
	/// D3D_PRIMITIVE_28_CONTROL_POINT_PATCH = 0x23i32
	_28ControlPointPatch = 0x23i32,
	/// D3D_PRIMITIVE_29_CONTROL_POINT_PATCH = 0x24i32
	_29ControlPointPatch = 0x24i32,
	/// D3D_PRIMITIVE_30_CONTROL_POINT_PATCH = 0x25i32
	_30ControlPointPatch = 0x25i32,
	/// D3D_PRIMITIVE_31_CONTROL_POINT_PATCH = 0x26i32
	_31ControlPointPatch = 0x26i32,
	/// D3D_PRIMITIVE_32_CONTROL_POINT_PATCH = 0x27i32
	_32ControlPointPatch = 0x27i32,
}

/// D3D_TESSELLATOR_OUTPUT_PRIMITIVE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DTessellatorOutputPrimitive
{
	/// D3D_TESSELLATOR_OUTPUT_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// D3D_TESSELLATOR_OUTPUT_POINT = 0x1i32
	Point                = 0x1i32,
	/// D3D_TESSELLATOR_OUTPUT_LINE = 0x2i32
	Line                 = 0x2i32,
	/// D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW = 0x3i32
	TriangleCw           = 0x3i32,
	/// D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW = 0x4i32
	TriangleCcw          = 0x4i32,
}

/// D3D_TESSELLATOR_PARTITIONING
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DTessellatorPartitioning
{
	/// D3D_TESSELLATOR_PARTITIONING_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// D3D_TESSELLATOR_PARTITIONING_INTEGER = 0x1i32
	Integer              = 0x1i32,
	/// D3D_TESSELLATOR_PARTITIONING_POW2 = 0x2i32
	Pow2                 = 0x2i32,
	/// D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD = 0x3i32
	FractionalOdd        = 0x3i32,
	/// D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN = 0x4i32
	FractionalEven       = 0x4i32,
}

/// D3D_TESSELLATOR_DOMAIN
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DTessellatorDomain
{
	/// D3D_TESSELLATOR_DOMAIN_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// D3D_TESSELLATOR_DOMAIN_ISOLINE = 0x1i32
	Isoline              = 0x1i32,
	/// D3D_TESSELLATOR_DOMAIN_TRI = 0x2i32
	Tri                  = 0x2i32,
	/// D3D_TESSELLATOR_DOMAIN_QUAD = 0x3i32
	Quad                 = 0x3i32,
}

/// D3D_SHADER_INPUT_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DShaderInputType
{
	/// D3D_SIT_CBUFFER = 0x0i32
	CBuffer              = 0x0i32,
	/// D3D_SIT_TBUFFER = 0x1i32
	TBuffer              = 0x1i32,
	/// D3D_SIT_TEXTURE = 0x2i32
	Texture              = 0x2i32,
	/// D3D_SIT_SAMPLER = 0x3i32
	Sampler              = 0x3i32,
	/// D3D_SIT_UAV_RWTYPED = 0x4i32
	UavRwTyped           = 0x4i32,
	/// D3D_SIT_STRUCTURED = 0x5i32
	Structured           = 0x5i32,
	/// D3D_SIT_UAV_RWSTRUCTURED = 0x6i32
	UavRwStructured      = 0x6i32,
	/// D3D_SIT_BYTEADDRESS = 0x7i32
	ByteAddress          = 0x7i32,
	/// D3D_SIT_UAV_RWBYTEADDRESS = 0x8i32
	UavRwByteAddress     = 0x8i32,
	/// D3D_SIT_UAV_APPEND_STRUCTURED = 0x9i32
	UavAppendStructured  = 0x9i32,
	/// D3D_SIT_UAV_CONSUME_STRUCTURED = 0xAi32
	UavConsumeStructured = 0xAi32,
	/// D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER = 0xBi32
	UavRwStructuredWithCounter = 0xBi32,
	/// D3D_SIT_RTACCELERATIONSTRUCTURE = 0xCi32
	RtAccelerationStructure = 0xCi32,
	/// D3D_SIT_UAV_FEEDBACKTEXTURE = 0xDi32
	UavFeedbackTexture   = 0xDi32,
}

impl D3DShaderInputType {
	/// D3D_SIT_RTACCELERATIONSTRUCTURE
	pub const RtAs                : Self = Self::RtAccelerationStructure;
}

/// D3D_RESOURCE_RETURN_TYPE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DResourceReturnType
{
	/// D3D_RETURN_TYPE_UNORM = 0x1i32
	Unorm                = 0x1i32,
	/// D3D_RETURN_TYPE_SNORM = 0x2i32
	Snorm                = 0x2i32,
	/// D3D_RETURN_TYPE_SINT = 0x3i32
	Sint                 = 0x3i32,
	/// D3D_RETURN_TYPE_UINT = 0x4i32
	Uint                 = 0x4i32,
	/// D3D_RETURN_TYPE_FLOAT = 0x5i32
	Float                = 0x5i32,
	/// D3D_RETURN_TYPE_MIXED = 0x6i32
	Mixed                = 0x6i32,
	/// D3D_RETURN_TYPE_DOUBLE = 0x7i32
	Double               = 0x7i32,
	/// D3D_RETURN_TYPE_CONTINUED = 0x8i32
	Continued            = 0x8i32,
}

/// D3D_SRV_DIMENSION
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DSrvDimension
{
	/// D3D_SRV_DIMENSION_UNKNOWN = 0x0i32
	Unknown              = 0x0i32,
	/// D3D_SRV_DIMENSION_BUFFER = 0x1i32
	Buffer               = 0x1i32,
	/// D3D_SRV_DIMENSION_TEXTURE1D = 0x2i32
	Texture1d            = 0x2i32,
	/// D3D_SRV_DIMENSION_TEXTURE1DARRAY = 0x3i32
	Texture1dArray       = 0x3i32,
	/// D3D_SRV_DIMENSION_TEXTURE2D = 0x4i32
	Texture2d            = 0x4i32,
	/// D3D_SRV_DIMENSION_TEXTURE2DARRAY = 0x5i32
	Texture2dArray       = 0x5i32,
	/// D3D_SRV_DIMENSION_TEXTURE2DMS = 0x6i32
	Texture2dMs          = 0x6i32,
	/// D3D_SRV_DIMENSION_TEXTURE2DMSARRAY = 0x7i32
	Texture2dMsArray     = 0x7i32,
	/// D3D_SRV_DIMENSION_TEXTURE3D = 0x8i32
	Texture3d            = 0x8i32,
	/// D3D_SRV_DIMENSION_TEXTURECUBE = 0x9i32
	TextureCube          = 0x9i32,
	/// D3D_SRV_DIMENSION_TEXTURECUBEARRAY = 0xAi32
	TextureCubeArray     = 0xAi32,
	/// D3D_SRV_DIMENSION_BUFFEREX = 0xBi32
	BufferEx             = 0xBi32,
}

impl D3DSrvDimension {
	/// D3D10_SRV_DIMENSION_UNKNOWN = 0x0i32
	pub const SrvDimensionUnknown : Self = unsafe { transmute(0x0i32) };
	/// D3D10_SRV_DIMENSION_BUFFER = 0x1i32
	pub const SrvDimensionBuffer  : Self = unsafe { transmute(0x1i32) };
	/// D3D10_SRV_DIMENSION_TEXTURE1D = 0x2i32
	pub const SrvDimensionTexture1d: Self = unsafe { transmute(0x2i32) };
	/// D3D10_SRV_DIMENSION_TEXTURE1DARRAY = 0x3i32
	pub const SrvDimensionTexture1dArray: Self = unsafe { transmute(0x3i32) };
	/// D3D10_SRV_DIMENSION_TEXTURE2D = 0x4i32
	pub const SrvDimensionTexture2d: Self = unsafe { transmute(0x4i32) };
	/// D3D10_SRV_DIMENSION_TEXTURE2DARRAY = 0x5i32
	pub const SrvDimensionTexture2dArray: Self = unsafe { transmute(0x5i32) };
	/// D3D10_SRV_DIMENSION_TEXTURE2DMS = 0x6i32
	pub const SrvDimensionTexture2dMs: Self = unsafe { transmute(0x6i32) };
	/// D3D10_SRV_DIMENSION_TEXTURE2DMSARRAY = 0x7i32
	pub const SrvDimensionTexture2dMsArray: Self = unsafe { transmute(0x7i32) };
	/// D3D10_SRV_DIMENSION_TEXTURE3D = 0x8i32
	pub const SrvDimensionTexture3d: Self = unsafe { transmute(0x8i32) };
	/// D3D10_SRV_DIMENSION_TEXTURECUBE = 0x9i32
	pub const SrvDimensionTextureCube: Self = unsafe { transmute(0x9i32) };
	/// D3D10_1_SRV_DIMENSION_UNKNOWN = 0x0i32
	pub const _1SrvDimensionUnknown: Self = unsafe { transmute(0x0i32) };
	/// D3D10_1_SRV_DIMENSION_BUFFER = 0x1i32
	pub const _1SrvDimensionBuffer: Self = unsafe { transmute(0x1i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURE1D = 0x2i32
	pub const _1SrvDimensionTexture1d: Self = unsafe { transmute(0x2i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURE1DARRAY = 0x3i32
	pub const _1SrvDimensionTexture1dArray: Self = unsafe { transmute(0x3i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURE2D = 0x4i32
	pub const _1SrvDimensionTexture2d: Self = unsafe { transmute(0x4i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURE2DARRAY = 0x5i32
	pub const _1SrvDimensionTexture2dArray: Self = unsafe { transmute(0x5i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURE2DMS = 0x6i32
	pub const _1SrvDimensionTexture2dMs: Self = unsafe { transmute(0x6i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURE2DMSARRAY = 0x7i32
	pub const _1SrvDimensionTexture2dMsArray: Self = unsafe { transmute(0x7i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURE3D = 0x8i32
	pub const _1SrvDimensionTexture3d: Self = unsafe { transmute(0x8i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURECUBE = 0x9i32
	pub const _1SrvDimensionTextureCube: Self = unsafe { transmute(0x9i32) };
	/// D3D10_1_SRV_DIMENSION_TEXTURECUBEARRAY = 0xAi32
	pub const _1SrvDimensionTextureCubeArray: Self = unsafe { transmute(0xAi32) };
	/// D3D11_SRV_DIMENSION_TEXTURECUBEARRAY = 0xAi32
	pub const SrvDimensionTextureCubeArray: Self = unsafe { transmute(0xAi32) };
	/// D3D11_SRV_DIMENSION_BUFFEREX = 0xBi32
	pub const SrvDimensionBufferEx: Self = unsafe { transmute(0xBi32) };
}

/// D3D_INTERPOLATION_MODE
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DInterpolationMode
{
	/// D3D_INTERPOLATION_UNDEFINED = 0x0i32
	Undefined            = 0x0i32,
	/// D3D_INTERPOLATION_CONSTANT = 0x1i32
	Constant             = 0x1i32,
	/// D3D_INTERPOLATION_LINEAR = 0x2i32
	Linear               = 0x2i32,
	/// D3D_INTERPOLATION_LINEAR_CENTROID = 0x3i32
	LinearCentroid       = 0x3i32,
	/// D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE = 0x4i32
	LinearNoPerspective  = 0x4i32,
	/// D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_CENTROID = 0x5i32
	LinearNoPerspectiveCentroid = 0x5i32,
	/// D3D_INTERPOLATION_LINEAR_SAMPLE = 0x6i32
	LinearSample         = 0x6i32,
	/// D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_SAMPLE = 0x7i32
	LinearNoPerspectiveSample = 0x7i32,
}

/// D3D_PARAMETER_FLAGS
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum D3DParameterFlags
{
	/// D3D_PF_NONE = 0x0i32
	None                 = 0x0i32,
	/// D3D_PF_IN = 0x1i32
	In                   = 0x1i32,
	/// D3D_PF_OUT = 0x2i32
	Out                  = 0x2i32,
	/// D3D_PF_FORCE_DWORD = 0x7FFFFFFFi32
	ForceDword           = 0x7FFFFFFFi32,
}

