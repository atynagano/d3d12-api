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
use crate::core::win32::security::*;
use crate::core::win32::system::threading::*;


pub fn WaitForSingleObject(handle: Handle, milliseconds: u32, ) -> u32 {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn WaitForSingleObject(handle: Handle, milliseconds: u32, ) -> u32;
		}
		let _ret_ = WaitForSingleObject(handle, milliseconds, );
		_ret_
	}
}

pub fn WaitForSingleObjectEx(handle: Handle, milliseconds: u32, alertable: bool, ) -> u32 {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn WaitForSingleObjectEx(handle: Handle, milliseconds: u32, alertable: Bool, ) -> u32;
		}
		let _ret_ = WaitForSingleObjectEx(handle, milliseconds, alertable.to_bool(), );
		_ret_
	}
}

pub fn CreateEventA(event_attributes: Option<&SecurityAttributes>, manual_reset: bool, initial_state: bool, name: Option<&str>, ) -> Option<Handle> {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn CreateEventA(event_attributes: *const c_void, manual_reset: Bool, initial_state: Bool, name: *const u8, ) -> *const c_void;
		}
		let _ret_ = CreateEventA(transmute(event_attributes), manual_reset.to_bool(), initial_state.to_bool(), name.map(str::to_null_terminated).as_ptr_or_null(), );
		transmute(_ret_)
	}
}

pub fn OpenEventA(desired_access: u32, inherit_handle: bool, name: &str, ) -> Option<Handle> {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn OpenEventA(desired_access: u32, inherit_handle: Bool, name: *const u8, ) -> *const c_void;
		}
		let _ret_ = OpenEventA(desired_access, inherit_handle.to_bool(), name.to_null_terminated().as_ptr_or_null(), );
		transmute(_ret_)
	}
}

pub fn CreateEventExA(event_attributes: Option<&SecurityAttributes>, name: Option<&str>, flags: CreateEvent, desired_access: u32, ) -> Option<Handle> {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn CreateEventExA(event_attributes: *const c_void, name: *const u8, flags: CreateEvent, desired_access: u32, ) -> *const c_void;
		}
		let _ret_ = CreateEventExA(transmute(event_attributes), name.map(str::to_null_terminated).as_ptr_or_null(), flags, desired_access, );
		transmute(_ret_)
	}
}

pub fn RegisterWaitForSingleObject(object: Handle, callback: WaitOrTimerCallback, context: *const (), milliseconds: u32, flags: WorkerThreadFlags, ) -> (bool, Option<Handle>, ) {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn RegisterWaitForSingleObject(_out_ph_new_wait_object: *mut c_void, object: Handle, callback: WaitOrTimerCallback, context: *const c_void, milliseconds: u32, flags: WorkerThreadFlags, ) -> Bool;
		}
		let mut _out_ph_new_wait_object: Option<Handle> = None;
		let _ret_ = RegisterWaitForSingleObject(transmute(&mut _out_ph_new_wait_object), object, callback, context as _, milliseconds, flags, );
		(_ret_.to_bool(), _out_ph_new_wait_object, )
	}
}


pub type WaitOrTimerCallback 
	= unsafe extern "system" fn(param0: &mut (), param1: bool, ) -> ();

