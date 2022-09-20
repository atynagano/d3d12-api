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


pub fn CloseHandle(object: Handle) -> bool {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn CloseHandle(hObject: Handle) -> Bool;
		} 
		let _ret_ = CloseHandle(object);
		_ret_.to_bool()
	}
}

pub fn GetLastError() -> Win32Error {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn GetLastError() -> Win32Error;
		} 
		let _ret_ = GetLastError();
		_ret_
	}
}

pub fn SetLastError(err_code: Win32Error) -> () {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn SetLastError(dwErrCode: Win32Error) -> ();
		} 
		let _ret_ = SetLastError(err_code);
	}
}

pub fn SetLastErrorEx(err_code: Win32Error, r#type: u32) -> () {
	unsafe {
		#[link(name = "USER32")]
		extern "system" {
			fn SetLastErrorEx(dwErrCode: Win32Error, dwType: u32) -> ();
		} 
		let _ret_ = SetLastErrorEx(err_code, r#type);
	}
}


