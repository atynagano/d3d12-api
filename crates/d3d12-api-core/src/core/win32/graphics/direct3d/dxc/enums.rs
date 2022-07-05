#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxcCp
{
	ACp                  = 0x0u32,
	Utf16                = 0x4B0u32,
	Utf8                 = 0xFDE9u32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DxcOutKind
{
	OutNone              = 0x0i32,
	OutObject            = 0x1i32,
	OutErrors            = 0x2i32,
	OutPdb               = 0x3i32,
	OutShaderHash        = 0x4i32,
	OutDisassembly       = 0x5i32,
	OutHlSl              = 0x6i32,
	OutText              = 0x7i32,
	OutReflection        = 0x8i32,
	OutRootSignature     = 0x9i32,
	OutExtraOutputs      = 0xAi32,
	OutForceDWord        = -0x1i32,
}

