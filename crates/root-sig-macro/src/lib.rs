use proc_macro::TokenStream;
use std::fmt::{Debug, Display, Formatter};
use std::mem::transmute;
use std::ops::{BitOrAssign, Deref};
use std::str::FromStr;
use proc_macro2::Group;
use quote::{quote, ToTokens};
use syn::*;
use syn::parse::*;
use syn::spanned::Spanned;
use d3d12_api_core::aliases::win32::graphics::direct3d12::*;
use d3d12_api_core::core::win32::graphics::direct3d12::{D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND, D3D12SerializeVersionedRootSignature, D3DRootSignatureVersion};

// todo: registerが先頭でなくても良いように
// todo: Err(input.error("")) を消せ

fn ptr_with_len<T>(value: &[T]) -> (Option<&T>, usize) {
    (value.first(), value.len())
}

struct RootSig {
    root_flags: RootSignatureFlags,
    root_params: Vec<RootParam>,
    static_samplers: Vec<StaticSamplerDesc>,
}

enum RootParam {
    DescriptorTable(DescTableArgs),
    RootConstants(RootConstantsArgs),
    Descriptor(DescriptorArgs),
}

struct RootFlagsArgs {
    raw: RootSignatureFlags,
}

impl Parse for RootFlagsArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        //
        if input.is_empty() {
            return Err(input.error("expected root flags arguments"));
        }
        let mut flags = RootSignatureFlags::None;

        while !input.is_empty() {
            let id = input.parse::<Ident>()?;
            flags = flags | match id.to_string().deref() {
                "None" => RootSignatureFlags::None,
                "AllowInputAssemblerInputLayout" => RootSignatureFlags::AllowInputAssemblerInputLayout,
                "DenyVertexShaderRootAccess" => RootSignatureFlags::DenyVertexShaderRootAccess,
                "DenyHullShaderRootAccess" => RootSignatureFlags::DenyHullShaderRootAccess,
                "DenyDomainShaderRootAccess" => RootSignatureFlags::DenyDomainShaderRootAccess,
                "DenyGeometryShaderRootAccess" => RootSignatureFlags::DenyGeometryShaderRootAccess,
                "DenyPixelShaderRootAccess" => RootSignatureFlags::DenyPixelShaderRootAccess,
                "AllowStreamOutput" => RootSignatureFlags::AllowStreamOutput,
                "LocalRootSignature" => RootSignatureFlags::LocalRootSignature,
                "DenyAmplificationShaderRootAccess" => RootSignatureFlags::DenyAmplificationShaderRootAccess,
                "DenyMeshShaderRootAccess" => RootSignatureFlags::DenyMeshShaderRootAccess,
                "CbvSrvUavHeapDirectlyIndexed" => RootSignatureFlags::CbvSrvUavHeapDirectlyIndexed,
                "SamplerHeapDirectlyIndexed" => RootSignatureFlags::SamplerHeapDirectlyIndexed,
                _ => {
                    return Err(Error::new(id.span(), "invalid root flag argument"));
                }
            };
            if input.is_empty() { break; }
            if input.peek(Token![|]) {
                input.parse::<Token![|]>()?;
                continue;
            } else if !input.is_empty() {
                return Err(input.error("expected a delimiter \"|\""));
            }
        }

        Ok(Self { raw: flags })
    }
}

struct DescriptorArgs {
    ty: RootParameterType,
    raw: RootDescriptor1,
    shader_visibility: ShaderVisibility,
}

impl DescriptorArgs {
    fn parse(ty: RootParameterType, input: ParseStream) -> Result<Self> {
        //
        let mut result = Self {
            ty,
            raw: RootDescriptor1 {
                shader_register: 0,
                register_space: 0,
                flags: RootDescriptorFlags::None,
            },
            shader_visibility: ShaderVisibility::All,
        };

        let id = input.parse::<Ident>()?;
        let id_str = id.to_string();
        {
            let mut chars = id_str.chars();
            let expect = match ty {
                // RootParameterType::DescriptorTable => {}
                // RootParameterType::_32BitConstants => {}
                RootParameterType::Cbv => 'b',
                RootParameterType::Srv => 't',
                RootParameterType::Uav => 'u',
                _ => { unreachable!() }
            };
            let first = chars.next().unwrap();
            if first != expect {
                // todo: expected 変じゃね？
                return Err(Error::new(id.span(), format!("expected {}, but get {}", expect, first)));
            }
        }

        let shader_register_str = id_str.get(1..).expect("expected register number");
        result.raw.shader_register = shader_register_str.parse().expect("expected register number");

        while !input.is_empty() {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else if !input.is_empty() {
                return Err(input.error("expected a delimiter \",\""));
            }

            let id = input.parse::<Ident>()?;

            match id.to_string().deref() {
                "space" => {
                    result.raw.register_space = parse_num(input)?;
                }
                "flags" => {
                    todo!();
                }
                "visibility" => {
                    result.shader_visibility = parse_visibility(input)?;
                }
                _ => {
                    return Err(Error::new(id.span(), "invalid identifier"));
                }
            }
        }

        Ok(result)
    }
}

struct CbvArgs(DescriptorArgs);

struct SrvArgs(DescriptorArgs);

struct UavArgs(DescriptorArgs);

impl Parse for CbvArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(DescriptorArgs::parse(RootParameterType::Cbv, input)?))
    }
}

impl Parse for SrvArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(DescriptorArgs::parse(RootParameterType::Srv, input)?))
    }
}

impl Parse for UavArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(DescriptorArgs::parse(RootParameterType::Uav, input)?))
    }
}

struct DescRangeArgs {
    raw: DescriptorRange1,
}

impl DescRangeArgs {
    fn parse(ty: DescriptorRangeType, input: ParseStream) -> Result<Self> {
        //
        let mut result = Self {
            raw: DescriptorRange1 {
                range_type: ty,
                num_descriptors: 0,
                base_shader_register: 0,
                register_space: 0,
                flags: DescriptorRangeFlags::None,
                offset_in_descriptors_from_table_start: D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
            }
        };

        let id = input.parse::<Ident>()?;
        let id_str = id.to_string();
        {
            let mut chars = id_str.chars();
            let expect = match ty {
                DescriptorRangeType::Srv => 't',
                DescriptorRangeType::Uav => 'u',
                DescriptorRangeType::Cbv => 'b',
                DescriptorRangeType::Sampler => { todo!() }
            };
            let first = chars.next().unwrap();
            if first != expect {
                // todo: expected 変じゃね？
                return Err(Error::new(id.span(), format!("expected {}, but get {}", expect, first)));
            }
        }

        let shader_register_str = id_str.get(1..).expect("expected register number");
        result.raw.base_shader_register = shader_register_str.parse().expect("expected register number");

        while !input.is_empty() {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else if !input.is_empty() {
                return Err(input.error("expected a delimiter \",\""));
            }

            let id = input.parse::<Ident>()?;
            let id_str = id.to_string();

            match id_str.deref() {
                "num_descriptors" => {
                    result.raw.num_descriptors = parse_num(input)?;
                }
                "space" => {
                    result.raw.register_space = parse_num(input)?;
                }
                "flags" => {
                    todo!();
                }
                "offset" => {
                    todo!();
                }
                _ => {
                    return Err(Error::new(id.span(), "invalid identifier"));
                }
            }
        }

        if result.raw.num_descriptors == 0 {
            return Err(Error::new(input.span(), "num_descriptors must be set to a value greater than 0"));
        }

        Ok(result)
    }
}

struct CbvDescRangeArgs(DescRangeArgs);

struct SrvDescRangeArgs(DescRangeArgs);

struct UavDescRangeArgs(DescRangeArgs);

struct SamplerDescRangeArgs(DescRangeArgs);

impl Parse for CbvDescRangeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(DescRangeArgs::parse(DescriptorRangeType::Cbv, input)?))
    }
}

impl Parse for SrvDescRangeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(DescRangeArgs::parse(DescriptorRangeType::Srv, input)?))
    }
}

impl Parse for UavDescRangeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(DescRangeArgs::parse(DescriptorRangeType::Uav, input)?))
    }
}

impl Parse for SamplerDescRangeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(DescRangeArgs::parse(DescriptorRangeType::Sampler, input)?))
    }
}

struct DescTableArgs {
    raw: Vec<DescriptorRange1>,
    shader_visibility: ShaderVisibility,
}

impl Parse for DescTableArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        //
        if input.is_empty() { return Err(input.error("a")); }

        let mut ranges = Vec::<DescriptorRange1>::new();
        let mut shader_visibility = ShaderVisibility::All;

        while !input.is_empty() {
            let id = input.parse::<Ident>()?;
            let id_str = id.to_string();

            if id_str.deref() == "visibility" {
                shader_visibility = parse_visibility(input)?;
            } else {
                let args = proc_macro::TokenStream::from(input.parse::<Group>()?.stream());
                let range = match id_str.deref() {
                    "CBV" | "Cbv" => parse_macro_input::parse::<CbvDescRangeArgs>(args)?.0.raw,
                    "SRV" | "Srv" => parse_macro_input::parse::<SrvDescRangeArgs>(args)?.0.raw,
                    "UAV" | "Uav" => parse_macro_input::parse::<UavDescRangeArgs>(args)?.0.raw,
                    "Sampler" => parse_macro_input::parse::<SamplerDescRangeArgs>(args)?.0.raw,
                    _ => {
                        return Err(Error::new(id.span(), "invalid descriptor range name"));
                    }
                };
                ranges.push(range);
            }

            if input.is_empty() { break; }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                continue;
            } else if !input.is_empty() {
                return Err(input.error("expected a delimiter \",\""));
            }
        }

        Ok(Self {
            raw: ranges,
            shader_visibility,
        })
    }
}

struct RootConstantsArgs {
    raw: RootConstants,
    shader_visibility: ShaderVisibility,
}

impl Parse for RootConstantsArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        //
        let mut result = Self {
            raw: RootConstants {
                shader_register: 0,
                register_space: 0,
                num_32bit_values: 0,
            },
            shader_visibility: ShaderVisibility::All,
        };

        while !input.is_empty() {
            let id = input.parse::<Ident>()?;

            match id.to_string().deref() {
                "space" => {
                    result.raw.register_space = parse_num(input)?;
                }
                "num_32bit_constants" => {
                    result.raw.num_32bit_values = parse_num(input)?;
                }
                "visibility" => {
                    result.shader_visibility = parse_visibility(input)?;
                }
                tmp @ _ if Some('b') == tmp.chars().next() => {
                    let shader_register_str = tmp.get(1..).expect("expected register number");
                    result.raw.shader_register = shader_register_str.parse().expect("expected register number");
                }
                _ => {
                    return Err(Error::new(id.span(), "invalid identifier"));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                continue;
            } else if !input.is_empty() {
                return Err(input.error("expected a delimiter \",\""));
            }
        }

        if result.raw.num_32bit_values == 0 {
            return Err(Error::new(input.span(), "num_32bit_values must be set to a value greater than 0"));
        }

        Ok(result)
    }
}

struct StaticSamplerArgs {
    raw: StaticSamplerDesc,
}

impl Parse for StaticSamplerArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!();
        Ok(Self {
            raw: StaticSamplerDesc {
                filter: Filter::MinMagMipPoint,
                address_u: TextureAddressMode::Wrap,
                address_v: TextureAddressMode::Wrap,
                address_w: TextureAddressMode::Wrap,
                mip_lod_bias: 0.0,
                max_anisotropy: 0,
                comparison_func: ComparisonFunc::Never,
                border_color: StaticBorderColor::TransparentBlack,
                min_lod: 0.0,
                max_lod: 0.0,
                shader_register: 0,
                register_space: 0,
                shader_visibility: ShaderVisibility::All,
            }
        })
    }
}

fn parse_num(input: ParseStream) -> Result<u32> {
    input.parse::<Token![:]>()?;
    Ok(input.parse::<LitInt>()?.base10_digits().parse()
        .expect("integer number expected"))
}

fn parse_visibility(input: ParseStream) -> Result<ShaderVisibility> {
    input.parse::<Token![:]>()?;
    let id = input.parse::<Ident>()?;
    let visibility = match id.to_string().deref() {
        "All" => ShaderVisibility::All,
        "Vertex" => ShaderVisibility::Vertex,
        "Hull" => ShaderVisibility::Hull,
        "Domain" => ShaderVisibility::Domain,
        "Geometry" => ShaderVisibility::Geometry,
        "Pixel" => ShaderVisibility::Pixel,
        "Amplification" => ShaderVisibility::Amplification,
        "Mesh" => ShaderVisibility::Mesh,
        _ => {
            return Err(Error::new(id.span(), "unknown shader visibility"));
        }
    };
    Ok(visibility)
}

// note: ParseStream::parse_terminated() というものもあるがいまいち融通が利かない
impl Parse for RootSig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        //
        let mut root_flags_defined = false;
        let mut root_flags = RootSignatureFlags::None;

        let mut root_params = Vec::<RootParam>::new();
        let mut static_samplers = Vec::<StaticSamplerDesc>::new();

        while !input.is_empty() {
            let id = input.parse::<Ident>()?;
            let args = proc_macro::TokenStream::from(input.parse::<Group>()?.stream());

            match id.to_string().deref() {
                "RootFlags" => {
                    if root_flags_defined {
                        return Err(syn::Error::new(id.span(), "RootFlags is defined multiple times"));
                    }
                    root_flags_defined = true;
                    root_flags = parse_macro_input::parse::<RootFlagsArgs>(args)?.raw;
                }
                "CBV" | "Cbv" => {
                    let args = parse_macro_input::parse::<CbvArgs>(args)?;
                    root_params.push(RootParam::Descriptor(args.0));
                }
                "SRV" | "Srv" => {
                    let args = parse_macro_input::parse::<SrvArgs>(args)?;
                    root_params.push(RootParam::Descriptor(args.0));
                }
                "UAV" | "Uav" => {
                    let args = parse_macro_input::parse::<UavArgs>(args)?;
                    root_params.push(RootParam::Descriptor(args.0));
                }
                "DescriptorTable" => {
                    let args = parse_macro_input::parse::<DescTableArgs>(args)?;
                    root_params.push(RootParam::DescriptorTable(args));
                }
                "RootConstants" => {
                    let args = parse_macro_input::parse::<RootConstantsArgs>(args)?;
                    root_params.push(RootParam::RootConstants(args));
                }
                "StaticSampler" => {
                    let args = parse_macro_input::parse::<StaticSamplerArgs>(args)?;
                    static_samplers.push(args.raw);
                }
                _ => {
                    return Err(Error::new(id.span(), "invalid root parameter name"));
                }
            }

            if input.is_empty() { break; }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                continue;
            } else if !input.is_empty() {
                return Err(input.error("expected a delimiter \",\""));
            }
        }

        Ok(RootSig {
            root_flags,
            root_params,
            static_samplers,
        })
    }
}

#[proc_macro]
pub fn hlsl_root_sig_core(args: TokenStream) -> TokenStream {
    let RootSig {
        root_flags,
        root_params,
        static_samplers,
    } = parse_macro_input!(args as RootSig);

    let mut root_params_ = Vec::<RootParameter1>::new();

    for root_param in root_params {
        match root_param {
            // note: descriptor tableがライフタイムを持っており面倒くさいので全部型を作り直した。
            RootParam::DescriptorTable(args) => {
                root_params_.push(RootParameter1 {
                    parameter_type: RootParameterType::DescriptorTable,
                    anonymous: RootParameter1AnonymousUnion {
                        descriptor_table: RootDescriptorTable1 {
                            num_descriptor_ranges: args.raw.len() as u32,
                            descriptor_ranges: unsafe { transmute(args.raw.first().unwrap()) },
                        }
                    },
                    shader_visibility: args.shader_visibility,
                })
            }
            RootParam::RootConstants(args) => {
                root_params_.push(RootParameter1 {
                    parameter_type: RootParameterType::_32BitConstants,
                    anonymous: RootParameter1AnonymousUnion {
                        constants: args.raw
                    },
                    shader_visibility: args.shader_visibility,
                })
            }
            RootParam::Descriptor(args) => {
                root_params_.push(RootParameter1 {
                    parameter_type: args.ty,
                    anonymous: RootParameter1AnonymousUnion {
                        descriptor: args.raw
                    },
                    shader_visibility: args.shader_visibility,
                })
            }
        }
    }

    let (rp_ptr, rp_len) = ptr_with_len(root_params_.deref());
    let (ss_ptr, ss_len) = ptr_with_len(static_samplers.deref());

    let desc = VersionedRootSignatureDesc {
        version: D3DRootSignatureVersion::_1_1,
        anonymous: VersionedRootSignatureDescAnonymousUnion {
            desc_1_1: RootSignatureDesc1 {
                num_parameters: rp_len as u32,
                parameters: rp_ptr,
                num_static_samplers: ss_len as u32,
                static_samplers: ss_ptr,
                flags: root_flags,
            }
        },
    };
    let blob = D3D12SerializeVersionedRootSignature(&desc, None).unwrap();
    let lit = format!("&{:?}", blob.as_slice());
    TokenStream::from_str(&*lit).unwrap()
}