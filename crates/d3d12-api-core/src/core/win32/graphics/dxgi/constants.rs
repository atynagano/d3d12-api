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

pub const DXGI_MAP_READ: u32 = 0x1u32;
pub const DXGI_MAP_WRITE: u32 = 0x2u32;
pub const DXGI_MAP_DISCARD: u32 = 0x4u32;
pub const DXGI_ENUM_MODES_INTERLACED: u32 = 0x1u32;
pub const DXGI_ENUM_MODES_SCALING: u32 = 0x2u32;
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 0x10u32;
pub const DXGI_ENUM_MODES_STEREO: u32 = 0x4u32;
pub const DXGI_ENUM_MODES_DISABLED_STEREO: u32 = 0x8u32;
pub const DXGI_SHARED_RESOURCE_READ: u32 = 0x80000000u32;
pub const DXGI_SHARED_RESOURCE_WRITE: u32 = 0x1u32;
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 0x1u32;
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0x0u32;
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 0x400u32;
pub const DXGI_CREATE_FACTORY_DEBUG: u32 = 0x1u32;
pub const DXGI_DEBUG_ALL: GUID = GUID::from_u128(0xe48ae283da80490b87e643e9a9cfda08u128);
pub const DXGI_DEBUG_DX: GUID = GUID::from_u128(0x35cdd7fc13b2421da5d77e4451287d64u128);
pub const DXGI_DEBUG_DXGI: GUID = GUID::from_u128(0x25cddaa4b1c647e1ac3e98875b5a2e2au128);
pub const DXGI_DEBUG_APP: GUID = GUID::from_u128(0x06cd6e0142194ebd870927ed23360c62u128);

