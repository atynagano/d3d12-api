#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables, unused_unsafe)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{MaybeUninit, size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use std::ops::{BitOr, BitOrAssign};

pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 0x100;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 0x200;
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 0x400;
pub const RESOURCE_ENUM_LN: u32 = 0x1;
pub const RESOURCE_ENUM_MUI: u32 = 0x2;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 0x4;
pub const RESOURCE_ENUM_VALIDATE: u32 = 0x8;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 0x10;
pub const SUPPORT_LANG_NUMBER: u32 = 0x20;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 0x1;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 0x2;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x4;
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 0x1;
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 0x8000;

