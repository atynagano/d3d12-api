#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use crate::helpers::*;
use super::*;
use crate::core::win32::system::com::*;
use crate::core::win32::graphics::direct3d12::*;
use crate::core::win32::foundation::*;
use crate::core::win32::graphics::dxgi::common::*;
use crate::core::win32::graphics::direct3d::*;
use crate::core::win32::system::com::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12CommandQueueDesc {
	pub ty: D3D12CommandListType,
	pub priority: i32,
	pub flags: D3D12CommandQueueFlags,
	pub node_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12InputElementDesc<'a> {
	pub semantic_name: PStr<'a>,
	pub semantic_index: u32,
	pub format: DxgiFormat,
	pub input_slot: u32,
	pub aligned_byte_offset: u32,
	pub input_slot_class: D3D12InputClassification,
	pub instance_data_step_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SoDeclarationEntry<'a> {
	pub stream: u32,
	pub semantic_name: PStr<'a>,
	pub semantic_index: u32,
	pub start_component: u8,
	pub component_count: u8,
	pub output_slot: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Viewport {
	pub top_left_x: f32,
	pub top_left_y: f32,
	pub width: f32,
	pub height: f32,
	pub min_depth: f32,
	pub max_depth: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Box {
	pub left: u32,
	pub top: u32,
	pub front: u32,
	pub right: u32,
	pub bottom: u32,
	pub back: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DepthStencilOpDesc {
	pub stencil_fail_op: D3D12StencilOp,
	pub stencil_depth_fail_op: D3D12StencilOp,
	pub stencil_pass_op: D3D12StencilOp,
	pub stencil_func: D3D12ComparisonFunc,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DepthStencilDesc {
	pub depth_enable: Bool,
	pub depth_write_mask: D3D12DepthWriteMask,
	pub depth_func: D3D12ComparisonFunc,
	pub stencil_enable: Bool,
	pub stencil_read_mask: u8,
	pub stencil_write_mask: u8,
	pub front_face: D3D12DepthStencilOpDesc,
	pub back_face: D3D12DepthStencilOpDesc,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DepthStencilDesc1 {
	pub depth_enable: Bool,
	pub depth_write_mask: D3D12DepthWriteMask,
	pub depth_func: D3D12ComparisonFunc,
	pub stencil_enable: Bool,
	pub stencil_read_mask: u8,
	pub stencil_write_mask: u8,
	pub front_face: D3D12DepthStencilOpDesc,
	pub back_face: D3D12DepthStencilOpDesc,
	pub depth_bounds_test_enable: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderTargetBlendDesc {
	pub blend_enable: Bool,
	pub logic_op_enable: Bool,
	pub src_blend: D3D12Blend,
	pub dest_blend: D3D12Blend,
	pub blend_op: D3D12BlendOp,
	pub src_blend_alpha: D3D12Blend,
	pub dest_blend_alpha: D3D12Blend,
	pub blend_op_alpha: D3D12BlendOp,
	pub logic_op: D3D12LogicOp,
	pub render_target_write_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12BlendDesc {
	pub alpha_to_coverage_enable: Bool,
	pub independent_blend_enable: Bool,
	pub render_target: [D3D12RenderTargetBlendDesc; 8],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RasterizerDesc {
	pub fill_mode: D3D12FillMode,
	pub cull_mode: D3D12CullMode,
	pub front_counter_clockwise: Bool,
	pub depth_bias: i32,
	pub depth_bias_clamp: f32,
	pub slope_scaled_depth_bias: f32,
	pub depth_clip_enable: Bool,
	pub multisample_enable: Bool,
	pub antialiased_line_enable: Bool,
	pub forced_sample_count: u32,
	pub conservative_raster: D3D12ConservativeRasterizationMode,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ShaderByteCode<'a> {
	pub shader_bytecode: &'a c_void,
	pub bytecode_length: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12StreamOutputDesc<'a> {
	pub so_declaration: &'a D3D12SoDeclarationEntry<'a>,
	pub num_entries: u32,
	pub buffer_strides: &'a u32,
	pub num_strides: u32,
	pub rasterized_stream: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12InputLayoutDesc<'a> {
	pub input_element_descs: &'a D3D12InputElementDesc<'a>,
	pub num_elements: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12CachedPipelineState<'a> {
	pub cached_blob: &'a c_void,
	pub cached_blob_size_in_bytes: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12GraphicsPipelineStateDesc<'a> {
	pub root_signature: Param<'a, D3D12RootSignature>,
	pub vs: D3D12ShaderByteCode<'a>,
	pub ps: D3D12ShaderByteCode<'a>,
	pub ds: D3D12ShaderByteCode<'a>,
	pub hs: D3D12ShaderByteCode<'a>,
	pub gs: D3D12ShaderByteCode<'a>,
	pub stream_output: D3D12StreamOutputDesc<'a>,
	pub blend_state: D3D12BlendDesc,
	pub sample_mask: u32,
	pub rasterizer_state: D3D12RasterizerDesc,
	pub depth_stencil_state: D3D12DepthStencilDesc,
	pub input_layout: D3D12InputLayoutDesc<'a>,
	pub ib_strip_cut_value: D3D12IndexBufferStripCutValue,
	pub primitive_topology_type: D3D12PrimitiveTopologyType,
	pub num_render_targets: u32,
	pub rtv_formats: [DxgiFormat; 8],
	pub dsv_format: DxgiFormat,
	pub sample_desc: DxgiSampleDesc,
	pub node_mask: u32,
	pub cached_pso: D3D12CachedPipelineState<'a>,
	pub flags: D3D12PipelineStateFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ComputePipelineStateDesc<'a> {
	pub root_signature: Param<'a, D3D12RootSignature>,
	pub cs: D3D12ShaderByteCode<'a>,
	pub node_mask: u32,
	pub cached_pso: D3D12CachedPipelineState<'a>,
	pub flags: D3D12PipelineStateFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RtFormatArray {
	pub rt_formats: [DxgiFormat; 8],
	pub num_render_targets: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12PipelineStateStreamDesc<'a> {
	pub size_in_bytes: usize,
	pub pipeline_state_subobject_stream: &'a c_void,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options {
	pub double_precision_float_shader_ops: Bool,
	pub output_merger_logic_op: Bool,
	pub min_precision_support: D3D12ShaderMinPrecisionSupport,
	pub tiled_resources_tier: D3D12TiledResourcesTier,
	pub resource_binding_tier: D3D12ResourceBindingTier,
	pub ps_specified_stencil_ref_supported: Bool,
	pub typed_uav_load_additional_formats: Bool,
	pub ro_vs_supported: Bool,
	pub conservative_rasterization_tier: D3D12ConservativeRasterizationTier,
	pub max_gpu_virtual_address_bits_per_resource: u32,
	pub standard_swizzle64kb_supported: Bool,
	pub cross_node_sharing_tier: D3D12CrossNodeSharingTier,
	pub cross_adapter_row_major_texture_supported: Bool,
	pub vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation: Bool,
	pub resource_heap_tier: D3D12ResourceHeapTier,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options1 {
	pub wave_ops: Bool,
	pub wave_lane_count_min: u32,
	pub wave_lane_count_max: u32,
	pub total_lane_count: u32,
	pub expanded_compute_resource_states: Bool,
	pub int64shader_ops: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options2 {
	pub depth_bounds_test_supported: Bool,
	pub programmable_sample_positions_tier: D3D12ProgrammableSamplePositionsTier,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataRootSignature {
	pub highest_version: D3DRootSignatureVersion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataArchitecture {
	pub node_index: u32,
	pub tile_based_renderer: Bool,
	pub uma: Bool,
	pub cache_coherent_uma: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataArchitecture1 {
	pub node_index: u32,
	pub tile_based_renderer: Bool,
	pub uma: Bool,
	pub cache_coherent_uma: Bool,
	pub isolated_mmu: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataFeatureLevels<'a> {
	pub num_feature_levels: u32,
	pub feature_levels_requested: &'a D3DFeatureLevel,
	pub max_supported_feature_level: D3DFeatureLevel,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataShaderModel {
	pub highest_shader_model: D3DShaderModel,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataFormatSupport {
	pub format: DxgiFormat,
	pub support1: D3D12FormatSupport1,
	pub support2: D3D12FormatSupport2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataMultiSampleQualityLevels {
	pub format: DxgiFormat,
	pub sample_count: u32,
	pub flags: D3D12MultiSampleQualityLevelFlags,
	pub num_quality_levels: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataFormatInfo {
	pub format: DxgiFormat,
	pub plane_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataGpuVirtualAddressSupport {
	pub max_gpu_virtual_address_bits_per_resource: u32,
	pub max_gpu_virtual_address_bits_per_process: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataShaderCache {
	pub support_flags: D3D12ShaderCacheSupportFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataCommandQueuePriority {
	pub command_list_type: D3D12CommandListType,
	pub priority: u32,
	pub priority_for_type_is_supported: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options3 {
	pub copy_queue_timestamp_queries_supported: Bool,
	pub casting_fully_typed_format_supported: Bool,
	pub write_buffer_immediate_support_flags: D3D12CommandListSupportFlags,
	pub view_instancing_tier: D3D12ViewInstancingTier,
	pub barycentrics_supported: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataExistingHeaps {
	pub supported: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataDisplayable {
	pub displayable_texture: Bool,
	pub shared_resource_compatibility_tier: D3D12SharedResourceCompatibilityTier,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options4 {
	pub msaa64kb_aligned_texture_supported: Bool,
	pub shared_resource_compatibility_tier: D3D12SharedResourceCompatibilityTier,
	pub native16bit_shader_ops_supported: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataSerialization {
	pub node_index: u32,
	pub heap_serialization_tier: D3D12HeapSerializationTier,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataCrossNode {
	pub sharing_tier: D3D12CrossNodeSharingTier,
	pub atomic_shader_instructions: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options5 {
	pub srv_only_tiled_resource_tier3: Bool,
	pub render_passes_tier: D3D12RenderPassTier,
	pub raytracing_tier: D3D12RaytracingTier,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options6 {
	pub additional_shading_rates_supported: Bool,
	pub per_primitive_shading_rate_supported_with_viewport_indexing: Bool,
	pub variable_shading_rate_tier: D3D12VariableShadingRateTier,
	pub shading_rate_image_tile_size: u32,
	pub background_processing_supported: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options7 {
	pub mesh_shader_tier: D3D12MeshShaderTier,
	pub sampler_feedback_tier: D3D12SamplerFeedbackTier,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataQueryMetaCommand<'a> {
	pub command_id: GUID,
	pub node_mask: u32,
	pub query_input_data: &'a c_void,
	pub query_input_data_size_in_bytes: usize,
	pub query_output_data: &'a c_void,
	pub query_output_data_size_in_bytes: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options8 {
	pub unaligned_block_textures_supported: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options9 {
	pub mesh_shader_pipeline_stats_supported: Bool,
	pub mesh_shader_supports_full_range_render_target_array_index: Bool,
	pub atomic_int64on_typed_resource_supported: Bool,
	pub atomic_int64on_group_shared_supported: Bool,
	pub derivatives_in_mesh_and_amplification_shaders_supported: Bool,
	pub wave_mma_tier: D3D12WaveMMaTier,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options10 {
	pub variable_rate_shading_sum_combiner_supported: Bool,
	pub mesh_shader_per_primitive_shading_rate_supported: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataD3D12Options11 {
	pub atomic_int64on_descriptor_heap_resource_supported: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ResourceAllocationInfo {
	pub size_in_bytes: u64,
	pub alignment: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ResourceAllocationInfo1 {
	pub offset: u64,
	pub alignment: u64,
	pub size_in_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12HeapProperties {
	pub ty: D3D12HeapType,
	pub cpu_page_property: D3D12CpuPageProperty,
	pub memory_pool_preference: D3D12MemoryPool,
	pub creation_node_mask: u32,
	pub visible_node_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12HeapDesc {
	pub size_in_bytes: u64,
	pub properties: D3D12HeapProperties,
	pub alignment: u64,
	pub flags: D3D12HeapFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12MipRegion {
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ResourceDesc {
	pub dimension: D3D12ResourceDimension,
	pub alignment: u64,
	pub width: u64,
	pub height: u32,
	pub depth_or_array_size: u16,
	pub mip_levels: u16,
	pub format: DxgiFormat,
	pub sample_desc: DxgiSampleDesc,
	pub layout: D3D12TextureLayout,
	pub flags: D3D12ResourceFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ResourceDesc1 {
	pub dimension: D3D12ResourceDimension,
	pub alignment: u64,
	pub width: u64,
	pub height: u32,
	pub depth_or_array_size: u16,
	pub mip_levels: u16,
	pub format: DxgiFormat,
	pub sample_desc: DxgiSampleDesc,
	pub layout: D3D12TextureLayout,
	pub flags: D3D12ResourceFlags,
	pub sampler_feedback_mip_region: D3D12MipRegion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DepthStencilValue {
	pub depth: f32,
	pub stencil: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12ClearValueAnonymousUnion {
	pub color: [f32; 4],
	pub depth_stencil: D3D12DepthStencilValue,
}

impl std::fmt::Debug for D3D12ClearValueAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ClearValue {
	pub format: DxgiFormat,
	pub anonymous: D3D12ClearValueAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Range {
	pub begin: usize,
	pub end: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RangeUInt64 {
	pub begin: u64,
	pub end: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SubresourceRangeUInt64 {
	pub subresource: u32,
	pub range: D3D12RangeUInt64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SubresourceInfo {
	pub offset: u64,
	pub row_pitch: u32,
	pub depth_pitch: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12TiledResourceCoordinate {
	pub x: u32,
	pub y: u32,
	pub z: u32,
	pub subresource: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12TileRegionSize {
	pub num_tiles: u32,
	pub use_box: Bool,
	pub width: u32,
	pub height: u16,
	pub depth: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SubresourceTiling {
	pub width_in_tiles: u32,
	pub height_in_tiles: u16,
	pub depth_in_tiles: u16,
	pub start_tile_index_in_overall_resource: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12TileShape {
	pub width_in_texels: u32,
	pub height_in_texels: u32,
	pub depth_in_texels: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12PackedMipInfo {
	pub num_standard_mips: u8,
	pub num_packed_mips: u8,
	pub num_tiles_for_packed_mips: u32,
	pub start_tile_index_in_overall_resource: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ResourceTransitionBarrier<'a> {
	pub resource: Param<'a, D3D12Resource>,
	pub subresource: u32,
	pub state_before: D3D12ResourceStates,
	pub state_after: D3D12ResourceStates,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ResourceAliasingBarrier<'a> {
	pub resource_before: Param<'a, D3D12Resource>,
	pub resource_after: Param<'a, D3D12Resource>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ResourceUavBarrier<'a> {
	pub resource: Param<'a, D3D12Resource>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12ResourceBarrierAnonymousUnion<'a> {
	pub transition: D3D12ResourceTransitionBarrier<'a>,
	pub aliasing: D3D12ResourceAliasingBarrier<'a>,
	pub uav: D3D12ResourceUavBarrier<'a>,
}

impl std::fmt::Debug for D3D12ResourceBarrierAnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ResourceBarrier<'a> {
	pub ty: D3D12ResourceBarrierType,
	pub flags: D3D12ResourceBarrierFlags,
	pub anonymous: D3D12ResourceBarrierAnonymousUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SubresourceFootprint {
	pub format: DxgiFormat,
	pub width: u32,
	pub height: u32,
	pub depth: u32,
	pub row_pitch: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12PlacedSubresourceFootprint {
	pub offset: u64,
	pub footprint: D3D12SubresourceFootprint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12TextureCopyLocationAnonymousUnion {
	pub placed_footprint: D3D12PlacedSubresourceFootprint,
	pub subresource_index: u32,
}

impl std::fmt::Debug for D3D12TextureCopyLocationAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12TextureCopyLocation<'a> {
	pub resource: Param<'a, D3D12Resource>,
	pub ty: D3D12TextureCopyType,
	pub anonymous: D3D12TextureCopyLocationAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SamplePosition {
	pub x: i8,
	pub y: i8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ViewInstanceLocation {
	pub viewport_array_index: u32,
	pub render_target_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ViewInstancingDesc<'a> {
	pub view_instance_count: u32,
	pub view_instance_locations: &'a D3D12ViewInstanceLocation,
	pub flags: D3D12ViewInstancingFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12BufferSrv {
	pub first_element: u64,
	pub num_elements: u32,
	pub structure_byte_stride: u32,
	pub flags: D3D12BufferSrvFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex1DSrv {
	pub most_detailed_mip: u32,
	pub mip_levels: u32,
	pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex1DArraySrv {
	pub most_detailed_mip: u32,
	pub mip_levels: u32,
	pub first_array_slice: u32,
	pub array_size: u32,
	pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DSrv {
	pub most_detailed_mip: u32,
	pub mip_levels: u32,
	pub plane_slice: u32,
	pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DArraySrv {
	pub most_detailed_mip: u32,
	pub mip_levels: u32,
	pub first_array_slice: u32,
	pub array_size: u32,
	pub plane_slice: u32,
	pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex3DSrv {
	pub most_detailed_mip: u32,
	pub mip_levels: u32,
	pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12TexCubeSrv {
	pub most_detailed_mip: u32,
	pub mip_levels: u32,
	pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12TexCubeArraySrv {
	pub most_detailed_mip: u32,
	pub mip_levels: u32,
	pub first2d_array_face: u32,
	pub num_cubes: u32,
	pub resource_min_lod_clamp: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DMsSrv {
	pub unused_field_nothing_to_define: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DMsArraySrv {
	pub first_array_slice: u32,
	pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingAccelerationStructureSrv {
	pub location: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12ShaderResourceViewDescAnonymousUnion {
	pub buffer: D3D12BufferSrv,
	pub texture1d: D3D12Tex1DSrv,
	pub texture1d_array: D3D12Tex1DArraySrv,
	pub texture2d: D3D12Tex2DSrv,
	pub texture2d_array: D3D12Tex2DArraySrv,
	pub texture2dms: D3D12Tex2DMsSrv,
	pub texture2dms_array: D3D12Tex2DMsArraySrv,
	pub texture3d: D3D12Tex3DSrv,
	pub texture_cube: D3D12TexCubeSrv,
	pub texture_cube_array: D3D12TexCubeArraySrv,
	pub raytracing_acceleration_structure: D3D12RaytracingAccelerationStructureSrv,
}

impl std::fmt::Debug for D3D12ShaderResourceViewDescAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ShaderResourceViewDesc {
	pub format: DxgiFormat,
	pub view_dimension: D3D12SrvDimension,
	pub shader4component_mapping: u32,
	pub anonymous: D3D12ShaderResourceViewDescAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ConstantBufferViewDesc {
	pub buffer_location: u64,
	pub size_in_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SamplerDesc {
	pub filter: D3D12Filter,
	pub address_u: D3D12TextureAddressMode,
	pub address_v: D3D12TextureAddressMode,
	pub address_w: D3D12TextureAddressMode,
	pub mip_lod_bias: f32,
	pub max_anisotropy: u32,
	pub comparison_func: D3D12ComparisonFunc,
	pub border_color: [f32; 4],
	pub min_lod: f32,
	pub max_lod: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12BufferUav {
	pub first_element: u64,
	pub num_elements: u32,
	pub structure_byte_stride: u32,
	pub counter_offset_in_bytes: u64,
	pub flags: D3D12BufferUavFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex1DUav {
	pub mip_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex1DArrayUav {
	pub mip_slice: u32,
	pub first_array_slice: u32,
	pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DUav {
	pub mip_slice: u32,
	pub plane_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DArrayUav {
	pub mip_slice: u32,
	pub first_array_slice: u32,
	pub array_size: u32,
	pub plane_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex3DUav {
	pub mip_slice: u32,
	pub first_w_slice: u32,
	pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12UnorderedAccessViewDescAnonymousUnion {
	pub buffer: D3D12BufferUav,
	pub texture1d: D3D12Tex1DUav,
	pub texture1d_array: D3D12Tex1DArrayUav,
	pub texture2d: D3D12Tex2DUav,
	pub texture2d_array: D3D12Tex2DArrayUav,
	pub texture3d: D3D12Tex3DUav,
}

impl std::fmt::Debug for D3D12UnorderedAccessViewDescAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12UnorderedAccessViewDesc {
	pub format: DxgiFormat,
	pub view_dimension: D3D12UavDimension,
	pub anonymous: D3D12UnorderedAccessViewDescAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12BufferRtv {
	pub first_element: u64,
	pub num_elements: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex1DRtv {
	pub mip_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex1DArrayRtv {
	pub mip_slice: u32,
	pub first_array_slice: u32,
	pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DRtv {
	pub mip_slice: u32,
	pub plane_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DMsRtv {
	pub unused_field_nothing_to_define: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DArrayRtv {
	pub mip_slice: u32,
	pub first_array_slice: u32,
	pub array_size: u32,
	pub plane_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DMsArrayRtv {
	pub first_array_slice: u32,
	pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex3DRtv {
	pub mip_slice: u32,
	pub first_w_slice: u32,
	pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12RenderTargetViewDescAnonymousUnion {
	pub buffer: D3D12BufferRtv,
	pub texture1d: D3D12Tex1DRtv,
	pub texture1d_array: D3D12Tex1DArrayRtv,
	pub texture2d: D3D12Tex2DRtv,
	pub texture2d_array: D3D12Tex2DArrayRtv,
	pub texture2dms: D3D12Tex2DMsRtv,
	pub texture2dms_array: D3D12Tex2DMsArrayRtv,
	pub texture3d: D3D12Tex3DRtv,
}

impl std::fmt::Debug for D3D12RenderTargetViewDescAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderTargetViewDesc {
	pub format: DxgiFormat,
	pub view_dimension: D3D12RtvDimension,
	pub anonymous: D3D12RenderTargetViewDescAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex1DDsv {
	pub mip_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex1DArrayDsv {
	pub mip_slice: u32,
	pub first_array_slice: u32,
	pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DDsv {
	pub mip_slice: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DArrayDsv {
	pub mip_slice: u32,
	pub first_array_slice: u32,
	pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DMsDsv {
	pub unused_field_nothing_to_define: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Tex2DMsArrayDsv {
	pub first_array_slice: u32,
	pub array_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12DepthStencilViewDescAnonymousUnion {
	pub texture1d: D3D12Tex1DDsv,
	pub texture1d_array: D3D12Tex1DArrayDsv,
	pub texture2d: D3D12Tex2DDsv,
	pub texture2d_array: D3D12Tex2DArrayDsv,
	pub texture2dms: D3D12Tex2DMsDsv,
	pub texture2dms_array: D3D12Tex2DMsArrayDsv,
}

impl std::fmt::Debug for D3D12DepthStencilViewDescAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DepthStencilViewDesc {
	pub format: DxgiFormat,
	pub view_dimension: D3D12DsvDimension,
	pub flags: D3D12DsvFlags,
	pub anonymous: D3D12DepthStencilViewDescAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DescriptorHeapDesc {
	pub ty: D3D12DescriptorHeapType,
	pub num_descriptors: u32,
	pub flags: D3D12DescriptorHeapFlags,
	pub node_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DescriptorRange {
	pub range_type: D3D12DescriptorRangeType,
	pub num_descriptors: u32,
	pub base_shader_register: u32,
	pub register_space: u32,
	pub offset_in_descriptors_from_table_start: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootDescriptorTable<'a> {
	pub num_descriptor_ranges: u32,
	pub descriptor_ranges: Option<&'a D3D12DescriptorRange>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootConstants {
	pub shader_register: u32,
	pub register_space: u32,
	pub num_32bit_values: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootDescriptor {
	pub shader_register: u32,
	pub register_space: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12RootParameterAnonymousUnion<'a> {
	pub descriptor_table: D3D12RootDescriptorTable<'a>,
	pub constants: D3D12RootConstants,
	pub descriptor: D3D12RootDescriptor,
}

impl std::fmt::Debug for D3D12RootParameterAnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootParameter<'a> {
	pub parameter_type: D3D12RootParameterType,
	pub anonymous: D3D12RootParameterAnonymousUnion<'a>,
	pub shader_visibility: D3D12ShaderVisibility,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12StaticSamplerDesc {
	pub filter: D3D12Filter,
	pub address_u: D3D12TextureAddressMode,
	pub address_v: D3D12TextureAddressMode,
	pub address_w: D3D12TextureAddressMode,
	pub mip_lod_bias: f32,
	pub max_anisotropy: u32,
	pub comparison_func: D3D12ComparisonFunc,
	pub border_color: D3D12StaticBorderColor,
	pub min_lod: f32,
	pub max_lod: f32,
	pub shader_register: u32,
	pub register_space: u32,
	pub shader_visibility: D3D12ShaderVisibility,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootSignatureDesc<'a> {
	pub num_parameters: u32,
	pub parameters: Option<&'a D3D12RootParameter<'a>>,
	pub num_static_samplers: u32,
	pub static_samplers: Option<&'a D3D12StaticSamplerDesc>,
	pub flags: D3D12RootSignatureFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DescriptorRange1 {
	pub range_type: D3D12DescriptorRangeType,
	pub num_descriptors: u32,
	pub base_shader_register: u32,
	pub register_space: u32,
	pub flags: D3D12DescriptorRangeFlags,
	pub offset_in_descriptors_from_table_start: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootDescriptorTable1<'a> {
	pub num_descriptor_ranges: u32,
	pub descriptor_ranges: Option<&'a D3D12DescriptorRange1>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootDescriptor1 {
	pub shader_register: u32,
	pub register_space: u32,
	pub flags: D3D12RootDescriptorFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12RootParameter1AnonymousUnion<'a> {
	pub descriptor_table: D3D12RootDescriptorTable1<'a>,
	pub constants: D3D12RootConstants,
	pub descriptor: D3D12RootDescriptor1,
}

impl std::fmt::Debug for D3D12RootParameter1AnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootParameter1<'a> {
	pub parameter_type: D3D12RootParameterType,
	pub anonymous: D3D12RootParameter1AnonymousUnion<'a>,
	pub shader_visibility: D3D12ShaderVisibility,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RootSignatureDesc1<'a> {
	pub num_parameters: u32,
	pub parameters: Option<&'a D3D12RootParameter1<'a>>,
	pub num_static_samplers: u32,
	pub static_samplers: Option<&'a D3D12StaticSamplerDesc>,
	pub flags: D3D12RootSignatureFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12VersionedRootSignatureDescAnonymousUnion<'a> {
	pub desc_1_0: D3D12RootSignatureDesc<'a>,
	pub desc_1_1: D3D12RootSignatureDesc1<'a>,
}

impl std::fmt::Debug for D3D12VersionedRootSignatureDescAnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12VersionedRootSignatureDesc<'a> {
	pub version: D3DRootSignatureVersion,
	pub anonymous: D3D12VersionedRootSignatureDescAnonymousUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12CpuDescriptorHandle {
	pub ptr: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12GpuDescriptorHandle {
	pub ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DiscardRegion<'a> {
	pub num_rects: u32,
	pub rects: Option<&'a Rect>,
	pub first_subresource: u32,
	pub num_subresources: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12QueryHeapDesc {
	pub ty: D3D12QueryHeapType,
	pub count: u32,
	pub node_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12QueryDataPipelineStatistics {
	pub ia_vertices: u64,
	pub ia_primitives: u64,
	pub vs_invocations: u64,
	pub gs_invocations: u64,
	pub gs_primitives: u64,
	pub invocations: u64,
	pub primitives: u64,
	pub ps_invocations: u64,
	pub hs_invocations: u64,
	pub ds_invocations: u64,
	pub cs_invocations: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12QueryDataPipelineStatistics1 {
	pub ia_vertices: u64,
	pub ia_primitives: u64,
	pub vs_invocations: u64,
	pub gs_invocations: u64,
	pub gs_primitives: u64,
	pub invocations: u64,
	pub primitives: u64,
	pub ps_invocations: u64,
	pub hs_invocations: u64,
	pub ds_invocations: u64,
	pub cs_invocations: u64,
	pub as_invocations: u64,
	pub ms_invocations: u64,
	pub ms_primitives: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12QueryDataSoStatistics {
	pub num_primitives_written: u64,
	pub primitives_storage_needed: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12StreamOutputBufferView {
	pub buffer_location: u64,
	pub size_in_bytes: u64,
	pub buffer_filled_size_location: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DrawArguments {
	pub vertex_count_per_instance: u32,
	pub instance_count: u32,
	pub start_vertex_location: u32,
	pub start_instance_location: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DrawIndexedArguments {
	pub index_count_per_instance: u32,
	pub instance_count: u32,
	pub start_index_location: u32,
	pub base_vertex_location: i32,
	pub start_instance_location: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DispatchArguments {
	pub thread_group_count_x: u32,
	pub thread_group_count_y: u32,
	pub thread_group_count_z: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12VertexBufferView {
	pub buffer_location: u64,
	pub size_in_bytes: u32,
	pub stride_in_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12IndexBufferView {
	pub buffer_location: u64,
	pub size_in_bytes: u32,
	pub format: DxgiFormat,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexBufferStruct {
	pub slot: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ConstantStruct {
	pub root_parameter_index: u32,
	pub dest_offset_in32bit_values: u32,
	pub num32bit_values_to_set: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ConstantBufferViewStruct {
	pub root_parameter_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderResourceViewStruct {
	pub root_parameter_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UnorderedAccessViewStruct {
	pub root_parameter_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12IndirectArgumentDescAnonymousUnion {
	pub vertex_buffer: VertexBufferStruct,
	pub constant: ConstantStruct,
	pub constant_buffer_view: ConstantBufferViewStruct,
	pub shader_resource_view: ShaderResourceViewStruct,
	pub unordered_access_view: UnorderedAccessViewStruct,
}

impl std::fmt::Debug for D3D12IndirectArgumentDescAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12IndirectArgumentDesc {
	pub ty: D3D12IndirectArgumentType,
	pub anonymous: D3D12IndirectArgumentDescAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12CommandSignatureDesc<'a> {
	pub byte_stride: u32,
	pub num_argument_descs: u32,
	pub argument_descs: Option<&'a D3D12IndirectArgumentDesc>,
	pub node_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12WriteBufferImmediateParameter {
	pub dest: u64,
	pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataProtectedResourceSessionSupport {
	pub node_index: u32,
	pub support: D3D12ProtectedResourceSessionSupportFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ProtectedResourceSessionDesc {
	pub node_mask: u32,
	pub flags: D3D12ProtectedResourceSessionFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12MetaCommandParameterDesc<'a> {
	pub name: PWStr<'a>,
	pub ty: D3D12MetaCommandParameterType,
	pub flags: D3D12MetaCommandParameterFlags,
	pub required_resource_state: D3D12ResourceStates,
	pub structure_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12MetaCommandDesc<'a> {
	pub id: GUID,
	pub name: PWStr<'a>,
	pub initialization_dirty_state: D3D12GraphicsStates,
	pub execution_dirty_state: D3D12GraphicsStates,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12StateSubobject<'a> {
	pub ty: D3D12StateSubobjectType,
	pub desc: &'a c_void,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12StateObjectConfig {
	pub flags: D3D12StateObjectFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12GlobalRootSignature<'a> {
	pub global_root_signature: Param<'a, D3D12RootSignature>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12LocalRootSignature<'a> {
	pub local_root_signature: Param<'a, D3D12RootSignature>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12NodeMask {
	pub node_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DxilLibraryDesc<'a> {
	pub dxil_library: D3D12ShaderByteCode<'a>,
	pub num_exports: u32,
	pub exports: Option<&'a D3D12ExportDesc<'a>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ExistingCollectionDesc<'a> {
	pub existing_collection: Param<'a, D3D12StateObject>,
	pub num_exports: u32,
	pub exports: Option<&'a D3D12ExportDesc<'a>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SubobjectToExportsAssociation<'a> {
	pub subobject_to_associate: &'a D3D12StateSubobject<'a>,
	pub num_exports: u32,
	pub exports: Option<&'a PWStr<'a>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DxilSubobjectToExportsAssociation<'a> {
	pub subobject_to_associate: PWStr<'a>,
	pub num_exports: u32,
	pub exports: Option<&'a PWStr<'a>>,
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingShaderConfig {
	pub max_payload_size_in_bytes: u32,
	pub max_attribute_size_in_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingPipelineConfig {
	pub max_trace_recursion_depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingPipelineConfig1 {
	pub max_trace_recursion_depth: u32,
	pub flags: D3D12RaytracingPipelineFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12StateObjectDesc<'a> {
	pub ty: D3D12StateObjectType,
	pub num_subobjects: u32,
	pub subobjects: Option<&'a D3D12StateSubobject<'a>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12GpuVirtualAddressAndStride {
	pub start_address: u64,
	pub stride_in_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12GpuVirtualAddressRange {
	pub start_address: u64,
	pub size_in_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12GpuVirtualAddressRangeAndStride {
	pub start_address: u64,
	pub size_in_bytes: u64,
	pub stride_in_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingGeometryTrianglesDesc {
	pub transform3x4: u64,
	pub index_format: DxgiFormat,
	pub vertex_format: DxgiFormat,
	pub index_count: u32,
	pub vertex_count: u32,
	pub index_buffer: u64,
	pub vertex_buffer: D3D12GpuVirtualAddressAndStride,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingAabb {
	pub min_x: f32,
	pub min_y: f32,
	pub min_z: f32,
	pub max_x: f32,
	pub max_y: f32,
	pub max_z: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingGeometryAabbsDesc {
	pub aabb_count: u64,
	pub aab_bs: D3D12GpuVirtualAddressAndStride,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingAccelerationStructurePostBuildInfoDesc {
	pub dest_buffer: u64,
	pub info_type: D3D12RaytracingAccelerationStructurePostBuildInfoType,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingAccelerationStructurePostBuildInfoCompactedSizeDesc {
	pub compacted_size_in_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingAccelerationStructurePostBuildInfoToolsVisualizationDesc {
	pub decoded_size_in_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12BuildRaytracingAccelerationStructureToolsVisualizationHeader {
	pub ty: D3D12RaytracingAccelerationStructureType,
	pub num_descs: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingAccelerationStructurePostBuildInfoSerializationDesc {
	pub serialized_size_in_bytes: u64,
	pub num_bottom_level_acceleration_structure_pointers: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SerializedDataDriverMatchingIdentifier {
	pub driver_opaque_guid: GUID,
	pub driver_opaque_versioning_data: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SerializedRaytracingAccelerationStructureHeader {
	pub driver_matching_identifier: D3D12SerializedDataDriverMatchingIdentifier,
	pub serialized_size_in_bytes_including_header: u64,
	pub deserialized_size_in_bytes: u64,
	pub num_bottom_level_acceleration_structure_pointers_after_header: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingAccelerationStructurePostBuildInfoCurrentSizeDesc {
	pub current_size_in_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12RaytracingGeometryDescAnonymousUnion {
	pub triangles: D3D12RaytracingGeometryTrianglesDesc,
	pub aab_bs: D3D12RaytracingGeometryAabbsDesc,
}

impl std::fmt::Debug for D3D12RaytracingGeometryDescAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingGeometryDesc {
	pub ty: D3D12RaytracingGeometryType,
	pub flags: D3D12RaytracingGeometryFlags,
	pub anonymous: D3D12RaytracingGeometryDescAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12BuildRaytracingAccelerationStructureInputs<'a> {
	pub ty: D3D12RaytracingAccelerationStructureType,
	pub flags: D3D12RaytracingAccelerationStructureBuildFlags,
	pub num_descs: u32,
	pub descs_layout: D3D12ElementsLayout,
	pub anonymous: D3D12BuildRaytracingAccelerationStructureInputsAnonymousUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12BuildRaytracingAccelerationStructureDesc<'a> {
	pub dest_acceleration_structure_data: u64,
	pub inputs: D3D12BuildRaytracingAccelerationStructureInputs<'a>,
	pub source_acceleration_structure_data: u64,
	pub scratch_acceleration_structure_data: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingAccelerationStructurePrebuildInfo {
	pub result_data_max_size_in_bytes: u64,
	pub scratch_data_size_in_bytes: u64,
	pub update_scratch_data_size_in_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DredAllocationNode<'a> {
	pub object_name_a: &'a u8,
	pub object_name_w: PWStr<'a>,
	pub allocation_type: D3D12DredAllocationType,
	pub next: &'a D3D12DredAllocationNode<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DredAllocationNode1<'a> {
	pub object_name_a: &'a u8,
	pub object_name_w: PWStr<'a>,
	pub allocation_type: D3D12DredAllocationType,
	pub next: &'a D3D12DredAllocationNode1<'a>,
	pub object: Param<'a, Unknown>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DredPageFaultOutput<'a> {
	pub page_fault_va: u64,
	pub head_existing_allocation_node: &'a D3D12DredAllocationNode<'a>,
	pub head_recent_freed_allocation_node: &'a D3D12DredAllocationNode<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DredPageFaultOutput1<'a> {
	pub page_fault_va: u64,
	pub head_existing_allocation_node: &'a D3D12DredAllocationNode1<'a>,
	pub head_recent_freed_allocation_node: &'a D3D12DredAllocationNode1<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DredPageFaultOutput2<'a> {
	pub page_fault_va: u64,
	pub head_existing_allocation_node: &'a D3D12DredAllocationNode1<'a>,
	pub head_recent_freed_allocation_node: &'a D3D12DredAllocationNode1<'a>,
	pub page_fault_flags: D3D12DredPageFaultFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataProtectedResourceSessionTypeCount {
	pub node_index: u32,
	pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FeatureDataProtectedResourceSessionTypes<'a> {
	pub node_index: u32,
	pub count: u32,
	pub types: &'a GUID,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ProtectedResourceSessionDesc1 {
	pub node_mask: u32,
	pub flags: D3D12ProtectedResourceSessionFlags,
	pub protection_type: GUID,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderPassBeginningAccessClearParameters {
	pub clear_value: D3D12ClearValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12RenderPassBeginningAccessAnonymousUnion {
	pub clear: D3D12RenderPassBeginningAccessClearParameters,
}

impl std::fmt::Debug for D3D12RenderPassBeginningAccessAnonymousUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderPassBeginningAccess {
	pub ty: D3D12RenderPassBeginningAccessType,
	pub anonymous: D3D12RenderPassBeginningAccessAnonymousUnion,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderPassEndingAccessResolveSubresourceParameters {
	pub src_subresource: u32,
	pub dst_subresource: u32,
	pub dst_x: u32,
	pub dst_y: u32,
	pub src_rect: Rect,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderPassEndingAccessResolveParameters<'a> {
	pub src_resource: Param<'a, D3D12Resource>,
	pub dst_resource: Param<'a, D3D12Resource>,
	pub subresource_count: u32,
	pub subresource_parameters: &'a D3D12RenderPassEndingAccessResolveSubresourceParameters,
	pub format: DxgiFormat,
	pub resolve_mode: D3D12ResolveMode,
	pub preserve_resolve_source: Bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12RenderPassEndingAccessAnonymousUnion<'a> {
	pub resolve: D3D12RenderPassEndingAccessResolveParameters<'a>,
}

impl std::fmt::Debug for D3D12RenderPassEndingAccessAnonymousUnion<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "union")
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderPassEndingAccess<'a> {
	pub ty: D3D12RenderPassEndingAccessType,
	pub anonymous: D3D12RenderPassEndingAccessAnonymousUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderPassRenderTargetDesc<'a> {
	pub cpu_descriptor: D3D12CpuDescriptorHandle,
	pub beginning_access: D3D12RenderPassBeginningAccess,
	pub ending_access: D3D12RenderPassEndingAccess<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RenderPassDepthStencilDesc<'a> {
	pub cpu_descriptor: D3D12CpuDescriptorHandle,
	pub depth_beginning_access: D3D12RenderPassBeginningAccess,
	pub stencil_beginning_access: D3D12RenderPassBeginningAccess,
	pub depth_ending_access: D3D12RenderPassEndingAccess<'a>,
	pub stencil_ending_access: D3D12RenderPassEndingAccess<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DispatchRaysDesc {
	pub ray_generation_shader_record: D3D12GpuVirtualAddressRange,
	pub miss_shader_table: D3D12GpuVirtualAddressRangeAndStride,
	pub hit_group_table: D3D12GpuVirtualAddressRangeAndStride,
	pub callable_shader_table: D3D12GpuVirtualAddressRangeAndStride,
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ShaderCacheSessionDesc {
	pub identifier: GUID,
	pub mode: D3D12ShaderCacheMode,
	pub flags: D3D12ShaderCacheFlags,
	pub maximum_in_memory_cache_size_bytes: u32,
	pub maximum_in_memory_cache_entries: u32,
	pub maximum_value_file_size_bytes: u32,
	pub version: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SubresourceData<'a> {
	pub data: &'a c_void,
	pub row_pitch: NonNull<c_void>,
	pub slice_pitch: NonNull<c_void>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12MemcpyDest<'a> {
	pub data: &'a c_void,
	pub row_pitch: usize,
	pub slice_pitch: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DebugDeviceGpuBasedValidationSettings {
	pub max_messages_per_command_list: u32,
	pub default_shader_patch_mode: D3D12GpuBasedValidationShaderPatchMode,
	pub pipeline_state_create_flags: D3D12GpuBasedValidationPipelineStateCreateFlags,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DebugDeviceGpuSlowdownPerformanceFactor {
	pub slowdown_factor: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DebugCommandListGpuBasedValidationSettings {
	pub shader_patch_mode: D3D12GpuBasedValidationShaderPatchMode,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12Message<'a> {
	pub category: D3D12MessageCategory,
	pub severity: D3D12MessageSeverity,
	pub id: D3D12MessageId,
	pub description: &'a u8,
	pub description_byte_length: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12InfoQueueFilterDesc<'a> {
	pub num_categories: u32,
	pub category_list: &'a D3D12MessageCategory,
	pub num_severities: u32,
	pub severity_list: &'a D3D12MessageSeverity,
	pub num_i_ds: u32,
	pub id_list: &'a D3D12MessageId,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12InfoQueueFilter<'a> {
	pub allow_list: D3D12InfoQueueFilterDesc<'a>,
	pub deny_list: D3D12InfoQueueFilterDesc<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12DispatchMeshArguments {
	pub thread_group_count_x: u32,
	pub thread_group_count_y: u32,
	pub thread_group_count_z: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12SignatureParameterDesc<'a> {
	pub semantic_name: PStr<'a>,
	pub semantic_index: u32,
	pub register: u32,
	pub system_value_type: D3DName,
	pub component_type: D3DRegisterComponentType,
	pub mask: u8,
	pub read_write_mask: u8,
	pub stream: u32,
	pub min_precision: D3DMinPrecision,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ShaderBufferDesc<'a> {
	pub name: PStr<'a>,
	pub ty: D3DCBufferType,
	pub variables: u32,
	pub size: u32,
	pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ShaderVariableDesc<'a> {
	pub name: PStr<'a>,
	pub start_offset: u32,
	pub size: u32,
	pub flags: u32,
	pub default_value: &'a c_void,
	pub start_texture: u32,
	pub texture_size: u32,
	pub start_sampler: u32,
	pub sampler_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ShaderTypeDesc<'a> {
	pub class: D3DShaderVariableClass,
	pub ty: D3DShaderVariableType,
	pub rows: u32,
	pub columns: u32,
	pub elements: u32,
	pub members: u32,
	pub offset: u32,
	pub name: PStr<'a>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ShaderDesc<'a> {
	pub version: u32,
	pub creator: PStr<'a>,
	pub flags: u32,
	pub constant_buffers: u32,
	pub bound_resources: u32,
	pub input_parameters: u32,
	pub output_parameters: u32,
	pub instruction_count: u32,
	pub temp_register_count: u32,
	pub temp_array_count: u32,
	pub def_count: u32,
	pub dcl_count: u32,
	pub texture_normal_instructions: u32,
	pub texture_load_instructions: u32,
	pub texture_comp_instructions: u32,
	pub texture_bias_instructions: u32,
	pub texture_gradient_instructions: u32,
	pub float_instruction_count: u32,
	pub int_instruction_count: u32,
	pub uint_instruction_count: u32,
	pub static_flow_control_count: u32,
	pub dynamic_flow_control_count: u32,
	pub macro_instruction_count: u32,
	pub array_instruction_count: u32,
	pub cut_instruction_count: u32,
	pub emit_instruction_count: u32,
	pub gs_output_topology: D3DPrimitiveTopology,
	pub gs_max_output_vertex_count: u32,
	pub input_primitive: D3DPrimitive,
	pub patch_constant_parameters: u32,
	pub gs_instance_count: u32,
	pub control_points: u32,
	pub hs_output_primitive: D3DTessellatorOutputPrimitive,
	pub hs_partitioning: D3DTessellatorPartitioning,
	pub tessellator_domain: D3DTessellatorDomain,
	pub barrier_instructions: u32,
	pub interlocked_instructions: u32,
	pub texture_store_instructions: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ShaderInputBindDesc<'a> {
	pub name: PStr<'a>,
	pub ty: D3DShaderInputType,
	pub bind_point: u32,
	pub bind_count: u32,
	pub flags: u32,
	pub return_type: D3DResourceReturnType,
	pub dimension: D3DSrvDimension,
	pub num_samples: u32,
	pub space: u32,
	pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12LibraryDesc<'a> {
	pub creator: PStr<'a>,
	pub flags: u32,
	pub function_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FunctionDesc<'a> {
	pub version: u32,
	pub creator: PStr<'a>,
	pub flags: u32,
	pub constant_buffers: u32,
	pub bound_resources: u32,
	pub instruction_count: u32,
	pub temp_register_count: u32,
	pub temp_array_count: u32,
	pub def_count: u32,
	pub dcl_count: u32,
	pub texture_normal_instructions: u32,
	pub texture_load_instructions: u32,
	pub texture_comp_instructions: u32,
	pub texture_bias_instructions: u32,
	pub texture_gradient_instructions: u32,
	pub float_instruction_count: u32,
	pub int_instruction_count: u32,
	pub uint_instruction_count: u32,
	pub static_flow_control_count: u32,
	pub dynamic_flow_control_count: u32,
	pub macro_instruction_count: u32,
	pub array_instruction_count: u32,
	pub mov_instruction_count: u32,
	pub movc_instruction_count: u32,
	pub conversion_instruction_count: u32,
	pub bitwise_instruction_count: u32,
	pub min_feature_level: D3DFeatureLevel,
	pub required_feature_flags: u64,
	pub name: PStr<'a>,
	pub function_parameter_count: i32,
	pub has_return: Bool,
	pub has10level9vertex_shader: Bool,
	pub has10level9pixel_shader: Bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ParameterDesc<'a> {
	pub name: PStr<'a>,
	pub semantic_name: PStr<'a>,
	pub ty: D3DShaderVariableType,
	pub class: D3DShaderVariableClass,
	pub rows: u32,
	pub columns: u32,
	pub interpolation_mode: D3DInterpolationMode,
	pub flags: D3DParameterFlags,
	pub first_in_register: u32,
	pub first_in_component: u32,
	pub first_out_register: u32,
	pub first_out_component: u32,
}

