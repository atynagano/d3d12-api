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
use crate::core::win32::security::*;
use crate::core::win32::system::threading::*;


pub fn WaitForSingleObject(handle: Handle, milliseconds: u32) -> u32 {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn WaitForSingleObject(hHandle: Handle, dwMilliseconds: u32) -> u32;
		} 
		let _ret_ = WaitForSingleObject(handle, milliseconds);
		_ret_
	}
}

pub fn WaitForSingleObjectEx(handle: Handle, milliseconds: u32, alertable: bool) -> u32 {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn WaitForSingleObjectEx(hHandle: Handle, dwMilliseconds: u32, bAlertable: Bool) -> u32;
		} 
		let _ret_ = WaitForSingleObjectEx(handle, milliseconds, alertable.to_bool());
		_ret_
	}
}

pub fn CreateEventA(event_attributes: Option<&SecurityAttributes>, manual_reset: bool, initial_state: bool, name: Option<&str>) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn CreateEventA(lpEventAttributes: *const c_void, bManualReset: Bool, bInitialState: Bool, lpName: *const u8) -> *const c_void;
		} 
		let _ret_ = CreateEventA(transmute(event_attributes), manual_reset.to_bool(), initial_state.to_bool(), name.map(str::to_null_terminated).as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn OpenEventA(desired_access: SynchronizationAccessRights, inherit_handle: bool, name: &str) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn OpenEventA(dwDesiredAccess: SynchronizationAccessRights, bInheritHandle: Bool, lpName: *const u8) -> *const c_void;
		} 
		let _ret_ = OpenEventA(desired_access, inherit_handle.to_bool(), name.to_null_terminated().as_ptr_or_null());
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub fn CreateEventExA(event_attributes: Option<&SecurityAttributes>, name: Option<&str>, flags: CreateEvent, desired_access: u32) -> Result<Handle, Win32Error> {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn CreateEventExA(lpEventAttributes: *const c_void, lpName: *const u8, dwFlags: CreateEvent, dwDesiredAccess: u32) -> *const c_void;
		} 
		let _ret_ = CreateEventExA(transmute(event_attributes), name.map(str::to_null_terminated).as_ptr_or_null(), flags, desired_access);
		if _ret_.is_null() { Err(GetLastError()) } else { Ok(transmute(_ret_)) }
	}
}

pub unsafe fn RegisterWaitForSingleObject() { todo!() }


pub type WaitOrTimerCallback 
	= unsafe extern "system" fn(param0: &mut (), param1: bool, ) -> ();

