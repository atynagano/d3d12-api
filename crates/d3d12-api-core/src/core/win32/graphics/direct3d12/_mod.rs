use std::mem::transmute;
use std::num::{NonZeroU64, NonZeroUsize};
use std::ops::{Add, AddAssign};
use std::ptr::{NonNull, null_mut};
use std::slice;
use crate::aliases::win32::graphics::direct3d12::GpuVirtualAddress;
use crate::core::win32::foundation::{HResult, PWStr};
use crate::core::win32::graphics::dxgi::common::{DxgiFormat, DxgiSampleDesc};
use crate::core::win32::system::com::{AsParam, Param, UnknownWrapper};
use super::*;

pub type D3D12PrimitiveTopology = crate::core::win32::graphics::direct3d::D3DPrimitiveTopology;
pub type D3D12Primitive = crate::core::win32::graphics::direct3d::D3DPrimitive;
pub type D3D12Rect = crate::core::win32::foundation::Rect;

#[repr(C)]
#[derive(Copy, Clone)]
pub union D3D12BuildRaytracingAccelerationStructureInputsAnonymousUnion<'a> {
    pub instance_descs: D3D12GpuVirtualAddress,
    pub p_geometry_descs: &'a D3D12RaytracingGeometryDesc,
    pub pp_geometry_descs: &'a &'a D3D12RaytracingGeometryDesc,
}

impl std::fmt::Debug for D3D12BuildRaytracingAccelerationStructureInputsAnonymousUnion<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "D3D12BuildRaytracingAccelerationStructureInputsAnonymousUnion")
    }
}

impl<'a> D3D12StateObjectDesc<'a> {
    pub fn new(obj_type: D3D12StateObjectType, objs: &'a [D3D12StateSubobject]) -> Self {
        Self {
            ty: obj_type,
            num_subobjects: objs.len() as u32,
            subobjects: objs.first(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12HitGroupDesc<'a> {
    pub hit_group_export: PWStr<'a>,
    pub ty: D3D12HitGroupType,
    pub any_hit_shader_import: Option<PWStr<'a>>,
    pub closest_hit_shader_import: Option<PWStr<'a>>,
    pub intersection_shader_import: Option<PWStr<'a>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12ExportDesc<'a> {
    pub name: PWStr<'a>,
    pub export_to_rename: Option<PWStr<'a>>,
    pub flags: D3D12ExportFlags,
}


impl Default for D3D12CommandListType { fn default() -> Self { Self::Direct } }

impl Default for D3D12CommandQueueFlags { fn default() -> Self { Self::None } }

impl Default for D3D12CommandQueuePriority { fn default() -> Self { Self::Normal } }

impl D3D12CommandQueueDesc {
    pub const Direct: Self = Self {
        ty: D3D12CommandListType::Direct,
        priority: 0,
        flags: D3D12CommandQueueFlags::None,
        node_mask: 0,
    };
    pub const Bundle: Self = Self { ty: D3D12CommandListType::Bundle, ..Self::Direct };
    pub const Compute: Self = Self { ty: D3D12CommandListType::Compute, ..Self::Direct };
    pub const Copy: Self = Self { ty: D3D12CommandListType::Copy, ..Self::Direct };
    pub const VideoDecode: Self = Self { ty: D3D12CommandListType::VideoDecode, ..Self::Direct };
    pub const VideoProcess: Self = Self { ty: D3D12CommandListType::VideoProcess, ..Self::Direct };
    pub const VideoEncode: Self = Self { ty: D3D12CommandListType::VideoEncode, ..Self::Direct };

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    pub fn with_flags(mut self, flags: D3D12CommandQueueFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn with_node_mask(mut self, node_mask: u32) -> Self {
        self.node_mask = node_mask;
        self
    }
}

impl Default for D3D12PrimitiveTopologyType { fn default() -> Self { Self::Undefined } }

impl Default for D3D12InputClassification { fn default() -> Self { Self::PerVertexData } }

// impl Default for D3D12InputElementDesc<'_> { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12FillMode { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12CullMode { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12SoDeclarationEntry<'_> { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12Viewport { fn default() -> Self { Zeroed::zeroed() } }
impl D3D12Viewport {
    pub const fn new(width: f32, height: f32) -> Self {
        Self {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width,
            height,
            min_depth: D3D12_MIN_DEPTH,
            max_depth: D3D12_MAX_DEPTH,
        }
    }
}

// impl Default for D3D12ComparisonFunc { fn default() -> Self { Zeroed::zeroed() } }
impl Default for D3D12DepthWriteMask { fn default() -> Self { Self::Zero } }

// impl Default for D3D12StencilOp { fn default() -> Self { Zeroed::zeroed() } }
/*
impl Default for D3D12DepthStencilOpDesc {
    fn default() -> Self {
        Self {
            stencil_fail_op: D3D12StencilOp::Keep,
            stencil_depth_fail_op: D3D12StencilOp::Keep,
            stencil_pass_op: D3D12StencilOp::Keep,
            stencil_func: D3D12ComparisonFunc::Never,
        }
    }
}*/

// impl Default for D3D12DepthStencilDesc { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12DepthStencilDesc1 { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12Blend { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12BlendOp { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12ColorWriteEnable { fn default() -> Self { Zeroed::zeroed() } }
// impl Default for D3D12LogicOp { fn default() -> Self { Self::Clear } }

impl Default for D3D12RenderTargetBlendDesc {
    fn default() -> Self {
        Self {
            blend_enable: false.into(),
            logic_op_enable: false.into(),
            src_blend: D3D12Blend::One,
            dest_blend: D3D12Blend::Zero,
            blend_op: D3D12BlendOp::Add,
            src_blend_alpha: D3D12Blend::One,
            dest_blend_alpha: D3D12Blend::One,
            blend_op_alpha: D3D12BlendOp::Add,
            logic_op: D3D12LogicOp::Noop,
            render_target_write_mask: 0,
        }
    }
}

// impl Default for D3D12BlendDesc { fn default() -> Self { Zeroed::zeroed() } }
impl Default for D3D12ConservativeRasterizationMode { fn default() -> Self { Self::Off } }

impl Default for D3D12RasterizerDesc {
    fn default() -> Self {
        Self {
            fill_mode: D3D12FillMode::Solid,
            cull_mode: D3D12CullMode::None,
            //..Zeroed::zeroed()
            front_counter_clockwise: false.into(),
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
            depth_clip_enable: false.into(),
            multisample_enable: false.into(),
            antialiased_line_enable: false.into(),
            forced_sample_count: 0,
            conservative_raster: D3D12ConservativeRasterizationMode::Off,
        }
    }
}

impl D3D12ShaderByteCode<'_> {
    pub fn new(blob: &[u8]) -> Self {
        Self {
            shader_bytecode: unsafe { transmute(blob.first().unwrap()) },
            bytecode_length: blob.len(),
        }
    }
}

impl From<&[u8]> for D3D12ShaderByteCode<'_> {
    fn from(blob: &[u8]) -> Self {
        Self::new(blob)
    }
}

impl D3D12StreamOutputDesc<'_> {}

impl<'a> D3D12InputLayoutDesc<'a> {
    pub fn new(desc: &'a [D3D12InputElementDesc]) -> Self {
        Self {
            input_element_descs: desc.first().unwrap(),
            num_elements: desc.len() as u32,
        }
    }
}

impl D3D12IndexBufferStripCutValue {}

impl D3D12CachedPipelineState<'_> {}

impl D3D12PipelineStateFlags {}

impl D3D12GraphicsPipelineStateDesc<'_> {}

impl D3D12ComputePipelineStateDesc<'_> {}

impl D3D12RtFormatArray {}

impl D3D12PipelineStateStreamDesc<'_> {}

impl D3D12PipelineStateSubobjectType {}

impl D3D12Feature {}

impl D3D12ShaderMinPrecisionSupport {}

impl D3D12TiledResourcesTier {}

impl D3D12ResourceBindingTier {}

impl D3D12ConservativeRasterizationTier {}

impl D3D12FormatSupport1 {}

impl D3D12FormatSupport2 {}

impl D3D12MultiSampleQualityLevelFlags {}

impl D3D12CrossNodeSharingTier {}

impl D3D12ResourceHeapTier {}

impl D3D12ProgrammableSamplePositionsTier {}

impl D3D12ViewInstancingTier {}

impl D3D12FeatureDataD3D12Options {}

impl D3D12FeatureDataD3D12Options1 {}

impl D3D12FeatureDataD3D12Options2 {}

impl D3DRootSignatureVersion {}

impl D3D12FeatureDataRootSignature {}

impl D3D12FeatureDataArchitecture {}

impl D3D12FeatureDataArchitecture1 {}

impl D3D12FeatureDataFeatureLevels<'_> {}

impl D3DShaderModel {}

impl D3D12FeatureDataShaderModel {}

impl D3D12FeatureDataFormatSupport {}

impl D3D12FeatureDataMultiSampleQualityLevels {}

impl D3D12FeatureDataFormatInfo {}

impl D3D12FeatureDataGpuVirtualAddressSupport {}

impl D3D12ShaderCacheSupportFlags {}

impl D3D12FeatureDataShaderCache {}

impl D3D12FeatureDataCommandQueuePriority {}

impl D3D12CommandListSupportFlags {}

impl D3D12FeatureDataD3D12Options3 {}

impl D3D12FeatureDataExistingHeaps {}

impl D3D12SharedResourceCompatibilityTier {}

impl D3D12FeatureDataDisplayable {}

impl D3D12FeatureDataD3D12Options4 {}

impl D3D12HeapSerializationTier {}

impl D3D12FeatureDataSerialization {}

impl D3D12FeatureDataCrossNode {}

impl D3D12RenderPassTier {}

impl D3D12RaytracingTier {}

impl D3D12FeatureDataD3D12Options5 {}

impl D3D12VariableShadingRateTier {}

impl D3D12FeatureDataD3D12Options6 {}

impl D3D12MeshShaderTier {}

impl D3D12SamplerFeedbackTier {}

impl D3D12FeatureDataD3D12Options7 {}

impl D3D12FeatureDataQueryMetaCommand<'_> {}

impl D3D12FeatureDataD3D12Options8 {}

impl D3D12WaveMmaTier {}

impl D3D12FeatureDataD3D12Options9 {}

impl D3D12FeatureDataD3D12Options10 {}

impl D3D12FeatureDataD3D12Options11 {}

impl D3D12ResourceAllocationInfo {}

impl D3D12ResourceAllocationInfo1 {}

impl D3D12HeapType {}

impl D3D12CpuPageProperty {}

impl D3D12MemoryPool {}

impl D3D12HeapProperties {
    pub const DEFAULT: Self =
        Self {
            ty: D3D12HeapType::Default,
            cpu_page_property: D3D12CpuPageProperty::Unknown,
            memory_pool_preference: D3D12MemoryPool::Unknown,
            creation_node_mask: 1,
            visible_node_mask: 1,
        };
    pub const UPLOAD: Self =
        Self {
            ty: D3D12HeapType::Upload,
            cpu_page_property: D3D12CpuPageProperty::Unknown,
            memory_pool_preference: D3D12MemoryPool::Unknown,
            creation_node_mask: 1,
            visible_node_mask: 1,
        };
    pub const READBACK: Self =
        Self {
            ty: D3D12HeapType::ReadBack,
            cpu_page_property: D3D12CpuPageProperty::Unknown,
            memory_pool_preference: D3D12MemoryPool::Unknown,
            creation_node_mask: 1,
            visible_node_mask: 1,
        };
}

impl D3D12HeapFlags {}

impl D3D12HeapDesc {}

impl D3D12ResourceDimension {}

impl D3D12TextureLayout {}

impl D3D12ResourceFlags {}

impl D3D12MipRegion {}

impl D3D12ResourceDesc {
    pub const fn Buffer(size: u64) -> Self {
        Self {
            dimension: D3D12ResourceDimension::Buffer,
            alignment: 0,
            width: size,
            height: 1,
            depth_or_array_size: 1,
            mip_levels: 1,
            format: DxgiFormat::Unknown,
            sample_desc: DxgiSampleDesc::new(1, 0),
            layout: D3D12TextureLayout::RowMajor,
            flags: D3D12ResourceFlags::None,
        }
    }
    pub const fn Tex1d(format: DxgiFormat, width: u64) -> Self {
        Self {
            dimension: D3D12ResourceDimension::Texture1d,
            alignment: 0,
            width,
            height: 1,
            depth_or_array_size: 1,
            mip_levels: 0,
            format,
            sample_desc: DxgiSampleDesc::new(1, 0),
            layout: D3D12TextureLayout::Unknown,
            flags: D3D12ResourceFlags::None,
        }
    }
    pub const fn Tex2d(format: DxgiFormat, width: u32, height: u32) -> Self {
        Self {
            dimension: D3D12ResourceDimension::Texture2d,
            alignment: 0,
            width: width as u64,
            height,
            depth_or_array_size: 1,
            mip_levels: 0,
            format,
            sample_desc: DxgiSampleDesc::new(1, 0),
            layout: D3D12TextureLayout::Unknown,
            flags: D3D12ResourceFlags::None,
        }
    }
    pub const fn Tex3d(format: DxgiFormat, width: u64, height: u32, depth: u16) -> Self {
        Self {
            dimension: D3D12ResourceDimension::Texture3d,
            alignment: 0,
            width,
            height,
            depth_or_array_size: depth,
            mip_levels: 0,
            format,
            sample_desc: DxgiSampleDesc::new(1, 0),
            layout: D3D12TextureLayout::Unknown,
            flags: D3D12ResourceFlags::None,
        }
    }
    pub const fn with_alignment(mut self, alignment: u64) -> Self {
        self.alignment = alignment;
        self
    }
    pub const fn with_width(mut self, width: u64) -> Self {
        self.width = width;
        self
    }
    pub const fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    pub const fn with_depth(mut self, depth: u16) -> Self {
        self.depth_or_array_size = depth;
        self
    }
    pub const fn with_array_size(mut self, mip_levels: u16) -> Self {
        self.depth_or_array_size = mip_levels;
        self
    }
    pub const fn with_mip_levels(mut self, mip_levels: u16) -> Self {
        self.mip_levels = mip_levels;
        self
    }
    pub const fn with_format(mut self, format: DxgiFormat) -> Self {
        self.format = format;
        self
    }
    pub const fn with_sample_desc(mut self, count: u32, quality: u32) -> Self {
        self.sample_desc = DxgiSampleDesc::new(count, quality);
        self
    }
    pub const fn with_layout(mut self, layout: D3D12TextureLayout) -> Self {
        self.layout = layout;
        self
    }
    pub const fn with_flags(mut self, flags: D3D12ResourceFlags) -> Self {
        self.flags = flags;
        self
    }
}

impl D3D12ResourceDesc1 {
    // todo
    pub const fn Buffer(size: u64) -> Self {
        Self {
            base: D3D12ResourceDesc::Buffer(size),
            sampler_feedback_mip_region: D3D12MipRegion {
                width: 0,
                height: 0,
                depth: 0,
            },
        }
    }
    pub const fn with_alignment(mut self, alignment: u64) -> Self {
        self.base.alignment = alignment;
        self
    }
    pub const fn with_width(mut self, width: u64) -> Self {
        self.base.width = width;
        self
    }
    pub const fn with_height(mut self, height: u32) -> Self {
        self.base.height = height;
        self
    }
    pub const fn with_depth(mut self, depth: u16) -> Self {
        self.base.depth_or_array_size = depth;
        self
    }
    pub const fn with_array_size(mut self, mip_levels: u16) -> Self {
        self.base.depth_or_array_size = mip_levels;
        self
    }
    pub const fn with_mip_levels(mut self, mip_levels: u16) -> Self {
        self.base.mip_levels = mip_levels;
        self
    }
    pub const fn with_format(mut self, format: DxgiFormat) -> Self {
        self.base.format = format;
        self
    }
    pub const fn with_sample_desc(mut self, count: u32, quality: u32) -> Self {
        self.base.sample_desc = DxgiSampleDesc::new(count, quality);
        self
    }
    pub const fn with_layout(mut self, layout: D3D12TextureLayout) -> Self {
        self.base.layout = layout;
        self
    }
    pub const fn with_flags(mut self, flags: D3D12ResourceFlags) -> Self {
        self.base.flags = flags;
        self
    }
}

impl D3D12DepthStencilValue {}

impl D3D12ClearValueAnonymousUnion {}

impl D3D12ClearValue {
    pub const fn Color(format: DxgiFormat, color: [f32; 4]) -> Self {
        Self {
            format,
            anonymous: D3D12ClearValueAnonymousUnion { color },
        }
    }
    pub const fn DepthStencil(format: DxgiFormat, depth: f32, stencil: u8) -> Self {
        Self {
            format,
            anonymous: D3D12ClearValueAnonymousUnion {
                depth_stencil: D3D12DepthStencilValue { depth, stencil }
            },
        }
    }
}

pub type D3D12Range = std::ops::Range<usize>;

impl D3D12RangeUint64 {}

impl D3D12SubresourceRangeUint64 {}

impl D3D12SubresourceInfo {}

impl D3D12TiledResourceCoordinate {}

impl D3D12TileRegionSize {}

impl D3D12TileRangeFlags {}

impl D3D12SubresourceTiling {}

impl D3D12TileShape {}

impl D3D12PackedMipInfo {}

impl D3D12TileMappingFlags {}

impl D3D12TileCopyFlags {}

impl D3D12ResourceStates {}

impl D3D12ResourceBarrierType {}

impl D3D12ResourceTransitionBarrier<'_> {}

impl D3D12ResourceAliasingBarrier<'_> {}

impl D3D12ResourceUavBarrier<'_> {}

impl D3D12ResourceBarrierFlags {}

impl D3D12ResourceBarrierAnonymousUnion<'_> {}

impl<'a> D3D12ResourceBarrier<'a> {
    pub fn Transition(
        resource: &'a impl ID3D12Resource,
        subresource: u32,
        before: D3D12ResourceStates,
        after: D3D12ResourceStates,
    ) -> Self {
        Self {
            ty: D3D12ResourceBarrierType::Transition,
            flags: D3D12ResourceBarrierFlags::None,
            anonymous: D3D12ResourceBarrierAnonymousUnion {
                transition: D3D12ResourceTransitionBarrier {
                    resource: resource.as_resource().as_param(),
                    subresource,
                    state_before: before,
                    state_after: after,
                }
            },
        }
    }
    pub fn Aliasing(before: &'a impl ID3D12Resource, after: &'a impl ID3D12Resource) -> Self {
        Self {
            ty: D3D12ResourceBarrierType::Aliasing,
            flags: D3D12ResourceBarrierFlags::None,
            anonymous: D3D12ResourceBarrierAnonymousUnion {
                aliasing: D3D12ResourceAliasingBarrier {
                    resource_before: before.as_resource().as_param(),
                    resource_after: after.as_resource().as_param(),
                }
            },
        }
    }
    pub fn Uav(resource: &'a impl ID3D12Resource) -> Self {
        Self {
            ty: D3D12ResourceBarrierType::Uav,
            flags: D3D12ResourceBarrierFlags::None,
            anonymous: D3D12ResourceBarrierAnonymousUnion {
                uav: D3D12ResourceUavBarrier {
                    resource: resource.as_resource().as_param(),
                }
            },
        }
    }
    pub fn with_flags(mut self, flags: D3D12ResourceBarrierFlags) -> Self {
        self.flags = flags;
        self
    }
}

impl D3D12SubresourceFootprint {}

impl D3D12PlacedSubresourceFootprint {}

impl D3D12TextureCopyType {}

impl D3D12TextureCopyLocationAnonymousUnion {}

impl<'a> D3D12TextureCopyLocation<'a> {
    pub fn SubresourceIndex(resource: &'a D3D12Resource, index: u32) -> Self {
        Self {
            resource: resource.as_resource().as_param(),
            ty: D3D12TextureCopyType::SubresourceIndex,
            anonymous: D3D12TextureCopyLocationAnonymousUnion {
                subresource_index: index
            },
        }
    }
    pub fn PlacedFootprint(resource: &'a D3D12Resource, footprint: D3D12PlacedSubresourceFootprint) -> Self {
        Self {
            resource: resource.as_resource().as_param(),
            ty: D3D12TextureCopyType::PlacedFootprint,
            anonymous: D3D12TextureCopyLocationAnonymousUnion {
                placed_footprint: footprint
            },
        }
    }
}

impl D3D12ResolveMode {}

impl D3D12SamplePosition {}

impl D3D12ViewInstanceLocation {}

impl D3D12ViewInstancingFlags {}

impl D3D12ViewInstancingDesc<'_> {}

impl D3D12ShaderComponentMapping {}

impl D3D12BufferSrvFlags {}

impl D3D12BufferSrv {}

impl D3D12Tex1dSrv {}

impl D3D12Tex1dArraySrv {}

impl D3D12Tex2dSrv {}

impl D3D12Tex2dArraySrv {}

impl D3D12Tex3dSrv {}

impl D3D12TexCubeSrv {}

impl D3D12TexCubeArraySrv {}

impl D3D12Tex2dMsSrv {}

impl D3D12Tex2dMsArraySrv {}

impl D3D12RaytracingAccelerationStructureSrv {}

impl D3D12SrvDimension {}

impl D3D12ShaderResourceViewDescAnonymousUnion {}

impl D3D12ShaderResourceViewDesc {}

impl D3D12ConstantBufferViewDesc {}

impl D3D12Filter {}

impl D3D12FilterType {}

impl D3D12FilterReductionType {}

impl D3D12TextureAddressMode {}

impl D3D12SamplerDesc {}

impl D3D12BufferUavFlags {}

impl D3D12BufferUav {}

impl D3D12Tex1dUav {}

impl D3D12Tex1dArrayUav {}

impl D3D12Tex2dUav {}

impl D3D12Tex2dArrayUav {}

impl D3D12Tex3dUav {}

impl D3D12UavDimension {}

impl D3D12UnorderedAccessViewDescAnonymousUnion {}

impl D3D12UnorderedAccessViewDesc {}

impl D3D12BufferRtv {}

impl D3D12Tex1dRtv {}

impl D3D12Tex1dArrayRtv {}

impl D3D12Tex2dRtv {}

impl D3D12Tex2dMsRtv {}

impl D3D12Tex2dArrayRtv {}

impl D3D12Tex2dMsArrayRtv {}

impl D3D12Tex3dRtv {}

impl D3D12RtvDimension {}

impl D3D12RenderTargetViewDescAnonymousUnion {}

impl D3D12RenderTargetViewDesc {}

impl D3D12Tex1dDsv {}

impl D3D12Tex1dArrayDsv {}

impl D3D12Tex2dDsv {}

impl D3D12Tex2dArrayDsv {}

impl D3D12Tex2dMsDsv {}

impl D3D12Tex2dMsArrayDsv {}

impl D3D12DsvFlags {}

impl D3D12DsvDimension {}

impl D3D12DepthStencilViewDescAnonymousUnion {}

impl D3D12DepthStencilViewDesc {}

impl D3D12ClearFlags {}

impl D3D12FenceFlags {}

impl D3D12DescriptorHeapType {}

impl D3D12DescriptorHeapFlags {}

impl D3D12DescriptorHeapDesc {
    // todo: ShaderVisibleかNoneか
    pub const fn Rtv(num_descs: u32) -> Self {
        Self {
            ty: D3D12DescriptorHeapType::Rtv,
            num_descriptors: num_descs,
            flags: D3D12DescriptorHeapFlags::None,
            node_mask: 0,
        }
    }
    pub const fn CbvSrvUav(num_descs: u32) -> Self {
        Self {
            ty: D3D12DescriptorHeapType::CbvSrvUav,
            num_descriptors: num_descs,
            flags: D3D12DescriptorHeapFlags::ShaderVisible,
            node_mask: 0,
        }
    }
    pub const fn NumTypes(num_descs: u32) -> Self {
        todo!();
        Self {
            ty: D3D12DescriptorHeapType::NumTypes,
            num_descriptors: num_descs,
            flags: D3D12DescriptorHeapFlags::None,
            node_mask: 0,
        }
    }
    pub const fn Sampler(num_descs: u32) -> Self {
        todo!();
        Self {
            ty: D3D12DescriptorHeapType::Sampler,
            num_descriptors: num_descs,
            flags: D3D12DescriptorHeapFlags::None,
            node_mask: 0,
        }
    }
    pub const fn Dsv(num_descs: u32) -> Self {
        todo!();
        Self {
            ty: D3D12DescriptorHeapType::Dsv,
            num_descriptors: num_descs,
            flags: D3D12DescriptorHeapFlags::None,
            node_mask: 0,
        }
    }
    pub const fn with_shader_visible(mut self, value: bool) -> Self {
        self.flags = if value {
            D3D12DescriptorHeapFlags::ShaderVisible
        } else {
            D3D12DescriptorHeapFlags::None
        };
        self
    }
    pub const fn with_node_mask(mut self, value: u32) -> Self {
        self.node_mask = value;
        self
    }
}

impl D3D12DescriptorRangeType {}

impl D3D12DescriptorRange {}

impl D3D12RootDescriptorTable<'_> {}

impl D3D12RootConstants {}

impl D3D12RootDescriptor {}

impl D3D12ShaderVisibility {}

impl D3D12RootParameterType {}

impl D3D12RootParameterAnonymousUnion<'_> {}

impl D3D12RootParameter<'_> {}

impl D3D12RootSignatureFlags {}

impl D3D12StaticBorderColor {}

impl D3D12StaticSamplerDesc {}

impl D3D12RootSignatureDesc<'_> {}

impl D3D12DescriptorRangeFlags {}

impl D3D12DescriptorRange1 {}

impl D3D12RootDescriptorTable1<'_> {}

impl D3D12RootDescriptorFlags {}

impl D3D12RootDescriptor1 {}

impl D3D12RootParameter1AnonymousUnion<'_> {}

impl D3D12RootParameter1<'_> {}

impl D3D12RootSignatureDesc1<'_> {}

impl D3D12VersionedRootSignatureDescAnonymousUnion<'_> {}

impl D3D12VersionedRootSignatureDesc<'_> {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct D3D12CpuDescriptorHandle {
    pub ptr: NonZeroUsize,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct D3D12GpuDescriptorHandle {
    pub ptr: NonZeroU64,
}

impl<T: Into<usize>> Add<T> for D3D12CpuDescriptorHandle {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        unsafe {
            Self::Output { ptr: NonZeroUsize::new_unchecked(self.ptr.get() + rhs.into()) }
        }
    }
}

impl<T: Into<usize>> AddAssign<T> for D3D12CpuDescriptorHandle {
    fn add_assign(&mut self, rhs: T) { *self = *self + rhs; }
}

impl<T: Into<u64>> Add<T> for D3D12GpuDescriptorHandle {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        unsafe {
            Self::Output { ptr: NonZeroU64::new_unchecked(self.ptr.get() + rhs.into()) }
        }
    }
}

impl<T: Into<u64>> AddAssign<T> for D3D12GpuDescriptorHandle {
    fn add_assign(&mut self, rhs: T) { *self = *self + rhs; }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12GpuVirtualAddress {
    pub ptr: NonZeroU64,
}

impl<T: Into<u64>> Add<T> for D3D12GpuVirtualAddress {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        unsafe {
            Self::Output { ptr: NonZeroU64::new_unchecked(self.ptr.get() + rhs.into()) }
        }
    }
}

impl<T: Into<u64>> AddAssign<T> for D3D12GpuVirtualAddress {
    fn add_assign(&mut self, rhs: T) { *self = *self + rhs; }
}

impl D3D12DiscardRegion<'_> {}

impl D3D12QueryHeapType {}

impl D3D12QueryHeapDesc {}

impl D3D12QueryType {}

impl D3D12PredicationOp {}

impl D3D12QueryDataPipelineStatistics {}

impl D3D12QueryDataPipelineStatistics1 {}

impl D3D12QueryDataSoStatistics {}

impl D3D12StreamOutputBufferView {}

impl D3D12DrawArguments {}

impl D3D12DrawIndexedArguments {}

impl D3D12DispatchArguments {}

impl D3D12VertexBufferView {}

impl D3D12IndexBufferView {}

impl D3D12IndirectArgumentType {}

impl D3D12IndirectArgumentDescAnonymousUnion {}

impl D3D12IndirectArgumentDesc {}

impl D3D12CommandSignatureDesc<'_> {}

impl D3D12WriteBufferImmediateParameter {}

impl D3D12WriteBufferImmediateMode {}

impl D3D12MultipleFenceWaitFlags {}

impl D3D12ResidencyPriority {}

impl D3D12ResidencyFlags {}

impl D3D12CommandListFlags {}

impl D3D12CommandPoolFlags {}

impl D3D12CommandRecorderFlags {}

impl D3D12ProtectedSessionStatus {}

impl D3D12ProtectedResourceSessionSupportFlags {}

impl D3D12FeatureDataProtectedResourceSessionSupport {}

impl D3D12ProtectedResourceSessionFlags {}

impl D3D12ProtectedResourceSessionDesc {}

impl D3D12LifetimeState {}

impl D3D12MetaCommandParameterType {}

impl D3D12MetaCommandParameterFlags {}

impl D3D12MetaCommandParameterStage {}

impl D3D12MetaCommandParameterDesc<'_> {}

impl D3D12GraphicsStates {}

impl D3D12MetaCommandDesc<'_> {}

impl D3D12StateSubobjectType {}

impl D3D12StateSubobject<'_> {}

impl D3D12StateObjectFlags {}

impl D3D12StateObjectConfig {}

impl<'a> D3D12GlobalRootSignature<'a> {
    pub fn new(root_sig: &'a impl ID3D12RootSignature) -> Self {
        Self {
            global_root_signature: root_sig.as_root_signature().as_param()
        }
    }
}

impl<'a> D3D12LocalRootSignature<'a> {
    pub fn new(root_sig: &'a impl ID3D12RootSignature) -> Self {
        Self {
            local_root_signature: root_sig.as_root_signature().as_param()
        }
    }
}

impl D3D12NodeMask {}

impl D3D12ExportFlags {}

impl D3D12DxilLibraryDesc<'_> {}

impl D3D12ExistingCollectionDesc<'_> {}

impl<'a> D3D12SubobjectToExportsAssociation<'a> {
    pub fn new(obj: &'a D3D12StateSubobject, exports: &'a [PWStr]) -> Self {
        Self {
            subobject_to_associate: obj,
            num_exports: exports.len() as u32,
            exports: exports.first(),
        }
    }
}

impl D3D12DxilSubobjectToExportsAssociation<'_> {}

impl D3D12HitGroupType {}

impl D3D12HitGroupDesc<'_> {}

impl D3D12RaytracingShaderConfig {}

impl D3D12RaytracingPipelineConfig {}

impl D3D12RaytracingPipelineFlags {}

impl D3D12RaytracingPipelineConfig1 {}

impl D3D12StateObjectType {}

impl D3D12RaytracingGeometryFlags {}

impl D3D12RaytracingGeometryType {}

impl D3D12RaytracingInstanceFlags {}

impl D3D12GpuVirtualAddressAndStride {}

impl D3D12GpuVirtualAddressRange {}

impl D3D12GpuVirtualAddressRangeAndStride {}

impl D3D12RaytracingGeometryTrianglesDesc {}

impl D3D12RaytracingAabb {}

impl D3D12RaytracingGeometryAabbsDesc {}

impl D3D12RaytracingAccelerationStructureBuildFlags {}

impl D3D12RaytracingAccelerationStructureCopyMode {}

impl D3D12RaytracingAccelerationStructureType {}

impl D3D12ElementsLayout {}

impl D3D12RaytracingAccelerationStructurePostBuildInfoType {}

impl D3D12RaytracingAccelerationStructurePostBuildInfoDesc {}

impl D3D12RaytracingAccelerationStructurePostBuildInfoCompactedSizeDesc {}

impl D3D12RaytracingAccelerationStructurePostBuildInfoToolsVisualizationDesc {}

impl D3D12BuildRaytracingAccelerationStructureToolsVisualizationHeader {}

impl D3D12RaytracingAccelerationStructurePostBuildInfoSerializationDesc {}

impl D3D12SerializedDataDriverMatchingIdentifier {}

impl D3D12SerializedDataType {}

impl D3D12DriverMatchingIdentifierStatus {}

impl D3D12SerializedRaytracingAccelerationStructureHeader {}

impl D3D12RaytracingAccelerationStructurePostBuildInfoCurrentSizeDesc {}

impl D3D12RaytracingGeometryDescAnonymousUnion {}

impl D3D12RaytracingGeometryDesc {}

impl D3D12BuildRaytracingAccelerationStructureInputs<'_> {}

impl<'a> D3D12BuildRaytracingAccelerationStructureDesc<'a> {
    pub fn new(
        dest_as: D3D12GpuVirtualAddress,
        inputs: D3D12BuildRaytracingAccelerationStructureInputs<'a>,
        source_as: Option<D3D12GpuVirtualAddress>,
        scratch_as: D3D12GpuVirtualAddress,
    ) -> Self {
        Self {
            dest_acceleration_structure_data: dest_as,
            inputs: inputs,
            source_acceleration_structure_data: source_as,
            scratch_acceleration_structure_data: scratch_as,
        }
    }
}

impl D3D12RaytracingAccelerationStructurePrebuildInfo {}

impl D3D12RayFlags {}

impl D3D12HitKind {}

impl D3D12AutoBreadcrumbOp {}

impl D3D12DredVersion {}

impl D3D12DredFlags {}

impl D3D12DredEnablement {}

impl D3D12DredAllocationType {}

impl D3D12DredAllocationNode<'_> {}

impl D3D12DredAllocationNode1<'_> {}

impl D3D12DredPageFaultOutput<'_> {}

impl D3D12DredPageFaultOutput1<'_> {}

impl D3D12DredPageFaultFlags {}

impl D3D12DredDeviceState {}

impl D3D12DredPageFaultOutput2<'_> {}

impl D3D12BackgroundProcessingMode {}

impl D3D12MeasurementsAction {}

impl D3D12FeatureDataProtectedResourceSessionTypeCount {}

impl D3D12FeatureDataProtectedResourceSessionTypes<'_> {}

impl D3D12ProtectedResourceSessionDesc1 {}

impl D3D12RenderPassBeginningAccessType {}

impl D3D12RenderPassBeginningAccessClearParameters {}

impl D3D12RenderPassBeginningAccessAnonymousUnion {}

impl D3D12RenderPassBeginningAccess {}

impl D3D12RenderPassEndingAccessType {}

impl D3D12RenderPassEndingAccessResolveSubresourceParameters {}

impl D3D12RenderPassEndingAccessResolveParameters<'_> {}

impl D3D12RenderPassEndingAccessAnonymousUnion<'_> {}

impl D3D12RenderPassEndingAccess<'_> {}

impl D3D12RenderPassRenderTargetDesc<'_> {}

impl D3D12RenderPassDepthStencilDesc<'_> {}

impl D3D12RenderPassFlags {}

impl D3D12DispatchRaysDesc {}

impl D3D12ShaderCacheMode {}

impl D3D12ShaderCacheFlags {}

impl D3D12ShaderCacheSessionDesc {}

impl D3D12ShaderCacheKindFlags {}

impl D3D12ShaderCacheControlFlags {}

impl D3D12SubresourceData<'_> {}

impl D3D12MemcpyDest<'_> {}

impl D3D12GpuBasedValidationFlags {}

impl D3D12RldoFlags {}

impl D3D12DebugDeviceParameterType {}

impl D3D12DebugFeature {}

impl D3D12GpuBasedValidationShaderPatchMode {}

impl D3D12GpuBasedValidationPipelineStateCreateFlags {}

impl D3D12DebugDeviceGpuBasedValidationSettings {}

impl D3D12DebugDeviceGpuSlowdownPerformanceFactor {}

impl D3D12DebugCommandListParameterType {}

impl D3D12DebugCommandListGpuBasedValidationSettings {}

impl D3D12MessageCategory {}

impl D3D12MessageSeverity {}

impl D3D12MessageId {}

impl D3D12InfoQueueFilterDesc<'_> {}

impl D3D12InfoQueueFilter<'_> {}

impl D3D12MessageCallbackFlags {}

impl D3D12AxisShadingRate {}

impl D3D12ShadingRate {}

impl D3D12ShadingRateCombiner {}

impl D3D12DispatchMeshArguments {}

impl D3D12ShaderVersionType {}

impl D3D12SignatureParameterDesc<'_> {}

impl D3D12ShaderBufferDesc<'_> {}

impl D3D12ShaderVariableDesc<'_> {}

impl D3D12ShaderTypeDesc<'_> {}

impl D3D12ShaderDesc<'_> {}

impl D3D12ShaderInputBindDesc<'_> {}

impl D3D12LibraryDesc<'_> {}

impl D3D12FunctionDesc<'_> {}

impl D3D12ParameterDesc<'_> {}

pub const fn D3D12CalcSubresource(mip_slice: u32, array_slice: u32, plane_slice: u32, mip_levels: u32, array_size: u32) -> u32 {
    mip_slice + array_slice * mip_levels + plane_slice * mip_levels * array_size
}

/* todo


//------------------------------------------------------------------------------------------------
template <typename T, typename U, typename V>
inline void D3D12DecomposeSubresource( UINT Subresource, UINT MipLevels, UINT ArraySize, _Out_ T& MipSlice, _Out_ U& ArraySlice, _Out_ V& PlaneSlice ) noexcept
{
    MipSlice = static_cast<T>(Subresource % MipLevels);
    ArraySlice = static_cast<U>((Subresource / MipLevels) % ArraySize);
    PlaneSlice = static_cast<V>(Subresource / (MipLevels * ArraySize));
}

//------------------------------------------------------------------------------------------------
inline UINT8 D3D12GetFormatPlaneCount(
    _In_ ID3D12Device* pDevice,
    DXGI_FORMAT Format
    ) noexcept
{
    D3D12_FEATURE_DATA_FORMAT_INFO formatInfo = { Format, 0 };
    if (FAILED(pDevice->CheckFeatureSupport(D3D12_FEATURE_FORMAT_INFO, &formatInfo, sizeof(formatInfo))))
    {
        return 0;
    }
    return formatInfo.PlaneCount;
}


//------------------------------------------------------------------------------------------------
// Row-by-row memcpy
inline void MemcpySubresource(
    _In_ const D3D12_MEMCPY_DEST* pDest,
    _In_ const D3D12_SUBRESOURCE_DATA* pSrc,
    SIZE_T RowSizeInBytes,
    UINT NumRows,
    UINT NumSlices) noexcept
{
    for (UINT z = 0; z < NumSlices; ++z)
    {
        auto pDestSlice = static_cast<BYTE*>(pDest->pData) + pDest->SlicePitch * z;
        auto pSrcSlice = static_cast<const BYTE*>(pSrc->pData) + pSrc->SlicePitch * LONG_PTR(z);
        for (UINT y = 0; y < NumRows; ++y)
        {
            memcpy(pDestSlice + pDest->RowPitch * y,
                   pSrcSlice + pSrc->RowPitch * LONG_PTR(y),
                   RowSizeInBytes);
        }
    }
}

//------------------------------------------------------------------------------------------------
// Row-by-row memcpy
inline void MemcpySubresource(
    _In_ const D3D12_MEMCPY_DEST* pDest,
    _In_ const void* pResourceData,
    _In_ const D3D12_SUBRESOURCE_INFO* pSrc,
    SIZE_T RowSizeInBytes,
    UINT NumRows,
    UINT NumSlices) noexcept
{
    for (UINT z = 0; z < NumSlices; ++z)
    {
        auto pDestSlice = static_cast<BYTE*>(pDest->pData) + pDest->SlicePitch * z;
        auto pSrcSlice = (static_cast<const BYTE*>(pResourceData) + pSrc->Offset) + pSrc->DepthPitch * ULONG_PTR(z);
        for (UINT y = 0; y < NumRows; ++y)
        {
            memcpy(pDestSlice + pDest->RowPitch * y,
                pSrcSlice + pSrc->RowPitch * ULONG_PTR(y),
                RowSizeInBytes);
        }
    }
}

//------------------------------------------------------------------------------------------------
// Returns required size of a buffer to be used for data upload
inline UINT64 GetRequiredIntermediateSize(
    _In_ ID3D12Resource* pDestinationResource,
    _In_range_(0,D3D12_REQ_SUBRESOURCES) UINT FirstSubresource,
    _In_range_(0,D3D12_REQ_SUBRESOURCES-FirstSubresource) UINT NumSubresources) noexcept
{
    const auto Desc = pDestinationResource->GetDesc();
    UINT64 RequiredSize = 0;

    ID3D12Device* pDevice = nullptr;
    pDestinationResource->GetDevice(IID_ID3D12Device, reinterpret_cast<void**>(&pDevice));
    pDevice->GetCopyableFootprints(&Desc, FirstSubresource, NumSubresources, 0, nullptr, nullptr, nullptr, &RequiredSize);
    pDevice->Release();

    return RequiredSize;
}

//------------------------------------------------------------------------------------------------
// All arrays must be populated (e.g. by calling GetCopyableFootprints)
inline UINT64 UpdateSubresources(
    _In_ ID3D12GraphicsCommandList* pCmdList,
    _In_ ID3D12Resource* pDestinationResource,
    _In_ ID3D12Resource* pIntermediate,
    _In_range_(0,D3D12_REQ_SUBRESOURCES) UINT FirstSubresource,
    _In_range_(0,D3D12_REQ_SUBRESOURCES-FirstSubresource) UINT NumSubresources,
    UINT64 RequiredSize,
    _In_reads_(NumSubresources) const D3D12_PLACED_SUBRESOURCE_FOOTPRINT* pLayouts,
    _In_reads_(NumSubresources) const UINT* pNumRows,
    _In_reads_(NumSubresources) const UINT64* pRowSizesInBytes,
    _In_reads_(NumSubresources) const D3D12_SUBRESOURCE_DATA* pSrcData) noexcept
{
    // Minor validation
    const auto IntermediateDesc = pIntermediate->GetDesc();
    const auto DestinationDesc = pDestinationResource->GetDesc();
    if (IntermediateDesc.Dimension != D3D12_RESOURCE_DIMENSION_BUFFER ||
        IntermediateDesc.Width < RequiredSize + pLayouts[0].Offset ||
        RequiredSize > SIZE_T(-1) ||
        (DestinationDesc.Dimension == D3D12_RESOURCE_DIMENSION_BUFFER &&
            (FirstSubresource != 0 || NumSubresources != 1)))
    {
        return 0;
    }

    BYTE* pData;
    HRESULT hr = pIntermediate->Map(0, nullptr, reinterpret_cast<void**>(&pData));
    if (FAILED(hr))
    {
        return 0;
    }

    for (UINT i = 0; i < NumSubresources; ++i)
    {
        if (pRowSizesInBytes[i] > SIZE_T(-1)) return 0;
        D3D12_MEMCPY_DEST DestData = { pData + pLayouts[i].Offset, pLayouts[i].Footprint.RowPitch, SIZE_T(pLayouts[i].Footprint.RowPitch) * SIZE_T(pNumRows[i]) };
        MemcpySubresource(&DestData, &pSrcData[i], static_cast<SIZE_T>(pRowSizesInBytes[i]), pNumRows[i], pLayouts[i].Footprint.Depth);
    }
    pIntermediate->Unmap(0, nullptr);

    if (DestinationDesc.Dimension == D3D12_RESOURCE_DIMENSION_BUFFER)
    {
        pCmdList->CopyBufferRegion(
            pDestinationResource, 0, pIntermediate, pLayouts[0].Offset, pLayouts[0].Footprint.Width);
    }
    else
    {
        for (UINT i = 0; i < NumSubresources; ++i)
        {
            const CD3DX12_TEXTURE_COPY_LOCATION Dst(pDestinationResource, i + FirstSubresource);
            const CD3DX12_TEXTURE_COPY_LOCATION Src(pIntermediate, pLayouts[i]);
            pCmdList->CopyTextureRegion(&Dst, 0, 0, 0, &Src, nullptr);
        }
    }
    return RequiredSize;
}

//------------------------------------------------------------------------------------------------
// All arrays must be populated (e.g. by calling GetCopyableFootprints)
inline UINT64 UpdateSubresources(
    _In_ ID3D12GraphicsCommandList* pCmdList,
    _In_ ID3D12Resource* pDestinationResource,
    _In_ ID3D12Resource* pIntermediate,
    _In_range_(0,D3D12_REQ_SUBRESOURCES) UINT FirstSubresource,
    _In_range_(0,D3D12_REQ_SUBRESOURCES-FirstSubresource) UINT NumSubresources,
    UINT64 RequiredSize,
    _In_reads_(NumSubresources) const D3D12_PLACED_SUBRESOURCE_FOOTPRINT* pLayouts,
    _In_reads_(NumSubresources) const UINT* pNumRows,
    _In_reads_(NumSubresources) const UINT64* pRowSizesInBytes,
    _In_ const void* pResourceData,
    _In_reads_(NumSubresources) const D3D12_SUBRESOURCE_INFO* pSrcData) noexcept
{
    // Minor validation
    const auto IntermediateDesc = pIntermediate->GetDesc();
    const auto DestinationDesc = pDestinationResource->GetDesc();
    if (IntermediateDesc.Dimension != D3D12_RESOURCE_DIMENSION_BUFFER ||
        IntermediateDesc.Width < RequiredSize + pLayouts[0].Offset ||
        RequiredSize > SIZE_T(-1) ||
        (DestinationDesc.Dimension == D3D12_RESOURCE_DIMENSION_BUFFER &&
            (FirstSubresource != 0 || NumSubresources != 1)))
    {
        return 0;
    }

    BYTE* pData;
    HRESULT hr = pIntermediate->Map(0, nullptr, reinterpret_cast<void**>(&pData));
    if (FAILED(hr))
    {
        return 0;
    }

    for (UINT i = 0; i < NumSubresources; ++i)
    {
        if (pRowSizesInBytes[i] > SIZE_T(-1)) return 0;
        D3D12_MEMCPY_DEST DestData = { pData + pLayouts[i].Offset, pLayouts[i].Footprint.RowPitch, SIZE_T(pLayouts[i].Footprint.RowPitch) * SIZE_T(pNumRows[i]) };
        MemcpySubresource(&DestData, pResourceData, &pSrcData[i], static_cast<SIZE_T>(pRowSizesInBytes[i]), pNumRows[i], pLayouts[i].Footprint.Depth);
    }
    pIntermediate->Unmap(0, nullptr);

    if (DestinationDesc.Dimension == D3D12_RESOURCE_DIMENSION_BUFFER)
    {
        pCmdList->CopyBufferRegion(
            pDestinationResource, 0, pIntermediate, pLayouts[0].Offset, pLayouts[0].Footprint.Width);
    }
    else
    {
        for (UINT i = 0; i < NumSubresources; ++i)
        {
            const CD3DX12_TEXTURE_COPY_LOCATION Dst(pDestinationResource, i + FirstSubresource);
            const CD3DX12_TEXTURE_COPY_LOCATION Src(pIntermediate, pLayouts[i]);
            pCmdList->CopyTextureRegion(&Dst, 0, 0, 0, &Src, nullptr);
        }
    }
    return RequiredSize;
}

//------------------------------------------------------------------------------------------------
// Heap-allocating UpdateSubresources implementation
inline UINT64 UpdateSubresources(
    _In_ ID3D12GraphicsCommandList* pCmdList,
    _In_ ID3D12Resource* pDestinationResource,
    _In_ ID3D12Resource* pIntermediate,
    UINT64 IntermediateOffset,
    _In_range_(0,D3D12_REQ_SUBRESOURCES) UINT FirstSubresource,
    _In_range_(0,D3D12_REQ_SUBRESOURCES-FirstSubresource) UINT NumSubresources,
    _In_reads_(NumSubresources) const D3D12_SUBRESOURCE_DATA* pSrcData) noexcept
{
    UINT64 RequiredSize = 0;
    const auto MemToAlloc = static_cast<UINT64>(sizeof(D3D12_PLACED_SUBRESOURCE_FOOTPRINT) + sizeof(UINT) + sizeof(UINT64)) * NumSubresources;
    if (MemToAlloc > SIZE_MAX)
    {
       return 0;
    }
    void* pMem = HeapAlloc(GetProcessHeap(), 0, static_cast<SIZE_T>(MemToAlloc));
    if (pMem == nullptr)
    {
       return 0;
    }
    auto pLayouts = static_cast<D3D12_PLACED_SUBRESOURCE_FOOTPRINT*>(pMem);
    auto pRowSizesInBytes = reinterpret_cast<UINT64*>(pLayouts + NumSubresources);
    auto pNumRows = reinterpret_cast<UINT*>(pRowSizesInBytes + NumSubresources);

    const auto Desc = pDestinationResource->GetDesc();
    ID3D12Device* pDevice = nullptr;
    pDestinationResource->GetDevice(IID_ID3D12Device, reinterpret_cast<void**>(&pDevice));
    pDevice->GetCopyableFootprints(&Desc, FirstSubresource, NumSubresources, IntermediateOffset, pLayouts, pNumRows, pRowSizesInBytes, &RequiredSize);
    pDevice->Release();

    const UINT64 Result = UpdateSubresources(pCmdList, pDestinationResource, pIntermediate, FirstSubresource, NumSubresources, RequiredSize, pLayouts, pNumRows, pRowSizesInBytes, pSrcData);
    HeapFree(GetProcessHeap(), 0, pMem);
    return Result;
}

//------------------------------------------------------------------------------------------------
// Heap-allocating UpdateSubresources implementation
inline UINT64 UpdateSubresources(
    _In_ ID3D12GraphicsCommandList* pCmdList,
    _In_ ID3D12Resource* pDestinationResource,
    _In_ ID3D12Resource* pIntermediate,
    UINT64 IntermediateOffset,
    _In_range_(0,D3D12_REQ_SUBRESOURCES) UINT FirstSubresource,
    _In_range_(0,D3D12_REQ_SUBRESOURCES-FirstSubresource) UINT NumSubresources,
    _In_ const void* pResourceData,
    _In_reads_(NumSubresources) const D3D12_SUBRESOURCE_INFO* pSrcData) noexcept
{
    UINT64 RequiredSize = 0;
    const auto MemToAlloc = static_cast<UINT64>(sizeof(D3D12_PLACED_SUBRESOURCE_FOOTPRINT) + sizeof(UINT) + sizeof(UINT64)) * NumSubresources;
    if (MemToAlloc > SIZE_MAX)
    {
        return 0;
    }
    void* pMem = HeapAlloc(GetProcessHeap(), 0, static_cast<SIZE_T>(MemToAlloc));
    if (pMem == nullptr)
    {
        return 0;
    }
    auto pLayouts = static_cast<D3D12_PLACED_SUBRESOURCE_FOOTPRINT*>(pMem);
    auto pRowSizesInBytes = reinterpret_cast<UINT64*>(pLayouts + NumSubresources);
    auto pNumRows = reinterpret_cast<UINT*>(pRowSizesInBytes + NumSubresources);

    const auto Desc = pDestinationResource->GetDesc();
    ID3D12Device* pDevice = nullptr;
    pDestinationResource->GetDevice(IID_ID3D12Device, reinterpret_cast<void**>(&pDevice));
    pDevice->GetCopyableFootprints(&Desc, FirstSubresource, NumSubresources, IntermediateOffset, pLayouts, pNumRows, pRowSizesInBytes, &RequiredSize);
    pDevice->Release();

    const UINT64 Result = UpdateSubresources(pCmdList, pDestinationResource, pIntermediate, FirstSubresource, NumSubresources, RequiredSize, pLayouts, pNumRows, pRowSizesInBytes, pResourceData, pSrcData);
    HeapFree(GetProcessHeap(), 0, pMem);
    return Result;
}

//------------------------------------------------------------------------------------------------
// Stack-allocating UpdateSubresources implementation
template <UINT MaxSubresources>
inline UINT64 UpdateSubresources(
    _In_ ID3D12GraphicsCommandList* pCmdList,
    _In_ ID3D12Resource* pDestinationResource,
    _In_ ID3D12Resource* pIntermediate,
    UINT64 IntermediateOffset,
    _In_range_(0,MaxSubresources) UINT FirstSubresource,
    _In_range_(1,MaxSubresources-FirstSubresource) UINT NumSubresources,
    _In_reads_(NumSubresources) const D3D12_SUBRESOURCE_DATA* pSrcData) noexcept
{
    UINT64 RequiredSize = 0;
    D3D12_PLACED_SUBRESOURCE_FOOTPRINT Layouts[MaxSubresources];
    UINT NumRows[MaxSubresources];
    UINT64 RowSizesInBytes[MaxSubresources];

    const auto Desc = pDestinationResource->GetDesc();
    ID3D12Device* pDevice = nullptr;
    pDestinationResource->GetDevice(IID_ID3D12Device, reinterpret_cast<void**>(&pDevice));
    pDevice->GetCopyableFootprints(&Desc, FirstSubresource, NumSubresources, IntermediateOffset, Layouts, NumRows, RowSizesInBytes, &RequiredSize);
    pDevice->Release();

    return UpdateSubresources(pCmdList, pDestinationResource, pIntermediate, FirstSubresource, NumSubresources, RequiredSize, Layouts, NumRows, RowSizesInBytes, pSrcData);
}

//------------------------------------------------------------------------------------------------
// Stack-allocating UpdateSubresources implementation
template <UINT MaxSubresources>
inline UINT64 UpdateSubresources(
    _In_ ID3D12GraphicsCommandList* pCmdList,
    _In_ ID3D12Resource* pDestinationResource,
    _In_ ID3D12Resource* pIntermediate,
    UINT64 IntermediateOffset,
    _In_range_(0,MaxSubresources) UINT FirstSubresource,
    _In_range_(1,MaxSubresources-FirstSubresource) UINT NumSubresources,
    _In_ const void* pResourceData,
    _In_reads_(NumSubresources) const D3D12_SUBRESOURCE_INFO* pSrcData) noexcept
{
    UINT64 RequiredSize = 0;
    D3D12_PLACED_SUBRESOURCE_FOOTPRINT Layouts[MaxSubresources];
    UINT NumRows[MaxSubresources];
    UINT64 RowSizesInBytes[MaxSubresources];

    const auto Desc = pDestinationResource->GetDesc();
    ID3D12Device* pDevice = nullptr;
    pDestinationResource->GetDevice(IID_ID3D12Device, reinterpret_cast<void**>(&pDevice));
    pDevice->GetCopyableFootprints(&Desc, FirstSubresource, NumSubresources, IntermediateOffset, Layouts, NumRows, RowSizesInBytes, &RequiredSize);
    pDevice->Release();

    return UpdateSubresources(pCmdList, pDestinationResource, pIntermediate, FirstSubresource, NumSubresources, RequiredSize, Layouts, NumRows, RowSizesInBytes, pResourceData, pSrcData);
}

//------------------------------------------------------------------------------------------------
constexpr bool D3D12IsLayoutOpaque( D3D12_TEXTURE_LAYOUT Layout ) noexcept
{ return Layout == D3D12_TEXTURE_LAYOUT_UNKNOWN || Layout == D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE; }

//------------------------------------------------------------------------------------------------
template <typename t_CommandListType>
inline ID3D12CommandList * const * CommandListCast(t_CommandListType * const * pp) noexcept
{
    // This cast is useful for passing strongly typed command list pointers into
    // ExecuteCommandLists.
    // This cast is valid as long as the const-ness is respected. D3D12 APIs do
    // respect the const-ness of their arguments.
    return reinterpret_cast<ID3D12CommandList * const *>(pp);
}


inline D3D12_PIPELINE_STATE_SUBOBJECT_TYPE D3DX12GetBaseSubobjectType(D3D12_PIPELINE_STATE_SUBOBJECT_TYPE SubobjectType) noexcept
{
    switch (SubobjectType)
    {
    case D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1:
        return D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL;
    default:
        return SubobjectType;
    }
}

*/

#[derive(Copy, Clone, Debug)]
pub struct D3D12CopyableFootprint {
    pub layout: D3D12PlacedSubresourceFootprint,
    pub num_row: u32,
    pub row_size_in_bytes: u64,
    pub total_bytes: u64,
}

impl D3D12Resource {
    pub fn map(&self, subresource: u32, read_range: Option<&D3D12Range>) -> Result<NonNull<()>, HResult> {
        let mut out: Option<NonNull<()>> = None;
        self.Map(subresource, read_range, Some(&mut out))?;
        if let Some(out) = out {
            return Ok(out);
        }
        Err(HResult::E_FAIL)
    }
}

impl D3D12GraphicsCommandList {
    pub fn resource_barrier_transition_all(
        &self,
        resource: &D3D12Resource,
        before: D3D12ResourceStates,
        after: D3D12ResourceStates,
    ) {
        self.ResourceBarrier(&[
            D3D12ResourceBarrier::Transition(resource, D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES, before, after)])
    }

    pub fn resource_barrier_uav(&self, resource: &D3D12Resource) {
        self.ResourceBarrier(&[D3D12ResourceBarrier::Uav(resource)])
    }
}

impl D3D12CommandQueue {
    pub fn execute_command_list(&self, command_list: &D3D12CommandList) -> () {
        self.ExecuteCommandLists(&[command_list.as_param()])
    }
}

impl D3D12Device {
    pub fn create_root_sig(&self, blob: &[u8]) -> Result<D3D12RootSignature, HResult> {
        self.CreateRootSignature(0, blob)
    }

    pub fn enable_break_on_error(&self) -> Result<(), HResult> {
        self.QueryInterface::<D3D12InfoQueue>()?.enable_break_on_error()
    }

    pub fn enable_break_on_corruption(&self) -> Result<(), HResult> {
        self.QueryInterface::<D3D12InfoQueue>()?.enable_break_on_corruption()
    }

    pub fn create_desc_heap_rtv<U: ID3D12DescriptorHeap + From<UnknownWrapper>>(&self, count: u32) -> Result<U, HResult> {
        self.CreateDescriptorHeap(&D3D12DescriptorHeapDesc::Rtv(count))
    }

    pub fn create_desc_heap_sampler<U: ID3D12DescriptorHeap + From<UnknownWrapper>>(&self, count: u32, shader_visible: bool) -> Result<U, HResult> {
        self.CreateDescriptorHeap(&D3D12DescriptorHeapDesc::Sampler(count).with_shader_visible(shader_visible))
    }

    pub fn create_desc_heap_dsv<U: ID3D12DescriptorHeap + From<UnknownWrapper>>(&self, count: u32) -> Result<U, HResult> {
        self.CreateDescriptorHeap(&D3D12DescriptorHeapDesc::Dsv(count))
    }

    pub fn create_desc_heap_cbv_srv_uav<U: ID3D12DescriptorHeap + From<UnknownWrapper>>(&self, count: u32, shader_visible: bool) -> Result<U, HResult> {
        self.CreateDescriptorHeap(&D3D12DescriptorHeapDesc::CbvSrvUav(count).with_shader_visible(shader_visible))
    }
}

pub type D3D12ShaderIdentifier = [u8; D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES as usize];

impl D3D12StateObjectProperties {
    pub fn get_shader_identifier(&self, export_name: &str) -> Option<&D3D12ShaderIdentifier> {
        unsafe { transmute(self.GetShaderIdentifier(export_name)) }
    }
}

impl D3D12Device5 {
    pub fn create_state_object(&self, obj_type: D3D12StateObjectType, objs: &[D3D12StateSubobject]) -> Result<D3D12StateObject, HResult> {
        self.CreateStateObject(&D3D12StateObjectDesc::new(obj_type, objs))
    }
}

pub struct D3D12Message {
    pub category: D3D12MessageCategory,
    pub severity: D3D12MessageSeverity,
    pub id: D3D12MessageId,
    pub description: String,
}

impl D3D12InfoQueue {
    // 5 HRESULT GetMessage([In] ulong MessageIndex, [MemorySize(BytesParamIndex = 2), Out, Optional] D3D12_MESSAGE* pMessage, [In, Out] UIntPtr* pMessageByteLength);
    // 13 HRESULT GetStorageFilter([MemorySize(BytesParamIndex = 1), Out, Optional] D3D12_INFO_QUEUE_FILTER* pFilter, [In, Out] UIntPtr* pFilterByteLength);
    // 21 HRESULT GetRetrievalFilter([MemorySize(BytesParamIndex = 1), Out, Optional] D3D12_INFO_QUEUE_FILTER* pFilter, [In, Out] UIntPtr* pFilterByteLength);

    pub fn get_message(&self, index: u64) -> Result<D3D12Message, HResult> {
        let vt = self.as_param();
        let f: extern "system" fn(Param<Self>, message_index: u64, p_message: *mut u8, p_message_byte_length: &mut usize) -> HResult
            = unsafe { transmute(vt[5]) };

        let mut length: usize = 0;
        f(vt, index, null_mut(), &mut length).ok()?;

        let mut vec = Vec::<u8>::with_capacity(length);
        let vec = vec.as_mut_ptr();
        f(vt, index, vec, &mut length).ok()?;

        let msg: &RawD3D12Message = unsafe { transmute(vec) };
        let size = msg.description_byte_length;
        let size = if size > 0 { size - 1 } else { 0 };
        let ptr: *const u8 = unsafe { transmute(msg.description) };
        let desc = String::from_utf8_lossy(unsafe { slice::from_raw_parts(ptr, size) }).to_string();

        Ok(D3D12Message {
            category: msg.category,
            severity: msg.severity,
            id: msg.id,
            description: desc,
        })
    }

    pub fn enable_break_on_error(&self) -> Result<(), HResult> {
        self.SetBreakOnSeverity(D3D12MessageSeverity::Error, true)
    }

    pub fn enable_break_on_corruption(&self) -> Result<(), HResult> {
        self.SetBreakOnSeverity(D3D12MessageSeverity::Corruption, true)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12RaytracingInstanceDesc {
    pub transform: [[f32; 4]; 3],
    pub bitfield1: u32,
    pub bitfield2: u32,
    pub acceleration_structure: GpuVirtualAddress,
}

impl D3D12RaytracingInstanceDesc {
    pub fn new(
        transform: &[[f32; 4]; 3],
        instance_id: u32,
        instance_mask: u8,
        hit_group_address_index: u32,
        flags: D3D12RaytracingInstanceFlags,
        acceleration_structure: GpuVirtualAddress,
    ) -> Self {
        if instance_id > 0xFFFFFF {
            panic!("instance_id should be less than 0x1000000");
        }
        if hit_group_address_index > 0xFFFFFF {
            panic!("instance_contribution_to_hit_group_index should be less than 0x1000000");
        }
        Self {
            transform: *transform,
            bitfield1: ((instance_mask as u32) << 24) | instance_id,
            bitfield2: ((flags.bits() as u32) << 24) | hit_group_address_index,
            acceleration_structure,
        }
    }
}