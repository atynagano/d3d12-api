#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use std::ops::{BitOr, BitOrAssign};
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 0x1;
pub const DxcValidatorFlags_Default: u32 = 0x0;
pub const DxcValidatorFlags_InPlaceEdit: u32 = 0x1;
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 0x2;
pub const DxcValidatorFlags_ModuleOnly: u32 = 0x4;
pub const DxcValidatorFlags_ValidMask: u32 = 0x7;
pub const DxcVersionInfoFlags_None: u32 = 0x0;
pub const DxcVersionInfoFlags_Debug: u32 = 0x1;
pub const DxcVersionInfoFlags_Internal: u32 = 0x2;
pub const CLSID_DxcCompiler: GUID = GUID::from_u128(0x73e22d93e6ce47f3b5bff0664f39c1b0u128);
pub const CLSID_DxcLinker: GUID = GUID::from_u128(0xef6a8087b0ea4d569e45d07e1a8b7806u128);
pub const CLSID_DxcDiaDataSource: GUID = GUID::from_u128(0xcd1f6b732ab0484d8edcebe7a43ca09fu128);
pub const CLSID_DxcCompilerArgs: GUID = GUID::from_u128(0x3e56ae82224d470fa1a1fe3016ee9f9du128);
pub const CLSID_DxcLibrary: GUID = GUID::from_u128(0x6245d6af66e048fd80b44d271796748cu128);
pub const CLSID_DxcValidator: GUID = GUID::from_u128(0x8ca3e215f7284cf38cdd88af917587a1u128);
pub const CLSID_DxcAssembler: GUID = GUID::from_u128(0xd728db68f9034f8094cddccf76ec7151u128);
pub const CLSID_DxcContainerReflection: GUID = GUID::from_u128(0xb9f5448955b8400cba3a1675e4728b91u128);
pub const CLSID_DxcOptimizer: GUID = GUID::from_u128(0xae2cd79fcc22453f9b6bb124e7a5204cu128);
pub const CLSID_DxcContainerBuilder: GUID = GUID::from_u128(0x94134294411f4574b4d08741e25240d2u128);
pub const CLSID_DxcPdbUtils: GUID = GUID::from_u128(0x54621dfbf2ce457eae8cec355faeec7cu128);

