use std::ffi::c_void;
use std::fmt::Debug;
use std::marker::PhantomPinned;
use std::mem::{transmute};
use std::ops::{Deref, DerefMut};
use std::pin::{Pin};
use std::ptr::{NonNull, null};
use crate::aliases::win32::graphics::direct3d12::{
    DxilLibraryDesc,
    DxilSubobjectToExportsAssociation,
    ExistingCollectionDesc,
    ExportFlags,
    GlobalRootSignature,
    HitGroupDesc,
    HitGroupType,
    IDevice5,
    IRootSignature,
    LocalRootSignature,
    NodeMask,
    RaytracingPipelineConfig,
    RaytracingPipelineConfig1,
    RaytracingShaderConfig,
    ShaderByteCode,
    StateObject,
    StateObjectConfig,
    StateObjectType,
    StateSubobject,
    StateSubobjectType,
    SubobjectToExportsAssociation,
};
use crate::core::win32::graphics::direct3d12::D3D12ExportDesc;
use crate::extensions::win32::graphics::direct3d12::ID3D12Device5Ex;
use crate::core::win32::foundation::{AsPWStr, HResult, PWStr};
use crate::core::win32::system::com::AsParam;

/// make state subobject data and their dependencies keeping alive
pub enum StateSubobjectData<'a> {
    StateObjectConfig(Box<StateObjectConfig>),
    GlobalRootSignature(Box<GlobalRootSignature<'a>>),
    LocalRootSignature(Box<LocalRootSignature<'a>>),
    NodeMask(Box<NodeMask>),
    DxilLibrary(Box<DxilLibraryDesc<'a>>),
    ExistingCollection(Box<ExistingCollectionDesc<'a>>),
    SubobjectToExportsAssociation(Box<SubobjectToExportsAssociation<'a>>),
    DxilSubobjectToExportsAssociation(Box<DxilSubobjectToExportsAssociation<'a>>),
    RaytracingShaderConfig(Box<RaytracingShaderConfig>),
    RaytracingPipelineConfig(Box<RaytracingPipelineConfig>),
    HitGroup(Box<HitGroupDesc<'a>>),
    RaytracingPipelineConfig1(Box<RaytracingPipelineConfig1>),
    //MaxValid(MaxValidDesc),
    Export(Vec<D3D12ExportDesc<'a>>),
    Vec(Vec<Vec<u16>>),
    PWStr(Vec<PWStr<'a>>),
}

#[repr(C)]
// #[derive(Copy, Clone, Debug)] note: ダメ
pub struct StateSubobjectWrapper<'a> {
    // note: private。勝手にインスタンス化されては困る
    raw: StateSubobject<'a>,
    location: *const StateSubobject<'a>,
}

impl StateSubobjectWrapper<'_> {
    fn new<T>(obj_type: StateSubobjectType, desc: &T) -> Self {
        Self {
            raw: StateSubobject {
                ty: obj_type,
                desc: unsafe { transmute(desc) },
            },
            location: null(),
        }
    }
}

// todo: StateObjectDescではかぶる
pub struct StateObjectDesc<'a> {
    objects: Vec<Box<StateSubobjectWrapper<'a>>>,
    descs: Vec<StateSubobjectData<'a>>,
}

pub struct ExportDesc<'a> {
    pub name: &'a str,
    pub export_to_rename: Option<&'a str>,
    pub flags: ExportFlags,
}

impl<'a> ExportDesc<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            export_to_rename: None,
            flags: ExportFlags::None,
        }
    }
}

impl<'a> StateObjectDesc<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            descs: Vec::new(),
        }
    }

    pub fn push_state_object_config(&mut self) -> &'a StateSubobjectWrapper<'a> {
        todo!()
    }

    pub fn push_global_root_sig(&mut self, root_sig: &'a impl IRootSignature) -> &'a StateSubobjectWrapper<'a> {
        let desc = Box::new(GlobalRootSignature { global_root_signature: root_sig.as_root_signature().as_param() });
        let obj = Box::new(StateSubobjectWrapper::new(StateSubobjectType::GlobalRootSignature, &*desc));
        let ptr = obj.deref() as *const StateSubobjectWrapper;
        self.descs.push(StateSubobjectData::GlobalRootSignature(desc));
        self.objects.push(obj);
        unsafe { &*ptr }
    }

    pub fn push_local_root_sig(&mut self, root_sig: &'a impl IRootSignature) -> &'a StateSubobjectWrapper<'a> {
        let desc = Box::new(LocalRootSignature { local_root_signature: root_sig.as_root_signature().as_param() });
        // note: Boxにしないと戻り値のptrを固定できない。
        let obj = Box::new(StateSubobjectWrapper::new(StateSubobjectType::LocalRootSignature, &*desc));
        let ptr = obj.deref() as *const StateSubobjectWrapper;
        self.descs.push(StateSubobjectData::LocalRootSignature(desc));
        self.objects.push(obj);
        unsafe { &*ptr }
    }

    pub fn push_node_mask(&mut self) -> &'a StateSubobjectWrapper<'a> {
        todo!()
    }

    pub fn push_dxil_lib(&mut self, shader: &'a [u8], exports: &[ExportDesc]) -> &'a StateSubobjectWrapper<'a> {
        // note: Vecがmutでは困るのだ。x2しないと。
        let mut vec = Vec::with_capacity(exports.len() * 2);
        let mut exports_ = Vec::with_capacity(exports.len());
        for e in exports {
            let tmp = e.name.encode_utf16().chain([0u16]).collect::<Vec<_>>();
            let name = unsafe { transmute(tmp.as_pwstr()) };
            vec.push(tmp);

            let etr = if let Some(ex) = e.export_to_rename {
                let tmp = ex.encode_utf16().chain([0u16]).collect::<Vec<_>>();
                let tmp2 = unsafe { transmute(tmp.as_pwstr()) };
                vec.push(tmp);
                Some(tmp2)
            } else {
                None
            };

            exports_.push(D3D12ExportDesc {
                name,
                export_to_rename: etr,
                flags: e.flags,
            })
        }
        let exports = exports_;

        let desc = DxilLibraryDesc {
            dxil_library: ShaderByteCode::new(shader),
            num_exports: exports.len() as u32,
            exports: unsafe { transmute(exports.first().unwrap()) },
        };
        let desc = Box::new(desc);
        let obj = Box::new(StateSubobjectWrapper::new(StateSubobjectType::DxilLibrary, &*desc));
        let ptr = obj.deref() as *const StateSubobjectWrapper;
        self.descs.push(StateSubobjectData::DxilLibrary(desc));
        self.descs.push(StateSubobjectData::Vec(vec));
        self.descs.push(StateSubobjectData::Export(exports));
        self.objects.push(obj);
        unsafe { &*ptr }
    }

    pub fn push_existing_collection(&mut self) -> &'a StateSubobjectWrapper<'a> {
        todo!()
    }

    pub fn push_subobject_to_exports_assoc(&mut self, obj: &'a StateSubobjectWrapper, exports: &[&str]) -> &'a StateSubobjectWrapper<'a> {
        let vec = exports
            .into_iter()
            .map(|&x| x.encode_utf16().chain([0u16]).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let exports = vec
            .iter()
            .map(|x| unsafe { transmute(x.as_pwstr()) })
            .collect::<Vec<_>>();

        let desc = Box::new(SubobjectToExportsAssociation {
            subobject_to_associate: unsafe { transmute(obj) },
            num_exports: exports.len() as u32,
            exports: unsafe { transmute(exports.first().unwrap()) },
        });
        let obj = Box::new(StateSubobjectWrapper::new(StateSubobjectType::SubobjectToExportsAssociation, &*desc));
        let ptr = obj.deref() as *const StateSubobjectWrapper;
        self.descs.push(StateSubobjectData::SubobjectToExportsAssociation(desc));
        self.descs.push(StateSubobjectData::Vec(vec));
        self.descs.push(StateSubobjectData::PWStr(exports));
        self.objects.push(obj);
        unsafe { &*ptr }
    }

    pub fn push_rt_shader_config(&mut self, payload: u32, attr: u32) -> &'a StateSubobjectWrapper<'a> {
        let desc = Box::new(RaytracingShaderConfig {
            max_payload_size_in_bytes: payload,
            max_attribute_size_in_bytes: attr,
        });
        let obj = Box::new(StateSubobjectWrapper::new(StateSubobjectType::RaytracingShaderConfig, &*desc));
        let ptr = obj.deref() as *const StateSubobjectWrapper;
        self.descs.push(StateSubobjectData::RaytracingShaderConfig(desc));
        self.objects.push(obj);
        unsafe { &*ptr }
    }

    pub fn push_rt_pipeline_config(&mut self, max_trace_recursion_depth: u32) -> &'a StateSubobjectWrapper<'a> {
        if max_trace_recursion_depth == 0 {
            panic!("max_trace_recursion_depth should be greater than 0");
        }
        let desc = Box::new(RaytracingPipelineConfig {
            max_trace_recursion_depth
        });
        let obj = Box::new(StateSubobjectWrapper::new(StateSubobjectType::RaytracingPipelineConfig, &*desc));
        let ptr = obj.deref() as *const StateSubobjectWrapper;
        self.descs.push(StateSubobjectData::RaytracingPipelineConfig(desc));
        self.objects.push(obj);
        unsafe { &*ptr }
    }

    pub fn push_hit_group(
        &mut self,
        hit_group_export: &str,
        ty: HitGroupType,
        any_hit_import: Option<&str>,
        closest_hit_import: &str,
        intersection_import: Option<&str>,
    ) -> &'a StateSubobjectWrapper<'a> {
        let mut vec = Vec::with_capacity(4);

        let hge = hit_group_export.encode_utf16().chain([0u16]).collect::<Vec<_>>();
        let phge = unsafe { transmute(hge.as_pwstr()) };
        vec.push(hge);

        let pahi = if let Some(ahi) = any_hit_import {
            let ahi = ahi.encode_utf16().chain([0u16]).collect::<Vec<_>>();
            let pahi = unsafe { transmute(ahi.as_pwstr()) };
            vec.push(ahi);
            Some(pahi)
        } else {
            None
        };

        let chi = closest_hit_import.encode_utf16().chain([0u16]).collect::<Vec<_>>();
        let pchi = unsafe { transmute(chi.as_pwstr()) };
        vec.push(chi);

        let pii = if let Some(ii) = intersection_import {
            let ii = ii.encode_utf16().chain([0u16]).collect::<Vec<_>>();
            let pii = unsafe { transmute(ii.as_pwstr()) };
            vec.push(ii);
            Some(pii)
        } else {
            None
        };

        let desc = Box::new(HitGroupDesc {
            hit_group_export: phge,
            ty,
            any_hit_shader_import: pahi,
            closest_hit_shader_import: pchi,
            intersection_shader_import: pii,
        });

        let obj = Box::new(StateSubobjectWrapper::new(StateSubobjectType::HitGroup, &*desc));
        let ptr = obj.deref() as *const StateSubobjectWrapper;
        self.descs.push(StateSubobjectData::HitGroup(desc));
        self.descs.push(StateSubobjectData::Vec(vec));
        self.objects.push(obj);
        unsafe { &*ptr }
    }

    pub fn push_rt_pipeline_config1(&mut self) -> &'a StateSubobjectWrapper<'a> {
        todo!()
    }

    pub fn push_max_valid(&mut self) -> &'a StateSubobjectWrapper<'a> {
        todo!()
    }

    pub fn create_state_object(&mut self, device: &impl IDevice5, obj_type: StateObjectType) -> Result<StateObject, HResult> {
        // note: 慎重に。with_capacityでないとreallocされてしまいlocationが変わってしまう。
        // note: https://github.com/microsoft/DirectX-Headers/blob/9f23d202a27adb285f53398a1850b5a377cd782e/include/directx/d3dx12.h#L3323
        let mut objects = Vec::with_capacity(self.objects.len());
        for obj in &mut self.objects {
            objects.push(obj.raw);
            obj.location = objects.last().unwrap();
        }
        for obj in &mut objects {
            unsafe {
                if obj.ty == StateSubobjectType::SubobjectToExportsAssociation {
                    unsafe {
                        let desc: *mut SubobjectToExportsAssociation = transmute(obj.desc);
                        let wrapper: &StateSubobjectWrapper = transmute((*desc).subobject_to_associate);
                        (*desc).subobject_to_associate = transmute(wrapper.location);
                    }
                }
            }
        }
        device.create_state_object(obj_type, objects.as_slice())
    }

    pub fn dump(&self) {
        fn search_index(objects: &[Box<StateSubobjectWrapper>], target: *const StateSubobjectWrapper) -> isize {
            for (i, obj) in objects.iter().enumerate() {
                if target == obj.deref() as *const StateSubobjectWrapper {
                    return i as isize;
                }
            }
            //-1
            panic!("Associated subobject for SubobjectToExportsAssociation not found")
        }

        println!("\n| D3D12 State Object: {{ len: {} }}", self.objects.len());
        for (i, obj) in self.objects.iter().enumerate() {
            match obj.raw.ty {
                StateSubobjectType::StateObjectConfig => {
                    todo!();
                }
                StateSubobjectType::GlobalRootSignature => {
                    let desc: &GlobalRootSignature = unsafe { transmute(obj.raw.desc) };
                    println!("| {:2}: Global Root Signature: {:?}", i, desc.global_root_signature);
                }
                StateSubobjectType::LocalRootSignature => {
                    let desc: &LocalRootSignature = unsafe { transmute(obj.raw.desc) };
                    println!("| {:2}: Local Root Signature: {:?}", i, desc.local_root_signature);
                }
                StateSubobjectType::NodeMask => {
                    todo!();
                }
                StateSubobjectType::DxilLibrary => {
                    let desc: &DxilLibraryDesc = unsafe { transmute(obj.raw.desc) };
                    println!("| {:2}: DXIL Library: {{ ptr: {:016?}, len: {} }}", i, desc.dxil_library.shader_bytecode as *const c_void, desc.dxil_library.bytecode_length);
                    for j in 0..(desc.num_exports as isize) {
                        let e: *const D3D12ExportDesc = unsafe { transmute(desc.exports) };
                        let e = unsafe { *e.offset(j) };
                        println!("|   - Export {}: {{ Flags: {:?}, Name: \"{}\", Rename: \"{:?}\" }} ", j, e.flags, e.name.to_string(), e.export_to_rename);
                    }
                }
                StateSubobjectType::ExistingCollection => {
                    todo!();
                }
                StateSubobjectType::SubobjectToExportsAssociation => {
                    let desc: &SubobjectToExportsAssociation = unsafe { transmute(obj.raw.desc) };
                    let index = search_index(self.objects.as_slice(), unsafe { transmute(desc.subobject_to_associate) });
                    println!("| {:2}: Subobject To Exports Association: -> {}", i, index);
                    for j in 0..(desc.num_exports as isize) {
                        let e: *const PWStr = unsafe { transmute(desc.exports) };
                        let e = unsafe { *e.offset(j) };
                        println!("|   - Export {}: \"{:?}\"", j, e);
                    }
                }
                StateSubobjectType::DxilSubobjectToExportsAssociation => {
                    todo!();
                }
                StateSubobjectType::RaytracingShaderConfig => {
                    let desc: &RaytracingShaderConfig = unsafe { transmute(obj.raw.desc) };
                    println!("| {:2}: Raytracing Shader Config: {{ payload: {}, attribute: {} }}", i, desc.max_payload_size_in_bytes, desc.max_attribute_size_in_bytes);
                }
                StateSubobjectType::RaytracingPipelineConfig => {
                    let desc: &RaytracingPipelineConfig = unsafe { transmute(obj.raw.desc) };
                    println!("| {:2}: Raytracing Pipeline Config: {{ MaxTraceRecursionDepth: {} }}", i, desc.max_trace_recursion_depth);
                }
                StateSubobjectType::HitGroup => {
                    let desc: &HitGroupDesc = unsafe { transmute(obj.raw.desc) };
                    println!("\
| {:2}: Hit Group: {{
|       HitGroupExport: \"{}\",
|       Type: {:?},
|       AnyHitShaderImport: \"{:?}\",
|       ClosestHitShaderImport: \"{:?}\",
|       IntersectionShaderImport: \"{:?}\" }}",
                             i,
                             desc.hit_group_export,
                             desc.ty,
                             desc.any_hit_shader_import,
                             desc.closest_hit_shader_import,
                             desc.intersection_shader_import);
                }
                StateSubobjectType::RaytracingPipelineConfig1 => {
                    todo!();
                }
                StateSubobjectType::MaxValid => {
                    todo!();
                }
            }
        }
    }
}