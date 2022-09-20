#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::mem::transmute;
use std::ops::{BitOr, BitOrAssign};

/// DXC_CP
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxcCp
{
	/// DXC_CP_ACP = 0x0u32
	Acp                  = 0x0u32,
	/// DXC_CP_UTF16 = 0x4B0u32
	Utf16                = 0x4B0u32,
	/// DXC_CP_UTF8 = 0xFDE9u32
	Utf8                 = 0xFDE9u32,
}

/// DXC_OUT_KIND
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DxcOutKind
{
	/// DXC_OUT_NONE = 0x0i32
	None                 = 0x0i32,
	/// DXC_OUT_OBJECT = 0x1i32
	Object               = 0x1i32,
	/// DXC_OUT_ERRORS = 0x2i32
	Errors               = 0x2i32,
	/// DXC_OUT_PDB = 0x3i32
	Pdb                  = 0x3i32,
	/// DXC_OUT_SHADER_HASH = 0x4i32
	ShaderHash           = 0x4i32,
	/// DXC_OUT_DISASSEMBLY = 0x5i32
	Disassembly          = 0x5i32,
	/// DXC_OUT_HLSL = 0x6i32
	HLSL                 = 0x6i32,
	/// DXC_OUT_TEXT = 0x7i32
	Text                 = 0x7i32,
	/// DXC_OUT_REFLECTION = 0x8i32
	Reflection           = 0x8i32,
	/// DXC_OUT_ROOT_SIGNATURE = 0x9i32
	RootSignature        = 0x9i32,
	/// DXC_OUT_EXTRA_OUTPUTS = 0xAi32
	ExtraOutputs         = 0xAi32,
	/// DXC_OUT_FORCE_DWORD = -0x1i32
	ForceDword           = -0x1i32,
}

