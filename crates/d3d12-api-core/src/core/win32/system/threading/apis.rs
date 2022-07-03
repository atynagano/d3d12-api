#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports, dead_code, unused_variables)]

use std::ffi::c_void;
use std::ptr::{NonNull, null};
use std::mem::{size_of_val, transmute};
use crate::helpers::*;
use super::*;
use crate::core::win32::foundation::*;
use crate::core::win32::system::com::*;

use crate::core::win32::foundation::*;
use crate::core::win32::security::*;
use crate::core::win32::system::threading::*;

pub fn WaitForSingleObject(handle: Handle, milliseconds: u32, ) -> (u32) {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn WaitForSingleObject(handle: Handle, milliseconds: u32, ) -> u32;
		}
		let ret = WaitForSingleObject(handle, milliseconds, );
		return (ret);
	}
}

pub fn WaitForSingleObjectEx(handle: Handle, milliseconds: u32, alertable: bool, ) -> (u32) {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn WaitForSingleObjectEx(handle: Handle, milliseconds: u32, alertable: Bool, ) -> u32;
		}
		let ret = WaitForSingleObjectEx(handle, milliseconds, alertable.to_bool(), );
		return (ret);
	}
}

pub fn CreateEventA(event_attributes: Option<&SecurityAttributes>, manual_reset: bool, initial_state: bool, name: Option<&str>, ) -> (Handle) {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn CreateEventA(event_attributes: Option<&SecurityAttributes>, manual_reset: Bool, initial_state: Bool, name: *const u8, ) -> Handle;
		}
		let ret = CreateEventA(event_attributes, manual_reset.to_bool(), initial_state.to_bool(), name.map(str::to_null_terminated).as_ptr_or_null(), );
		return (ret);
	}
}

pub fn OpenEventA(desired_access: u32, inherit_handle: bool, name: &str, ) -> (Handle) {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn OpenEventA(desired_access: u32, inherit_handle: Bool, name: *const u8, ) -> Handle;
		}
		let ret = OpenEventA(desired_access, inherit_handle.to_bool(), name.to_null_terminated().as_ptr_or_null(), );
		return (ret);
	}
}

pub fn CreateEventExA(event_attributes: Option<&SecurityAttributes>, name: Option<&str>, flags: CreateEvent, desired_access: u32, ) -> (Handle) {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn CreateEventExA(event_attributes: Option<&SecurityAttributes>, name: *const u8, flags: CreateEvent, desired_access: u32, ) -> Handle;
		}
		let ret = CreateEventExA(event_attributes, name.map(str::to_null_terminated).as_ptr_or_null(), flags, desired_access, );
		return (ret);
	}
}

pub fn RegisterWaitForSingleObject(object: Handle, callback: WAiTorTimerCallback, context: Option<*const c_void>, milliseconds: u32, flags: WorkerThreadFlags, ) -> (bool, Handle) {
	unsafe {
		#[link(name = "KERNEL32")]
		extern "system" {
			fn RegisterWaitForSingleObject(_ph_new_wait_object: &mut Handle, object: Handle, callback: WAiTorTimerCallback, context: *const c_void, milliseconds: u32, flags: WorkerThreadFlags, ) -> Bool;
		}
		let mut _ph_new_wait_object: Handle = Handle::zeroed();
		let ret = RegisterWaitForSingleObject(&mut _ph_new_wait_object, object, callback, context.as_ptr_or_null(), milliseconds, flags, );
		return (ret.to_bool(), _ph_new_wait_object);
	}
}


pub type WAiTorTimerCallback 
	= unsafe extern "system" fn(param0: &mut (), param1: Boolean, ) -> ();

