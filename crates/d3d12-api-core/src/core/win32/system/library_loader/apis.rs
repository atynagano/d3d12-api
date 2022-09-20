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


pub fn GetModuleHandleA(module_name: Option<&str>) -> Result<HInstance, Win32Error> {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn GetModuleHandleA(lpModuleName: *const u8) -> *const c_void;
		} 
		let _ret_ = GetModuleHandleA(module_name.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}


