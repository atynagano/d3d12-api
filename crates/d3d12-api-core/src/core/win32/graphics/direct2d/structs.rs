#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::direct2d::common::*;
use crate::core::win32::graphics::direct2d::*;
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::dxgi::common::*;

/// D2D1_BITMAP_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1BitmapProperties {
	pub pixel_format: D2D1PixelFormat,
	pub dpi_x: f32,
	pub dpi_y: f32,
}

/// D2D1_GRADIENT_STOP
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1GradientStop {
	pub position: f32,
	pub color: D2D1ColorF,
}

/// D2D1_BRUSH_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1BrushProperties {
	pub opacity: f32,
	pub transform: D2DMatrix3x2F,
}

/// D2D1_BITMAP_BRUSH_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1BitmapBrushProperties {
	pub extend_mode_x: D2D1ExtendMode,
	pub extend_mode_y: D2D1ExtendMode,
	pub interpolation_mode: D2D1BitmapInterpolationMode,
}

/// D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1LinearGradientBrushProperties {
	pub start_point: D2DPoint2F,
	pub end_point: D2DPoint2F,
}

/// D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1RadialGradientBrushProperties {
	pub center: D2DPoint2F,
	pub gradient_origin_offset: D2DPoint2F,
	pub radius_x: f32,
	pub radius_y: f32,
}

/// D2D1_TRIANGLE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1Triangle {
	pub point1: D2DPoint2F,
	pub point2: D2DPoint2F,
	pub point3: D2DPoint2F,
}

/// D2D1_ARC_SEGMENT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1ArcSegment {
	pub point: D2DPoint2F,
	pub size: D2DSizeF,
	pub rotation_angle: f32,
	pub sweep_direction: D2D1SweepDirection,
	pub arc_size: D2D1ArcSize,
}

/// D2D1_QUADRATIC_BEZIER_SEGMENT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1QuadraticBezierSegment {
	pub point1: D2DPoint2F,
	pub point2: D2DPoint2F,
}

/// D2D1_ELLIPSE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1Ellipse {
	pub point: D2DPoint2F,
	pub radius_x: f32,
	pub radius_y: f32,
}

/// D2D1_ROUNDED_RECT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1RoundedRect {
	pub rect: D2DRectF,
	pub radius_x: f32,
	pub radius_y: f32,
}

/// D2D1_STROKE_STYLE_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1StrokeStyleProperties {
	pub start_cap: D2D1CapStyle,
	pub end_cap: D2D1CapStyle,
	pub dash_cap: D2D1CapStyle,
	pub line_join: D2D1LineJoin,
	pub miter_limit: f32,
	pub dash_style: D2D1DashStyle,
	pub dash_offset: f32,
}

/// D2D1_LAYER_PARAMETERS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1LayerParameters<'a> {
	pub content_bounds: D2DRectF,
	pub geometric_mask: Param<'a, D2D1Geometry>,
	pub mask_antialias_mode: D2D1AntiAliasMode,
	pub mask_transform: D2DMatrix3x2F,
	pub opacity: f32,
	pub opacity_brush: Param<'a, D2D1Brush>,
	pub layer_options: D2D1LayerOptions,
}

/// D2D1_RENDER_TARGET_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1RenderTargetProperties {
	pub ty: D2D1RenderTargetType,
	pub pixel_format: D2D1PixelFormat,
	pub dpi_x: f32,
	pub dpi_y: f32,
	pub usage: D2D1RenderTargetUsage,
	pub min_level: D2D1FeatureLevel,
}

/// D2D1_HWND_RENDER_TARGET_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1HWndRenderTargetProperties {
	pub hwnd: HWnd,
	pub pixel_size: D2DSizeU,
	pub present_options: D2D1PresentOptions,
}

/// D2D1_DRAWING_STATE_DESCRIPTION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1DrawingStateDescription {
	pub antialias_mode: D2D1AntiAliasMode,
	pub text_antialias_mode: D2D1TextAntiAliasMode,
	pub tag1: u64,
	pub tag2: u64,
	pub transform: D2DMatrix3x2F,
}

/// D2D1_FACTORY_OPTIONS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1FactoryOptions {
	pub debug_level: D2D1DebugLevel,
}

/// D2D1_BITMAP_PROPERTIES1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1BitmapProperties1<'a> {
	pub base: D2D1BitmapProperties,
	pub bitmap_options: D2D1BitmapOptions,
	pub color_context: Param<'a, D2D1ColorContext>,
}

impl<'a> Deref for D2D1BitmapProperties1<'a> {
	type Target = D2D1BitmapProperties;
	fn deref(&self) -> &Self::Target { &self.base }
}

impl<'a> DerefMut for D2D1BitmapProperties1<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

/// D2D1_MAPPED_RECT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1MappedRect<'a> {
	pub pitch: u32,
	pub bits: &'a u8,
}

/// D2D1_RENDERING_CONTROLS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1RenderingControls {
	pub buffer_precision: D2D1BufferPrecision,
	pub tile_size: D2DSizeU,
}

/// D2D1_EFFECT_INPUT_DESCRIPTION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1EffectInputDescription<'a> {
	pub effect: Param<'a, D2D1Effect>,
	pub input_index: u32,
	pub input_rectangle: D2DRectF,
}

/// D2D1_POINT_DESCRIPTION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1PointDescription {
	pub point: D2DPoint2F,
	pub unit_tangent_vector: D2DPoint2F,
	pub end_segment: u32,
	pub end_figure: u32,
	pub length_to_end_segment: f32,
}

/// D2D1_IMAGE_BRUSH_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1ImageBrushProperties {
	pub source_rectangle: D2DRectF,
	pub extend_mode_x: D2D1ExtendMode,
	pub extend_mode_y: D2D1ExtendMode,
	pub interpolation_mode: D2D1InterpolationMode,
}

/// D2D1_BITMAP_BRUSH_PROPERTIES1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1BitmapBrushProperties1 {
	pub extend_mode_x: D2D1ExtendMode,
	pub extend_mode_y: D2D1ExtendMode,
	pub interpolation_mode: D2D1InterpolationMode,
}

/// D2D1_STROKE_STYLE_PROPERTIES1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1StrokeStyleProperties1 {
	pub base: D2D1StrokeStyleProperties,
	pub transform_type: D2D1StrokeTransformType,
}

impl Deref for D2D1StrokeStyleProperties1 {
	type Target = D2D1StrokeStyleProperties;
	fn deref(&self) -> &Self::Target { &self.base }
}

impl DerefMut for D2D1StrokeStyleProperties1 {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

/// D2D1_LAYER_PARAMETERS1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1LayerParameters1<'a> {
	pub content_bounds: D2DRectF,
	pub geometric_mask: Param<'a, D2D1Geometry>,
	pub mask_antialias_mode: D2D1AntiAliasMode,
	pub mask_transform: D2DMatrix3x2F,
	pub opacity: f32,
	pub opacity_brush: Param<'a, D2D1Brush>,
	pub layer_options: D2D1LayerOptions1,
}

/// D2D1_DRAWING_STATE_DESCRIPTION1
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1DrawingStateDescription1 {
	pub base: D2D1DrawingStateDescription,
	pub primitive_blend: D2D1PrimitiveBlend,
	pub unit_mode: D2D1UnitMode,
}

impl Deref for D2D1DrawingStateDescription1 {
	type Target = D2D1DrawingStateDescription;
	fn deref(&self) -> &Self::Target { &self.base }
}

impl DerefMut for D2D1DrawingStateDescription1 {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

/// D2D1_PRINT_CONTROL_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1PrintControlProperties {
	pub font_subset: D2D1PrintFontSubsetMode,
	pub raster_dpi: f32,
	pub color_space: D2D1ColorSpace,
}

/// D2D1_CREATION_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1CreationProperties {
	pub threading_mode: D2D1ThreadingMode,
	pub debug_level: D2D1DebugLevel,
	pub options: D2D1DeviceContextOptions,
}

/// Matrix4x3F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Matrix4x3F {
	pub _anonymous_base_d2d1_1helper_l45_c31: D2DMatrix4x3F,
}

/// Matrix4x4F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Matrix4x4F {
	pub _anonymous_base_d2d1_1helper_l97_c31: D2DMatrix4x4F,
}

/// Matrix5x4F
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Matrix5x4F {
	pub _anonymous_base_d2d1_1helper_l472_c31: D2DMatrix5x4F,
}

/// D2D1_PROPERTY_BINDING
#[repr(C)]
pub struct D2D1PropertyBinding<'a> {
	pub property_name: PWStr<'a>,
	pub set_function: PD2d1PropertySetFunction,
	pub get_function: PD2d1PropertyGetFunction,
}

/// D2D1_RESOURCE_TEXTURE_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1ResourceTextureProperties<'a> {
	pub extents: &'a u32,
	pub dimensions: u32,
	pub buffer_precision: D2D1BufferPrecision,
	pub channel_depth: D2D1ChannelDepth,
	pub filter: D2D1Filter,
	pub extend_modes: &'a D2D1ExtendMode,
}

/// D2D1_INPUT_ELEMENT_DESC
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1InputElementDesc<'a> {
	pub semantic_name: PStr<'a>,
	pub semantic_index: u32,
	pub format: DxgiFormat,
	pub input_slot: u32,
	pub aligned_byte_offset: u32,
}

/// D2D1_VERTEX_BUFFER_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1VertexBufferProperties<'a> {
	pub input_count: u32,
	pub usage: D2D1VertexUsage,
	pub data: &'a u8,
	pub byte_width: u32,
}

/// D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1CustomVertexBufferProperties<'a> {
	pub shader_buffer_with_input_signature: &'a u8,
	pub shader_buffer_size: u32,
	pub input_elements: &'a D2D1InputElementDesc<'a>,
	pub element_count: u32,
	pub stride: u32,
}

/// D2D1_VERTEX_RANGE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1VertexRange {
	pub start_vertex: u32,
	pub vertex_count: u32,
}

/// D2D1_BLEND_DESCRIPTION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1BlendDescription {
	pub source_blend: D2D1Blend,
	pub destination_blend: D2D1Blend,
	pub blend_operation: D2D1BlendOperation,
	pub source_blend_alpha: D2D1Blend,
	pub destination_blend_alpha: D2D1Blend,
	pub blend_operation_alpha: D2D1BlendOperation,
	pub blend_factor: [f32; 4],
}

/// D2D1_INPUT_DESCRIPTION
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1InputDescription {
	pub filter: D2D1Filter,
	pub level_of_detail_count: u32,
}

/// D2D1_FEATURE_DATA_DOUBLES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1FeatureDataDoubles {
	pub double_precision_float_shader_ops: Bool,
}

/// D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1FeatureDataD3D10XHardwareOptions {
	pub compute_shaders_plus_raw_and_structured_buffers_via_shader_4_x: Bool,
}

/// D2D1_SVG_LENGTH
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1SvgLength {
	pub value: f32,
	pub units: D2D1SvgLengthUnits,
}

/// D2D1_SVG_PRESERVE_ASPECT_RATIO
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1SvgPreserveAspectRatio {
	pub defer: Bool,
	pub align: D2D1SvgAspectAlign,
	pub meet_or_slice: D2D1SvgAspectScaling,
}

/// D2D1_SVG_VIEWBOX
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1SvgViewBox {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

/// D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1TransformedImageSourceProperties {
	pub orientation: D2D1Orientation,
	pub scale_x: f32,
	pub scale_y: f32,
	pub interpolation_mode: D2D1InterpolationMode,
	pub options: D2D1TransformedImageSourceOptions,
}

/// D2D1_INK_POINT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1InkPoint {
	pub x: f32,
	pub y: f32,
	pub radius: f32,
}

/// D2D1_INK_BEZIER_SEGMENT
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1InkBezierSegment {
	pub point1: D2D1InkPoint,
	pub point2: D2D1InkPoint,
	pub point3: D2D1InkPoint,
}

/// D2D1_INK_STYLE_PROPERTIES
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1InkStyleProperties {
	pub nib_shape: D2D1InkNibShape,
	pub nib_transform: D2DMatrix3x2F,
}

/// D2D1_GRADIENT_MESH_PATCH
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1GradientMeshPatch {
	pub point00: D2DPoint2F,
	pub point01: D2DPoint2F,
	pub point02: D2DPoint2F,
	pub point03: D2DPoint2F,
	pub point10: D2DPoint2F,
	pub point11: D2DPoint2F,
	pub point12: D2DPoint2F,
	pub point13: D2DPoint2F,
	pub point20: D2DPoint2F,
	pub point21: D2DPoint2F,
	pub point22: D2DPoint2F,
	pub point23: D2DPoint2F,
	pub point30: D2DPoint2F,
	pub point31: D2DPoint2F,
	pub point32: D2DPoint2F,
	pub point33: D2DPoint2F,
	pub color00: D2D1ColorF,
	pub color03: D2D1ColorF,
	pub color30: D2D1ColorF,
	pub color33: D2D1ColorF,
	pub top_edge_mode: D2D1PatchEdgeMode,
	pub left_edge_mode: D2D1PatchEdgeMode,
	pub bottom_edge_mode: D2D1PatchEdgeMode,
	pub right_edge_mode: D2D1PatchEdgeMode,
}

/// D2D1_SIMPLE_COLOR_PROFILE
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D2D1SimpleColorProfile {
	pub red_primary: D2DPoint2F,
	pub green_primary: D2DPoint2F,
	pub blue_primary: D2DPoint2F,
	pub white_point_xz: D2DPoint2F,
	pub gamma: D2D1Gamma1,
}

