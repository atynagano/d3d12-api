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

pub const DWM_BB_ENABLE: u32 = 0x1u32;
pub const DWM_BB_BLURREGION: u32 = 0x2u32;
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 0x4u32;
pub const DWMWA_COLOR_DEFAULT: u32 = 0xFFFFFFFFu32;
pub const DWMWA_COLOR_NONE: u32 = 0xFFFFFFFEu32;
pub const DWM_CLOAKED_APP: u32 = 0x1u32;
pub const DWM_CLOAKED_SHELL: u32 = 0x2u32;
pub const DWM_CLOAKED_INHERITED: u32 = 0x4u32;
pub const DWM_TNP_RECTDESTINATION: u32 = 0x1u32;
pub const DWM_TNP_RECTSOURCE: u32 = 0x2u32;
pub const DWM_TNP_OPACITY: u32 = 0x4u32;
pub const DWM_TNP_VISIBLE: u32 = 0x8u32;
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 0x10u32;
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -0x1i32;
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0x0u32;
pub const DWM_EC_ENABLECOMPOSITION: u32 = 0x1u32;
pub const DWM_SIT_DISPLAYFRAME: u32 = 0x1u32;
pub const c_DwmMaxQueuedBuffers: u32 = 0x8u32;
pub const c_DwmMaxMonitors: u32 = 0x10u32;
pub const c_DwmMaxAdapters: u32 = 0x10u32;

