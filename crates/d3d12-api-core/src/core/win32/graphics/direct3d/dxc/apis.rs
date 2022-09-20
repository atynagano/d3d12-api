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

use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;



pub type DxcCreateInstanceProc 
	= unsafe extern "system" fn(rclsid: &GUID, riid: &GUID, ppv: &mut &(), ) -> HResult;

pub type DxcCreateInstance2Proc 
	= unsafe extern "system" fn(malloc: Malloc, rclsid: &GUID, riid: &GUID, ppv: &mut &(), ) -> HResult;

