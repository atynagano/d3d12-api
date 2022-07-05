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

use crate::core::win32::foundation::*;


pub fn GetModuleHandleA(module_name: Option<&str>, ) -> HInstance {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn GetModuleHandleA(module_name: *const u8, ) -> HInstance;
		}
		let _ret_ = GetModuleHandleA(module_name.map(str::to_null_terminated).as_ptr_or_null(), );
		_ret_
	}
}


