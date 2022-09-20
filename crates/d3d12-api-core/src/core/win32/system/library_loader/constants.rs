#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::num::NonZeroUsize;
use std::mem::{MaybeUninit, size_of_val, transmute};
use std::ops::Deref;
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use std::ops::{BitOr, BitOrAssign};

pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 0x100u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 0x200u32;
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 0x400u32;
pub const RESOURCE_ENUM_LN: u32 = 0x1u32;
pub const RESOURCE_ENUM_MUI: u32 = 0x2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 0x4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 0x8u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 0x10u32;
pub const SUPPORT_LANG_NUMBER: u32 = 0x20u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 0x1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 0x2u32;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x4u32;
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 0x1u32;
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 0x8000u32;

