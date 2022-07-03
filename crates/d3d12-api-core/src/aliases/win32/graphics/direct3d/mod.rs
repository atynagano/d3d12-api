pub mod dxc;

use crate::core::win32::graphics::direct3d::*;

pub type FeatureLevel = D3DFeatureLevel;
pub type Name = D3DName;
pub type RegisterComponentType = D3DRegisterComponentType;
pub type MinPrecision = D3DMinPrecision;
pub type CBufferType = D3DCBufferType;
pub type ShaderVariableClass = D3DShaderVariableClass;
pub type ShaderVariableType = D3DShaderVariableType;
pub type PrimitiveTopology = D3DPrimitiveTopology;
pub type Primitive = D3DPrimitive;
pub type TessellatorOutputPrimitive = D3DTessellatorOutputPrimitive;
pub type TessellatorPartitioning = D3DTessellatorPartitioning;
pub type TessellatorDomain = D3DTessellatorDomain;
pub type ShaderInputType = D3DShaderInputType;
pub type ResourceReturnType = D3DResourceReturnType;
pub type SrvDimension = D3DSrvDimension;
pub type InterpolationMode = D3DInterpolationMode;
pub type ParameterFlags = D3DParameterFlags;
pub type Blob = D3DBlob;
pub trait IBlob: ID3DBlob {}
impl<T: ID3DBlob> IBlob for T {}
